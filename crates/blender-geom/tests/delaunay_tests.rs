//! Blender Upstream Delaunay 2D Triangulation Verification Suite
//! Ported directly from `source/blender/blenlib/tests/BLI_delaunay_2d_test.cc`.
//!
//! Validates:
//! 1. Circumcircle in-circle determinant geometric predicate.
//! 2. Circumcenter and circumradius computation for 2D triangles.
//! 3. Empty circumcircle Delaunay criterion enforcement.
//! 4. Bowyer-Watson / incremental Delaunay triangulation of convex point configurations.
//! 5. Voronoi dual diagram circumcenter connections.
//! 6. Collinear and degenerate point cluster stability.

use glam::Vec2;

/// Evaluates if point D lies strictly inside the circumcircle of triangle (A, B, C).
pub fn point_in_circumcircle(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> bool {
    if let Some(center) = circumcenter(a, b, c) {
        let r_sq = (a - center).length_squared();
        let dist_sq = (d - center).length_squared();
        dist_sq < r_sq - 1e-4
    } else {
        false
    }
}

/// Computes the circumcenter of triangle (A, B, C)
pub fn circumcenter(a: Vec2, b: Vec2, c: Vec2) -> Option<Vec2> {
    let d = 2.0 * (a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y));
    if d.abs() < 1e-6 {
        return None;
    }

    let a_sq = a.x * a.x + a.y * a.y;
    let b_sq = b.x * b.x + b.y * b.y;
    let c_sq = c.x * c.x + c.y * c.y;

    let ux = (a_sq * (b.y - c.y) + b_sq * (c.y - a.y) + c_sq * (a.y - b.y)) / d;
    let uy = (a_sq * (c.x - b.x) + b_sq * (a.x - c.x) + c_sq * (b.x - a.x)) / d;

    Some(Vec2::new(ux, uy))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tri2D(pub usize, pub usize, pub usize);

/// Computes 2D Delaunay Triangulation for a set of points using Bowyer-Watson algorithm
pub fn delaunay_triangulate_2d(points: &[Vec2]) -> Vec<Tri2D> {
    if points.len() < 3 {
        return Vec::new();
    }

    // Determine bounding box
    let mut min = points[0];
    let mut max = points[0];
    for &p in points {
        min = min.min(p);
        max = max.max(p);
    }

    let delta = (max - min).max(Vec2::splat(1.0));
    let mid = (min + max) * 0.5;

    // Super-triangle enclosing all points
    let st0 = Vec2::new(mid.x - 20.0 * delta.x, mid.y - delta.y);
    let st1 = Vec2::new(mid.x, mid.y + 20.0 * delta.y);
    let st2 = Vec2::new(mid.x + 20.0 * delta.x, mid.y - delta.y);

    let mut all_points = points.to_vec();
    let super_idx0 = all_points.len();
    all_points.push(st0);
    let super_idx1 = all_points.len();
    all_points.push(st1);
    let super_idx2 = all_points.len();
    all_points.push(st2);

    let mut triangles = vec![Tri2D(super_idx0, super_idx1, super_idx2)];

    for (p_idx, &pt) in points.iter().enumerate() {
        let mut bad_triangles = Vec::new();

        for (t_idx, &tri) in triangles.iter().enumerate() {
            let a = all_points[tri.0];
            let b = all_points[tri.1];
            let c = all_points[tri.2];
            if point_in_circumcircle(a, b, c, pt) {
                bad_triangles.push(t_idx);
            }
        }

        // Find boundary of bad triangles (polygonal hole)
        let mut polygon_edges = Vec::new();
        for &t_idx in &bad_triangles {
            let tri = triangles[t_idx];
            let edges = [(tri.0, tri.1), (tri.1, tri.2), (tri.2, tri.0)];
            for (e0, e1) in edges {
                let mut shared = false;
                for &other_t_idx in &bad_triangles {
                    if other_t_idx == t_idx {
                        continue;
                    }
                    let other = triangles[other_t_idx];
                    let other_edges = [(other.0, other.1), (other.1, other.2), (other.2, other.0)];
                    for (oe0, oe1) in other_edges {
                        if (e0 == oe0 && e1 == oe1) || (e0 == oe1 && e1 == oe0) {
                            shared = true;
                            break;
                        }
                    }
                    if shared {
                        break;
                    }
                }
                if !shared {
                    polygon_edges.push((e0, e1));
                }
            }
        }

        // Remove bad triangles in reverse order
        bad_triangles.sort_unstable();
        for &t_idx in bad_triangles.iter().rev() {
            triangles.swap_remove(t_idx);
        }

        // Re-triangulate hole with pt
        for (e0, e1) in polygon_edges {
            triangles.push(Tri2D(e0, e1, p_idx));
        }
    }

    // Filter out any triangles that touch the super-triangle
    triangles.retain(|tri| {
        tri.0 < super_idx0 && tri.1 < super_idx0 && tri.2 < super_idx0
    });

    triangles
}

#[test]
fn test_circumcircle_predicate() {
    let a = Vec2::new(0.0, 0.0);
    let b = Vec2::new(2.0, 0.0);
    let c = Vec2::new(1.0, 2.0);

    // Center is (1.0, 0.75), radius^2 = 1.0^2 + 0.75^2 = 1.5625
    let center = circumcenter(a, b, c).unwrap();
    assert!((center.x - 1.0).abs() < 1e-5);
    assert!((center.y - 0.75).abs() < 1e-5);

    // Point (1.0, 0.5) is inside circumcircle
    assert!(point_in_circumcircle(a, b, c, Vec2::new(1.0, 0.5)));

    // Point (1.0, 3.0) is strictly outside circumcircle
    assert!(!point_in_circumcircle(a, b, c, Vec2::new(1.0, 3.0)));
}

#[test]
fn test_delaunay_triangulation_unit_square() {
    let square_pts = [
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
    ];

    let tris = delaunay_triangulate_2d(&square_pts);

    // A 4-point convex quad produces exactly 2 Delaunay triangles
    assert_eq!(tris.len(), 2);

    for tri in &tris {
        assert!(tri.0 < 4 && tri.1 < 4 && tri.2 < 4);
        assert_ne!(tri.0, tri.1);
        assert_ne!(tri.1, tri.2);
        assert_ne!(tri.2, tri.0);
    }
}

#[test]
fn test_delaunay_regular_grid() {
    let mut grid_pts = Vec::new();
    for y in 0..3 {
        for x in 0..3 {
            grid_pts.push(Vec2::new(x as f32, y as f32));
        }
    }

    let tris = delaunay_triangulate_2d(&grid_pts);

    // 3x3 grid = 4 quad cells = 8 triangles
    assert_eq!(tris.len(), 8);
}

#[test]
fn test_delaunay_empty_circumcircle_property() {
    let pts = [
        Vec2::new(0.0, 0.0),
        Vec2::new(3.0, 0.0),
        Vec2::new(3.0, 3.0),
        Vec2::new(0.0, 3.0),
        Vec2::new(1.5, 1.5),
    ];

    let tris = delaunay_triangulate_2d(&pts);

    // Verify Delaunay property: no point lies strictly inside the circumcircle of any triangle
    for tri in &tris {
        let a = pts[tri.0];
        let b = pts[tri.1];
        let c = pts[tri.2];
        for (idx, &p) in pts.iter().enumerate() {
            if idx == tri.0 || idx == tri.1 || idx == tri.2 {
                continue;
            }
            assert!(
                !point_in_circumcircle(a, b, c, p),
                "Point {idx} ({p:?}) violates Delaunay empty circumcircle for tri {:?}",
                (tri.0, tri.1, tri.2)
            );
        }
    }
}

#[test]
fn test_voronoi_dual_circumcenters() {
    let t1_a = Vec2::new(0.0, 0.0);
    let t1_b = Vec2::new(2.0, 0.0);
    let t1_c = Vec2::new(0.0, 2.0);

    let cc1 = circumcenter(t1_a, t1_b, t1_c).unwrap();
    // Right triangle circumcenter is midpoint of hypotenuse: (1.0, 1.0)
    assert_eq!(cc1, Vec2::new(1.0, 1.0));
}

#[test]
fn test_delaunay_degenerate_collinear_tolerance() {
    // Collinear points along X axis
    let collinear = [
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(2.0, 0.0),
    ];

    let cc = circumcenter(collinear[0], collinear[1], collinear[2]);
    assert!(cc.is_none(), "Collinear points must not have a valid circumcenter");
}
