#!/usr/bin/env python3
"""
scripts/zima_transpile.py — Remote ZimaBoard Neural Transpilation Helper
========================================================================
Offloads C/C++ to Safe Rust translation tasks to the ZimaBoard node (192.168.0.206)
running Ollama with Gemma 2B or Qwen 2.5 Coder 0.5B.

Usage:
  python3 scripts/zima_transpile.py --source upstream/source/blender/makesdna/DNA_meshdata_types.h --target blender-dna
  python3 scripts/zima_transpile.py --prompt "Convert: int max(int a, int b) { return a > b ? a : b; }"
"""

import os
import sys
import json
import time
import urllib.request
import argparse
import subprocess
from pathlib import Path
from typing import Optional

ROOT_DIR = Path(__file__).resolve().parent.parent
TELEMETRY_FILE = ROOT_DIR / "telemetry" / "zima_telemetry.jsonl"
TELEMETRY_FILE.parent.mkdir(parents=True, exist_ok=True)

ZIMA_HOST = os.environ.get("ZIMA_HOST", "http://192.168.0.206:11434")
DEFAULT_MODEL = os.environ.get("ZIMA_MODEL", "qwen2.5-coder:0.5b")

def log_telemetry(record: dict):
    try:
        with open(TELEMETRY_FILE, "a") as f:
            f.write(json.dumps(record) + "\n")
    except Exception:
        pass

def query_zima(prompt: str, model: str = DEFAULT_MODEL, max_tokens: int = 256) -> str:
    url = f"{ZIMA_HOST}/api/generate"
    payload = {
        "model": model,
        "prompt": prompt,
        "stream": False,
        "options": {
            "num_predict": max_tokens,
            "temperature": 0.2
        }
    }
    data = json.dumps(payload).encode("utf-8")
    req = urllib.request.Request(url, data=data, headers={"Content-Type": "application/json"})
    start_time = time.time()
    try:
        with urllib.request.urlopen(req, timeout=600) as resp:
            res = json.loads(resp.read().decode("utf-8"))
            elapsed = time.time() - start_time
            response_text = res.get("response", "")
            eval_count = res.get("eval_count", 0)
            tok_s = round(eval_count / max(0.001, elapsed), 2)
            log_telemetry({
                "timestamp": time.time(),
                "model": model,
                "eval_count": eval_count,
                "elapsed": elapsed,
                "tokens_per_sec": tok_s,
                "status": "success"
            })
            return response_text
    except Exception as e:
        log_telemetry({
            "timestamp": time.time(),
            "model": model,
            "error": str(e),
            "status": "failed"
        })
        raise RuntimeError(f"ZimaBoard query failed: {e}")

def extract_rust_code(response: str) -> str:
    if "```rust" in response:
        parts = response.split("```rust")
        code = parts[1].split("```")[0]
        return code.strip()
    elif "```" in response:
        parts = response.split("```")
        code = parts[1].split("```")[0]
        return code.strip()
    return response.strip()

def transpile_file(source_file: Path, target_crate: str, model: str = DEFAULT_MODEL) -> Optional[str]:
    print(f"[*] Reading source: {source_file}")
    try:
        source_code = source_file.read_text(errors="ignore")
    except Exception as e:
        print(f"[!] Error reading file: {e}")
        return None
    lines = source_code.splitlines()

    # If file is too large for single chunk on 0.5B / 2B model, truncate or handle first struct/fn
    sample = "\n".join(lines[:60])
    system_prompt = (
        "You are an expert Rust systems engineer. Convert the following C/C++ Blender struct or function "
        "definitions into 100% idiomatic, Safe Rust. Use #[derive(Debug, Clone, PartialEq, Default)] and "
        "serde #[derive(Serialize, Deserialize)] where appropriate. Return ONLY valid Rust code inside a ```rust block.\n\n"
        f"{sample}"
    )

    print(f"[*] Sending prompt to ZimaBoard ({model})...")
    start = time.time()
    try:
        raw = query_zima(system_prompt, model=model)
    except Exception as e:
        print(f"[!] Remote query failed: {e}")
        return None
    elapsed = time.time() - start
    rust_code = extract_rust_code(raw)

    print(f"[✓] Received {len(rust_code.splitlines())} lines of Rust in {elapsed:.1f}s")
    return rust_code

def main():
    parser = argparse.ArgumentParser(description="ZimaBoard Remote Transpile Helper")
    parser.add_argument("--source", type=Path, help="C/C++ source file to transpile")
    parser.add_argument("--target", type=str, default="blender-dna", help="Target crate")
    parser.add_argument("--model", type=str, default=DEFAULT_MODEL, help="Model to use (gemma:2b or qwen2.5-coder:0.5b)")
    parser.add_argument("--prompt", type=str, help="Direct prompt test")
    args = parser.parse_args()

    if args.prompt:
        print(f"[*] Prompting ZimaBoard ({args.model})...")
        res = query_zima(args.prompt, model=args.model)
        print(extract_rust_code(res))
    elif args.source:
        transpile_file(args.source, args.target, model=args.model)
    else:
        # Default health check
        print(f"[*] Checking ZimaBoard status at {ZIMA_HOST}...")
        try:
            req = urllib.request.Request(f"{ZIMA_HOST}/api/tags")
            with urllib.request.urlopen(req, timeout=3) as resp:
                data = json.loads(resp.read().decode())
                models = [m.get("name") for m in data.get("models", [])]
                print(f"[✓] ZimaBoard online! Available models: {', '.join(models)}")
        except Exception as e:
            print(f"[!] Could not connect to ZimaBoard: {e}")
            sys.exit(1)

if __name__ == "__main__":
    main()
