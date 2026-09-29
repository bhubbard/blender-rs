#!/usr/bin/env python3
"""
blender-rs Autonomous Zero-Token Port Runner (with Live Progress Bars)
=====================================================================
Orchestrates file-by-file translation from Blender C/C++ to Safe Rust:
- Zero-token classification & routing via `zev-rs`
- Zero-cloud-token on-device code generation via `apfel-rs` (Apple Intelligence)
- Real-time animated progress bars:
    * Overall project progress bar (X / 6,440 files)
    * Batch subsystem progress bar (X / N in subsystem)
    * Live per-file elapsed timer & spinner while apfel-rs generates tokens
- Strict compiler error burndown & rollback discipline (keeps trunk 100% green)
"""

import os
import sys
import json
import time
import threading
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
TELEMETRY_DIR = ROOT_DIR / "telemetry"
TELEMETRY_DIR.mkdir(parents=True, exist_ok=True)
APFEL_TELEMETRY_FILE = TELEMETRY_DIR / "apfel_telemetry.jsonl"
ZEV_TELEMETRY_FILE = TELEMETRY_DIR / "zev_routing_telemetry.jsonl"

def log_apfel_telemetry(record: dict):
    """Appends structured interaction telemetry for improving apfel-rs."""
    try:
        with open(APFEL_TELEMETRY_FILE, "a") as f:
            f.write(json.dumps(record) + "\n")
    except Exception:
        pass

def log_zev_telemetry(record: dict):
    """Appends structured classification record compatible with zev_benchmarks dataset."""
    try:
        with open(ZEV_TELEMETRY_FILE, "a") as f:
            f.write(json.dumps(record) + "\n")
    except Exception:
        pass

import shutil

# Executable paths
CARGO_BIN = Path("/Users/bhubbard/.cargo/bin")
ZEV_BIN = Path(os.environ.get("ZEV_BIN", str(CARGO_BIN / "zev") if (CARGO_BIN / "zev").exists() else (shutil.which("zev") or "/Users/bhubbard/.cargo/bin/zev")))
APFEL_BIN = Path(os.environ.get("APFEL_BIN", str(CARGO_BIN / "apfel") if (CARGO_BIN / "apfel").exists() else (shutil.which("apfel") or "/Users/bhubbard/.cargo/bin/apfel")))
APFEL_TRANSPILE_BIN = Path(os.environ.get("APFEL_TRANSPILE_BIN", str(CARGO_BIN / "apfel-transpile") if (CARGO_BIN / "apfel-transpile").exists() else (shutil.which("apfel-transpile") or "/Users/bhubbard/.cargo/bin/apfel-transpile")))

# Terminal colors
RESET = "\033[0m"
BOLD = "\033[1m"
DIM = "\033[2m"
CYAN = "\033[36m"
GREEN = "\033[32m"
YELLOW = "\033[33m"
RED = "\033[31m"
BLUE = "\033[34m"

ROUTES = {
    "blender-math": "Vectors, matrices, quaternions, bounding boxes, coordinate math, BLI_math",
    "blender-mem": "Chunked memory pools, allocators, BLI_mempool, MEM_guardedalloc",
    "blender-dna": "DNA struct definitions, mesh headers, serialized schemas, makesdna",
    "blender-bmesh": "BMesh half-edge data structures, BMVert, BMEdge, BMLoop, BMFace, Euler operators",
    "blender-io": "File import and export, Wavefront OBJ, PLY, STL, USD",
    "skip": "CMakeLists, build scripts, tests, private platform glue, or non-portable code"
}

def render_bar(current: int, total: int, width: int = 28, fill: str = "█", empty: str = "░", color: str = CYAN) -> str:
    if total <= 0:
        return f"[{color}{empty * width}{RESET}]   0.0%"
    pct = min(1.0, max(0.0, current / total))
    filled_len = int(width * pct)
    bar = f"{color}{fill * filled_len}{DIM}{empty * (width - filled_len)}{RESET}"
    return f"[{bar}] {pct * 100:>5.1f}%"

def count_all_upstream_files() -> int:
    """Counts total C/C++ files across all upstream subsystems."""
    if not UPSTREAM_DIR.exists():
        return 6440
    return sum(1 for _ in UPSTREAM_DIR.rglob("*.[ch]") if not _.name.startswith(".")) + \
           sum(1 for _ in UPSTREAM_DIR.rglob("*.cc") if not _.name.startswith(".")) + \
           sum(1 for _ in UPSTREAM_DIR.rglob("*.hh") if not _.name.startswith("."))

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

    t0 = time.time()
    cmd = [
        str(ZEV_BIN), "route",
        "--file", str(file_path),
        "--routes", routes_json
    ]
    try:
        proc = subprocess.run(cmd, capture_output=True, text=True, check=True)
        res = json.loads(proc.stdout)
        dest = res.get("destination", "skip")
        prob = res.get("probability", 0.0)
        latency_us = int((time.time() - t0) * 1_000_000)
        log_zev_telemetry({
            "task": "codebase_module_routing",
            "timestamp": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
            "source_file": str(file_path.relative_to(ROOT_DIR)),
            "predicted_destination": dest,
            "confidence": prob,
            "latency_us": latency_us,
        })
        return dest, prob
    except Exception as e:
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

def run_with_live_spinner(cmd: List[str], label: str, timeout: int = 120) -> Tuple[int, str, str]:
    """Runs a subprocess while displaying a live animated spinner & elapsed timer."""
    spinner_frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]
    start_time = time.time()
    done = False
    result_holder = {}

    def worker():
        try:
            proc = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)
            result_holder["rc"] = proc.returncode
            result_holder["stdout"] = proc.stdout
            result_holder["stderr"] = proc.stderr
        except subprocess.TimeoutExpired:
            result_holder["rc"] = -1
            result_holder["stdout"] = ""
            result_holder["stderr"] = "timed out"
        except Exception as e:
            result_holder["rc"] = -2
            result_holder["stdout"] = ""
            result_holder["stderr"] = str(e)
        finally:
            nonlocal done
            done = True

    t = threading.Thread(target=worker, daemon=True)
    t.start()

    i = 0
    while not done:
        elapsed = time.time() - start_time
        frame = spinner_frames[i % len(spinner_frames)]
        sys.stdout.write(f"\r  {CYAN}{frame}{RESET} {label} {DIM}({elapsed:4.1f}s){RESET} ")
        sys.stdout.flush()
        time.sleep(0.1)
        i += 1

    elapsed = time.time() - start_time
    rc = result_holder.get("rc", -1)
    if rc == 0:
        sys.stdout.write(f"\r  {GREEN}✓{RESET} {label} {DIM}({elapsed:4.1f}s){RESET}\n")
    else:
        sys.stdout.write(f"\r  {RED}✗{RESET} {label} {DIM}({elapsed:4.1f}s){RESET}\n")
    sys.stdout.flush()
    return rc, result_holder.get("stdout", ""), result_holder.get("stderr", "")

def chunk_c_cpp_file(source_file: Path, max_lines: int = 75) -> Tuple[str, List[str]]:
    """Splits a C/C++ file into function and struct bounded chunks with common preamble."""
    lines = source_file.read_text(errors='ignore').splitlines()
    preamble = []
    chunks = []
    current_chunk = []
    depth = 0
    namespace_depth = 0

    for line in lines:
        stripped = line.strip()
        if 'namespace' in stripped and '{' in stripped:
            namespace_depth += 1
            preamble.append(line)
            continue
        elif not chunks and not current_chunk and (stripped.startswith('#') or stripped.startswith('//') or stripped.startswith('/*') or not stripped):
            preamble.append(line)
            continue

        brace_open = line.count('{')
        brace_close = line.count('}')
        depth += brace_open - brace_close

        current_chunk.append(line)
        # Function boundary is reached when depth returns to namespace level
        if len(current_chunk) >= max_lines and depth <= namespace_depth:
            chunks.append('\n'.join(current_chunk))
            current_chunk = []

    if current_chunk:
        chunks.append('\n'.join(current_chunk))

    return '\n'.join(preamble), chunks

def combine_rust_chunks(chunks_rust: List[str]) -> str:
    """Combines translated Rust chunks, deduplicating imports."""
    uses = set()
    bodies = []

    for chunk in chunks_rust:
        for line in chunk.splitlines():
            s = line.strip()
            if s.startswith("use ") and s.endswith(";"):
                uses.add(s)
            else:
                bodies.append(line)

    sorted_uses = sorted(list(uses))
    assembled = "\n".join(sorted_uses) + "\n\n" + "\n".join(bodies)
    return assembled.strip()

def translate_large_file_chunked(source_file: Path, target_crate: str, lifetimes: Dict) -> Optional[str]:
    """Automated struct/function chunker for large files exceeding FoundationModels 4K context."""
    module_name = source_file.stem
    preamble, chunks = chunk_c_cpp_file(source_file, max_lines=75)
    total_chunks = len(chunks)

    if total_chunks <= 1:
        # Fall back to single-shot if not divisible
        return None

    print(f"  {CYAN}[⚡ Chunker]{RESET} File has {sum(len(c.splitlines()) for c in chunks)} lines → Split into {total_chunks} function/struct chunks (≤75 lines/ea)")
    translated_chunks = []

    relevant_lifetimes = []
    for struct_name, entries in lifetimes.items():
        if struct_name.lower() in source_file.name.lower():
            relevant_lifetimes.extend(entries)

    system_prompt = build_system_prompt(target_crate, relevant_lifetimes)

    tmp_dir = ROOT_DIR / "target" / "chunks"
    tmp_dir.mkdir(parents=True, exist_ok=True)

    for i, chunk in enumerate(chunks, start=1):
        chunk_file = tmp_dir / f"{module_name}_part_{i}.cpp"
        # Prepend preamble for type context
        chunk_file.write_text(f"// File Preamble Context\n{preamble}\n\n// Target Chunk {i}/{total_chunks}\n{chunk}\n")

        user_prompt = (
            f"Mechanically convert Part {i}/{total_chunks} of {source_file.name} into safe Rust for crate '{target_crate}'. "
            f"Only translate the functions and structs in this chunk. Output only pure Rust code without commentary."
        )

        cmd = [
            str(APFEL_BIN),
            "--code",
            "--auto-continue",
            "--permissive",
            "--telemetry", str(TELEMETRY_DIR / "apfel_engine_telemetry.jsonl"),
            "--temperature", "0",
            "--max-tokens", "2048",
            "-s", system_prompt,
            "-f", str(chunk_file),
            user_prompt
        ]

        rc, stdout, stderr = run_with_live_spinner(cmd, f"Chunk {i}/{total_chunks} ({len(chunk.splitlines())} lines)...", timeout=90)
        if chunk_file.exists():
            chunk_file.unlink()

        if rc == 0 and stdout.strip():
            translated_chunks.append(stdout.strip())
        else:
            print(f"  {RED}[!] Chunk {i}/{total_chunks} failed translation.{RESET}")
            return None

    assembled_rust = combine_rust_chunks(translated_chunks)
    print(f"  {GREEN}[✓ Chunker]{RESET} Successfully assembled {total_chunks} chunks into {module_name}.rs")
    return assembled_rust

def extract_code_fence(text: str) -> str:
    """Extracts code block from markdown fences if present."""
    if "```" in text:
        parts = text.split("```")
        if len(parts) >= 3:
            block = parts[1]
            lines = block.splitlines()
            if lines and lines[0].strip().lower() in ("rust", "rs", "c", "cpp"):
                return "\n".join(lines[1:]).strip()
            return block.strip()
    return text.strip()

def translate_file_with_apfel(source_file: Path, target_crate: str, lifetimes: Dict) -> Optional[str]:
    """Invokes on-device Apple Intelligence FoundationModels via apfel-transpile with live progress."""
    module_name = source_file.stem.replace(".", "_")
    try:
        file_bytes = source_file.stat().st_size
        file_lines = sum(1 for _ in open(source_file, "r", errors="ignore"))
    except Exception:
        file_bytes, file_lines = 0, 0

    relevant_lifetimes = []
    for struct_name, entries in lifetimes.items():
        if struct_name.lower() in source_file.name.lower():
            relevant_lifetimes.extend(entries)

    system_prompt = build_system_prompt(target_crate, relevant_lifetimes)

    cmd = [
        str(APFEL_TRANSPILE_BIN),
        "-f", str(source_file),
        "-t", target_crate,
        "-m", module_name,
        "--chunk-lines", "75",
        "--auto-continue",
        "-s", system_prompt,
        "--telemetry", str(TELEMETRY_DIR / "apfel_engine_telemetry.jsonl"),
        "--temperature", "0",
        "--max-tokens", "2048",
    ]

    t0 = time.time()
    rc, stdout, stderr = run_with_live_spinner(cmd, f"Transpiling {source_file.name} with apfel-transpile...", timeout=180)
    dur = round(time.time() - t0, 2)

    log_apfel_telemetry({
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "source_file": str(source_file.relative_to(ROOT_DIR)),
        "file_lines": file_lines,
        "file_bytes": file_bytes,
        "target_crate": target_crate,
        "exit_code": rc,
        "duration_s": dur,
        "stdout_len": len(stdout) if stdout else 0,
        "stderr": stderr[:300] if stderr else "",
        "success": (rc == 0 and bool(stdout.strip()))
    })
    if rc == 0 and stdout.strip():
        return stdout.strip()
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
            print(f"  {GREEN}[✓] Crate '{target_crate}' compiles cleanly!{RESET}")
            return True

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
            return False

        error_context = "\n".join(errors[:8])
        fix_prompt = (
            f"Fix the following Rust compiler errors in this module:\n\n{error_context}\n\n"
            "If any types, structs, or functions are missing, add placeholder definitions, structs, enums, or type aliases so that this module compiles cleanly.\n"
            "Output the complete fixed Rust module only inside markdown fences."
        )

        file_len = module_path.stat().st_size if module_path.exists() else 0
        prompt_len = len(fix_prompt) + 200
        est_input_tokens = (file_len + prompt_len) // 4
        avail_tokens = max(256, min(2048, 3800 - est_input_tokens))

        fix_cmd = [
            str(APFEL_BIN),
            "--permissive",
            "--telemetry", str(TELEMETRY_DIR / "apfel_engine_telemetry.jsonl"),
            "--temperature", "0",
            "--max-tokens", str(avail_tokens),
            "-s", "You are an expert Rust compiler debugger. Fix the compilation errors precisely. Output ONLY the fixed Rust module inside markdown fences.",
            "-f", str(module_path),
            fix_prompt
        ]
        rc, stdout, _ = run_with_live_spinner(fix_cmd, f"Compiler repair pass {attempt}/{max_attempts} (budget: {avail_tokens} tokens)...", timeout=90)
        if rc == 0 and stdout.strip():
            fixed_code = extract_code_fence(stdout.strip())
            with open(module_path, "w") as f:
                f.write(fixed_code + "\n")
        else:
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

def unregister_module_in_crate(target_crate: str, module_name: str):
    """Safely removes `pub mod <module_name>;` from lib.rs if module failed compilation."""
    lib_rs = ROOT_DIR / "crates" / target_crate / "src" / "lib.rs"
    mod_decl = f"pub mod {module_name};"
    if lib_rs.exists():
        lines = lib_rs.read_text().splitlines()
        filtered = [l for l in lines if l.strip() != mod_decl]
        lib_rs.write_text("\n".join(filtered) + "\n")

def git_commit_file(target_crate: str, module_path: Path, source_rel: str):
    """Commits single ported file with strict git discipline."""
    try:
        subprocess.run(["git", "add", str(module_path)], cwd=str(ROOT_DIR), check=True)
        lib_rs = ROOT_DIR / "crates" / target_crate / "src" / "lib.rs"
        subprocess.run(["git", "add", str(lib_rs)], cwd=str(ROOT_DIR), check=True)
        msg = f"port({target_crate}): mechanically port {module_path.stem} from {source_rel}"
        subprocess.run(["git", "commit", "-m", msg], cwd=str(ROOT_DIR), check=True)
        print(f"  {GREEN}[✓] Committed:{RESET} {msg}")
    except Exception as e:
        print(f"  {RED}[!] Git commit error:{RESET} {e}")

def run_port_loop(subsystem: str = "bmesh", limit: int = 10, dry_run: bool = False):
    """Main execution loop driven by porting-workflow-loops discipline with progress bars."""
    progress = load_progress()
    lifetimes = load_lifetimes()
    total_project_files = count_all_upstream_files()

    target_subsystem_dir = UPSTREAM_DIR / subsystem
    if not target_subsystem_dir.exists():
        print(f"{RED}[!] Subsystem path {target_subsystem_dir} does not exist.{RESET}")
        return

    files_to_process = []
    for ext in ("*.cc", "*.c", "*.hh", "*.h"):
        files_to_process.extend(target_subsystem_dir.rglob(ext))

    files_to_process = sorted(files_to_process)
    total_subsystem_files = len(files_to_process)

    print(f"\n{BOLD}{CYAN}======================================================================{RESET}")
    print(f"{BOLD} 🚀 blender-rs Autonomous Port Loop: {subsystem} {RESET}")
    print(f"{BOLD}{CYAN}======================================================================{RESET}\n")

    processed_count = 0
    for idx, file_path in enumerate(files_to_process):
        rel_str = str(file_path.relative_to(ROOT_DIR))
        if rel_str in progress["completed"] or rel_str in progress["skipped"]:
            continue

        # Calculate progress stats
        n_completed = len(progress.get("completed", {}))
        n_skipped = len(progress.get("skipped", {}))
        n_failed = len(progress.get("failed", {}))
        n_total_done = n_completed + n_skipped + n_failed

        # Display Dual Progress Bars: Project-wide + Batch Subsystem
        proj_bar = render_bar(n_total_done, total_project_files, width=28, color=CYAN)
        sub_bar = render_bar(idx + 1, total_subsystem_files, width=28, color=GREEN)

        print(f"\n{BOLD}Overall Project:{RESET} {proj_bar} ({n_total_done:,} / {total_project_files:,} files) "
              f"[{GREEN}Pass: {n_completed}{RESET} | {YELLOW}Skip: {n_skipped}{RESET} | {RED}Fail: {n_failed}{RESET}]")
        print(f"{BOLD}Subsystem [{subsystem}]:{RESET} {sub_bar} ({idx + 1} / {total_subsystem_files} files)")
        print(f"{BOLD}Active File:{RESET}     {CYAN}{rel_str}{RESET}")

        # Routing with Zev
        dest_crate, prob = route_file_with_zev(file_path)
        print(f"  [*] Zev routing → {BOLD}{dest_crate}{RESET} (confidence: {prob:.2f})")

        if dest_crate == "skip" or prob < 0.5:
            if "bmesh" in rel_str:
                dest_crate = "blender-bmesh"
            elif "makesdna" in rel_str or "dna" in rel_str:
                dest_crate = "blender-dna"
            elif "io" in rel_str:
                dest_crate = "blender-io"
            elif "math" in rel_str or "vec" in rel_str or "mat" in rel_str:
                dest_crate = "blender-math"
            elif "mem" in rel_str or "alloc" in rel_str:
                dest_crate = "blender-mem"
            else:
                print(f"  {YELLOW}[-] Skipping non-target / build / low-confidence file.{RESET}")
                progress["skipped"][rel_str] = {"reason": "zev_route_skip", "confidence": prob}
                save_progress(progress)
                continue

        if dry_run:
            print(f"  {BLUE}[DRY-RUN] Would translate and compile.{RESET}")
            continue

        module_name = file_path.stem.replace(".", "_")
        target_file = ROOT_DIR / "crates" / dest_crate / "src" / f"{module_name}.rs"

        # Generate translation via Apfel (with live spinner)
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
            progress.get("failed", {}).pop(rel_str, None)
            save_progress(progress)
        else:
            # Revert from lib.rs so the trunk stays compiling
            unregister_module_in_crate(dest_crate, module_name)
            if target_file.exists():
                target_file.unlink()
            progress["failed"][rel_str] = "compiler_burndown_exceeded"
            save_progress(progress)
            print(f"  {RED}[✗] Rolled back uncompiling {module_name}.rs (keeps crate green){RESET}")

        save_progress(progress)
        processed_count += 1
        if processed_count >= limit:
            print(f"\n{YELLOW}[!] Reached batch limit of {limit} files. Pausing loop.{RESET}")
            break

    print(f"\n{BOLD}{CYAN}=== Batch Finished: {subsystem} ==={RESET}")
    print(f"Completed: {len(progress['completed'])}")
    print(f"Skipped:   {len(progress['skipped'])}")
    print(f"Failed:    {len(progress['failed'])}\n")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Blender Rust Port Runner with Progress Bars")
    parser.add_argument("--subsystem", default="bmesh", help="Subsystem inside upstream/source/blender (default: bmesh)")
    parser.add_argument("--limit", type=int, default=5, help="Number of files to process per run")
    parser.add_argument("--dry-run", action="store_true", help="Route files with Zev without modifying code")
    args = parser.parse_args()

    run_port_loop(subsystem=args.subsystem, limit=args.limit, dry_run=args.dry_run)
