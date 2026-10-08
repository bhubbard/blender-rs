//! Converts between [`blender_obj::ObjMesh`] and [`blender_bmesh::BMesh`].
//!
//! UVs and normals are not carried by the BMesh kernel yet, so only positions and face
//! topology round-trip.

use blender_bmesh::{BMesh, BMeshError, VertHandle};
use blender_obj::{Corner, Face, ObjMesh};
use std::collections::HashMap;

/// Builds a BMesh from OBJ data. Returns the mesh and the handle of each OBJ position,
/// in OBJ order. Faces that fail to build (e.g. repeated vertices) abort the conversion.
pub fn obj_to_bmesh(obj: &ObjMesh) -> Result<(BMesh, Vec<VertHandle>), BMeshError> {
    let mut bm = BMesh::new();
    let verts: Vec<VertHandle> = obj
        .positions
        .iter()
        .map(|&p| bm.vert_create(Some(p), None))
        .collect();
    for face in &obj.faces {
        let vs: Vec<VertHandle> = face.corners.iter().map(|c| verts[c.v as usize]).collect();
        bm.face_create(&vs, None)?;
    }
    Ok((bm, verts))
}

/// Exports live BMesh elements. Vertices are numbered in pool order, so deleted vertices
/// leave no gaps. Wire edges and loose vertices (no faces) are kept as positions only.
pub fn bmesh_to_obj(bm: &BMesh) -> ObjMesh {
    let mut obj = ObjMesh::default();
    let mut index_of: HashMap<u32, u32> = HashMap::new();
    for (h, v) in bm.vpool.iter() {
        index_of.insert(h.index, obj.positions.len() as u32);
        obj.positions.push(v.co);
    }
    for (fh, _) in bm.fpool.iter() {
        let corners: Vec<Corner> = bm
            .face_verts(fh)
            .into_iter()
            .map(|v| Corner { v: index_of[&v.index], vt: None, vn: None })
            .collect();
        obj.faces.push(Face { corners, group: None });
    }
    obj
}

#[cfg(test)]
mod tests {
    use super::*;
    use blender_obj::parse_obj;

    const CUBE: &str = "\
v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nv 0 0 1\nv 1 0 1\nv 1 1 1\nv 0 1 1\n\
f 1 4 3 2\nf 5 6 7 8\nf 1 2 6 5\nf 2 3 7 6\nf 3 4 8 7\nf 4 1 5 8\n";

    /// A face as a rotation-independent vertex cycle.
    fn canonical(face: &[u32]) -> Vec<u32> {
        let start = face.iter().enumerate().min_by_key(|&(_, v)| *v).unwrap().0;
        (0..face.len()).map(|i| face[(start + i) % face.len()]).collect()
    }

    fn face_set(m: &ObjMesh) -> Vec<Vec<u32>> {
        let mut f: Vec<_> = m
            .faces
            .iter()
            .map(|f| canonical(&f.corners.iter().map(|c| c.v).collect::<Vec<_>>()))
            .collect();
        f.sort();
        f
    }

    #[test]
    fn cube_is_a_closed_manifold() {
        let (bm, _) = obj_to_bmesh(&parse_obj(CUBE).unwrap()).unwrap();
        assert_eq!((bm.totvert(), bm.totedge(), bm.totface()), (8, 12, 6));
        assert_eq!(bm.totvert() + bm.totface() - bm.totedge(), 2); // Euler characteristic
        bm.validate_topology().unwrap();
        assert!(bm.epool.iter().all(|(e, _)| bm.edge_is_manifold(e)));
        for (v, _) in bm.vpool.iter() {
            assert_eq!(bm.vert_edges(v).len(), 3);
            assert_eq!(bm.vert_faces(v).len(), 3);
        }
    }

    #[test]
    fn roundtrip_preserves_positions_and_faces() {
        let obj = parse_obj(CUBE).unwrap();
        let (bm, _) = obj_to_bmesh(&obj).unwrap();
        let back = bmesh_to_obj(&bm);
        assert_eq!(back.positions, obj.positions);
        assert_eq!(face_set(&back), face_set(&obj));
    }

    #[test]
    fn edits_survive_export() {
        let (mut bm, verts) = obj_to_bmesh(&parse_obj(CUBE).unwrap()).unwrap();
        // Split an edge and delete a vertex's neighbourhood, then export what is left.
        let e = bm.edge_exists(verts[0], verts[1]).unwrap();
        bm.edge_split(e, 0.5).unwrap();
        bm.validate_topology().unwrap();
        let obj = bmesh_to_obj(&bm);
        assert_eq!(obj.positions.len(), 9);
        assert_eq!(obj.faces.len(), 6);
        assert_eq!(obj.faces.iter().map(|f| f.corners.len()).sum::<usize>(), 26);
        bm.vert_kill(verts[6]).unwrap();
        let obj = bmesh_to_obj(&bm);
        assert_eq!(obj.positions.len(), 8);
        assert_eq!(obj.faces.len(), 3);
        for f in &obj.faces {
            assert!(f.corners.iter().all(|c| (c.v as usize) < obj.positions.len()));
        }
        // Exported OBJ text parses back.
        let text = blender_obj::write_obj(&obj);
        assert_eq!(parse_obj(&text).unwrap(), obj);
    }

    #[test]
    fn invalid_face_is_rejected() {
        let src = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 2\n";
        let obj = parse_obj(src).unwrap();
        assert!(obj_to_bmesh(&obj).is_err());
    }

    #[test]
    fn triangulated_obj_matches_bmesh_counts() {
        let obj = parse_obj(CUBE).unwrap();
        let tris = obj.triangulated();
        let tri_obj = ObjMesh {
            positions: obj.positions.clone(),
            faces: tris
                .iter()
                .map(|t| Face {
                    corners: t.iter().map(|&v| Corner { v, vt: None, vn: None }).collect(),
                    group: None,
                })
                .collect(),
            ..Default::default()
        };
        let (bm, _) = obj_to_bmesh(&tri_obj).unwrap();
        assert_eq!((bm.totvert(), bm.totedge(), bm.totface()), (8, 18, 12));
        bm.validate_topology().unwrap();
    }
}
