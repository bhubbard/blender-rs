//! Tests for BVH tree and 3D KDTree spatial search structures,
//! porting upstream Blender's BLI_kdopbvh_test.cc and BLI_kdtree_test.cc.

use blender_geom::bvh::Bvh;
use blender_geom::kdtree::KdTree;
use glam::Vec3;

#[test]
fn test_bvh_construction_and_bounds() {
    // Construct a simple unit cube represented by 12 triangles (2 per face)
    let p0 = Vec3::new(0.0, 0.0, 0.0);
    let p1 = Vec3::new(1.0, 0.0, 0.0);
    let p2 = Vec3::new(1.0, 1.0, 0.0);
    let p3 = Vec3::new(0.0, 1.0, 0.0);
    let p4 = Vec3::new(0.0, 0.0, 1.0);
    let p5 = Vec3::new(1.0, 0.0, 1.0);
    let p6 = Vec3::new(1.0, 1.0, 1.0);
    let p7 = Vec3::new(0.0, 1.0, 1.0);

    let tris = vec![
        // Bottom (-Z)
        [p0, p2, p1], [p0, p3, p2],
        // Top (+Z)
        [p4, p5, p6], [p4, p6, p7],
        // Front (-Y)
        [p0, p1, p5], [p0, p5, p4],
        // Back (+Y)
        [p3, p6, p2], [p3, p7, p6],
        // Left (-X)
        [p0, p4, p7], [p0, p7, p3],
        // Right (+X)
        [p1, p2, p6], [p1, p6, p5],
    ];

    let bvh = Bvh::new(&tris);
    assert_eq!(bvh.len(), 12);
    assert!(!bvh.is_empty());

    let b = bvh.bounds();
    assert!((b.min.x - 0.0).abs() < 1e-5);
    assert!((b.min.y - 0.0).abs() < 1e-5);
    assert!((b.min.z - 0.0).abs() < 1e-5);
    assert!((b.max.x - 1.0).abs() < 1e-5);
    assert!((b.max.y - 1.0).abs() < 1e-5);
    assert!((b.max.z - 1.0).abs() < 1e-5);
}

#[test]
fn test_bvh_ray_cast_hits_and_misses() {
    let t0 = [Vec3::new(-1.0, -1.0, 0.0), Vec3::new(1.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0)];
    let bvh = Bvh::new(&[t0]);

    // Ray directed downwards towards triangle center (0, 0, 0)
    let hit = bvh.ray_cast(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));
    assert!(hit.is_some());
    let hit = hit.unwrap();
    assert_eq!(hit.index, 0);
    assert!((hit.t - 5.0).abs() < 1e-5);

    // Ray shooting away should miss
    let miss = bvh.ray_cast(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 1.0));
    assert!(miss.is_none());

    // Ray parallel to triangle plane should miss
    let miss_parallel = bvh.ray_cast(Vec3::new(0.0, 0.0, 5.0), Vec3::new(1.0, 0.0, 0.0));
    assert!(miss_parallel.is_none());

    // Ray aiming outside triangle bounds should miss
    let miss_outside = bvh.ray_cast(Vec3::new(5.0, 5.0, 5.0), Vec3::new(0.0, 0.0, -1.0));
    assert!(miss_outside.is_none());
}

#[test]
fn test_bvh_find_nearest_surface_point() {
    let t0 = [Vec3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0)];
    let bvh = Bvh::new(&[t0]);

    // Query point directly above the interior of the triangle at (0.5, 0.5, 3.0)
    let query = Vec3::new(0.5, 0.5, 3.0);
    let nearest = bvh.find_nearest(query);
    assert!(nearest.is_some());
    let res = nearest.unwrap();
    assert_eq!(res.index, 0);
    assert!((res.point.x - 0.5).abs() < 1e-5);
    assert!((res.point.y - 0.5).abs() < 1e-5);
    assert!((res.point.z - 0.0).abs() < 1e-5);
    assert!((res.dist_sq - 9.0).abs() < 1e-5);

    // Query point closest to a vertex (-1.0, -1.0, 0.0) -> closest is (0, 0, 0)
    let query_v = Vec3::new(-1.0, -1.0, 0.0);
    let res_v = bvh.find_nearest(query_v).unwrap();
    assert!((res_v.point.x - 0.0).abs() < 1e-5);
    assert!((res_v.point.y - 0.0).abs() < 1e-5);
    assert!((res_v.dist_sq - 2.0).abs() < 1e-5);
}

#[test]
fn test_kdtree_find_nearest_single_point() {
    let pts = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(10.0, 0.0, 0.0),
        Vec3::new(0.0, 10.0, 0.0),
        Vec3::new(0.0, 0.0, 10.0),
        Vec3::new(5.0, 5.0, 5.0),
    ];
    let kd = KdTree::new(&pts);
    assert_eq!(kd.len(), 5);
    assert!(!kd.is_empty());

    // Query near (0, 0, 0)
    let hit = kd.find_nearest(Vec3::new(0.1, 0.2, -0.1)).unwrap();
    assert_eq!(hit.index, 0);
    assert!(hit.dist_sq < 0.1);

    // Query near (5, 5, 5)
    let hit2 = kd.find_nearest(Vec3::new(4.9, 5.1, 5.0)).unwrap();
    assert_eq!(hit2.index, 4);
}

#[test]
fn test_kdtree_find_nearest_n_neighbors() {
    let pts = vec![
        Vec3::new(0.0, 0.0, 0.0), // 0: dist^2 = 0
        Vec3::new(1.0, 0.0, 0.0), // 1: dist^2 = 1
        Vec3::new(2.0, 0.0, 0.0), // 2: dist^2 = 4
        Vec3::new(3.0, 0.0, 0.0), // 3: dist^2 = 9
        Vec3::new(4.0, 0.0, 0.0), // 4: dist^2 = 16
    ];
    let kd = KdTree::new(&pts);

    let top3 = kd.find_nearest_n(Vec3::ZERO, 3);
    assert_eq!(top3.len(), 3);
    assert_eq!(top3[0].index, 0);
    assert_eq!(top3[1].index, 1);
    assert_eq!(top3[2].index, 2);
    assert!(top3[0].dist_sq <= top3[1].dist_sq);
    assert!(top3[1].dist_sq <= top3[2].dist_sq);
}

#[test]
fn test_kdtree_range_search_radius() {
    let pts = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(5.0, 0.0, 0.0),
        Vec3::new(10.0, 0.0, 0.0),
    ];
    let kd = KdTree::new(&pts);

    // Range search within radius 2.1 from origin
    let within = kd.range_search(Vec3::ZERO, 2.1);
    let mut indices: Vec<usize> = within.iter().map(|h| h.index).collect();
    indices.sort_unstable();

    // Should include 0 (dist 0), 1 (dist 1), 2 (dist 2)
    assert_eq!(indices, vec![0, 1, 2]);

    // Range search with very small radius
    let tiny = kd.range_search(Vec3::new(100.0, 100.0, 100.0), 1.0);
    assert!(tiny.is_empty());
}
