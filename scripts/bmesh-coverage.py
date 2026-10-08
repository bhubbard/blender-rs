#!/usr/bin/env python3
"""BMesh API coverage: upstream public BM_* functions vs hand-ported Rust methods.

Scans the upstream bmesh headers for declared `BM_*` functions and reports which have a
Rust equivalent listed in PORTED below (name -> Rust method on `BMesh`).

Usage: scripts/bmesh-coverage.py [--missing] [--header NAME]
"""
import argparse
import re
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
HEADERS = ROOT / "upstream/source/blender/bmesh"

PORTED = {
    "BM_vert_create": "vert_create",
    "BM_edge_create": "edge_create",
    "BM_face_create": "face_create",
    "BM_face_kill": "face_kill",
    "BM_edge_kill": "edge_kill",
    "BM_vert_kill": "vert_kill",
    "BM_edge_split": "edge_split",
    "BM_face_split": "face_split",
    "BM_faces_join_pair": "faces_join_pair",
    "BM_faces_join": "faces_join",
    "BM_edge_collapse": "edge_collapse",
    "BM_vert_collapse_edge": "edge_collapse",
    "BM_edge_rotate": "edge_rotate",
    "BM_vert_separate": "vert_separate",
    "BM_face_normal_flip": "face_reverse",
    "BM_face_triangulate": "face_triangulate_fan",
    "BM_edge_exists": "edge_exists",
    "BM_edge_other_vert": "edge_other_vert",
    "BM_edge_is_wire": "edge_is_wire",
    "BM_edge_is_boundary": "edge_is_boundary",
    "BM_edge_is_manifold": "edge_is_manifold",
    "BM_vert_in_face": "vert_in_face",
    "BM_vert_is_wire": "vert_is_wire",
    "BM_edge_share_face_check": "edges_share_face",
    "BM_vert_select_set": "vert_select_set",
    "BM_elem_float_data_set": "elem_float_data_set",
    "BM_elem_float_data_get": "elem_float_data_get",
    "BM_mesh_elem_count": "elem_count",
    "BM_data_interp_from_verts": "vert_data_interp_pair",
    "BM_edge_calc_length": "edge_calc_length",
    "BM_edge_calc_length_squared": "edge_calc_length_squared",
    "BM_face_calc_normal": "face_calc_normal",
    "BM_face_calc_area": "face_calc_area",
    "BM_face_calc_perimeter": "face_calc_perimeter",
    "BM_face_calc_center_median": "face_calc_center_median",
    "BM_face_calc_center_bounds": "face_calc_center_bounds",
    "BM_edge_share_vert": "edge_share_vert",
    "BM_edge_share_vert_check": "edge_share_vert_check",
    "BM_face_share_edge_count": "face_share_edge_count",
    "BM_face_share_edge_check": "face_share_edge_check",
    "BM_face_share_vert_count": "face_share_vert_count",
    "BM_face_share_vert_check": "face_share_vert_check",
    "BM_face_exists": "face_exists",
    "BM_face_find_double": "face_find_double",
    "BM_edge_find_double": "edge_find_double",
    "BM_face_find_longest_loop": "face_find_longest_loop",
    "BM_face_find_shortest_loop": "face_find_shortest_loop",
    "BM_edge_calc_face_angle": "edge_calc_face_angle",
    "BM_edge_calc_face_angle_ex": "edge_calc_face_angle_ex",
    "BM_edge_calc_face_angle_signed": "edge_calc_face_angle_signed",
    "BM_edge_calc_face_angle_signed_ex": "edge_calc_face_angle_signed_ex",
    "BM_edge_is_convex": "edge_is_convex",
}

DECL = re.compile(r"\b(BM_[A-Za-z0-9_]+)\s*\(")


def declared():
    by_header = defaultdict(set)
    for h in sorted(HEADERS.rglob("*.hh")) + sorted(HEADERS.rglob("*.h")):
        for line in h.read_text(errors="replace").splitlines():
            s = line.strip()
            # Declarations only: skip macros, comments and call sites.
            if s.startswith(("#", "//", "*", "/*", "return")) or "=" in s.split("(")[0]:
                continue
            m = DECL.search(s)
            if m and not m.group(1).isupper():
                by_header[h.name].add(m.group(1))
    return by_header


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--missing", action="store_true")
    ap.add_argument("--header")
    args = ap.parse_args()
    by_header = declared()
    allfn = set().union(*by_header.values()) if by_header else set()
    done = {f for f in allfn if f in PORTED}
    if args.missing:
        for f in sorted(allfn - done):
            print(f)
        return
    print(f"{'header':<34}{'ported':>7}{'total':>7}")
    for h, fns in sorted(by_header.items(), key=lambda kv: -len(kv[1])):
        if args.header and args.header not in h:
            continue
        p = len(fns & done)
        if p or len(fns) >= 10:
            print(f"{h:<34}{p:>7}{len(fns):>7}")
    pct = 100 * len(done) / max(len(allfn), 1)
    print(f"\nTOTAL: {len(done)} / {len(allfn)} upstream BM_* functions ported ({pct:.1f}%)")
    stale = [k for k in PORTED if k not in allfn]
    if stale:
        print("note: PORTED names not found in headers:", ", ".join(stale))


if __name__ == "__main__":
    main()
