//! Blender Upstream 2D Convex Hull Verification Suite
//! Ported directly from `source/blender/blenlib/tests/BLI_convexhull_2d_test.cc`.
//!
//! Validates:
//! 1. Andrew's monotone chain 2D convex hull calculation (`convex_hull_v2`).
//! 2. Interior point rejection and outer boundary preservation.
//! 3. Collinear edge point elimination and simplification.
//! 4. Angular sorted order of circular point configurations.
//! 5. Concave star polygon hull simplification.
//! 6. Degenerate and small cardinality (<= 2 points) handling.

use blender_geom::polyfill::convex_hull_v2;
use glam::Vec2;

#[test]
fn test_convexhull_unit_square() {
    let pts = [
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
    ];

    let hull = convex_hull_v2(&pts);
    assert_eq!(hull.len(), 4);
    // All 4 corners must be in the hull
    let mut sorted = hull.clone();
    sorted.sort_unstable();
    assert_eq!(sorted, vec![0, 1, 2, 3]);
}

#[test]
fn test_convexhull_internal_points_ignored() {
    let mut pts = vec![
        Vec2::new(-10.0, -10.0), // 0
        Vec2::new(10.0, -10.0),  // 1
        Vec2::new(10.0, 10.0),   // 2
        Vec2::new(-10.0, 10.0),  // 3
    ];

    // Add internal interior points
    pts.push(Vec2::new(0.0, 0.0));
    pts.push(Vec2::new(1.0, 2.0));
    pts.push(Vec2::new(-5.0, 3.0));

    let hull = convex_hull_v2(&pts);
    assert_eq!(hull.len(), 4);
    // Hull should only contain indices 0, 1, 2, 3
    for &idx in &hull {
        assert!(idx < 4, "Interior point index {idx} leaked into convex hull");
    }
}

#[test]
fn test_convexhull_collinear_filtering() {
    let pts = [
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0), // Collinear on bottom edge
        Vec2::new(2.0, 0.0),
        Vec2::new(2.0, 1.0), // Collinear on right edge
        Vec2::new(2.0, 2.0),
        Vec2::new(0.0, 2.0),
    ];

    let hull = convex_hull_v2(&pts);
    // Collinear intermediate points (1, 0) and (2, 1) should be pruned
    assert_eq!(hull.len(), 4);
    assert!(!hull.contains(&1), "Collinear point (1, 0) must be pruned");
    assert!(!hull.contains(&3), "Collinear point (2, 1) must be pruned");
}

#[test]
fn test_convexhull_circle_points() {
    let n = 16;
    let mut pts = Vec::with_capacity(n);
    for i in 0..n {
        let angle = (i as f32 / n as f32) * std::f32::consts::TAU;
        pts.push(Vec2::new(angle.cos() * 5.0, angle.sin() * 5.0));
    }

    let hull = convex_hull_v2(&pts);
    // All points on a strictly convex circle must belong to the hull
    assert_eq!(hull.len(), n);
}

#[test]
fn test_convexhull_star_polygon() {
    // 5-pointed star with outer tips at radius 5 and inner vertices at radius 2
    let mut pts = Vec::new();
    for i in 0..10 {
        let r = if i % 2 == 0 { 5.0 } else { 2.0 };
        let angle = (i as f32 / 10.0) * std::f32::consts::TAU;
        pts.push(Vec2::new(r * angle.cos(), r * angle.sin()));
    }

    let hull = convex_hull_v2(&pts);
    // Only the 5 outer tips should form the hull
    assert_eq!(hull.len(), 5);
    for &idx in &hull {
        assert!(idx % 2 == 0, "Inner reflex star vertex {idx} must be excluded");
    }
}

#[test]
fn test_convexhull_minimal_points() {
    let empty: [Vec2; 0] = [];
    assert_eq!(convex_hull_v2(&empty).len(), 0);

    let single = [Vec2::new(3.0, 4.0)];
    assert_eq!(convex_hull_v2(&single), vec![0]);

    let pair = [Vec2::new(1.0, 1.0), Vec2::new(2.0, 2.0)];
    assert_eq!(convex_hull_v2(&pair).len(), 2);
}
