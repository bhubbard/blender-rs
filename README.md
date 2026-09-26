# blender-rs

A memory-safe, high-performance Rust port of [Blender](https://github.com/blender/blender), orchestrated using the `port-to-rust-playbook`, `rust-crate-decomposition`, and `rust-lifetimes-analysis` methodology.

---

## Architecture & Crate Workspace

```
blender-rs/
├── crates/
│   ├── blender-math      # SIMD vectors, matrices, bounds (BLI_math_*)
│   ├── blender-mem       # Generational chunked memory pools (BLI_mempool)
│   ├── blender-dna       # Serialized core data schemas (makesdna)
│   ├── blender-bmesh     # Half-edge polygon modeling core (source/blender/bmesh)
│   └── blender-io        # Wavefront OBJ, PLY, USD I/O filters (source/blender/io)
├── upstream/             # Upstream Blender C/C++ git tree for 1:1 test parity & diffing
├── PORTING.md            # Architectural decisions, type mappings, and execution sequence
└── LIFETIMES.tsv         # Ownership & lifetime resolutions for BMesh pointer graphs
```

---

## Key Guarantees & Philosophy

1. **Mechanical Port First:** 1:1 parity with Blender upstream data structures and algorithms before refactoring into idiomatic Rust.
2. **Generational Safety:** Self-referential BMesh pointer cycles (`BMVert*`, `BMEdge*`, `BMLoop*`, `BMFace*`) are mapped to generational handles inside chunked memory pools, eliminating use-after-free and dangling pointer bugs.
3. **Upstream Test Parity:** Verified against upstream Blender test suites (e.g. `bmesh_core_test.cc`).

---

## Building and Testing

```bash
cargo build
cargo test
```
