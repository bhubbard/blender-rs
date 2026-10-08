#!/usr/bin/env python3
"""
scripts/parallel_supervisor.py — 4-Worker Local Parallel Autonomous Transpile Supervisor
========================================================================================
Maximizes migration throughput on Apple Silicon with 4 concurrent local workers:
- Worker 1..4: Concurrent on-device neural transpile via `apfel-transpile`
- Instantaneous AST fast-path across headers
- Thread-safe serialized compilation verification (keeps trunk 100% green)
- Live WATCHDOG_STATUS.md heartbeat reporting
- Autonomous git commit & push discipline
"""

import os
import sys
import json
import time
import fcntl
import signal
import threading
import subprocess
from pathlib import Path
from datetime import datetime
from concurrent.futures import ThreadPoolExecutor, as_completed

ROOT_DIR = Path(__file__).resolve().parent.parent
TARGET_DIR = ROOT_DIR / "target"
LOCK_FILE = TARGET_DIR / "parallel_supervisor.lock"
STATUS_FILE = ROOT_DIR / "WATCHDOG_STATUS.md"
PROGRESS_FILE = ROOT_DIR / "PORTING_PROGRESS.json"
CRASH_LOG = ROOT_DIR / "telemetry" / "supervisor_crashes.log"

NUM_WORKERS = 4
SUBSYSTEMS = [
    "makesdna",
    "blenlib",
    "bmesh",
    "geometry",
    "blenkernel",
    "animrig",
    "nodes",
    "imbuf",
    "depsgraph",
    "compositor",
    "makesrna",
    "io",
    "render",
    "draw",
    "windowmanager",
]

CARGO_LOCK = threading.Lock()
PROGRESS_LOCK = threading.Lock()
ACTIVE_WORKERS = {i: "Idle" for i in range(1, NUM_WORKERS + 1)}

def route_file(rel_str: str) -> str:
    if "bmesh" in rel_str:
        return "blender-bmesh"
    elif "makesdna" in rel_str or "dna" in rel_str:
        return "blender-dna"
    elif "io" in rel_str:
        return "blender-io"
    elif "math" in rel_str or "vec" in rel_str:
        return "blender-math"
    elif "mem" in rel_str or "alloc" in rel_str:
        return "blender-mem"
    return "blender-io"

def acquire_lock(lock_fd):
    try:
        fcntl.flock(lock_fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
        return True
    except (IOError, BlockingIOError):
        return False

def read_progress() -> dict:
    if PROGRESS_FILE.exists():
        try:
            with open(PROGRESS_FILE, "r") as f:
                return json.load(f)
        except Exception:
            pass
    return {"completed": {}, "failed": {}, "skipped": {}}

def write_progress(prog: dict):
    with open(PROGRESS_FILE, "w") as f:
        json.dump(prog, f, indent=2)

def update_status_file(cycle: int, active_subsys: str, status_msg: str):
    with PROGRESS_LOCK:
        prog = read_progress()
        completed = len(prog.get("completed", {}))
        skipped = len(prog.get("skipped", {}))
        failed = len(prog.get("failed", {}))

    worker_lines = "\n".join([f"  - **Worker {w}:** `{status}`" for w, status in ACTIVE_WORKERS.items()])
    now_str = datetime.now().strftime("%Y-%m-%d %H:%M:%S")

    content = f"""# 🛡️ Blender-rs 4-Worker Parallel Supervisor Status

**Last Heartbeat:** `{now_str}`  
**Supervisor Status:** `{status_msg}`  
**Current Subsystem:** `{active_subsys}` (Cycle #{cycle})  
**Active Concurrency:** `{NUM_WORKERS} Local Apple Silicon Workers`  

---

### Active Worker Pool
{worker_lines}

---

### Conversion Progress
- **Completed Modules:** `{completed}`
- **Skipped / Filtered:** `{skipped}`
- **Needs Burndown / Failed:** `{failed}`

---

### Operational Guardrails
- **Trunk Green Guarantee:** Serialized verification mutex active
- **Concurrency Lock:** Active (`target/parallel_supervisor.lock`)
- **Fast-Path Engine:** Sub-50ms deterministic AST parsing enabled
- **Neural Engine:** apfel-transpile (Apple Intelligence / Metal ANE)
"""
    try:
        STATUS_FILE.write_text(content)
    except Exception:
        pass

def register_module(target_crate: str, module_name: str) -> None:
    lib_rs = ROOT_DIR / "crates" / target_crate / "src" / "lib.rs"
    if not lib_rs.exists():
        return
    content = lib_rs.read_text()
    mod_stmt = f"pub mod {module_name};"
    use_stmt = f"pub use {module_name}::*;"
    if mod_stmt not in content:
        updated = f"{content.rstrip()}\n{mod_stmt}\n{use_stmt}\n"
        lib_rs.write_text(updated)

def unregister_module(target_crate: str, module_name: str) -> None:
    lib_rs = ROOT_DIR / "crates" / target_crate / "src" / "lib.rs"
    if not lib_rs.exists():
        return
    content = lib_rs.read_text()
    mod_stmt = f"pub mod {module_name};"
    use_stmt = f"pub use {module_name}::*;"
    updated = content.replace(mod_stmt, "").replace(use_stmt, "")
    lib_rs.write_text(updated)

def verify_and_commit(target_crate: str, module_name: str, dest_file: Path, source_rel: str) -> bool:
    """Acquires lock, checks crate compilation, commits on success or rolls back."""
    with CARGO_LOCK:
        register_module(target_crate, module_name)
        res = subprocess.run(["cargo", "check", "-p", target_crate], cwd=str(ROOT_DIR), capture_output=True, text=True)
        if res.returncode == 0:
            # Commit to git
            lib_rs = ROOT_DIR / "crates" / target_crate / "src" / "lib.rs"
            subprocess.run(["git", "add", str(dest_file), str(lib_rs)], cwd=str(ROOT_DIR), capture_output=True)
            msg = f"port({target_crate}): port {module_name} via 4-worker supervisor ({source_rel})"
            subprocess.run(["git", "commit", "-m", msg], cwd=str(ROOT_DIR), capture_output=True)
            return True
        else:
            # Rollback
            unregister_module(target_crate, module_name)
            if dest_file.exists():
                try:
                    actual_name = dest_file.resolve().name
                    if actual_name == f"{module_name}.rs":
                        dest_file.unlink()
                except Exception:
                    pass
            return False

def process_file_worker(worker_id: int, file_path: Path, cycle: int) -> bool:
    rel_str = str(file_path.relative_to(ROOT_DIR))
    stem = file_path.stem.replace(".", "_")
    dest_crate = route_file(rel_str)
    dest_file = ROOT_DIR / "crates" / dest_crate / "src" / f"{stem}.rs"

    ACTIVE_WORKERS[worker_id] = f"Transpiling {file_path.name} → {dest_crate}"
    t0 = time.time()

    is_header = file_path.suffix in [".h", ".hh"]
    cmd = [
        "apfel-transpile",
        "-f", str(file_path),
        "-t", dest_crate,
        "-m", stem,
        "-o", str(dest_file),
    ]
    if is_header:
        cmd.append("--fast-only")

    try:
        proc = subprocess.run(cmd, cwd=str(ROOT_DIR), capture_output=True, text=True, timeout=180)
        if proc.returncode != 0 or not dest_file.exists():
            ACTIVE_WORKERS[worker_id] = f"Failed {file_path.name}"
            with PROGRESS_LOCK:
                prog = read_progress()
                prog["failed"][rel_str] = "transpile_error"
                write_progress(prog)
            return False

        # Verification step
        ACTIVE_WORKERS[worker_id] = f"Verifying {stem}.rs in {dest_crate}"
        success = verify_and_commit(dest_crate, stem, dest_file, rel_str)
        dur = time.time() - t0

        with PROGRESS_LOCK:
            prog = read_progress()
            if success:
                prog["completed"][rel_str] = {
                    "crate": dest_crate,
                    "module": f"{stem}.rs",
                    "mode": "parallel_worker",
                    "worker": worker_id,
                    "time_s": round(dur, 2)
                }
                if rel_str in prog.get("failed", {}):
                    del prog["failed"][rel_str]
                write_progress(prog)
                print(f"[Worker {worker_id}] ✓ {file_path.name} committed to {dest_crate} in {dur:.1f}s")
            else:
                prog["failed"][rel_str] = "compile_check_failed"
                write_progress(prog)
                print(f"[Worker {worker_id}] ✗ {file_path.name} compilation check failed (rolled back)")

        ACTIVE_WORKERS[worker_id] = "Idle"
        return success
    except subprocess.TimeoutExpired:
        ACTIVE_WORKERS[worker_id] = f"Timeout {file_path.name}"
        with PROGRESS_LOCK:
            prog = read_progress()
            prog["failed"][rel_str] = "timeout"
            write_progress(prog)
        if dest_file.exists():
            try:
                dest_file.unlink()
            except Exception:
                pass
        return False
    except Exception as e:
        ACTIVE_WORKERS[worker_id] = f"Exception: {e}"
        return False

def run_subsystem_parallel(subsystem: str, cycle: int, batch_limit: int = 30):
    subsys_dir = ROOT_DIR / "upstream" / "source" / "blender" / subsystem
    if not subsys_dir.exists():
        return

    with PROGRESS_LOCK:
        prog = read_progress()
        completed_set = set(prog.get("completed", {}).keys())
        skipped_set = set(prog.get("skipped", {}).keys())

    target_files = []
    for root, _, files in os.walk(subsys_dir):
        for f in files:
            if subsystem == "makesdna" and not f.endswith((".h", ".hh")):
                continue
            if f.endswith((".h", ".hh", ".c", ".cc")) and not f.startswith("."):
                full_path = Path(root) / f
                rel_str = str(full_path.relative_to(ROOT_DIR))
                if rel_str not in completed_set and rel_str not in skipped_set:
                    target_files.append(full_path)

    # Sort headers first (for maximum speed), then implementations
    target_files.sort(key=lambda p: (0 if p.suffix in [".h", ".hh"] else 1, str(p)))
    target_files = target_files[:batch_limit]

    if not target_files:
        return

    print(f"\n======================================================================")
    print(f"[*] Subsystem: {subsystem} | Queue: {len(target_files)} files | Concurrency: {NUM_WORKERS}")
    print(f"======================================================================")

    update_status_file(cycle, subsystem, f"Processing {len(target_files)} files")

    with ThreadPoolExecutor(max_workers=NUM_WORKERS) as executor:
        futures = {}
        for idx, fpath in enumerate(target_files):
            worker_id = (idx % NUM_WORKERS) + 1
            f = executor.submit(process_file_worker, worker_id, fpath, cycle)
            futures[f] = fpath

        for f in as_completed(futures):
            fpath = futures[f]
            try:
                f.result()
            except Exception as e:
                print(f"[!] Error processing {fpath.name}: {e}")

def main():
    TARGET_DIR.mkdir(parents=True, exist_ok=True)
    CRASH_LOG.parent.mkdir(parents=True, exist_ok=True)

    lock_file = open(LOCK_FILE, "w")
    if not acquire_lock(lock_file):
        print(f"[!] Another parallel supervisor is already running on {LOCK_FILE}. Exiting.")
        sys.exit(0)

    print("======================================================================")
    print(" ⚡ Blender-rs 4-Worker Local Parallel Supervisor Started")
    print(f" PID: {os.getpid()} | Concurrency: {NUM_WORKERS} Workers | Apple Silicon ANE")
    print("======================================================================")

    cycle = 1
    commit_push_counter = 0

    while True:
        for subsys in SUBSYSTEMS:
            run_subsystem_parallel(subsys, cycle, batch_limit=32)
            update_status_file(cycle, subsys, "Cycle Active")

            # Periodic git push every 10 commits
            try:
                subprocess.run(["git", "push", "origin", "main"], cwd=str(ROOT_DIR), capture_output=True)
            except Exception:
                pass

        cycle += 1
        time.sleep(2)

if __name__ == "__main__":
    main()
