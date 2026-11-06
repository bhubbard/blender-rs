//! Tests for BMesh queries, custom data layers, radial loop links,
//! and element selection, porting upstream Blender's bmesh_query_test.cc.

use blender_bmesh::{
    BMesh, CustomDataType, MeshElemFlags, Vec3,
};

#[test]
fn test_bmesh_edge_queries() {
    let mut bm = BMesh::new();
    let v0 = bm.vert_create(Some(Vec3::new(0.0, 0.0, 0.0)), None);
    let v1 = bm.vert_create(Some(Vec3::new(1.0, 0.0, 0.0)), None);
    let v2 = bm.vert_create(Some(Vec3::new(0.0, 1.0, 0.0)), None);

    // Prior to creation, edge should not exist
    assert!(bm.edge_exists(v0, v1).is_none());
    assert!(bm.edge_exists(v1, v0).is_none());

    // Create edge between v0 and v1
    let e01 = bm.edge_create(v0, v1, None).expect("Edge creation should succeed");

    // Both orderings should query the same edge
    assert_eq!(bm.edge_exists(v0, v1), Some(e01));
    assert_eq!(bm.edge_exists(v1, v0), Some(e01));

    // v0 to v2 should still not exist
    assert!(bm.edge_exists(v0, v2).is_none());

    // edge_find_or_create should return existing handle without duplicating
    let e01_found = bm.edge_find_or_create(v0, v1).unwrap();
    assert_eq!(e01_found, e01);
    assert_eq!(bm.totedge(), 1);

    // edge_find_or_create between new pair creates new edge
    let e12 = bm.edge_find_or_create(v1, v2).unwrap();
    assert_ne!(e12, e01);
    assert_eq!(bm.totedge(), 2);
}

#[test]
fn test_bmesh_selection_flags() {
    let mut bm = BMesh::new();
    let v0 = bm.vert_create(Some(Vec3::ZERO), None);
    let v1 = bm.vert_create(Some(Vec3::X), None);

    // Initial state: not selected
    assert!(!bm.vert_flag_test(v0, MeshElemFlags::SELECT));
    assert!(!bm.vert_flag_test(v1, MeshElemFlags::SELECT));

    // Select v0
    assert!(bm.vert_select_set(v0, true));
    assert!(bm.vert_flag_test(v0, MeshElemFlags::SELECT));
    assert!(!bm.vert_flag_test(v1, MeshElemFlags::SELECT));

    // Deselect v0
    assert!(bm.vert_select_set(v0, false));
    assert!(!bm.vert_flag_test(v0, MeshElemFlags::SELECT));
}

#[test]
fn test_bmesh_custom_float_data_layers() {
    let mut bm = BMesh::new();
    let v0 = bm.vert_create(Some(Vec3::new(0.0, 0.0, 0.0)), None);
    let v1 = bm.vert_create(Some(Vec3::new(1.0, 1.0, 1.0)), None);

    // Unset custom data should return None
    assert!(bm.elem_float_data_get(v0, CustomDataType::BWeight).is_none());

    // Set bevel weight and crease
    bm.elem_float_data_set(v0, CustomDataType::BWeight, 0.75);
    bm.elem_float_data_set(v0, CustomDataType::Crease, 0.5);
    bm.elem_float_data_set(v1, CustomDataType::BWeight, 0.25);

    assert_eq!(bm.elem_float_data_get(v0, CustomDataType::BWeight), Some(0.75));
    assert_eq!(bm.elem_float_data_get(v0, CustomDataType::Crease), Some(0.5));
    assert_eq!(bm.elem_float_data_get(v1, CustomDataType::BWeight), Some(0.25));
    assert!(bm.elem_float_data_get(v1, CustomDataType::Crease).is_none());
}

#[test]
fn test_bmesh_face_loop_cycle_integrity() {
    let mut bm = BMesh::new();
    let v0 = bm.vert_create(Some(Vec3::new(0.0, 0.0, 0.0)), None);
    let v1 = bm.vert_create(Some(Vec3::new(1.0, 0.0, 0.0)), None);
    let v2 = bm.vert_create(Some(Vec3::new(1.0, 1.0, 0.0)), None);
    let v3 = bm.vert_create(Some(Vec3::new(0.0, 1.0, 0.0)), None);

    let f0 = bm.face_create(&[v0, v1, v2, v3], None).expect("Face create should succeed");
    let face = bm.fpool.get(f0).unwrap();
    assert_eq!(face.len, 4);

    // Traverse the loop cycle forward and verify it closes after exactly 4 steps
    let start_loop = face.l_first;
    let mut curr_loop = start_loop;
    let mut visited_verts = Vec::new();

    for _ in 0..4 {
        let loop_elem = bm.lpool.get(curr_loop).unwrap();
        visited_verts.push(loop_elem.v);
        curr_loop = loop_elem.next;
    }

    assert_eq!(curr_loop, start_loop, "Loop cycle must return to start after 4 steps");
    assert_eq!(visited_verts, vec![v0, v1, v2, v3]);

    // Traverse the loop cycle backward and verify it matches reversed order
    let mut rev_visited = Vec::new();
    curr_loop = start_loop;
    for _ in 0..4 {
        let loop_elem = bm.lpool.get(curr_loop).unwrap();
        rev_visited.push(loop_elem.v);
        curr_loop = loop_elem.prev;
    }
    assert_eq!(curr_loop, start_loop, "Backward cycle must return to start");
    assert_eq!(rev_visited, vec![v0, v3, v2, v1]);
}

#[test]
fn test_bmesh_shared_edge_radial_cycle() {
    let mut bm = BMesh::new();
    // Two adjacent triangles sharing edge (v1, v2)
    let v0 = bm.vert_create(Some(Vec3::new(0.0, 0.0, 0.0)), None);
    let v1 = bm.vert_create(Some(Vec3::new(1.0, 0.0, 0.0)), None);
    let v2 = bm.vert_create(Some(Vec3::new(1.0, 1.0, 0.0)), None);
    let v3 = bm.vert_create(Some(Vec3::new(2.0, 0.0, 0.0)), None);

    let _f1 = bm.face_create(&[v0, v1, v2], None).unwrap();
    let _f2 = bm.face_create(&[v1, v3, v2], None).unwrap();

    let shared_edge = bm.edge_exists(v1, v2).expect("Shared edge must exist");
    let edge_elem = bm.epool.get(shared_edge).unwrap();
    assert!(edge_elem.loop_.is_some(), "Shared edge must reference a loop");

    let l1 = edge_elem.loop_.unwrap();
    let loop1 = bm.lpool.get(l1).unwrap();
    // Radial cycle should connect to the other face's loop
    let l2 = loop1.radial_next;
    assert_ne!(l1, l2);
    let loop2 = bm.lpool.get(l2).unwrap();
    assert_eq!(loop2.radial_next, l1, "Radial cycle must loop back between the two faces");
}
