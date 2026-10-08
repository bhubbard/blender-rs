//! Topology-splitting operators, porting `BM_edge_split` (`bmesh_semv`) and
//! `BM_face_split` (`bmesh_sfme`) from `upstream/source/blender/bmesh/intern/bmesh_core.cc`.

use crate::*;

impl BMesh {
    /// Splits `edge` at `fac` (0 = `v1`, 1 = `v2`), returning the new vertex and the new edge.
    ///
    /// `edge` keeps `v1` and now ends at the new vertex; the new edge runs new vertex → old `v2`.
    /// Every face using the edge gains one loop.
    pub fn edge_split(
        &mut self,
        edge: EdgeHandle,
        fac: f32,
    ) -> Result<(VertHandle, EdgeHandle), BMeshError> {
        let (v1, v2) = match self.epool.get(edge) {
            Some(e) => (e.v1, e.v2),
            None => return Err(BMeshError::InvalidEdge(edge)),
        };
        let co1 = self.vpool.get(v1).map(|v| v.co).unwrap_or(Vec3::ZERO);
        let co2 = self.vpool.get(v2).map(|v| v.co).unwrap_or(Vec3::ZERO);
        let vn = self.vert_create(Some(co1.lerp(co2, fac)), Some(v1));
        self.vert_data_interp_pair(vn, v1, v2, fac);

        // Move the far end of `edge` from v2 to the new vertex.
        self.disk_edge_remove(v2, edge);
        if let Some(e) = self.epool.get_mut(edge) {
            e.v2 = vn;
        }
        self.disk_edge_append(vn, edge);
        let ne = self.edge_create(vn, v2, Some(edge))?;

        let loops = self.edge_loops(edge);
        for l in loops {
            let (lv, f, next, prev) = match self.lpool.get(l) {
                Some(lp) => (lp.v, lp.f, lp.next, lp.prev),
                None => continue,
            };
            let mut nl = BMLoop {
                head: BMHeader::new(ElemType::Loop),
                v: vn,
                e: ne,
                f,
                next: l,
                prev: l,
                radial_next: LoopHandle::invalid(),
                radial_prev: LoopHandle::invalid(),
            };
            let nlh;
            if lv == v1 {
                // Loop runs v1 -> v2: new loop (vn -> v2) follows it.
                nl.prev = l;
                nl.next = next;
                nlh = self.lpool.alloc(nl);
                if let Some(n) = self.lpool.get_mut(next) {
                    n.prev = nlh;
                }
                if let Some(lp) = self.lpool.get_mut(l) {
                    lp.next = nlh;
                }
            } else {
                // Loop runs v2 -> v1: it now starts at vn; new loop (v2 -> vn) precedes it.
                nl.v = v2;
                nl.next = l;
                nl.prev = prev;
                nlh = self.lpool.alloc(nl);
                if let Some(p) = self.lpool.get_mut(prev) {
                    p.next = nlh;
                }
                if let Some(lp) = self.lpool.get_mut(l) {
                    lp.prev = nlh;
                    lp.v = vn;
                }
            }
            self.radial_loop_append(ne, nlh);
            if let Some(face) = self.fpool.get_mut(f) {
                face.len += 1;
            }
        }
        Ok((vn, ne))
    }

    /// Splits `face` along a new edge between `v1` and `v2`, returning the new face and edge.
    ///
    /// `face` keeps the loops from `v1` up to (excluding) `v2`; the new face takes the rest.
    pub fn face_split(
        &mut self,
        face: FaceHandle,
        v1: VertHandle,
        v2: VertHandle,
    ) -> Result<(FaceHandle, EdgeHandle), BMeshError> {
        if self.fpool.get(face).is_none() {
            return Err(BMeshError::InvalidFace(face));
        }
        if v1 == v2 {
            return Err(BMeshError::InvalidSplit);
        }
        let loops = self.face_loops(face);
        let find = |bm: &BMesh, v: VertHandle| {
            loops
                .iter()
                .copied()
                .find(|&l| bm.lpool.get(l).is_some_and(|lp| lp.v == v))
        };
        let (l1, l2) = match (find(self, v1), find(self, v2)) {
            (Some(a), Some(b)) => (a, b),
            _ => return Err(BMeshError::InvalidSplit),
        };
        let (p1, n1) = {
            let lp = self.lpool.get(l1).ok_or(BMeshError::InvalidSplit)?;
            (lp.prev, lp.next)
        };
        let (p2, n2) = {
            let lp = self.lpool.get(l2).ok_or(BMeshError::InvalidSplit)?;
            (lp.prev, lp.next)
        };
        if n1 == l2 || n2 == l1 {
            return Err(BMeshError::InvalidSplit);
        }

        let edge = self.edge_create(v1, v2, None)?;
        let no = self.fpool.get(face).map(|f| f.no).unwrap_or(Vec3::ZERO);
        let new_face = self.fpool.alloc(BMFace {
            head: BMHeader::new(ElemType::Face),
            l_first: l2,
            len: 0,
            no,
        });
        let mk = |v, f, next, prev| BMLoop {
            head: BMHeader::new(ElemType::Loop),
            v,
            e: edge,
            f,
            next,
            prev,
            radial_next: LoopHandle::invalid(),
            radial_prev: LoopHandle::invalid(),
        };
        // Closing loop of `face` (v2 -> v1) and of `new_face` (v1 -> v2).
        let c_a = self.lpool.alloc(mk(v2, face, l1, p2));
        let c_b = self.lpool.alloc(mk(v1, new_face, l2, p1));
        self.radial_loop_append(edge, c_a);
        self.radial_loop_append(edge, c_b);

        // Relink perimeter of `face`: l1 .. p2 -> c_a -> l1
        if let Some(l) = self.lpool.get_mut(p2) {
            l.next = c_a;
        }
        if let Some(l) = self.lpool.get_mut(l1) {
            l.prev = c_a;
        }
        // Perimeter of `new_face`: l2 .. p1 -> c_b -> l2
        if let Some(l) = self.lpool.get_mut(p1) {
            l.next = c_b;
        }
        if let Some(l) = self.lpool.get_mut(l2) {
            l.prev = c_b;
        }

        if let Some(f) = self.fpool.get_mut(face) {
            f.l_first = l1;
        }
        let a_len = self.walk_assign(l1, face);
        let b_len = self.walk_assign(l2, new_face);
        if let Some(f) = self.fpool.get_mut(face) {
            f.len = a_len;
        }
        if let Some(f) = self.fpool.get_mut(new_face) {
            f.len = b_len;
        }
        Ok((new_face, edge))
    }

    /// Walks a loop cycle from `start`, assigning ownership to `face`; returns the length.
    fn walk_assign(&mut self, start: LoopHandle, face: FaceHandle) -> u32 {
        let mut n = 0;
        let mut l = start;
        loop {
            let next = match self.lpool.get_mut(l) {
                Some(lp) => {
                    lp.f = face;
                    lp.next
                }
                None => break,
            };
            n += 1;
            if next == start {
                break;
            }
            l = next;
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verts_of(bm: &BMesh, f: FaceHandle) -> Vec<VertHandle> {
        bm.face_loops(f)
            .into_iter()
            .map(|l| bm.lpool.get(l).unwrap().v)
            .collect()
    }

    #[test]
    fn edge_split_triangle() {
        let mut bm = BMesh::new();
        let v = [
            bm.vert_create(Some(Vec3::new(0.0, 0.0, 0.0)), None),
            bm.vert_create(Some(Vec3::new(2.0, 0.0, 0.0)), None),
            bm.vert_create(Some(Vec3::new(0.0, 2.0, 0.0)), None),
        ];
        let f = bm.face_create(&v, None).unwrap();
        let e = bm.edge_exists(v[0], v[1]).unwrap();
        let (vn, ne) = bm.edge_split(e, 0.5).unwrap();
        assert_eq!(bm.vpool.get(vn).unwrap().co, Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(bm.totedge(), 4);
        assert_eq!(bm.lpool.len(), 4);
        assert_eq!(bm.fpool.get(f).unwrap().len, 4);
        assert_eq!(bm.edge_exists(vn, v[1]), Some(ne));
        assert!(bm.edge_exists(v[0], v[1]).is_none());
        // Winding is preserved: v0, vn, v1, v2 in some rotation.
        let vs = verts_of(&bm, f);
        let start = vs.iter().position(|&x| x == v[0]).unwrap();
        let rot: Vec<_> = (0..4).map(|i| vs[(start + i) % 4]).collect();
        assert_eq!(rot, vec![v[0], vn, v[1], v[2]]);
        bm.validate_topology().unwrap();
    }

    #[test]
    fn edge_split_shared_edge_updates_both_faces() {
        let mut bm = BMesh::new();
        let v: Vec<_> = (0..4)
            .map(|i| bm.vert_create(Some(Vec3::new(i as f32, 0.0, 0.0)), None))
            .collect();
        let f1 = bm.face_create(&[v[0], v[1], v[2]], None).unwrap();
        let f2 = bm.face_create(&[v[0], v[2], v[3]], None).unwrap();
        let shared = bm.edge_exists(v[0], v[2]).unwrap();
        bm.edge_split(shared, 0.5).unwrap();
        assert_eq!(bm.fpool.get(f1).unwrap().len, 4);
        assert_eq!(bm.fpool.get(f2).unwrap().len, 4);
        bm.validate_topology().unwrap();
    }

    #[test]
    fn face_split_quad() {
        let mut bm = BMesh::new();
        let v: Vec<_> = (0..4).map(|_| bm.vert_create(None, None)).collect();
        let f = bm.face_create(&v, None).unwrap();
        let (nf, ne) = bm.face_split(f, v[0], v[2]).unwrap();
        assert_eq!(bm.totface(), 2);
        assert_eq!(bm.totedge(), 5);
        assert_eq!(bm.lpool.len(), 6);
        assert_eq!(bm.fpool.get(f).unwrap().len, 3);
        assert_eq!(bm.fpool.get(nf).unwrap().len, 3);
        assert_eq!(bm.edge_loops(ne).len(), 2);
        bm.validate_topology().unwrap();
    }

    #[test]
    fn face_split_rejects_bad_input() {
        let mut bm = BMesh::new();
        let v: Vec<_> = (0..4).map(|_| bm.vert_create(None, None)).collect();
        let outsider = bm.vert_create(None, None);
        let f = bm.face_create(&v, None).unwrap();
        assert_eq!(bm.face_split(f, v[0], v[0]), Err(BMeshError::InvalidSplit));
        assert_eq!(bm.face_split(f, v[0], v[1]), Err(BMeshError::InvalidSplit));
        assert_eq!(bm.face_split(f, v[0], outsider), Err(BMeshError::InvalidSplit));
        assert_eq!(bm.totface(), 1);
    }

    #[test]
    fn split_then_kill_stays_consistent() {
        let mut bm = BMesh::new();
        let v: Vec<_> = (0..4).map(|_| bm.vert_create(None, None)).collect();
        let f = bm.face_create(&v, None).unwrap();
        let (nf, ne) = bm.face_split(f, v[0], v[2]).unwrap();
        bm.face_kill(nf).unwrap();
        assert_eq!(bm.edge_loops(ne).len(), 1);
        bm.validate_topology().unwrap();
        bm.vert_kill(v[1]).unwrap();
        bm.validate_topology().unwrap();
    }
}
