#!/usr/bin/env bash
# Autonomous Blender C/C++ to Rust Subsystem Batch Campaign
# Powered by zev-rs (microsecond zero-token routing) and apfel-rs (Apple Intelligence)
set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$ROOT_DIR"

SUBSYSTEMS=(
    "bmesh"
    "makesdna"
    "blenlib"
    "io/wavefront_obj"
    "io/ply"
    "io/stl"
    "geometry"
    "blenkernel"
)

echo "================================================================"
echo " Starting Full Blender to Rust Autonomous Conversion Campaign"
echo " Workspace: $ROOT_DIR"
echo " Time: $(date)"
echo "================================================================"

for subsys in "${SUBSYSTEMS[@]}"; do
    echo ""
    echo ">>> Processing Subsystem: $subsys <<<"
    ./scripts/port-runner.py --subsystem "$subsys" --limit 50 || {
        echo "[-] Subsystem $subsys had failures or completed batch. Continuing to next..."
    }
done

echo ""
echo "================================================================"
echo " Subsystem Campaign Pass Complete"
echo " Running Full Test Suite..."
echo "================================================================"
cargo test
