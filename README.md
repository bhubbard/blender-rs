# Blender-rs (`blender`)

[![Rust 2021](https://img.shields.io/badge/rust-2021_edition-000000?style=flat-square&logo=rust)](https://github.com/bhubbard/blender-rs)
[![Live 3D Playground](https://img.shields.io/badge/live_docs_%26_3D_playground-code.brandonhubbard.com%2Fblender--rs-0055ff?style=flat-square)](https://code.brandonhubbard.com/blender-rs/)
[![Benchmarks](https://img.shields.io/badge/benchmarks-1.11x--1.31x_vs_C%2B%2B-16a34a?style=flat-square)](./BENCHMARKS.md)
[![License: GPL-2.0-or-later](https://img.shields.io/badge/license-GPL--2.0--or--later-orange?style=flat-square)](https://www.gnu.org/licenses/old-licenses/gpl-2.0.html)

**Blender-rs** is a memory-safe, SIMD-accelerated Rust re-architecture of the [Blender](https://www.blender.org/) 3D creation suite (`blenlib`, `makesdna`, `bmesh`, `blenkernel`, and `io`).

Spanning **11 workspace crates**, **808 Rust source files**, and **69,902 lines of Rust code** across **773 ported Blender C/C++ modules**, `blender-rs` eliminates raw C pointer aliasing and use-after-free bugs in self-referential half-edge meshes while outperforming upstream C/C++ kernels by **1.11x–1.31x**.

- **Interactive 3D BMesh Playground & Docs**: [https://code.brandonhubbard.com/blender-rs/](https://code.brandonhubbard.com/blender-rs/)
- **Empirical Benchmarks Report**: [BENCHMARKS.md](./BENCHMARKS.md)
- **Porting Architecture & Mapping Guide**: [PORTING.md](./PORTING.md)

---

## Workspace Architecture (11 Crates)

| Crate | Files / LOC | Upstream Blender Module | Description |
| :--- | ---: | :--- | :--- |
| **[`blender-math`](./crates/blender-math)** | 57 / 1,570 | `source/blender/blenlib/BLI_math_*` | SIMD-accelerated linear algebra (`glam` `Vec2`/`Vec3`/`Vec4`/`Mat3`/`Mat4`/`Quat`), `Bounds2D`, `BoundingBox`, Garland-Heckbert `Quadric` error metrics, XPBD physics constraints, and Gaussian/Deriche/Van Vliet blur kernels. |
| **[`blender-mem`](./crates/blender-mem)** | 21 / 662 | `source/blender/blenlib/BLI_mempool.h` | Generational `Handle<T>` (`index: u32, generation: u32`) and `MemPool<T>` free-list arena allocator eliminating dangling pointers in cyclic graph topologies. |
| **[`blender-dna`](./crates/blender-dna)** | 172 / 42,090 | `source/blender/makesdna` | Serialized Blender DNA data schemas (`DNA_*_types.h`), `MeshElemFlags` bitflags (`SELECT`, `HIDE`, `SEAM`, `SHARP`, `SMOOTH`), and `CustomDataType` layer definitions. |
| **[`blender-bmesh`](./crates/blender-bmesh)** | 115 / 6,086 | `source/blender/bmesh` | Full non-manifold-capable half-edge polygon mesh kernel (`BMVert`, `BMEdge`, `BMLoop`, `BMFace`), disk/radial/face-loop cycles, Newell normals, Euler operators (`vert_create`, `edge_create`, `face_create`, `edge_split`), and mesh operators (`bevel`, `bisect`, `bridge`, `connect`, `dissolve`, `extrude`, `subdivide`, `triangulate`). |
| **[`blender-geom`](./crates/blender-geom)** | 13 / 1,917 | `source/blender/blenlib/intern/math_geom.cc` | Hand-ported computational geometry kernels: top-down median-split `Bvh` (raycast & nearest-point), implicit pointerless 3D `KdTree` (`find_nearest`, `find_nearest_n`, `range_search`), ear-clipping `triangulate_polygon_v2`, monotone chain `convex_hull_v2`, and ray/triangle/plane/line intersections. |
| **[`blender-io`](./crates/blender-io)** | 419 / 15,511 | `source/blender/io/*` | High-throughput mesh importers, exporters, and scene filter pipelines for Wavefront OBJ, Stanford PLY, STL, and Universal Scene Description (USD). |
| **[`blender-obj`](./crates/blender-obj)** | 1 / 327 | `source/blender/io/wavefront_obj` | Standalone zero-dependency Wavefront `.obj` parser and writer supporting arbitrary n-gons, negative relative indices, `vn`/`vt` attributes, and ear-clipping fan triangulation. |
| **[`blender-stl`](./crates/blender-stl)** | 1 / 304 | `source/blender/io/stl` | Auto-detecting binary and ASCII `.stl` reader/writer with bit-identical `[u32; 3]` vertex welding (`to_indexed` / `from_indexed`). |
| **[`blender-bridge`](./crates/blender-bridge)** | 1 / 142 | `source/blender/blenkernel/intern/mesh_convert.cc` | Lossless bidirectional conversion (`obj_to_bmesh`, `bmesh_to_obj`) preserving n-gon face loops and verifying manifold Euler characteristic ($\chi = V - E + F = 2$). |
| **[`blender-cluster`](./crates/blender-cluster)** | 5 / 905 | Transpilation & verification tooling | AST call-graph clusterer and automated C/C++ to Rust subsystem verification harness. |
| **[`blender-app`](./crates/blender-app)** | 3 / 388 | `source/creator` | `blender` CLI binary with procedural primitive generation (`cube`, `sphere`), multi-pass edge subdivision, Wavefront OBJ export, real-time 3D terminal Bresenham wireframe viewport, and automated benchmark runner. |

---

## Quickstart & CLI Usage

### 1. Build and Test the Workspace

```bash
cargo build --workspace --release
cargo test --workspace
```

### 2. Render the 3D Terminal Wireframe Viewport

```bash
# Default Cube with 1 subdivision pass rendered in the terminal 3D Bresenham viewport
cargo run --release -p blender-app --bin blender -- --primitive cube --subdivide 1 --viewport

# UV Sphere (32x16) rendered in the terminal viewport and exported to sphere.obj
cargo run --release -p blender-app --bin blender -- --primitive sphere --export-obj sphere.obj
```

### 3. Run the Built-In Performance Benchmark

```bash
cargo run --release -p blender-app --bin blender -- --benchmark
```

### 4. Code Coverage Analysis

Generate pure Rust LLVM source-based code coverage reports across the entire workspace:

```bash
# Terminal summary table
cargo coverage-summary
# Or: make coverage-summary

# Interactive HTML report (saved to target/llvm-cov/html/index.html)
cargo coverage
# Or: make coverage

# Open the interactive HTML report directly in your default browser
cargo coverage-open
# Or: make coverage-open

# Standard LCOV file for IDEs & CI (saved to lcov.info)
cargo coverage-lcov
# Or: make coverage-lcov
```

---

## Rust SDK Examples

### Half-Edge `BMesh` Construction & Euler Validation

```rust
use blender_bmesh::BMesh;
use blender_math::Vec3;

let mut bm = BMesh::new();
let v0 = bm.vert_create(Some(Vec3::new(0.0, 0.0, 0.0)), None);
let v1 = bm.vert_create(Some(Vec3::new(1.0, 0.0, 0.0)), None);
let v2 = bm.vert_create(Some(Vec3::new(1.0, 1.0, 0.0)), None);
let v3 = bm.vert_create(Some(Vec3::new(0.0, 1.0, 0.0)), None);

let face_h = bm.face_create(&[v0, v1, v2, v3], None).unwrap();
bm.recalc_face_normals();
assert!(bm.validate_topology().is_ok());

// Split the first edge at its midpoint (t = 0.5), promoting the quad into a 5-gon
let first_edge = bm.epool.iter().next().unwrap().0;
let (_new_edge, mid_vert) = bm.edge_split(first_edge, 0.5).unwrap();
assert_eq!(bm.vpool.get(mid_vert).unwrap().co, Vec3::new(0.5, 0.0, 0.0));
assert_eq!(bm.fpool.get(face_h).unwrap().len, 5);
```

### Spatial Queries with `Bvh` and `KdTree` (`blender-geom`)

```rust
use blender_geom::{bvh::Bvh, kdtree::KdTree};
use glam::Vec3;

// 1. Triangle BVH raycasting & closest surface point query
let tri = [Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 4.0, 0.0)];
let bvh = Bvh::new(&[tri]);
let hit = bvh.ray_cast(Vec3::new(1.0, 1.0, 3.0), -Vec3::Z).unwrap();
assert_eq!(hit.index, 0);
assert!((hit.t - 3.0).abs() < 1e-6);

// 2. Implicit array 3D KD-Tree k-nearest and radius queries
let points = [Vec3::ZERO, Vec3::X, Vec3::Y, Vec3::Z];
let kdtree = KdTree::new(&points);
let nearest = kdtree.find_nearest_n(Vec3::new(0.9, 0.1, 0.0), 2);
assert_eq!(nearest[0].index, 1); // Vec3::X is closest
```

---

## Benchmark Summary

See [BENCHMARKS.md](./BENCHMARKS.md) for full methodology and reproduction commands.

| Subsystem | Workload | `blender-rs` (Rust) | Blender 4.2 (C/C++) | Speedup |
| :--- | :--- | ---: | ---: | :--- |
| `blender-mem` | 1M `MemPool<BMVert>` alloc + 50% churn | **14.2 ms** | 16.8 ms | **1.18x faster** |
| `blender-bmesh` | Cube + 3x `edge_split` + topology validation | **41.8 µs** | 46.5 µs | **1.11x faster** |
| `blender-geom` | `Bvh::new` (200,000 triangles) | **28.4 ms** | 34.1 ms | **1.20x faster** |
| `blender-geom` | `KdTree::new` (100K pts) + 10K `find_nearest_n(16)` | **15.0 ms** | 19.3 ms | **1.29x faster** |
| `blender-stl` | Binary STL parse (100K tris) + vertex weld | **8.7 ms** | 11.4 ms | **1.31x faster** |
| `blender-bridge` | Lossless `obj_to_bmesh` + `bmesh_to_obj` (10K faces) | **6.2 ms** | 7.9 ms | **1.27x faster** |

---

## License

Licensed under the [GNU General Public License v2.0 or later (GPL-2.0-or-later)](https://www.gnu.org/licenses/old-licenses/gpl-2.0.html), preserving full license compatibility with upstream Blender.
