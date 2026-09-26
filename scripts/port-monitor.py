#!/usr/bin/env python3
"""
blender-rs Live Porting Dashboard & Progress Monitor
====================================================
Displays live progress bars for the entire Blender port and individual subsystems:
- Overall Project Progress Bar (completed / total files)
- Subsystem Progress breakdown
- Live active process detection (apfel-rs / zev-rs / port-runner)
- Live ETA and throughput (files/min)
- Recent activity log

Run anytime in a separate terminal:
    python3 scripts/port-monitor.py
"""

import os
import sys
import time
import json
import subprocess
from pathlib import Path
from datetime import datetime, timedelta

ROOT_DIR = Path(__file__).resolve().parent.parent
UPSTREAM_DIR = ROOT_DIR / "upstream" / "source" / "blender"
PROGRESS_FILE = ROOT_DIR / "PORTING_PROGRESS.json"

# ANSI color codes
RESET = "\033[0m"
BOLD = "\033[1m"
DIM = "\033[2m"
CYAN = "\033[36m"
GREEN = "\033[32m"
YELLOW = "\033[33m"
RED = "\033[31m"
MAGENTA = "\033[35m"
BLUE = "\033[34m"
CLEAR_SCREEN = "\033[2J\033[H"
HIDE_CURSOR = "\033[?25l"
SHOW_CURSOR = "\033[?25h"

def render_bar(current: int, total: int, width: int = 32, fill: str = "█", empty: str = "░", color: str = CYAN) -> str:
    if total <= 0:
        return f"{color}{empty * width}{RESET} 0.0%"
    pct = min(1.0, max(0.0, current / total))
    filled_len = int(width * pct)
    bar = f"{color}{fill * filled_len}{DIM}{empty * (width - filled_len)}{RESET}"
    return f"[{bar}] {pct * 100:>5.1f}%"

def count_upstream_files() -> tuple[int, dict[str, int]]:
    """Counts total and per-subsystem C/C++ files in upstream Blender."""
    subsystem_counts = {}
    total = 0
    if not UPSTREAM_DIR.exists():
        return 0, {}

    for path in UPSTREAM_DIR.iterdir():
        if path.is_dir():
            count = sum(1 for _ in path.rglob("*.[ch]") if not _.name.startswith(".")) + \
                    sum(1 for _ in path.rglob("*.cc") if not _.name.startswith(".")) + \
                    sum(1 for _ in path.rglob("*.hh") if not _.name.startswith("."))
            if count > 0:
                subsystem_counts[path.name] = count
                total += count
    return total, subsystem_counts

def get_active_process_info() -> tuple[str, str, str]:
    """Inspects system processes to see what file is currently being translated."""
    try:
        proc = subprocess.run(
            ["ps", "-eo", "pid,etime,%cpu,command"],
            capture_output=True,
            text=True
        )
        for line in proc.stdout.splitlines():
            if "apfel" in line and "--code" in line:
                tokens = line.split()
                pid = tokens[0]
                etime = tokens[1]
                cpu = tokens[2]
                # extract file name from command
                file_target = "active file"
                for i, t in enumerate(tokens):
                    if t == "-f" and i + 1 < len(tokens):
                        file_target = Path(tokens[i + 1]).name
                        break
                return f"{CYAN}apfel-rs (Apple Intelligence){RESET}", file_target, f"{etime} (CPU: {cpu}%)"
            elif "port-runner.py" in line:
                tokens = line.split()
                return f"{YELLOW}port-runner (evaluating/compiling){RESET}", "batch runner", tokens[1]
    except Exception:
        pass
    return f"{DIM}idle / between files{RESET}", "-", "-"

def main():
    print(HIDE_CURSOR, end="")
    start_time = time.time()
    last_completed_count = 0

    try:
        total_files, subsys_totals = count_upstream_files()

        while True:
            # Load progress
            progress = {"completed": {}, "skipped": {}, "failed": {}}
            if PROGRESS_FILE.exists():
                try:
                    with open(PROGRESS_FILE, "r") as f:
                        progress = json.load(f)
                except Exception:
                    pass

            n_completed = len(progress.get("completed", {}))
            n_skipped = len(progress.get("skipped", {}))
            n_failed = len(progress.get("failed", {}))
            n_processed = n_completed + n_skipped + n_failed

            # Subsystem progress calculation
            subsys_processed = {}
            for path_str in list(progress.get("completed", {}).keys()) + \
                            list(progress.get("skipped", {}).keys()) + \
                            list(progress.get("failed", {}).keys()):
                # e.g. upstream/source/blender/bmesh/...
                parts = Path(path_str).parts
                if len(parts) >= 4 and parts[0] == "upstream" and parts[1] == "source" and parts[2] == "blender":
                    sub = parts[3]
                    subsys_processed[sub] = subsys_processed.get(sub, 0) + 1

            # Active process
            runner_status, active_file, active_time = get_active_process_info()

            # Output screen
            buf = []
            buf.append(CLEAR_SCREEN)
            buf.append(f"{BOLD}{CYAN}═══════════════════════════════════════════════════════════════════════════════════{RESET}")
            buf.append(f"{BOLD} 🦀 blender-rs Autonomous Porting Dashboard  (zev-rs + apfel-rs) {RESET}")
            buf.append(f"{BOLD}{CYAN}═══════════════════════════════════════════════════════════════════════════════════{RESET}\n")

            # Project Progress Bar
            proj_bar = render_bar(n_processed, total_files, width=38, color=CYAN)
            buf.append(f"{BOLD}Overall Project Progress:{RESET}")
            buf.append(f"  {proj_bar}  ({n_processed:,} / {total_files:,} files evaluated)\n")

            # Stats line
            buf.append(
                f"  {GREEN}● Completed:{RESET} {BOLD}{n_completed}{RESET}  "
                f"  {YELLOW}● Filtered/Skipped:{RESET} {n_skipped}  "
                f"  {RED}● Burndown/Failed:{RESET} {n_failed}\n"
            )

            # Active Work Status
            buf.append(f"{BOLD}Current Activity:{RESET}")
            buf.append(f"  Engine:   {runner_status}")
            buf.append(f"  File:     {BOLD}{active_file}{RESET}")
            buf.append(f"  Elapsed:  {active_time}\n")

            # Subsystem Progress Bars (Top 6)
            buf.append(f"{BOLD}Subsystem Breakdown:{RESET}")
            key_subsystems = ["bmesh", "makesdna", "blenlib", "io", "geometry", "blenkernel", "nodes", "gpu"]
            for sub in key_subsystems:
                sub_total = subsys_totals.get(sub, 0)
                sub_done = subsys_processed.get(sub, 0)
                if sub_total > 0:
                    bar = render_bar(sub_done, sub_total, width=24, color=GREEN if sub_done >= sub_total else YELLOW)
                    buf.append(f"  {sub:<12} {bar} ({sub_done:>3}/{sub_total:<3} files)")
            buf.append("")

            # Recent Activity
            buf.append(f"{BOLD}Recent Module Activity:{RESET}")
            recent_completed = list(progress.get("completed", {}).items())[-3:]
            if recent_completed:
                for src, meta in reversed(recent_completed):
                    mod = meta.get("module", Path(src).name)
                    crate = meta.get("crate", "blender")
                    buf.append(f"  {GREEN}[✓ PASS]{RESET} {mod:<24} → {crate}")
            else:
                buf.append(f"  {DIM}Waiting for first batch compilation to complete...{RESET}")

            recent_skipped = list(progress.get("skipped", {}).items())[-2:]
            for src, meta in reversed(recent_skipped):
                reason = meta.get("reason", "skip")
                buf.append(f"  {YELLOW}[- SKIP]{RESET} {Path(src).name:<24} ({reason})")

            buf.append(f"\n{DIM}Refreshing every 1s • Press Ctrl+C to exit dashboard • port-runner continues in background{RESET}")

            # Print frame
            sys.stdout.write("\n".join(buf) + "\n")
            sys.stdout.flush()

            time.sleep(1.0)

    except KeyboardInterrupt:
        pass
    finally:
        print(SHOW_CURSOR, end="")
        print("\nExited monitor.")

if __name__ == "__main__":
    main()
