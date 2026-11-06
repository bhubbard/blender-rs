//! Geometric intersection tests porting upstream Blender's
//! BLI_math_geom_test.cc.

use blender_geom::isect::{isect_line_line_v2, isect_line_line_v3, isect_seg_seg_v2};
use blender_geom::tri::{barycentric_weights_v3, closest_on_tri_to_point_v3, isect_ray_tri_v3};
use glam::{Vec2, Vec3};

#[test]
fn test_ray_triangle_moller_trumbore() {
    let v0 = Vec3::new(0.0, 0.0, 0.0);
    let v1 = Vec3::new(2.0, 0.0, 0.0);
    let v2 = Vec3::new(0.0, 2.0, 0.0);

    // Direct hit in center
    let ray_orig = Vec3::new(0.5, 0.5, 5.0);
    let ray_dir = Vec3::new(0.0, 0.0, -1.0);
    let hit = isect_ray_tri_v3(ray_orig, ray_dir, v0, v1, v2);
    assert!(hit.is_some());
    let hit = hit.unwrap();
    assert!((hit.t - 5.0).abs() < 1e-5);
    assert!((hit.u - 0.25).abs() < 1e-4);
    assert!((hit.v - 0.25).abs() < 1e-4);

    // Miss outside hypotenuse (x=1.5, y=1.5 => x+y = 3 > 2)
    let ray_miss = Vec3::new(1.5, 1.5, 5.0);
    let miss = isect_ray_tri_v3(ray_miss, ray_dir, v0, v1, v2);
    assert!(miss.is_none());

    // Hit from behind (backface)
    let ray_back = Vec3::new(0.5, 0.5, -5.0);
    let ray_back_dir = Vec3::new(0.0, 0.0, 1.0);
    let hit_back = isect_ray_tri_v3(ray_back, ray_back_dir, v0, v1, v2);
    assert!(hit_back.is_some());
    assert!((hit_back.unwrap().t - 5.0).abs() < 1e-5);
}

#[test]
fn test_barycentric_weights_3d() {
    let a = Vec3::new(0.0, 0.0, 0.0);
    let b = Vec3::new(1.0, 0.0, 0.0);
    let c = Vec3::new(0.0, 1.0, 0.0);

    // Vertex A should have weight (1, 0, 0)
    let w_a = barycentric_weights_v3(a, a, b, c).unwrap();
    assert!((w_a.x - 1.0).abs() < 1e-5);
    assert!(w_a.y.abs() < 1e-5);
    assert!(w_a.z.abs() < 1e-5);

    // Vertex B should have weight (0, 1, 0)
    let w_b = barycentric_weights_v3(b, a, b, c).unwrap();
    assert!(w_b.x.abs() < 1e-5);
    assert!((w_b.y - 1.0).abs() < 1e-5);
    assert!(w_b.z.abs() < 1e-5);

    // Vertex C should have weight (0, 0, 1)
    let w_c = barycentric_weights_v3(c, a, b, c).unwrap();
    assert!(w_c.x.abs() < 1e-5);
    assert!(w_c.y.abs() < 1e-5);
    assert!((w_c.z - 1.0).abs() < 1e-5);

    // Centroid (1/3, 1/3, 0)
    let centroid = (a + b + c) / 3.0;
    let w_cent = barycentric_weights_v3(centroid, a, b, c).unwrap();
    assert!((w_cent.x - 1.0 / 3.0).abs() < 1e-5);
    assert!((w_cent.y - 1.0 / 3.0).abs() < 1e-5);
    assert!((w_cent.z - 1.0 / 3.0).abs() < 1e-5);
}

#[test]
fn test_closest_on_triangle_ericson() {
    let a = Vec3::new(0.0, 0.0, 0.0);
    let b = Vec3::new(2.0, 0.0, 0.0);
    let c = Vec3::new(0.0, 2.0, 0.0);

    // Point in Voronoi region of vertex A
    let cp_a = closest_on_tri_to_point_v3(Vec3::new(-1.0, -1.0, 0.0), a, b, c);
    assert!((cp_a - a).length() < 1e-5);

    // Point along edge AB
    let cp_ab = closest_on_tri_to_point_v3(Vec3::new(1.0, -1.0, 0.0), a, b, c);
    assert!((cp_ab - Vec3::new(1.0, 0.0, 0.0)).length() < 1e-5);

    // Point directly above interior
    let cp_in = closest_on_tri_to_point_v3(Vec3::new(0.5, 0.5, 4.0), a, b, c);
    assert!((cp_in - Vec3::new(0.5, 0.5, 0.0)).length() < 1e-5);
}

#[test]
fn test_segment_segment_intersection_2d() {
    // Standard X crossing: (0, 0)->(2, 2) and (0, 2)->(2, 0) intersect at (1, 1)
    let hit = isect_seg_seg_v2(
        Vec2::new(0.0, 0.0),
        Vec2::new(2.0, 2.0),
        Vec2::new(0.0, 2.0),
        Vec2::new(2.0, 0.0),
    );
    assert!(hit.is_some());
    let hit = hit.unwrap();
    assert!((hit.point.x - 1.0).abs() < 1e-5);
    assert!((hit.point.y - 1.0).abs() < 1e-5);
    assert!((hit.t_a - 0.5).abs() < 1e-5);
    assert!((hit.t_b - 0.5).abs() < 1e-5);

    // Parallel segments
    let par = isect_seg_seg_v2(
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(1.0, 1.0),
    );
    assert!(par.is_none());

    // Non-intersecting segments (would intersect if extended)
    let non = isect_seg_seg_v2(
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(2.0, -1.0),
        Vec2::new(2.0, 1.0),
    );
    assert!(non.is_none());
}

#[test]
fn test_line_line_intersection_2d() {
    let p = isect_line_line_v2(
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 2.0),
        Vec2::new(1.0, 1.0),
    );
    assert!(p.is_some());
    let p = p.unwrap();
    assert!((p.x - 1.0).abs() < 1e-5);
    assert!((p.y - 1.0).abs() < 1e-5);
}

#[test]
fn test_skew_lines_closest_points_3d() {
    // Line 1 along X axis at z = 0: (0, 0, 0) to (1, 0, 0)
    // Line 2 along Y axis at z = 1: (0, 0, 1) to (0, 1, 1)
    // Closest points should be (0, 0, 0) on Line 1 and (0, 0, 1) on Line 2
    let (p1, p2) = isect_line_line_v3(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 1.0, 1.0),
    ).unwrap();

    assert!((p1.x - 0.0).abs() < 1e-5);
    assert!((p1.y - 0.0).abs() < 1e-5);
    assert!((p1.z - 0.0).abs() < 1e-5);

    assert!((p2.x - 0.0).abs() < 1e-5);
    assert!((p2.y - 0.0).abs() < 1e-5);
    assert!((p2.z - 1.0).abs() < 1e-5);
}
