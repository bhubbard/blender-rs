//! Merging operators, porting `BM_faces_join_pair` (`bmesh_jfke`) and
//! `BM_edge_collapse` (`bmesh_jekv`) from `bmesh_core.cc` / `bmesh_mods.cc`.

use crate::*;

impl BMesh {
    /// Joins the two faces sharing `edge` into one, deleting the edge.
    /// Returns the surviving face (the first face of the edge's radial cycle).
    pub fn faces_join_pair(&mut self, edge: EdgeHandle) -> Result<FaceHandle, BMeshError> {
        if self.epool.get(edge).is_none() {
            return Err(BMeshError::InvalidEdge(edge));
        }
        let loops = self.edge_loops(edge);
        if loops.len() != 2 {
            return Err(BMeshError::InvalidJoin);
        }
        let (l_a, l_b) = (loops[0], loops[1]);
        let f_a = self.lpool.get(l_a).map(|l| l.f).ok_or(BMeshError::InvalidJoin)?;
        let f_b = self.lpool.get(l_b).map(|l| l.f).ok_or(BMeshError::InvalidJoin)?;
        if f_a == f_b {
            return Err(BMeshError::InvalidJoin);
        }
        let (a_first, a_last) = {
            let l = self.lpool.get(l_a).ok_or(BMeshError::InvalidJoin)?;
            (l.next, l.prev)
        };
        let (b_first, b_last) = {
            let l = self.lpool.get(l_b).ok_or(BMeshError::InvalidJoin)?;
            (l.next, l.prev)
        };
        let len_a = self.fpool.get(f_a).map(|f| f.len).unwrap_or(0);
        let len_b = self.fpool.get(f_b).map(|f| f.len).unwrap_or(0);

        // Splice: a_first .. a_last -> b_first .. b_last -> a_first
        if let Some(l) = self.lpool.get_mut(a_last) {
            l.next = b_first;
        }
        if let Some(l) = self.lpool.get_mut(b_first) {
            l.prev = a_last;
        }
        if let Some(l) = self.lpool.get_mut(b_last) {
            l.next = a_first;
        }
        if let Some(l) = self.lpool.get_mut(a_first) {
            l.prev = b_last;
        }

        // Drop the two loops on the shared edge, the edge itself and the second face.
        self.radial_loop_remove(l_a);
        self.radial_loop_remove(l_b);
        self.lpool.free(l_a);
        self.lpool.free(l_b);
        let (v1, v2) = {
            let e = self.epool.get(edge).ok_or(BMeshError::InvalidEdge(edge))?;
            (e.v1, e.v2)
        };
        self.disk_edge_remove(v1, edge);
        self.disk_edge_remove(v2, edge);
        self.epool.free(edge);
        self.fpool.free(f_b);

        if let Some(f) = self.fpool.get_mut(f_a) {
            f.l_first = a_first;
            f.len = len_a + len_b - 2;
        }
        // Reassign ownership of every loop of the merged face.
        let mut l = a_first;
        loop {
            let next = match self.lpool.get_mut(l) {
                Some(lp) => {
                    lp.f = f_a;
                    lp.next
                }
                None => break,
            };
            if next == a_first {
                break;
            }
            l = next;
        }
        Ok(f_a)
    }

    /// Collapses `edge` by merging `v_kill` into the other endpoint, which is returned.
    ///
    /// Triangles using the edge degenerate and are removed; larger faces lose one corner.
    /// Edges that would duplicate an existing edge of the surviving vertex are merged.
    pub fn edge_collapse(
        &mut self,
        edge: EdgeHandle,
        v_kill: VertHandle,
    ) -> Result<VertHandle, BMeshError> {
        let v_keep = self
            .edge_other_vert(edge, v_kill)
            .ok_or(BMeshError::InvalidEdge(edge))?;

        // 1. Faces on the edge: kill triangles, drop one loop from the rest.
        for l in self.edge_loops(edge) {
            let Some((f, next)) = self.lpool.get(l).map(|lp| (lp.f, lp.next)) else {
                continue; // face already removed as a triangle
            };
            let len = self.fpool.get(f).map(|fc| fc.len).unwrap_or(0);
            if len <= 3 {
                let _ = self.face_kill(f);
                continue;
            }
            let prev = self.lpool.get(l).map(|lp| lp.prev).unwrap_or(l);
            if let Some(p) = self.lpool.get_mut(prev) {
                p.next = next;
            }
            if let Some(n) = self.lpool.get_mut(next) {
                n.prev = prev;
            }
            if let Some(fc) = self.fpool.get_mut(f) {
                fc.len -= 1;
                if fc.l_first == l {
                    fc.l_first = next;
                }
            }
            self.radial_loop_remove(l);
            self.lpool.free(l);
        }

        // 2. Detach the collapsed edge from both endpoints.
        self.disk_edge_remove(v_kill, edge);
        self.disk_edge_remove(v_keep, edge);
        self.epool.free(edge);

        // 3. Move every remaining edge of v_kill over to v_keep.
        for ek in self.vert_edges(v_kill) {
            let Some(w) = self.edge_other_vert(ek, v_kill) else {
                continue;
            };
            for l in self.edge_loops(ek) {
                if let Some(lp) = self.lpool.get_mut(l) {
                    if lp.v == v_kill {
                        lp.v = v_keep;
                    }
                }
            }
            if let Some(et) = self.edge_exists(v_keep, w) {
                // Duplicate: fold ek's loops into the existing edge.
                for l in self.edge_loops(ek) {
                    self.radial_loop_remove(l);
                    if let Some(lp) = self.lpool.get_mut(l) {
                        lp.e = et;
                    }
                    self.radial_loop_append(et, l);
                }
                self.disk_edge_remove(v_kill, ek);
                self.disk_edge_remove(w, ek);
                self.epool.free(ek);
            } else {
                self.disk_edge_remove(v_kill, ek);
                if let Some(ed) = self.epool.get_mut(ek) {
                    if ed.v1 == v_kill {
                        ed.v1 = v_keep;
                    } else if ed.v2 == v_kill {
                        ed.v2 = v_keep;
                    }
                }
                self.disk_edge_append(v_keep, ek);
            }
        }
        self.vpool.free(v_kill);
        Ok(v_keep)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid_2x1() -> (BMesh, Vec<VertHandle>, [FaceHandle; 2]) {
        // 3 --- 4 --- 5
        // |  A  |  B  |
        // 0 --- 1 --- 2
        let mut bm = BMesh::new();
        let v: Vec<_> = (0..6)
            .map(|i| {
                bm.vert_create(Some(Vec3::new((i % 3) as f32, (i / 3) as f32, 0.0)), None)
            })
            .collect();
        let a = bm.face_create(&[v[0], v[1], v[4], v[3]], None).unwrap();
        let b = bm.face_create(&[v[1], v[2], v[5], v[4]], None).unwrap();
        (bm, v, [a, b])
    }

    #[test]
    fn join_two_quads_into_hexagon() {
        let (mut bm, v, [a, b]) = grid_2x1();
        let shared = bm.edge_exists(v[1], v[4]).unwrap();
        let f = bm.faces_join_pair(shared).unwrap();
        assert_eq!(f, a);
        assert!(bm.fpool.get(b).is_none());
        assert_eq!(bm.totface(), 1);
        assert_eq!(bm.totedge(), 6);
        assert_eq!(bm.lpool.len(), 6);
        assert_eq!(bm.fpool.get(f).unwrap().len, 6);
        assert_eq!(bm.face_verts(f).len(), 6);
        bm.validate_topology().unwrap();
    }

    #[test]
    fn join_rejects_boundary_edge() {
        let (mut bm, v, _) = grid_2x1();
        let boundary = bm.edge_exists(v[0], v[1]).unwrap();
        assert_eq!(bm.faces_join_pair(boundary), Err(BMeshError::InvalidJoin));
    }

    #[test]
    fn collapse_shared_edge_of_quads() {
        let (mut bm, v, [a, b]) = grid_2x1();
        let shared = bm.edge_exists(v[1], v[4]).unwrap();
        let kept = bm.edge_collapse(shared, v[1]).unwrap();
        assert_eq!(kept, v[4]);
        assert!(bm.vpool.get(v[1]).is_none());
        assert_eq!(bm.totvert(), 5);
        assert_eq!(bm.fpool.get(a).unwrap().len, 3);
        assert_eq!(bm.fpool.get(b).unwrap().len, 3);
        assert_eq!(bm.totedge(), 6);
        bm.validate_topology().unwrap();
    }

    #[test]
    fn collapse_degenerate_triangles_and_merge_duplicates() {
        let mut bm = BMesh::new();
        let v: Vec<_> = (0..4).map(|_| bm.vert_create(None, None)).collect();
        bm.face_create(&[v[0], v[1], v[2]], None).unwrap();
        bm.face_create(&[v[0], v[2], v[3]], None).unwrap();
        let diag = bm.edge_exists(v[0], v[2]).unwrap();
        bm.edge_collapse(diag, v[0]).unwrap();
        assert_eq!(bm.totface(), 0);
        assert_eq!(bm.lpool.len(), 0);
        assert_eq!(bm.totvert(), 3);
        assert_eq!(bm.totedge(), 2);
        bm.validate_topology().unwrap();
    }

    #[test]
    fn collapse_boundary_edge_of_fan() {
        // Hexagon fan: collapse a rim edge, expect center faces to stay valid.
        let mut bm = BMesh::new();
        let c = bm.vert_create(None, None);
        let rim: Vec<_> = (0..5).map(|_| bm.vert_create(None, None)).collect();
        for i in 0..5 {
            bm.face_create(&[c, rim[i], rim[(i + 1) % 5]], None).unwrap();
        }
        let e = bm.edge_exists(rim[0], rim[1]).unwrap();
        bm.edge_collapse(e, rim[0]).unwrap();
        assert_eq!(bm.totvert(), 5);
        assert_eq!(bm.totface(), 4);
        bm.validate_topology().unwrap();
    }
}
