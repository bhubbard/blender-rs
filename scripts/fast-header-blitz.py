#!/usr/bin/env python3
"""
scripts/fast-header-blitz.py — Multi-Core Parallel Fast-Path Header Transpilation Engine
========================================================================================
Leverages deterministic AST fast-path parsing via `apfel-transpile --fast-only` across
all Apple Silicon CPU cores in parallel. Converts hundreds of C/C++ headers into Safe Rust
in minutes without relying on slow neural model token limits.
"""

import os
import sys
import json
import time
import subprocess
from pathlib import Path
from typing import List, Tuple, Optional
from concurrent.futures import ProcessPoolExecutor, as_completed

ROOT_DIR = Path(__file__).resolve().parent.parent
UPSTREAM_DIR = ROOT_DIR / "upstream" / "source" / "blender"
PROGRESS_FILE = ROOT_DIR / "PORTING_PROGRESS.json"
CRATES_DIR = ROOT_DIR / "crates"

def load_progress() -> dict:
    if PROGRESS_FILE.exists():
        with open(PROGRESS_FILE, "r") as f:
            return json.load(f)
    return {"completed": {}, "skipped": {}, "failed": {}}

def save_progress(progress: dict):
    with open(PROGRESS_FILE, "w") as f:
        json.dump(progress, f, indent=2)

def transpile_worker(header_path_str: str, target_crate: str, module_name: str) -> Tuple[str, bool, str]:
    """Worker process: runs apfel-transpile --fast-only on a single header."""
    cmd = [
        "apfel-transpile",
        "--file", header_path_str,
        "--target-crate", target_crate,
        "--module-name", module_name,
        "--fast-only"
    ]
    try:
        proc = subprocess.run(cmd, capture_output=True, text=True, timeout=15)
        if proc.returncode == 0 and len(proc.stdout.strip()) > 50:
            return header_path_str, True, proc.stdout
        return header_path_str, False, proc.stderr
    except Exception as e:
        return header_path_str, False, str(e)

def register_module(target_crate: str, module_name: str):
    lib_rs = CRATES_DIR / target_crate / "src" / "lib.rs"
    decl = f"pub mod {module_name};\n"
    if lib_rs.exists():
        content = lib_rs.read_text()
        if f"pub mod {module_name};" not in content:
            with open(lib_rs, "a") as f:
                f.write(f"\n{decl}")

def unregister_module(target_crate: str, module_name: str):
    lib_rs = CRATES_DIR / target_crate / "src" / "lib.rs"
    target_line = f"pub mod {module_name};"
    if lib_rs.exists():
        lines = lib_rs.read_text().splitlines()
        filtered = [l for l in lines if l.strip() != target_line]
        lib_rs.write_text("\n".join(filtered) + "\n")

def check_crate_compilation(target_crate: str) -> bool:
    res = subprocess.run(["cargo", "check", "-p", target_crate], cwd=str(ROOT_DIR), capture_output=True, text=True)
    return res.returncode == 0

def run_blitz(subsystem: str = "makesdna", target_crate: str = "blender-dna", max_workers: Optional[int] = None):
    subsystem_dir = UPSTREAM_DIR / subsystem
    if not subsystem_dir.exists():
        print(f"[!] Subsystem directory {subsystem_dir} not found.")
        return

    progress = load_progress()
    headers = []
    for ext in ("*.h", "*.hh"):
        headers.extend(subsystem_dir.rglob(ext))

    headers = sorted(headers)
    candidates = []
    for h in headers:
        rel = str(h.relative_to(ROOT_DIR))
        if rel in progress["completed"] or rel in progress["skipped"]:
            continue
        candidates.append(h)

    print(f"\n⚡ Fast-Header Blitz: {subsystem} → {target_crate}")
    print(f"   Found {len(candidates)} unported headers to evaluate across {max_workers or os.cpu_count()} CPU workers...")

    # Phase 1: Parallel AST Transpilation
    t0 = time.time()
    generated = {}
    with ProcessPoolExecutor(max_workers=max_workers) as executor:
        futures = {
            executor.submit(
                transpile_worker,
                str(h),
                target_crate,
                h.stem.replace(".", "_")
            ): h for h in candidates
        }
        for fut in as_completed(futures):
            h_path, ok, output = fut.result()
            if ok:
                h = Path(h_path)
                mod_name = h.stem.replace(".", "_")
                generated[h] = (mod_name, output)

    dur = time.time() - t0
    print(f"   AST parsing complete in {dur:.2f}s! Successfully generated {len(generated)} / {len(candidates)} modules.")

    # Phase 2: Sequential verification & landing
    landed = 0
    failed = 0

    for h, (mod_name, rust_code) in generated.items():
        rel = str(h.relative_to(ROOT_DIR))
        target_file = CRATES_DIR / target_crate / "src" / f"{mod_name}.rs"

        target_file.write_text(rust_code + "\n")
        register_module(target_crate, mod_name)

        if check_crate_compilation(target_crate):
            subprocess.run(["git", "add", str(target_file)], cwd=str(ROOT_DIR), check=True)
            lib_rs = CRATES_DIR / target_crate / "src" / "lib.rs"
            subprocess.run(["git", "add", str(lib_rs)], cwd=str(ROOT_DIR), check=True)
            msg = f"port({target_crate}): fast-path AST port {mod_name} from {rel}"
            subprocess.run(["git", "commit", "-m", msg], cwd=str(ROOT_DIR), check=True)

            progress["completed"][rel] = {
                "crate": target_crate,
                "module": f"{mod_name}.rs"
            }
            progress.get("failed", {}).pop(rel, None)
            save_progress(progress)
            landed += 1
            print(f"  [✓] Landed {mod_name} ({landed} green)")
        else:
            unregister_module(target_crate, mod_name)
            if target_file.exists():
                target_file.unlink()
            progress["failed"][rel] = "fast_path_type_resolution"
            save_progress(progress)
            failed += 1
            print(f"  [-] Deferred {mod_name} (needs downstream dependencies)")

    print(f"\n🎉 Blitz Batch Finished!")
    print(f"   Landed:   {landed} modules")
    print(f"   Deferred: {failed} modules")
    print(f"   Completed Total: {len(progress['completed'])}")

if __name__ == "__main__":
    subsystem = sys.argv[1] if len(sys.argv) > 1 else "makesdna"
    crate = sys.argv[2] if len(sys.argv) > 2 else "blender-dna"
    run_blitz(subsystem, crate)
