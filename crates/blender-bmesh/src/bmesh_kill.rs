//! Element removal for [`BMesh`], porting `BM_face_kill`, `BM_edge_kill` and
//! `BM_vert_kill` from `upstream/source/blender/bmesh/intern/bmesh_core.cc`.
//!
//! Incidence is resolved through the disk and radial cycles (O(valence)).

use crate::*;

impl BMesh {
    /// Removes a face and all of its loops. Edges and vertices are left in place.
    pub fn face_kill(&mut self, face: FaceHandle) -> Result<(), BMeshError> {
        if self.fpool.get(face).is_none() {
            return Err(BMeshError::InvalidFace(face));
        }
        for l in self.face_loops(face) {
            self.radial_loop_remove(l);
            self.lpool.free(l);
        }
        self.fpool.free(face);
        Ok(())
    }

    /// Removes an edge, first removing every face that uses it.
    pub fn edge_kill(&mut self, edge: EdgeHandle) -> Result<(), BMeshError> {
        let (v1, v2) = match self.epool.get(edge) {
            Some(e) => (e.v1, e.v2),
            None => return Err(BMeshError::InvalidEdge(edge)),
        };
        let faces: Vec<FaceHandle> = self
            .edge_loops(edge)
            .into_iter()
            .filter_map(|l| self.lpool.get(l).map(|lp| lp.f))
            .collect();
        for f in faces {
            // A face may appear more than once; ignore already-killed ones.
            let _ = self.face_kill(f);
        }
        self.disk_edge_remove(v1, edge);
        self.disk_edge_remove(v2, edge);
        self.epool.free(edge);
        Ok(())
    }

    /// Removes a vertex, first removing every edge (and thus face) that uses it.
    pub fn vert_kill(&mut self, vert: VertHandle) -> Result<(), BMeshError> {
        if self.vpool.get(vert).is_none() {
            return Err(BMeshError::InvalidVert(vert));
        }
        for e in self.vert_edges(vert) {
            let _ = self.edge_kill(e);
        }
        self.vpool.free(vert);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quad_pair() -> (BMesh, [VertHandle; 4], [FaceHandle; 2]) {
        let mut bm = BMesh::new();
        let v = [
            bm.vert_create(Some(Vec3::new(0.0, 0.0, 0.0)), None),
            bm.vert_create(Some(Vec3::new(1.0, 0.0, 0.0)), None),
            bm.vert_create(Some(Vec3::new(1.0, 1.0, 0.0)), None),
            bm.vert_create(Some(Vec3::new(0.0, 1.0, 0.0)), None),
        ];
        let f1 = bm.face_create(&[v[0], v[1], v[2]], None).unwrap();
        let f2 = bm.face_create(&[v[0], v[2], v[3]], None).unwrap();
        (bm, v, [f1, f2])
    }

    #[test]
    fn face_kill_removes_loops_keeps_edges() {
        let (mut bm, _, f) = quad_pair();
        assert_eq!(bm.lpool.len(), 6);
        bm.face_kill(f[0]).unwrap();
        assert_eq!(bm.totface(), 1);
        assert_eq!(bm.lpool.len(), 3);
        assert_eq!(bm.totedge(), 5);
        assert_eq!(bm.face_kill(f[0]), Err(BMeshError::InvalidFace(f[0])));
    }

    #[test]
    fn edge_kill_removes_adjacent_faces() {
        let (mut bm, v, _) = quad_pair();
        let diag = bm.edge_find_or_create(v[0], v[2]).unwrap();
        bm.edge_kill(diag).unwrap();
        assert_eq!(bm.totface(), 0);
        assert_eq!(bm.lpool.len(), 0);
        assert_eq!(bm.totedge(), 4);
        for h in v {
            let e = bm.vpool.get(h).unwrap().edge;
            assert_ne!(e, Some(diag));
            assert_eq!(bm.vert_edges(h).len(), 2);
        }
    }

    #[test]
    fn vert_kill_cascades() {
        let (mut bm, v, _) = quad_pair();
        bm.vert_kill(v[1]).unwrap();
        assert_eq!(bm.totvert(), 3);
        assert_eq!(bm.totface(), 1);
        assert_eq!(bm.totedge(), 3);
        assert_eq!(bm.vert_kill(v[1]), Err(BMeshError::InvalidVert(v[1])));
    }
}
