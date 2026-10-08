#!/usr/bin/env python3
"""Audit ported modules: separate real implementations from transpiled stubs.

The port-runner counts any module that compiles as "completed", but many only
contain opaque struct placeholders or constants. This classifies every
`crates/*/src/*.rs` by content so progress can be measured honestly.

  real   : defines at least one function with a body (excluding tests)
  consts : only constants / type aliases / enums
  stub   : only `_opaque` placeholder structs (or empty)

Usage: scripts/port-audit.py [--list stub|consts|real] [--json]
"""
import argparse
import json
import re
from collections import Counter, defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FN_RE = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:unsafe\s+)?fn\s+\w+", re.M)
CONST_RE = re.compile(r"^\s*(?:pub\s+)?(?:const|static|type|enum)\s+\w+", re.M)


def classify(path: Path) -> str:
    text = path.read_text(errors="replace")
    # Ignore everything from the test module onwards.
    text = text.split("#[cfg(test)]")[0]
    if FN_RE.search(text):
        return "real"
    if CONST_RE.search(text):
        return "consts"
    return "stub"


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--list", choices=["stub", "consts", "real"])
    ap.add_argument("--json", action="store_true")
    args = ap.parse_args()

    per_crate = defaultdict(Counter)
    files = defaultdict(list)
    for p in sorted(ROOT.glob("crates/*/src/*.rs")):
        if p.name in ("lib.rs", "main.rs"):
            continue
        kind = classify(p)
        per_crate[p.parts[-3]][kind] += 1
        files[kind].append(str(p.relative_to(ROOT)))

    if args.list:
        print("\n".join(files[args.list]))
        return
    if args.json:
        print(json.dumps({c: dict(v) for c, v in per_crate.items()}, indent=2))
        return

    print(f"{'crate':<18}{'real':>6}{'consts':>8}{'stub':>6}")
    total = Counter()
    for crate, c in sorted(per_crate.items()):
        print(f"{crate:<18}{c['real']:>6}{c['consts']:>8}{c['stub']:>6}")
        total.update(c)
    print(f"{'TOTAL':<18}{total['real']:>6}{total['consts']:>8}{total['stub']:>6}")


if __name__ == "__main__":
    main()
