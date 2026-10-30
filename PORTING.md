# Blender C/C++ to Rust Porting Architecture

This document records the subsystem mapping, architectural invariants, and module burndown status for **`blender-rs`**.

## 1. Subsystem Mapping (11 Workspace Crates)

| Original Blender Path | Rust Workspace Crate | Porting Strategy & Architectural Notes |
| :--- | :--- | :--- |
| `source/blender/blenlib/BLI_math_*` | [`crates/blender-math`](./crates/blender-math) | Replaced scalar C float arrays (`float v[3]`, `float m[4][4]`) with `glam` SIMD types (`Vec2`, `Vec3`, `Vec4`, `Mat3`, `Mat4`, `Quat`), `Bounds2D`, `BoundingBox`, Garland-Heckbert `Quadric`, XPBD constraints, and Gaussian/Deriche/Van Vliet blur filters. |
| `source/blender/blenlib/BLI_mempool.h` | [`crates/blender-mem`](./crates/blender-mem) | Replaced raw pointer pool arenas with `MemPool<T>` and generational `Handle<T>` (`index: u32`, `generation: u32`), providing $O(1)$ free-list recycling with compile-time and runtime use-after-free protection. |
| `source/blender/makesdna/DNA_*` | [`crates/blender-dna`](./crates/blender-dna) | Ported 172 DNA header definitions (`DNA_mesh_types`, `DNA_object_types`, `DNA_scene_types`, `DNA_modifier_types`, `DNA_node_types`, etc.) into idiomatic `#[repr(C)]` Rust structs, `bitflags!` (`MeshElemFlags`), and strongly-typed enums (`CustomDataType`). |
| `source/blender/bmesh/*` | [`crates/blender-bmesh`](./crates/blender-bmesh) | Ported the full half-edge polygon mesh kernel (`BMVert`, `BMEdge`, `BMLoop`, `BMFace`), disk/radial/face-loop cycles, Newell polygon normal calculation, Euler operators (`vert_create`, `edge_create`, `face_create`, `edge_split`), and high-level mesh operators (`bevel`, `bisect`, `bridge`, `connect`, `dissolve`, `extrude`, `subdivide`, `triangulate`). |
| `source/blender/blenlib/intern/math_geom.cc` | [`crates/blender-geom`](./crates/blender-geom) | Hand-ported computational geometry primitives: `Aabb`, `Bvh` (top-down median-split raycasting & closest-point), implicit pointerless 3D `KdTree`, `polyfill` 2D ear-clipping triangulation, monotone-chain `convex_hull_v2`, and ray/triangle/plane/line intersections. |
| `source/blender/io/*` | [`crates/blender-io`](./crates/blender-io) | Ported 419 I/O modules across Wavefront OBJ, Stanford PLY, STL, and USD importers/exporters. |
| `source/blender/io/wavefront_obj` | [`crates/blender-obj`](./crates/blender-obj) | Standalone zero-dependency Wavefront `.obj` parser/serializer supporting n-gons, negative relative indices, `vn`/`vt` attributes, and ear-clipping triangulation. |
| `source/blender/io/stl` | [`crates/blender-stl`](./crates/blender-stl) | Standalone ASCII & binary `.stl` reader/writer with bit-identical `[u32; 3]` vertex welding (`to_indexed` / `from_indexed`). |
| `source/blender/blenkernel/intern/mesh_convert.cc` | [`crates/blender-bridge`](./crates/blender-bridge) | Lossless bidirectional conversion (`obj_to_bmesh`, `bmesh_to_obj`) preserving n-gon face loops and verifying manifold Euler characteristic ($\chi = V - E + F = 2$). |
| `tools/cluster_verifier` | [`crates/blender-cluster`](./crates/blender-cluster) | AST dependency clusterer and automated C/C++ to Rust subsystem verification harness. |
| `source/creator` | [`crates/blender-app`](./crates/blender-app) | `blender` CLI binary with procedural primitive generation (`cube`, `sphere`), multi-pass edge subdivision, Wavefront OBJ export, real-time 3D terminal Bresenham wireframe viewport, and benchmark harness. |

---

## 2. Key Architectural Decisions

1. **Eliminating Raw Pointer Cycles in `BMesh`**:
   In C, `BMesh` is a tightly coupled graph of cyclic doubly-linked lists (`BMDiskLink` around vertices, `BMLoop` radial cycles around edges, and `BMLoop` boundary cycles around faces). Direct pointer translation in Rust would require pervasive `unsafe` blocks or `Rc<RefCell<...>>` reference cycles. By allocating `BMVert`, `BMEdge`, `BMLoop`, and `BMFace` in dedicated `MemPool<T>` arenas and linking them via 8-byte `Handle<T>` values, all topological traversal and mutation (`edge_split`, `face_create`, `disk_edge_append`, `radial_loop_append`) is 100% safe, deterministic, and cache-contiguous.
2. **Topological Self-Verification (`validate_topology`)**:
   Every primitive constructor and mesh operator validates four structural invariants:
   - Every edge's `v1` and `v2` handles resolve to live vertices in `vpool` (`v1 != v2`).
   - Every edge appears in the circular disk cycle of both of its endpoint vertices (`disk_contains_edge`).
   - Every face loop's `next`/`prev` pointers form a closed cycle of exact length `face.len`, and its `radial_next`/`radial_prev` pointers form a valid radial cycle on its parent edge.
   - Closed genus-0 manifolds satisfy the Euler-Poincaré formula $\chi = V - E + F = 2$.
3. **Pointerless Spatial Trees (`Bvh` & `KdTree`)**:
   Both `blender_geom::bvh::Bvh` and `blender_geom::kdtree::KdTree` flatten tree nodes into single contiguous `Vec<Node>` buffers. `KdTree` eliminates child indices entirely by storing the median of each subrange at the midpoint of the slice (`nodes[mid]`), cutting node size by 50% compared to `BLI_kdtree_3d`.

---

## 3. Porting Telemetry Summary

- **Workspace Crates**: 11
- **Rust Source Files**: 808 `.rs` files (`69,902` lines of Rust code)
- **Verified Ported C/C++ Modules**: 773 (tracked in [`PORTING_PROGRESS.json`](./PORTING_PROGRESS.json))
- **Empirical Benchmarks**: Documented in [`BENCHMARKS.md`](./BENCHMARKS.md)
