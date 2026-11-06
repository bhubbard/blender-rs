//! Tests for Wavefront OBJ import/export,
//! porting upstream Blender's io_wavefront_obj_test.cc.

use blender_obj::{parse_obj, write_obj, Corner, Face, ObjMesh};
use glam::{Vec2, Vec3};

#[test]
fn test_obj_parse_vertices_uvs_normals() {
    let src = r#"
# Blender OBJ File
o Cube
v 0.0 0.0 0.0
v 1.0 0.0 0.0
v 1.0 1.0 0.0
v 0.0 1.0 0.0
vt 0.0 0.0
vt 1.0 0.0
vt 1.0 1.0
vt 0.0 1.0
vn 0.0 0.0 1.0
f 1/1/1 2/2/1 3/3/1 4/4/1
"#;
    let mesh = parse_obj(src).expect("OBJ parsing should succeed");
    assert_eq!(mesh.positions.len(), 4);
    assert_eq!(mesh.uvs.len(), 4);
    assert_eq!(mesh.normals.len(), 1);
    assert_eq!(mesh.faces.len(), 1);

    let f = &mesh.faces[0];
    assert_eq!(f.corners.len(), 4);
    assert_eq!(f.group.as_deref(), Some("Cube"));
    assert_eq!(f.corners[0], Corner { v: 0, vt: Some(0), vn: Some(0) });
    assert_eq!(f.corners[2], Corner { v: 2, vt: Some(2), vn: Some(0) });
}

#[test]
fn test_obj_negative_relative_indices() {
    let src = r#"
v 1.0 2.0 3.0
v 4.0 5.0 6.0
v 7.0 8.0 9.0
f -1 -2 -3
"#;
    let mesh = parse_obj(src).expect("Negative indices should resolve relative to EOF");
    assert_eq!(mesh.positions.len(), 3);
    assert_eq!(mesh.faces.len(), 1);
    let f = &mesh.faces[0];
    // -1 refers to index 2 (7, 8, 9), -2 refers to 1, -3 refers to 0
    assert_eq!(f.corners[0].v, 2);
    assert_eq!(f.corners[1].v, 1);
    assert_eq!(f.corners[2].v, 0);
}

#[test]
fn test_obj_various_face_corner_formats() {
    let src = r#"
v 0 0 0
v 1 0 0
v 1 1 0
vt 0.5 0.5
vn 0 0 1
f 1 2 3
f 1/1 2/1 3/1
f 1//1 2//1 3//1
f 1/1/1 2/1/1 3/1/1
"#;
    let mesh = parse_obj(src).unwrap();
    assert_eq!(mesh.faces.len(), 4);

    // v only
    assert_eq!(mesh.faces[0].corners[0], Corner { v: 0, vt: None, vn: None });
    // v/vt
    assert_eq!(mesh.faces[1].corners[0], Corner { v: 0, vt: Some(0), vn: None });
    // v//vn
    assert_eq!(mesh.faces[2].corners[0], Corner { v: 0, vt: None, vn: Some(0) });
    // v/vt/vn
    assert_eq!(mesh.faces[3].corners[0], Corner { v: 0, vt: Some(0), vn: Some(0) });
}

#[test]
fn test_obj_face_triangulation() {
    // Quad face at z = 0
    let mut mesh = ObjMesh::default();
    mesh.positions = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    ];
    mesh.faces = vec![
        Face {
            corners: vec![
                Corner { v: 0, vt: None, vn: None },
                Corner { v: 1, vt: None, vn: None },
                Corner { v: 2, vt: None, vn: None },
                Corner { v: 3, vt: None, vn: None },
            ],
            group: None,
        }
    ];

    let tris = mesh.face_triangles(0);
    assert_eq!(tris.len(), 2);
    // Should split into two triangles covering all 4 vertices
    let mut visited = std::collections::HashSet::new();
    for tri in &tris {
        for &idx in tri {
            visited.insert(idx);
        }
    }
    assert_eq!(visited.len(), 4);
}

#[test]
fn test_obj_roundtrip_serialization() {
    let mut orig = ObjMesh::default();
    orig.positions = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(1.0, 2.0, 0.0),
    ];
    orig.uvs = vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(0.5, 1.0),
    ];
    orig.normals = vec![
        Vec3::new(0.0, 0.0, 1.0),
    ];
    orig.faces = vec![
        Face {
            corners: vec![
                Corner { v: 0, vt: Some(0), vn: Some(0) },
                Corner { v: 1, vt: Some(1), vn: Some(0) },
                Corner { v: 2, vt: Some(2), vn: Some(0) },
            ],
            group: Some("TriangleGroup".into()),
        }
    ];

    let text = write_obj(&orig);
    assert!(text.contains("g TriangleGroup"));
    assert!(text.contains("f 1/1/1 2/2/1 3/3/1"));

    let parsed = parse_obj(&text).expect("Serialized OBJ must parse cleanly");
    assert_eq!(parsed.positions.len(), orig.positions.len());
    assert_eq!(parsed.uvs.len(), orig.uvs.len());
    assert_eq!(parsed.normals.len(), orig.normals.len());
    assert_eq!(parsed.faces.len(), orig.faces.len());
    assert_eq!(parsed.faces[0].corners, orig.faces[0].corners);
}
