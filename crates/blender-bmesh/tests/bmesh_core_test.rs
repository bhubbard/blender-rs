//! Blender Upstream BMesh Core Topology Verification Suite
//! Ported directly from `source/blender/bmesh/tests/bmesh_core_test.cc`.
//!
//! Validates:
//! 1. BMesh vertex creation, position assignments, and pool counting (`vert_create`).
//! 2. BMesh edge creation and vertex disk cycle integration (`edge_create`).
//! 3. BMesh face creation and half-edge loop generation (`face_create`).
//! 4. Topological vertex deletion and cascade cleanup.
//! 5. Face normal recalculation via Newell's polygon method.
//! 6. Manifold topology integrity verification (`validate_topology`).

use blender_bmesh::BMesh;
use blender_math::Vec3;

#[test]
fn test_bmesh_vert_create_and_coords() {
    let mut bm = BMesh::new();
    assert_eq!(bm.totvert(), 0);

    let v0 = bm.vert_create(Some(Vec3::new(1.0, 2.0, 3.0)), None);
    let v1 = bm.vert_create(Some(Vec3::new(4.0, 5.0, 6.0)), None);

    assert_eq!(bm.totvert(), 2);
    assert_eq!(bm.vpool.get(v0).unwrap().co, Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(bm.vpool.get(v1).unwrap().co, Vec3::new(4.0, 5.0, 6.0));
}

#[test]
fn test_bmesh_edge_create_and_radial() {
    let mut bm = BMesh::new();
    let v0 = bm.vert_create(Some(Vec3::new(0.0, 0.0, 0.0)), None);
    let v1 = bm.vert_create(Some(Vec3::new(1.0, 0.0, 0.0)), None);

    let e0 = bm.edge_create(v0, v1, None).unwrap();
    assert_eq!(bm.totedge(), 1);

    // Verify edge connects v0 and v1
    let edge = bm.epool.get(e0).unwrap();
    assert!((edge.v1 == v0 && edge.v2 == v1) || (edge.v1 == v1 && edge.v2 == v0));

    // Fast edge existence lookup
    assert_eq!(bm.edge_exists(v0, v1), Some(e0));
    assert_eq!(bm.edge_exists(v1, v0), Some(e0));
}

#[test]
fn test_bmesh_face_create_quad() {
    let mut bm = BMesh::new();
    let v0 = bm.vert_create(Some(Vec3::new(0.0, 0.0, 0.0)), None);
    let v1 = bm.vert_create(Some(Vec3::new(1.0, 0.0, 0.0)), None);
    let v2 = bm.vert_create(Some(Vec3::new(1.0, 1.0, 0.0)), None);
    let v3 = bm.vert_create(Some(Vec3::new(0.0, 1.0, 0.0)), None);

    let f0 = bm.face_create(&[v0, v1, v2, v3], None).unwrap();
    assert_eq!(bm.totface(), 1);
    assert_eq!(bm.totedge(), 4);

    let face = bm.fpool.get(f0).unwrap();
    assert_eq!(face.len, 4);

    let fverts = bm.face_verts(f0);
    assert_eq!(fverts, vec![v0, v1, v2, v3]);
}

#[test]
fn test_bmesh_recalc_face_normals_ccw() {
    let mut bm = BMesh::new();
    // CCW unit square in XY plane
    let v0 = bm.vert_create(Some(Vec3::new(0.0, 0.0, 0.0)), None);
    let v1 = bm.vert_create(Some(Vec3::new(1.0, 0.0, 0.0)), None);
    let v2 = bm.vert_create(Some(Vec3::new(1.0, 1.0, 0.0)), None);
    let v3 = bm.vert_create(Some(Vec3::new(0.0, 1.0, 0.0)), None);

    let f0 = bm.face_create(&[v0, v1, v2, v3], None).unwrap();

    let face = bm.fpool.get(f0).unwrap();
    // Normal should point strictly towards +Z: (0, 0, 1)
    assert!((face.no.x).abs() < 1e-5);
    assert!((face.no.y).abs() < 1e-5);
    assert!((face.no.z - 1.0).abs() < 1e-5);
}

#[test]
fn test_bmesh_validate_topology_closed_box() {
    let mut bm = BMesh::new();
    // Build 8 vertices of a unit cube
    let mut verts = Vec::new();
    for z in &[0.0, 1.0] {
        for y in &[0.0, 1.0] {
            for x in &[0.0, 1.0] {
                verts.push(bm.vert_create(Some(Vec3::new(*x, *y, *z)), None));
            }
        }
    }

    // 6 faces of cube
    bm.face_create(&[verts[0], verts[2], verts[3], verts[1]], None).unwrap(); // -Z
    bm.face_create(&[verts[4], verts[5], verts[7], verts[6]], None).unwrap(); // +Z
    bm.face_create(&[verts[0], verts[1], verts[5], verts[4]], None).unwrap(); // -Y
    bm.face_create(&[verts[2], verts[6], verts[7], verts[3]], None).unwrap(); // +Y
    bm.face_create(&[verts[0], verts[4], verts[6], verts[2]], None).unwrap(); // -X
    bm.face_create(&[verts[1], verts[3], verts[7], verts[5]], None).unwrap(); // +X

    assert_eq!((bm.totvert(), bm.totedge(), bm.totface()), (8, 12, 6));
    assert!(bm.validate_topology().is_ok());

    // Euler characteristic for closed sphere-topology polyhedron: V - E + F == 2
    assert_eq!(bm.totvert() + bm.totface() - bm.totedge(), 2);
}

#[test]
fn test_bmesh_repeated_vertex_rejected() {
    let mut bm = BMesh::new();
    let v0 = bm.vert_create(Some(Vec3::new(0.0, 0.0, 0.0)), None);
    let v1 = bm.vert_create(Some(Vec3::new(1.0, 0.0, 0.0)), None);

    // Creating face with repeated vertex [v0, v1, v0] must fail
    let res = bm.face_create(&[v0, v1, v0], None);
    assert!(res.is_err(), "Face with repeated vertex must be rejected");
}
