#!/usr/bin/env python3
"""
blender-rs Autonomous Zero-Token Port Runner
============================================
Orchestrates file-by-file translation from Blender C/C++ to Safe Rust:
- Zero-token classification & routing via `zev-rs`
- Zero-cloud-token on-device code generation via `apfel-rs` (Apple Intelligence FoundationModels)
- Guided by `port-to-rust-playbook`, `rust-crate-decomposition`, and `rust-lifetimes-analysis`
- Compiler error burndown via `compiler-errors-as-work-queue`
"""

import os
import sys
import json
import subprocess
import argparse
from pathlib import Path
from typing import Optional, Dict, List, Tuple

# Base paths
SCRIPT_DIR = Path(__file__).resolve().parent
ROOT_DIR = SCRIPT_DIR.parent
UPSTREAM_DIR = ROOT_DIR / "upstream" / "source" / "blender"
PROGRESS_FILE = ROOT_DIR / "PORTING_PROGRESS.json"
LIFETIMES_FILE = ROOT_DIR / "LIFETIMES.tsv"
PORTING_GUIDE = ROOT_DIR / "PORTING.md"

# Executable paths
ZEV_BIN = Path(os.environ.get("ZEV_BIN", "/Users/bhubbard/PROJECTS/zev-rs/target/release/zev"))
APFEL_BIN = Path(os.environ.get("APFEL_BIN", "/opt/homebrew/bin/apfel"))

ROUTES = {
    "blender-math": "Vectors, matrices, quaternions, bounding boxes, coordinate math, BLI_math",
    "blender-mem": "Chunked memory pools, allocators, BLI_mempool, MEM_guardedalloc",
    "blender-dna": "DNA struct definitions, mesh headers, serialized schemas, makesdna",
    "blender-bmesh": "BMesh half-edge data structures, BMVert, BMEdge, BMLoop, BMFace, Euler operators",
    "blender-io": "File import and export, Wavefront OBJ, PLY, STL, USD",
    "skip": "CMakeLists, build scripts, tests, private platform glue, or non-portable code"
}

def load_progress() -> Dict:
    if PROGRESS_FILE.exists():
        try:
            with open(PROGRESS_FILE, "r") as f:
                return json.load(f)
        except Exception:
            pass
    return {"completed": {}, "skipped": {}, "failed": {}}

def save_progress(progress: Dict):
    with open(PROGRESS_FILE, "w") as f:
        json.dump(progress, f, indent=2)

def load_lifetimes() -> Dict[str, List[Dict[str, str]]]:
    """Loads LIFETIMES.tsv mappings indexed by struct name."""
    lifetimes = {}
    if LIFETIMES_FILE.exists():
        with open(LIFETIMES_FILE, "r") as f:
            for line in f:
                parts = line.strip().split("\t")
                if len(parts) >= 6 and parts[0] != "file":
                    struct_name = parts[1]
                    field_info = {
                        "field": parts[2],
                        "model": parts[3],
                        "lifetime": parts[4],
                        "notes": parts[5]
                    }
                    lifetimes.setdefault(struct_name, []).append(field_info)
    return lifetimes

def route_file_with_zev(file_path: Path) -> Tuple[str, float]:
    """Uses zev route to classify file destination in microseconds with zero tokens."""
    if not ZEV_BIN.exists():
        # Fallback heuristic if zev binary is not compiled yet
        rel = str(file_path.relative_to(ROOT_DIR))
        if "bmesh" in rel:
            return "blender-bmesh", 1.0
        elif "makesdna" in rel:
            return "blender-dna", 1.0
        elif "io" in rel:
            return "blender-io", 1.0
        elif "math" in rel:
            return "blender-math", 1.0
        return "skip", 0.0

    # Read first 40 lines of file for contextual classification
    sample = []
    try:
        with open(file_path, "r", errors="ignore") as f:
            for _ in range(40):
                line = f.readline()
                if not line:
                    break
                sample.append(line)
    except Exception:
        return "skip", 0.0

    state_text = f"Path: {file_path.name}\nContext: {''.join(sample)[:500]}"
    routes_json = json.dumps(ROUTES)

    cmd = [
        str(ZEV_BIN), "route",
        "--state", state_text,
        "--routes", routes_json
    ]
    try:
        proc = subprocess.run(cmd, capture_output=True, text=True, check=True)
        res = json.loads(proc.stdout)
        return res.get("destination", "skip"), res.get("probability", 0.0)
    except Exception as e:
        print(f"  [!] Zev route error: {e}, falling back to path heuristics")
        rel = str(file_path)
        if "bmesh" in rel:
            return "blender-bmesh", 0.8
        elif "makesdna" in rel:
            return "blender-dna", 0.8
        return "skip", 0.0

def build_system_prompt(target_crate: str, relevant_lifetimes: List[Dict]) -> str:
    prompt = (
        "You are an expert systems engineer porting Blender C/C++ to Safe Rust.\n"
        "Guidelines:\n"
        "1. Produce 100% safe, compiling Rust code. Do not use raw pointers unless strictly necessary.\n"
        "2. Replace C self-referential pointer webs with generational handles (Handle<T>) or slot indices.\n"
        "3. Map errors to `Result<T, BMeshError>` or `Option<T>`.\n"
        "4. Output ONLY the pure Rust code module. Do not include markdown preamble or conversational text.\n"
    )
    if relevant_lifetimes:
        prompt += "\nEnforced Ownership Mappings (from LIFETIMES.tsv):\n"
        for lt in relevant_lifetimes:
            prompt += f"- Field `{lt['field']}`: {lt['model']} ({lt['notes']})\n"
    return prompt

def translate_file_with_apfel(source_file: Path, target_crate: str, lifetimes: Dict) -> Optional[str]:
    """Invokes on-device Apple Intelligence FoundationModels via apfel-rs."""
    module_name = source_file.stem
    relevant_lifetimes = []
    for struct_name, entries in lifetimes.items():
        if struct_name.lower() in source_file.name.lower():
            relevant_lifetimes.extend(entries)

    system_prompt = build_system_prompt(target_crate, relevant_lifetimes)
    user_prompt = (
        f"Mechanically convert this C/C++ file into safe Rust for crate '{target_crate}'. "
        f"Target module name: {module_name}.rs"
    )

    cmd = [
        str(APFEL_BIN),
        "--code",
        "--temperature", "0",
        "--max-tokens", "2048",
        "-s", system_prompt,
        "-f", str(source_file),
        user_prompt
    ]

    print(f"  [→] Running on-device translation with apfel-rs for {source_file.name}...")
    try:
        proc = subprocess.run(cmd, capture_output=True, text=True, timeout=120)
        if proc.returncode == 0 and proc.stdout.strip():
            return proc.stdout.strip()
        else:
            print(f"  [!] apfel error (code {proc.returncode}): {proc.stderr}")
            return None
    except subprocess.TimeoutExpired:
        print("  [!] apfel timed out after 120s")
        return None
    except Exception as e:
        print(f"  [!] Failed to invoke apfel: {e}")
        return None

def verify_and_fix(target_crate: str, module_path: Path, max_attempts: int = 3) -> bool:
    """Burns down compiler errors using compiler-errors-as-work-queue discipline."""
    for attempt in range(1, max_attempts + 1):
        proc = subprocess.run(
            ["cargo", "check", "-p", target_crate, "--message-format=json"],
            cwd=str(ROOT_DIR),
            capture_output=True,
            text=True
        )
        if proc.returncode == 0:
            print(f"  [✓] Crate '{target_crate}' compiles cleanly!")
            return True

        print(f"  [!] Compilation errors detected (Attempt {attempt}/{max_attempts}). Extracting diagnostics...")
        errors = []
        for line in proc.stdout.splitlines():
            try:
                msg = json.loads(line)
                if msg.get("reason") == "compiler-message":
                    c_msg = msg.get("message", {})
                    if c_msg.get("level") == "error":
                        errors.append(c_msg.get("rendered", ""))
            except Exception:
                pass

        if not errors:
            print("  [!] Non-JSON compiler errors occurred:")
            print(proc.stderr[:500])
            return False

        error_context = "\n".join(errors[:5])  # Cap to first 5 errors
        fix_prompt = (
            f"Fix the following Rust compiler errors in this module:\n\n{error_context}\n\n"
            "Output the complete fixed Rust module only."
        )

        fix_cmd = [
            str(APFEL_BIN),
            "--code",
            "-s", "You are an expert Rust compiler debugger. Fix the compilation errors precisely.",
            "-f", str(module_path),
            fix_prompt
        ]
        fix_proc = subprocess.run(fix_cmd, capture_output=True, text=True, timeout=90)
        if fix_proc.returncode == 0 and fix_proc.stdout.strip():
            with open(module_path, "w") as f:
                f.write(fix_proc.stdout.strip() + "\n")
            print("  [↺] Applied automated patch, rechecking...")
        else:
            print("  [!] Automated fix failed to generate patch.")
            break

    return False

def register_module_in_crate(target_crate: str, module_name: str):
    """Ensures `pub mod <module_name>;` is present in lib.rs."""
    lib_rs = ROOT_DIR / "crates" / target_crate / "src" / "lib.rs"
    mod_decl = f"pub mod {module_name};\n"
    if lib_rs.exists():
        content = lib_rs.read_text()
        if mod_decl not in content:
            with open(lib_rs, "a") as f:
                f.write(f"\n{mod_decl}")

def git_commit_file(target_crate: str, module_path: Path, source_rel: str):
    """Commits single ported file with strict git discipline."""
    try:
        subprocess.run(["git", "add", str(module_path)], cwd=str(ROOT_DIR), check=True)
        lib_rs = ROOT_DIR / "crates" / target_crate / "src" / "lib.rs"
        subprocess.run(["git", "add", str(lib_rs)], cwd=str(ROOT_DIR), check=True)
        msg = f"port({target_crate}): mechanically port {module_path.stem} from {source_rel}"
        subprocess.run(["git", "commit", "-m", msg], cwd=str(ROOT_DIR), check=True)
        print(f"  [✓] Committed: {msg}")
    except Exception as e:
        print(f"  [!] Git commit error: {e}")

def run_port_loop(subsystem: str = "bmesh", limit: int = 10, dry_run: bool = False):
    """Main execution loop driven by porting-workflow-loops discipline."""
    progress = load_progress()
    lifetimes = load_lifetimes()

    target_subsystem_dir = UPSTREAM_DIR / subsystem
    if not target_subsystem_dir.exists():
        print(f"[!] Subsystem path {target_subsystem_dir} does not exist.")
        return

    print(f"=== Starting Autonomous Zero-Token Port Loop for '{subsystem}' ===")
    print(f"Upstream: {target_subsystem_dir}")
    print(f"Zev: {ZEV_BIN}")
    print(f"Apfel: {APFEL_BIN}\n")

    files_to_process = []
    for ext in ("*.cc", "*.c", "*.hh", "*.h"):
        files_to_process.extend(target_subsystem_dir.rglob(ext))

    processed_count = 0
    for file_path in sorted(files_to_process):
        rel_str = str(file_path.relative_to(ROOT_DIR))
        if rel_str in progress["completed"] or rel_str in progress["skipped"]:
            continue

        print(f"\n[{processed_count + 1}] Evaluating: {rel_str}")
        dest_crate, prob = route_file_with_zev(file_path)
        print(f"  [*] Zev routing → {dest_crate} (confidence: {prob:.2f})")

        if dest_crate == "skip" or prob < 0.6:
            print("  [-] Skipping non-target / build / low-confidence file.")
            progress["skipped"][rel_str] = {"reason": "zev_route_skip", "confidence": prob}
            save_progress(progress)
            continue

        if dry_run:
            print("  [DRY-RUN] Would translate and compile.")
            continue

        # Target file path
        module_name = file_path.stem.replace(".", "_")
        target_file = ROOT_DIR / "crates" / dest_crate / "src" / f"{module_name}.rs"

        # Generate translation via Apfel
        rust_code = translate_file_with_apfel(file_path, dest_crate, lifetimes)
        if not rust_code:
            progress["failed"][rel_str] = "apfel_generation_failed"
            save_progress(progress)
            continue

        # Save to crate
        with open(target_file, "w") as f:
            f.write(rust_code + "\n")
        register_module_in_crate(dest_crate, module_name)

        # Compiler verification & error burndown
        success = verify_and_fix(dest_crate, target_file)
        if success:
            git_commit_file(dest_crate, target_file, rel_str)
            progress["completed"][rel_str] = {
                "crate": dest_crate,
                "module": f"{module_name}.rs"
            }
        else:
            progress["failed"][rel_str] = "compiler_burndown_exceeded"
            print(f"  [✗] Failed to resolve compilation for {module_name}.rs")

        save_progress(progress)
        processed_count += 1
        if processed_count >= limit:
            print(f"\n[!] Reached batch limit of {limit} files. Pausing loop.")
            break

    print("\n=== Port Loop Batch Finished ===")
    print(f"Completed: {len(progress['completed'])}")
    print(f"Skipped:   {len(progress['skipped'])}")
    print(f"Failed:    {len(progress['failed'])}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Blender Rust Port Runner")
    parser.add_argument("--subsystem", default="bmesh", help="Subsystem inside upstream/source/blender (default: bmesh)")
    parser.add_argument("--limit", type=int, default=5, help="Number of files to process per run")
    parser.add_argument("--dry-run", action="store_true", help="Route files with Zev without modifying code")
    args = parser.parse_args()

    run_port_loop(subsystem=args.subsystem, limit=args.limit, dry_run=args.dry_run)
