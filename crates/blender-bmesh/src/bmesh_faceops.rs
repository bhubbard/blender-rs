//! Face-level operators: `BM_faces_join`, `BM_face_normal_flip` and fan triangulation
//! (`BM_face_triangulate` with the fan method), built on the verified join/split primitives.

use crate::*;

impl BMesh {
    /// Joins a connected set of faces into one by removing the manifold edges between them.
    /// Fails (leaving the mesh untouched) if the set is empty, has duplicates, or is not
    /// connected through edges shared by exactly two faces of the set.
    pub fn faces_join(&mut self, faces: &[FaceHandle]) -> Result<FaceHandle, BMeshError> {
        let Some(&first) = faces.first() else {
            return Err(BMeshError::InvalidJoin);
        };
        for (i, f) in faces.iter().enumerate() {
            if self.fpool.get(*f).is_none() {
                return Err(BMeshError::InvalidFace(*f));
            }
            if faces[..i].contains(f) {
                return Err(BMeshError::InvalidJoin);
            }
        }
        // Edges to remove: manifold edges whose two faces are both in the set. A spanning
        // tree suffices; extra shared edges would create holes, so they are also rejected.
        let mut seen = vec![first];
        let mut tree_edges: Vec<EdgeHandle> = Vec::new();
        let mut frontier = vec![first];
        while let Some(f) = frontier.pop() {
            for e in self.face_edges(f) {
                let fs = self.edge_faces(e);
                if fs.len() != 2 || fs[0] == fs[1] {
                    continue;
                }
                let other = if fs[0] == f { fs[1] } else { fs[0] };
                if !faces.contains(&other) {
                    continue;
                }
                if seen.contains(&other) {
                    if !tree_edges.contains(&e) {
                        // A second shared edge between faces already connected: joining
                        // would pinch off a hole.
                        return Err(BMeshError::InvalidJoin);
                    }
                    continue;
                }
                seen.push(other);
                tree_edges.push(e);
                frontier.push(other);
            }
        }
        if seen.len() != faces.len() {
            return Err(BMeshError::InvalidJoin);
        }
        let mut survivor = first;
        for e in tree_edges {
            survivor = self.faces_join_pair(e)?;
        }
        Ok(survivor)
    }

    /// Reverses a face's winding, returning the replacement face (the old handle dies).
    pub fn face_reverse(&mut self, face: FaceHandle) -> Result<FaceHandle, BMeshError> {
        if self.fpool.get(face).is_none() {
            return Err(BMeshError::InvalidFace(face));
        }
        let mut verts = self.face_verts(face);
        verts.reverse();
        self.face_kill(face)?;
        self.face_create(&verts, None)
    }

    /// Triangulates a face as a fan from its first vertex; correct for convex faces.
    /// Returns the `len - 2` resulting faces (just `[face]` for a triangle).
    pub fn face_triangulate_fan(&mut self, face: FaceHandle) -> Result<Vec<FaceHandle>, BMeshError> {
        if self.fpool.get(face).is_none() {
            return Err(BMeshError::InvalidFace(face));
        }
        let verts = self.face_verts(face);
        let n = verts.len();
        let mut tris = Vec::with_capacity(n.saturating_sub(2));
        let mut cur = face;
        for &v in &verts[2..n.saturating_sub(1)] {
            let (rest, _) = self.face_split(cur, verts[0], v)?;
            tris.push(cur);
            cur = rest;
        }
        tris.push(cur);
        Ok(tris)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strip(n: usize) -> (BMesh, Vec<FaceHandle>) {
        let mut bm = BMesh::new();
        let bottom: Vec<_> = (0..=n).map(|i| bm.vert_create(Some(Vec3::new(i as f32, 0.0, 0.0)), None)).collect();
        let top: Vec<_> = (0..=n).map(|i| bm.vert_create(Some(Vec3::new(i as f32, 1.0, 0.0)), None)).collect();
        let faces = (0..n)
            .map(|i| bm.face_create(&[bottom[i], bottom[i + 1], top[i + 1], top[i]], None).unwrap())
            .collect();
        (bm, faces)
    }

    #[test]
    fn join_strip_into_one_polygon() {
        let (mut bm, faces) = strip(3);
        let f = bm.faces_join(&faces).unwrap();
        assert_eq!(bm.totface(), 1);
        assert_eq!(bm.fpool.get(f).unwrap().len, 8);
        assert_eq!(bm.totedge(), 8);
        assert_eq!(bm.lpool.len(), 8);
        bm.validate_topology().unwrap();
    }

    #[test]
    fn join_subset_and_single() {
        let (mut bm, faces) = strip(3);
        let f = bm.faces_join(&faces[..2]).unwrap();
        assert_eq!(bm.totface(), 2);
        assert_eq!(bm.fpool.get(f).unwrap().len, 6);
        bm.validate_topology().unwrap();
        // A single face is returned unchanged.
        assert_eq!(bm.faces_join(&[f]).unwrap(), f);
        assert_eq!(bm.totface(), 2);
    }

    #[test]
    fn join_rejects_disconnected_duplicate_and_ring() {
        let (mut bm, faces) = strip(3);
        let before = (bm.totface(), bm.totedge());
        assert_eq!(bm.faces_join(&[faces[0], faces[2]]), Err(BMeshError::InvalidJoin));
        assert_eq!(bm.faces_join(&[faces[0], faces[0]]), Err(BMeshError::InvalidJoin));
        assert_eq!(bm.faces_join(&[]), Err(BMeshError::InvalidJoin));
        assert_eq!((bm.totface(), bm.totedge()), before);
        bm.validate_topology().unwrap();

        // A closed fan of three triangles shares an edge between every pair; the third shared
        // edge would enclose a hole, so joining all of them is rejected.
        let mut bm = BMesh::new();
        let c = bm.vert_create(None, None);
        let r: Vec<_> = (0..3).map(|_| bm.vert_create(None, None)).collect();
        let fs: Vec<_> = (0..3).map(|i| bm.face_create(&[c, r[i], r[(i + 1) % 3]], None).unwrap()).collect();
        assert_eq!(bm.faces_join(&fs), Err(BMeshError::InvalidJoin));
        assert_eq!(bm.totface(), 3);
    }

    #[test]
    fn reverse_flips_normal() {
        let mut bm = BMesh::new();
        let v: Vec<_> = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
            .iter()
            .map(|&(x, y)| bm.vert_create(Some(Vec3::new(x, y, 0.0)), None))
            .collect();
        let f = bm.face_create(&v, None).unwrap();
        let g = bm.face_reverse(f).unwrap();
        assert!(bm.fpool.get(f).is_none());
        let mut want = v.clone();
        want.reverse();
        let got = bm.face_verts(g);
        let start = got.iter().position(|&x| x == want[0]).unwrap();
        let rot: Vec<_> = (0..4).map(|i| got[(start + i) % 4]).collect();
        assert_eq!(rot, want);
        assert_eq!(bm.totedge(), 4);
        bm.validate_topology().unwrap();
        assert!(bm.fpool.get(g).unwrap().no.z < 0.0, "reversed face must face -Z");
    }

    #[test]
    fn fan_triangulation_of_polygons() {
        for n in 3..=8usize {
            let mut bm = BMesh::new();
            let v: Vec<_> = (0..n)
                .map(|i| {
                    let a = i as f32 / n as f32 * std::f32::consts::TAU;
                    bm.vert_create(Some(Vec3::new(a.cos(), a.sin(), 0.0)), None)
                })
                .collect();
            let f = bm.face_create(&v, None).unwrap();
            let tris = bm.face_triangulate_fan(f).unwrap();
            assert_eq!(tris.len(), n - 2, "n={n}");
            assert_eq!(bm.totface(), n - 2);
            assert_eq!(bm.totedge(), n + (n - 3));
            for &t in &tris {
                assert_eq!(bm.fpool.get(t).unwrap().len, 3, "n={n}");
                assert!(bm.vert_in_face(v[0], t));
            }
            bm.validate_topology().unwrap();
        }
    }

    #[test]
    fn invalid_handles() {
        let mut bm = BMesh::new();
        let v: Vec<_> = (0..3).map(|_| bm.vert_create(None, None)).collect();
        let f = bm.face_create(&v, None).unwrap();
        bm.face_kill(f).unwrap();
        assert_eq!(bm.face_reverse(f), Err(BMeshError::InvalidFace(f)));
        assert_eq!(bm.face_triangulate_fan(f), Err(BMeshError::InvalidFace(f)));
        assert_eq!(bm.faces_join(&[f]), Err(BMeshError::InvalidFace(f)));
    }
}
