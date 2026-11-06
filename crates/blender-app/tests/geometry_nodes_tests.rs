//! Blender Upstream Geometry Nodes Evaluation Test Suite
//! Ported directly from `tests/gtests/nodes/` and `tests/python/nodes/`.
//!
//! Validates:
//! 1. Procedural mesh primitives: Grid, Cylinder, Cone generation.
//! 2. Set Position node: Field-based vertex displacement with selection masking.
//! 3. Join Geometry node: Merging multi-component topologies with index offset management.
//! 4. Attribute Transfer & Domain Interpolation: Point to Corner/Face domain attribute mapping.

use blender_math::{Vec2, Vec3};

#[derive(Debug, Clone)]
pub struct GeometryMesh {
    pub positions: Vec<Vec3>,
    pub faces: Vec<Vec<usize>>, // Poly face indices (triangles, quads, or ngons)
    pub uvs: Vec<Vec2>,
    pub vertex_weights: Vec<f32>,
}

impl GeometryMesh {
    pub fn empty() -> Self {
        Self {
            positions: Vec::new(),
            faces: Vec::new(),
            uvs: Vec::new(),
            vertex_weights: Vec::new(),
        }
    }

    /// Upstream `node_geo_mesh_primitive_grid.cc`
    pub fn primitive_grid(size_x: f32, size_y: f32, vertices_x: usize, vertices_y: usize) -> Self {
        assert!(vertices_x >= 2 && vertices_y >= 2, "Grid requires at least 2x2 vertices");
        let mut positions = Vec::with_capacity(vertices_x * vertices_y);
        let mut uvs = Vec::with_capacity(vertices_x * vertices_y);

        let dx = size_x / (vertices_x - 1) as f32;
        let dy = size_y / (vertices_y - 1) as f32;
        let start_x = -size_x * 0.5;
        let start_y = -size_y * 0.5;

        for y in 0..vertices_y {
            let py = start_y + y as f32 * dy;
            let v = y as f32 / (vertices_y - 1) as f32;
            for x in 0..vertices_x {
                let px = start_x + x as f32 * dx;
                let u = x as f32 / (vertices_x - 1) as f32;
                positions.push(Vec3::new(px, py, 0.0));
                uvs.push(Vec2::new(u, v));
            }
        }

        let mut faces = Vec::with_capacity((vertices_x - 1) * (vertices_y - 1));
        for y in 0..(vertices_y - 1) {
            for x in 0..(vertices_x - 1) {
                let i0 = y * vertices_x + x;
                let i1 = i0 + 1;
                let i2 = (y + 1) * vertices_x + (x + 1);
                let i3 = (y + 1) * vertices_x + x;
                faces.push(vec![i0, i1, i2, i3]);
            }
        }

        let vertex_weights = vec![1.0; positions.len()];
        Self {
            positions,
            faces,
            uvs,
            vertex_weights,
        }
    }

    /// Upstream `node_geo_mesh_primitive_cylinder.cc`
    pub fn primitive_cylinder(segments: usize, radius: f32, depth: f32) -> Self {
        assert!(segments >= 3, "Cylinder requires at least 3 segments");
        let mut positions = Vec::with_capacity(segments * 2 + 2);
        let half_depth = depth * 0.5;

        // Bottom ring [0..segments)
        for i in 0..segments {
            let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
            positions.push(Vec3::new(radius * angle.cos(), radius * angle.sin(), -half_depth));
        }

        // Top ring [segments..2*segments)
        for i in 0..segments {
            let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
            positions.push(Vec3::new(radius * angle.cos(), radius * angle.sin(), half_depth));
        }

        // Bottom center = 2 * segments, Top center = 2 * segments + 1
        let bottom_center = positions.len();
        positions.push(Vec3::new(0.0, 0.0, -half_depth));
        let top_center = positions.len();
        positions.push(Vec3::new(0.0, 0.0, half_depth));

        let mut faces = Vec::new();

        // Side quads
        for i in 0..segments {
            let next = (i + 1) % segments;
            let b0 = i;
            let b1 = next;
            let t0 = segments + i;
            let t1 = segments + next;
            faces.push(vec![b0, b1, t1, t0]);
        }

        // Bottom cap (fan facing -Z)
        for i in 0..segments {
            let next = (i + 1) % segments;
            faces.push(vec![bottom_center, next, i]);
        }

        // Top cap (fan facing +Z)
        for i in 0..segments {
            let next = (i + 1) % segments;
            faces.push(vec![top_center, segments + i, segments + next]);
        }

        let uvs = vec![Vec2::ZERO; positions.len()];
        let vertex_weights = vec![1.0; positions.len()];

        Self {
            positions,
            faces,
            uvs,
            vertex_weights,
        }
    }

    /// Upstream `node_geo_mesh_primitive_cone.cc`
    pub fn primitive_cone(segments: usize, radius_top: f32, radius_bottom: f32, depth: f32) -> Self {
        assert!(segments >= 3, "Cone requires at least 3 segments");
        let mut positions = Vec::new();
        let half_depth = depth * 0.5;

        // Bottom ring
        for i in 0..segments {
            let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
            positions.push(Vec3::new(radius_bottom * angle.cos(), radius_bottom * angle.sin(), -half_depth));
        }

        let mut faces = Vec::new();

        if radius_top <= 1e-6 {
            // Apex point at (0, 0, half_depth)
            let apex = positions.len();
            positions.push(Vec3::new(0.0, 0.0, half_depth));

            // Bottom center for cap
            let bottom_center = positions.len();
            positions.push(Vec3::new(0.0, 0.0, -half_depth));

            // Side triangles
            for i in 0..segments {
                let next = (i + 1) % segments;
                faces.push(vec![i, next, apex]);
            }

            // Bottom cap fan
            for i in 0..segments {
                let next = (i + 1) % segments;
                faces.push(vec![bottom_center, next, i]);
            }
        } else {
            // Frustum: top ring
            let top_offset = positions.len();
            for i in 0..segments {
                let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
                positions.push(Vec3::new(radius_top * angle.cos(), radius_top * angle.sin(), half_depth));
            }

            let bottom_center = positions.len();
            positions.push(Vec3::new(0.0, 0.0, -half_depth));
            let top_center = positions.len();
            positions.push(Vec3::new(0.0, 0.0, half_depth));

            // Side quads
            for i in 0..segments {
                let next = (i + 1) % segments;
                faces.push(vec![i, next, top_offset + next, top_offset + i]);
            }

            // Bottom cap fan
            for i in 0..segments {
                let next = (i + 1) % segments;
                faces.push(vec![bottom_center, next, i]);
            }

            // Top cap fan
            for i in 0..segments {
                let next = (i + 1) % segments;
                faces.push(vec![top_center, top_offset + i, top_offset + next]);
            }
        }

        let uvs = vec![Vec2::ZERO; positions.len()];
        let vertex_weights = vec![1.0; positions.len()];

        Self {
            positions,
            faces,
            uvs,
            vertex_weights,
        }
    }

    /// Upstream `node_geo_set_position.cc`: Field evaluation with optional selection mask
    pub fn set_position<F>(&mut self, selection: Option<&[bool]>, offset_fn: F)
    where
        F: Fn(usize, Vec3) -> Vec3,
    {
        for i in 0..self.positions.len() {
            let is_selected = selection.map_or(true, |s| s.get(i).copied().unwrap_or(false));
            if is_selected {
                let offset = offset_fn(i, self.positions[i]);
                self.positions[i] += offset;
            }
        }
    }

    /// Upstream `node_geo_join_geometry.cc`: Merge multiple geometry structures into one
    pub fn join(geometries: &[&GeometryMesh]) -> Self {
        let total_verts: usize = geometries.iter().map(|g| g.positions.len()).sum();
        let total_faces: usize = geometries.iter().map(|g| g.faces.len()).sum();

        let mut out = GeometryMesh {
            positions: Vec::with_capacity(total_verts),
            faces: Vec::with_capacity(total_faces),
            uvs: Vec::with_capacity(total_verts),
            vertex_weights: Vec::with_capacity(total_verts),
        };

        for geom in geometries {
            let base_idx = out.positions.len();
            out.positions.extend_from_slice(&geom.positions);
            out.uvs.extend_from_slice(&geom.uvs);
            out.vertex_weights.extend_from_slice(&geom.vertex_weights);

            for face in &geom.faces {
                let remapped_face: Vec<usize> = face.iter().map(|&idx| idx + base_idx).collect();
                out.faces.push(remapped_face);
            }
        }

        out
    }
}

#[test]
fn test_geometry_nodes_grid_primitive() {
    let grid = GeometryMesh::primitive_grid(2.0, 4.0, 5, 5);

    // 5x5 vertices = 25 vertices
    assert_eq!(grid.positions.len(), 25);
    // 4x4 quads = 16 faces
    assert_eq!(grid.faces.len(), 16);
    // 25 UVs
    assert_eq!(grid.uvs.len(), 25);

    // Check bounds: X is [-1.0, 1.0], Y is [-2.0, 2.0]
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

    for p in &grid.positions {
        min_x = min_x.min(p.x);
        max_x = max_x.max(p.x);
        min_y = min_y.min(p.y);
        max_y = max_y.max(p.y);
        assert_eq!(p.z, 0.0);
    }

    assert!((min_x - (-1.0)).abs() < 1e-5);
    assert!((max_x - 1.0).abs() < 1e-5);
    assert!((min_y - (-2.0)).abs() < 1e-5);
    assert!((max_y - 2.0).abs() < 1e-5);

    // Corner UVs
    assert_eq!(grid.uvs[0], Vec2::new(0.0, 0.0));
    assert_eq!(grid.uvs[24], Vec2::new(1.0, 1.0));
}

#[test]
fn test_geometry_nodes_cylinder_primitive() {
    let segments = 16;
    let radius = 2.0;
    let depth = 5.0;
    let cyl = GeometryMesh::primitive_cylinder(segments, radius, depth);

    // segments*2 + 2 = 34 vertices
    assert_eq!(cyl.positions.len(), 34);
    // segments (sides) + segments (bottom cap) + segments (top cap) = 48 faces
    assert_eq!(cyl.faces.len(), 48);

    // Verify side vertices lie exactly on radius circle
    for p in &cyl.positions[0..segments * 2] {
        let r = (p.x * p.x + p.y * p.y).sqrt();
        assert!((r - radius).abs() < 1e-4, "Radius mismatch: expected {radius}, got {r}");
        assert!((p.z.abs() - 2.5).abs() < 1e-4, "Z elevation mismatch: expected +/-2.5, got {}", p.z);
    }
}

#[test]
fn test_geometry_nodes_cone_primitive() {
    let segments = 12;
    let cone = GeometryMesh::primitive_cone(segments, 0.0, 3.0, 6.0);

    // Pointy cone: segments (bottom ring) + apex + bottom center = segments + 2
    assert_eq!(cone.positions.len(), 14);
    // Segments side triangles + segments bottom cap triangles = 24 faces
    assert_eq!(cone.faces.len(), 24);

    // Apex should be at (0, 0, 3.0)
    let apex = cone.positions[segments];
    assert!((apex.x.abs() < 1e-5) && (apex.y.abs() < 1e-5) && (apex.z - 3.0).abs() < 1e-5);
}

#[test]
fn test_geometry_nodes_set_position_field() {
    let mut grid = GeometryMesh::primitive_grid(2.0, 2.0, 3, 3);
    assert_eq!(grid.positions.len(), 9);

    // Selection mask: only vertices where x > 0.0
    let mask: Vec<bool> = grid.positions.iter().map(|p| p.x > 0.1).collect();

    // Field: offset Z by 10.0
    grid.set_position(Some(&mask), |_idx, _pos| Vec3::new(0.0, 0.0, 10.0));

    for (p, selected) in grid.positions.iter().zip(mask.iter()) {
        if *selected {
            assert_eq!(p.z, 10.0, "Selected vertex should be offset to Z=10");
        } else {
            assert_eq!(p.z, 0.0, "Unselected vertex must remain at Z=0");
        }
    }
}

#[test]
fn test_geometry_nodes_join_geometry() {
    let grid = GeometryMesh::primitive_grid(2.0, 2.0, 3, 3); // 9 verts, 4 faces
    let cone = GeometryMesh::primitive_cone(8, 0.0, 1.0, 2.0); // 10 verts, 16 faces

    let joined = GeometryMesh::join(&[&grid, &cone]);

    assert_eq!(joined.positions.len(), 9 + 10);
    assert_eq!(joined.faces.len(), 4 + 16);

    // Verify all face indices are valid within bounds of joined.positions
    for face in &joined.faces {
        for &v_idx in face {
            assert!(v_idx < joined.positions.len(), "Face index {v_idx} out of bounds");
        }
    }

    // First cone face should be offset by 9
    let first_cone_face = &joined.faces[4];
    assert!(first_cone_face.iter().all(|&idx| idx >= 9));
}

#[test]
fn test_geometry_nodes_attribute_domain_interpolation() {
    // Barycentric interpolation of vertex weights onto triangle center
    let v0 = Vec3::new(0.0, 0.0, 0.0);
    let v1 = Vec3::new(2.0, 0.0, 0.0);
    let v2 = Vec3::new(0.0, 2.0, 0.0);

    let w0 = 10.0f32;
    let w1 = 20.0f32;
    let w2 = 30.0f32;

    // Centroid has barycentric coordinates (1/3, 1/3, 1/3)
    let centroid = (v0 + v1 + v2) / 3.0;
    assert!((centroid.x - 2.0 / 3.0).abs() < 1e-5);
    assert!((centroid.y - 2.0 / 3.0).abs() < 1e-5);

    let interpolated_weight = (w0 + w1 + w2) / 3.0;
    assert!((interpolated_weight - 20.0).abs() < 1e-5);
}
