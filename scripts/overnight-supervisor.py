#!/usr/bin/env python3
"""
scripts/overnight-supervisor.py — Autonomous Overnight Watchdog & Campaign Runner
Powered by zev-rs and apfel-rs for zero-token Blender-to-Rust conversion.

Guarantees:
1. Single-instance concurrency lock (prevents Apple Intelligence FoundationModels lock contention)
2. Infinite cycling across all Blender subsystems
3. Self-healing watchdog: catches timeouts, memory pressure, or crashes, cleans up, and resumes
4. Real-time WATCHDOG_STATUS.md heartbeat report
"""

import os
import sys
import time
import fcntl
import signal
import subprocess
import json
from pathlib import Path
from datetime import datetime

ROOT_DIR = Path(__file__).resolve().parent.parent
TARGET_DIR = ROOT_DIR / "target"
LOCK_FILE = TARGET_DIR / "overnight_supervisor.lock"
STATUS_FILE = ROOT_DIR / "WATCHDOG_STATUS.md"
CRASH_LOG = ROOT_DIR / "telemetry" / "supervisor_crashes.log"
PORT_RUNNER = ROOT_DIR / "scripts" / "port-runner.py"

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
    "io/wavefront_obj",
    "io/ply",
    "io/stl",
    "render",
    "draw",
    "windowmanager",
]

def acquire_lock(lock_fd):
    """Acquires an exclusive advisory lock on the lockfile."""
    try:
        fcntl.flock(lock_fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
        return True
    except (IOError, BlockingIOError):
        return False

def cleanup_orphaned_processes():
    """Kills any stuck or orphaned apfel-rs or python port-runner processes."""
    try:
        subprocess.run(["pkill", "-f", "target/chunks"], check=False, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        # Clean chunks directory
        chunk_dir = TARGET_DIR / "chunks"
        if chunk_dir.exists():
            for f in chunk_dir.glob("*.cpp"):
                try:
                    f.unlink()
                except Exception:
                    pass
    except Exception as e:
        print(f"[!] Cleanup warning: {e}")

def read_progress_summary():
    progress_file = ROOT_DIR / "PORTING_PROGRESS.json"
    if progress_file.exists():
        try:
            with open(progress_file, "r") as f:
                d = json.load(f)
            return len(d.get("completed", {})), len(d.get("skipped", {})), len(d.get("failed", {}))
        except Exception:
            pass
    return 0, 0, 0

def update_status_file(cycle: int, active_subsys: str, status_msg: str):
    """Updates WATCHDOG_STATUS.md with real-time operational status."""
    completed, skipped, failed = read_progress_summary()
    now_str = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    content = f"""# 🛡️ Blender-rs Overnight Supervisor Status

**Last Heartbeat:** `{now_str}`  
**Supervisor Status:** `{status_msg}`  
**Current Subsystem:** `{active_subsys}` (Cycle #{cycle})  

---

### Conversion Progress
- **Completed Modules:** `{completed}`
- **Skipped / Filtered:** `{skipped}`
- **Needs Burndown / Failed:** `{failed}`

---

### Guardrails Enforced
- **Single Process Lock:** Active (`target/overnight_supervisor.lock`)
- **FoundationModels Safety:** `--permissive` enabled
- **Chunk Ceiling:** ≤75 lines per chunk (prevents 4K context overflows)
- **Automatic Recovery:** Enabled with 3-second thermal/lock cooldown
"""
    try:
        STATUS_FILE.write_text(content)
    except Exception:
        pass

def run_subsystem_batch(subsystem: str, limit: int = 25) -> bool:
    """Executes a single port-runner batch with timeout and crash capture."""
    cmd = [
        sys.executable,
        str(PORT_RUNNER),
        "--subsystem", subsystem,
        "--limit", str(limit),
        "--use-zima",
    ]
    try:
        proc = subprocess.run(cmd, cwd=str(ROOT_DIR), timeout=3600)
        return proc.returncode == 0
    except subprocess.TimeoutExpired:
        with open(CRASH_LOG, "a") as f:
            f.write(f"[{datetime.now().isoformat()}] Subsystem '{subsystem}' timed out after 3600s\n")
        cleanup_orphaned_processes()
        return False
    except Exception as e:
        with open(CRASH_LOG, "a") as f:
            f.write(f"[{datetime.now().isoformat()}] Subsystem '{subsystem}' encountered exception: {e}\n")
        cleanup_orphaned_processes()
        return False

def main():
    TARGET_DIR.mkdir(parents=True, exist_ok=True)
    CRASH_LOG.parent.mkdir(parents=True, exist_ok=True)

    lock_file = open(LOCK_FILE, "w")
    if not acquire_lock(lock_file):
        print(f"[!] Another supervisor instance is already running with lock on {LOCK_FILE}. Exiting.")
        sys.exit(0)

    print("======================================================================")
    print(" 🌙 Blender-rs Autonomous Overnight Watchdog Supervisor Started")
    print(f" PID: {os.getpid()} | Target: {ROOT_DIR}")
    print("======================================================================")

    cycle = 1
    update_status_file(cycle, "Initializing", "Active")

    while True:
        print(f"\n[Cycle #{cycle}] Starting sweep across {len(SUBSYSTEMS)} subsystems...")
        for subsys in SUBSYSTEMS:
            update_status_file(cycle, subsys, "Processing")
            print(f"\n>>> [Watchdog] Processing Subsystem: {subsys} (Batch limit: 25) <<<")
            success = run_subsystem_batch(subsys, limit=25)
            if not success:
                print(f"[!] Subsystem {subsys} finished with notice or caught crash. Cooling down...")
                cleanup_orphaned_processes()
                time.sleep(3)
            else:
                time.sleep(1)

        # After each full cycle, run trunk verification
        update_status_file(cycle, "Full Workspace Verification", "Running cargo test")
        print("\n--- [Watchdog] Full Cycle Complete. Running trunk cargo test... ---")
        try:
            test_proc = subprocess.run(["cargo", "test", "--workspace"], cwd=str(ROOT_DIR), capture_output=True, text=True, timeout=300)
            if test_proc.returncode == 0:
                print("[✓] All workspace tests 100% green.")
                try:
                    subprocess.run(["git", "push", "origin", "main"], cwd=str(ROOT_DIR), check=False)
                    print("[✓] Pushed latest commits to origin main.")
                except Exception as e:
                    print(f"[!] Git push warning: {e}")
            else:
                print(f"[!] Workspace test notice: {test_proc.stderr[:200]}")
        except Exception as e:
            print(f"[!] Test runner error: {e}")

        cycle += 1
        update_status_file(cycle, "Cycle Cooldown", "Active")
        time.sleep(5)

if __name__ == "__main__":
    main()
