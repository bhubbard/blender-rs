# Blender to Rust Porting Guide (PORTING.md)

This document defines the formal rules, architecture, type mappings, and execution sequence for porting the Blender codebase to Rust (`blender-rs`), following the `port-to-rust-playbook`, `rust-crate-decomposition`, and `rust-lifetimes-analysis` methodology.

---

## 1. Decision Record

### Incremental vs. Big-Bang Workspace
* **Architecture:** **Acyclic Crate Workspace with Subsystem Milestones.**
* **Strategy:** Blender is ~4M lines of C/C++. A monolithic single-crate port causes compilation bottlenecks and cyclic dependencies. We decompose Blender into decoupled, leaf-to-root crates (`blender-math`, `blender-mem`, `blender-dna`, `blender-bmesh`, `blender-io`, etc.).
* **First Milestone:** **`blender-bmesh`** (Blender's core BRep/half-edge polygon mesh modeling kernel) and supporting leaf crates.

### Mechanical vs. Idiomatic Translation
* **Rule: Mechanical First, Idiomatic Post-Ship.**
* In accordance with `port-planning`, translation must mirror Blender's internal data flow and algorithmic structures 1:1 initially.
* This ensures verification against Blender's C/C++ test suite and eliminates "redesign bugs" during porting.
* Once each crate achieves test parity, refactor passes optimize for idiomatic Rust patterns.

---

## 2. Crate Architecture (Acyclic Graph)

```
        blender-io (Wavefront OBJ, PLY, USD)
             │
             ▼
        blender-bmesh (BMesh half-edge core & Euler ops)
        ┌────┴──────────────────────────┐
        ▼                               ▼
   blender-dna (Data structs)     blender-mem (Mempool & Arenas)
        └──────────────┬────────────────┘
                       ▼
                 blender-math (SIMD vectors, matrices, bounds)
```

| Crate | Upstream C/C++ Equivalent | Purpose |
|---|---|---|
| `blender-math` | `source/blender/blenlib/BLI_math*.h` | Vectors, matrices, quaternions, geometry math |
| `blender-mem` | `BLI_mempool`, `MEM_guardedalloc` | Fast fixed-size block allocators, slot maps |
| `blender-dna` | `source/blender/makesdna` | Blender core serialized data schemas (`DNA_mesh_types.h`) |
| `blender-bmesh` | `source/blender/bmesh` | Vert, Edge, Loop, Face half-edge mesh kernel & Euler ops |
| `blender-io` | `source/blender/io` | Importers & Exporters for roundtrip validation |

---

## 3. Type & Pattern Mappings

### Core Types & Math
| C/C++ (Blender) | Rust Target | Rationale |
|---|---|---|
| `float v[3]`, `float3` | `glam::Vec3` or `[f32; 3]` | 16-byte SIMD alignment, standard 3D math |
| `float m[4][4]`, `float4x4` | `glam::Mat4` | Column-major 4x4 matrix operations |
| `bool`, `char`, `int`, `uint` | `bool`, `i8`, `i32`, `u32` | Explicitly sized integers |
| `size_t`, `intptr_t` | `usize`, `isize` | Platform-native indexing |
| `const char *` | `&str` / `CString` / `SmartString` | Zero-allocation strings where possible |

### Memory Management & BMesh Topology
In Blender C, BMesh is a self-referential pointer graph:
```c
struct BMVert { BMHeader head; float co[3], no[3]; BMEdge *e; ... };
struct BMEdge { BMHeader head; BMVert *v1, *v2; BMLoop *l; ... };
struct BMLoop { BMHeader head; BMVert *v; BMEdge *e; BMFace *f; ... };
struct BMFace { BMHeader head; BMLoop *l_first; int len; ... };
```
In Safe Rust, self-referential raw pointer webs violate borrow checker rules and cause alias ambiguity:
* **Storage Solution:** `Arena` / `SlotMap` / `IndexHandle` design.
* **Identifiers:** `VertId(u32)`, `EdgeId(u32)`, `LoopId(u32)`, `FaceId(u32)`.
* **Generations:** Versioned slot indices to prevent use-after-free or dangling references when vertices/edges are dissolved or collapsed.
* **Memory Pool:** `blender-mem` provides a `Pool<T>` mirroring `BLI_mempool` with cache-coherent chunked allocations.

### Error Handling
| C Idiom | Rust Pattern |
|---|---|
| `NULL` return on failure | `Option<T>` |
| Integer status codes (`0`, `-1`) | `Result<T, BMeshError>` |
| `BLI_assert(cond)` | `debug_assert!(cond)` (Note: never put side effects inside `debug_assert!`) |

---

## 4. Porting Traps Checklist (from `rust-porting-mistakes`)

1. **No side effects in `debug_assert!`**: Never write `debug_assert!(pool.remove(v))`; the call disappears in release builds!
2. **Bounds checks in hot loops**: Blender C assumes unchecked pointer arithmetic. Use iterators or chunking; only use `get_unchecked` where verified with benchmarks.
3. **Struct layout & alignment**: When mapping DNA structures that need binary compatibility with `.blend` files, annotate with `#[repr(C)]`.
4. **Float NaN & comparison**: Rust `f32::total_cmp` vs standard partial ordering when sorting vertices or KD-tree partitioning.

---

## 5. Trial Run Sequence
Before bulk porting, complete the 3-file trial run:
1. `blender-math`: Vector/matrix projection and intersection tests (`BLI_math_geom.c`).
2. `blender-mem`: Chunked memory pool allocator (`BLI_mempool.c`).
3. `blender-bmesh`: Mesh creation and Euler operator `BM_vert_create` + `BM_edge_create` + `BM_face_create`.
