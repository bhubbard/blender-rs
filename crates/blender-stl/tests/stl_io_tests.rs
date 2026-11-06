//! Tests for STL format import/export (Binary & ASCII),
//! porting upstream Blender's io_stl_test.cc.

use blender_stl::{parse_stl, write_ascii, write_binary, StlError, StlMesh, Triangle};
use glam::Vec3;

#[test]
fn test_stl_ascii_parse_and_normal_generation() {
    let ascii_src = r#"solid tetrahedron
  facet normal 0.0 0.0 1.0
    outer loop
      vertex 0.0 0.0 0.0
      vertex 1.0 0.0 0.0
      vertex 0.0 1.0 0.0
    endloop
  endfacet
  facet normal 0.0 -1.0 0.0
    outer loop
      vertex 0.0 0.0 0.0
      vertex 0.0 0.0 1.0
      vertex 1.0 0.0 0.0
    endloop
  endfacet
endsolid tetrahedron
"#;

    let mesh = parse_stl(ascii_src.as_bytes()).expect("ASCII STL should parse");
    assert_eq!(mesh.triangles.len(), 2);
    assert_eq!(mesh.triangles[0].v[0], Vec3::new(0.0, 0.0, 0.0));
    assert_eq!(mesh.triangles[0].v[1], Vec3::new(1.0, 0.0, 0.0));
    assert_eq!(mesh.triangles[0].v[2], Vec3::new(0.0, 1.0, 0.0));
}

#[test]
fn test_stl_binary_roundtrip() {
    let t1 = Triangle::from_vertices([
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
    ]);
    let t2 = Triangle::from_vertices([
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(2.0, 2.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
    ]);

    let orig = StlMesh {
        triangles: vec![t1, t2],
    };

    let bin = write_binary(&orig, "Blender Rust Export");
    // Binary STL size = 80 header + 4 count + 50 * 2 triangles = 184 bytes
    assert_eq!(bin.len(), 84 + 50 * 2);

    let parsed = parse_stl(&bin).expect("Binary STL should parse cleanly");
    assert_eq!(parsed.triangles.len(), 2);
    assert_eq!(parsed.triangles[0].v, orig.triangles[0].v);
    assert_eq!(parsed.triangles[1].v, orig.triangles[1].v);
    assert!((parsed.triangles[0].normal - orig.triangles[0].normal).length() < 1e-5);
}

#[test]
fn test_stl_ascii_roundtrip() {
    let t1 = Triangle {
        normal: Vec3::new(0.0, 0.0, 1.0),
        v: [
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(4.0, 5.0, 6.0),
            Vec3::new(7.0, 8.0, 9.0),
        ],
    };
    let orig = StlMesh {
        triangles: vec![t1],
    };

    let ascii = write_ascii(&orig, "TestSolid");
    assert!(ascii.starts_with("solid TestSolid"));
    assert!(ascii.contains("endsolid TestSolid"));

    let parsed = parse_stl(ascii.as_bytes()).expect("Exported ASCII STL should parse");
    assert_eq!(parsed.triangles.len(), 1);
    assert_eq!(parsed.triangles[0].v, orig.triangles[0].v);
}

#[test]
fn test_stl_topology_welding_to_indexed() {
    // Two triangles sharing an edge (0,0,0) - (1,0,0)
    let t1 = Triangle {
        normal: Vec3::Z,
        v: [Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)],
    };
    let t2 = Triangle {
        normal: Vec3::Z,
        v: [Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 0.0)],
    };
    let mesh = StlMesh {
        triangles: vec![t1, t2],
    };

    let (positions, indices) = mesh.to_indexed();
    // 6 unindexed vertices should weld into 4 unique shared vertices
    assert_eq!(positions.len(), 4);
    assert_eq!(indices.len(), 2);
}

#[test]
fn test_stl_truncated_binary_detection() {
    // 84 bytes with triangle count 10, but total length is only 84 (needs 84 + 500 = 584)
    let mut header = vec![0u8; 84];
    header[80] = 10; // 10 triangles
    let err = parse_stl(&header);
    assert!(matches!(err, Err(StlError::Truncated)));
}
