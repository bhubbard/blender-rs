//! Dissolve and rotate operators, porting `BM_vert_collapse_faces`-style valence-2
//! dissolve (`bmesh_jekv`) and `BM_edge_rotate` from `bmesh_mods.cc`.

use crate::*;

impl BMesh {
    /// Dissolves a vertex with exactly two edges, merging them into a single edge.
    /// Faces around the vertex lose one corner (triangles degenerate and are removed).
    /// Returns the neighbour the vertex was merged into.
    pub fn vert_dissolve_pair(&mut self, v: VertHandle) -> Result<VertHandle, BMeshError> {
        if self.vpool.get(v).is_none() {
            return Err(BMeshError::InvalidVert(v));
        }
        let edges = self.vert_edges(v);
        if edges.len() != 2 {
            return Err(BMeshError::InvalidDissolve);
        }
        let a = self.edge_other_vert(edges[0], v);
        let b = self.edge_other_vert(edges[1], v);
        if a.is_none() || a == b {
            return Err(BMeshError::InvalidDissolve);
        }
        self.edge_collapse(edges[0], v)
    }

    /// Rotates the edge shared by two triangles so it joins the two opposite corners.
    /// Returns the new edge. The input edge handle is invalidated.
    pub fn edge_rotate(&mut self, edge: EdgeHandle) -> Result<EdgeHandle, BMeshError> {
        let (a, b) = match self.epool.get(edge) {
            Some(e) => (e.v1, e.v2),
            None => return Err(BMeshError::InvalidEdge(edge)),
        };
        let faces = self.edge_faces(edge);
        if faces.len() != 2 || faces[0] == faces[1] {
            return Err(BMeshError::InvalidJoin);
        }
        let apex = |bm: &BMesh, f: FaceHandle| {
            let vs = bm.face_verts(f);
            if vs.len() != 3 {
                return None;
            }
            vs.into_iter().find(|&x| x != a && x != b)
        };
        let (c, d) = match (apex(self, faces[0]), apex(self, faces[1])) {
            (Some(c), Some(d)) if c != d => (c, d),
            _ => return Err(BMeshError::InvalidSplit),
        };
        if self.edge_exists(c, d).is_some() {
            return Err(BMeshError::InvalidSplit);
        }
        let quad = self.faces_join_pair(edge)?;
        let (_, new_edge) = self.face_split(quad, c, d)?;
        Ok(new_edge)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dissolve_split_vertex_restores_quads() {
        let mut bm = BMesh::new();
        let v: Vec<_> = (0..6)
            .map(|i| bm.vert_create(Some(Vec3::new((i % 3) as f32, (i / 3) as f32, 0.0)), None))
            .collect();
        let a = bm.face_create(&[v[0], v[1], v[4], v[3]], None).unwrap();
        let b = bm.face_create(&[v[1], v[2], v[5], v[4]], None).unwrap();
        let shared = bm.edge_exists(v[1], v[4]).unwrap();
        let (vn, _) = bm.edge_split(shared, 0.5).unwrap();
        assert_eq!(bm.fpool.get(a).unwrap().len, 5);
        bm.dissolve_check();
        bm.vert_dissolve_pair(vn).unwrap();
        assert_eq!(bm.totvert(), 6);
        assert_eq!(bm.totedge(), 7);
        assert_eq!(bm.fpool.get(a).unwrap().len, 4);
        assert_eq!(bm.fpool.get(b).unwrap().len, 4);
        bm.validate_topology().unwrap();
    }

    #[test]
    fn dissolve_rejects_other_valences() {
        let mut bm = BMesh::new();
        let c = bm.vert_create(None, None);
        let r: Vec<_> = (0..3).map(|_| bm.vert_create(None, None)).collect();
        for &x in &r {
            bm.edge_create(c, x, None).unwrap();
        }
        assert_eq!(bm.vert_dissolve_pair(c), Err(BMeshError::InvalidDissolve));
        assert_eq!(bm.vert_dissolve_pair(r[0]), Err(BMeshError::InvalidDissolve));
    }

    #[test]
    fn rotate_diagonal_of_two_triangles() {
        let mut bm = BMesh::new();
        let v: Vec<_> = (0..4).map(|_| bm.vert_create(None, None)).collect();
        bm.face_create(&[v[0], v[1], v[2]], None).unwrap();
        bm.face_create(&[v[0], v[2], v[3]], None).unwrap();
        let diag = bm.edge_exists(v[0], v[2]).unwrap();
        let ne = bm.edge_rotate(diag).unwrap();
        assert_eq!(bm.edge_exists(v[1], v[3]), Some(ne));
        assert!(bm.edge_exists(v[0], v[2]).is_none());
        assert_eq!(bm.totface(), 2);
        assert_eq!(bm.totedge(), 5);
        for (_, f) in bm.fpool.iter() {
            assert_eq!(f.len, 3);
        }
        assert!(bm.edge_is_manifold(ne));
        bm.validate_topology().unwrap();
    }

    #[test]
    fn rotate_rejects_boundary_and_quads() {
        let mut bm = BMesh::new();
        let v: Vec<_> = (0..4).map(|_| bm.vert_create(None, None)).collect();
        bm.face_create(&v, None).unwrap();
        let e = bm.edge_exists(v[0], v[1]).unwrap();
        assert_eq!(bm.edge_rotate(e), Err(BMeshError::InvalidJoin));
    }

    impl BMesh {
        fn dissolve_check(&self) {
            self.validate_topology().unwrap();
        }
    }
}
