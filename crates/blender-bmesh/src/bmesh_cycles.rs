//! Disk and radial cycle maintenance, porting `bmesh_disk_*` / `bmesh_radial_*` from
//! `upstream/source/blender/bmesh/intern/bmesh_core.cc`.
//!
//! * Disk cycle: circular list of the edges around a vertex (`BMVert::edge` is the entry).
//! * Radial cycle: circular list of the loops around an edge (`BMEdge::loop_` is the entry).

use crate::*;

/// Upper bound guard so a corrupted cycle can never hang a walk.
const CYCLE_GUARD: usize = 1 << 24;

impl BMesh {
    /// Returns `(prev, next)` of `e` in the disk cycle of `v`.
    pub fn disk_links(&self, e: EdgeHandle, v: VertHandle) -> Option<(EdgeHandle, EdgeHandle)> {
        let ed = self.epool.get(e)?;
        if ed.v1 == v {
            Some((ed.v1_disk_prev?, ed.v1_disk_next?))
        } else if ed.v2 == v {
            Some((ed.v2_disk_prev?, ed.v2_disk_next?))
        } else {
            None
        }
    }

    fn set_disk_links(
        &mut self,
        e: EdgeHandle,
        v: VertHandle,
        prev: Option<EdgeHandle>,
        next: Option<EdgeHandle>,
    ) {
        if let Some(ed) = self.epool.get_mut(e) {
            if ed.v1 == v {
                ed.v1_disk_prev = prev;
                ed.v1_disk_next = next;
            } else if ed.v2 == v {
                ed.v2_disk_prev = prev;
                ed.v2_disk_next = next;
            }
        }
    }

    /// Inserts `e` into the disk cycle of `v` (`bmesh_disk_edge_append`).
    pub fn disk_edge_append(&mut self, v: VertHandle, e: EdgeHandle) {
        let first = self.vpool.get(v).and_then(|vert| vert.edge);
        match first {
            None => {
                self.set_disk_links(e, v, Some(e), Some(e));
                if let Some(vert) = self.vpool.get_mut(v) {
                    vert.edge = Some(e);
                }
            }
            Some(first) => {
                let last = self.disk_links(first, v).map(|(p, _)| p).unwrap_or(first);
                self.set_disk_links(e, v, Some(last), Some(first));
                if let Some((lp, _)) = self.disk_links(last, v) {
                    self.set_disk_links(last, v, Some(lp), Some(e));
                }
                if let Some((_, fnext)) = self.disk_links(first, v) {
                    self.set_disk_links(first, v, Some(e), Some(fnext));
                }
            }
        }
    }

    /// Removes `e` from the disk cycle of `v` (`bmesh_disk_edge_remove`).
    pub fn disk_edge_remove(&mut self, v: VertHandle, e: EdgeHandle) {
        let Some((prev, next)) = self.disk_links(e, v) else {
            return;
        };
        if next == e {
            if let Some(vert) = self.vpool.get_mut(v) {
                if vert.edge == Some(e) {
                    vert.edge = None;
                }
            }
        } else {
            if let Some((pp, _)) = self.disk_links(prev, v) {
                self.set_disk_links(prev, v, Some(pp), Some(next));
            }
            if let Some((_, nn)) = self.disk_links(next, v) {
                self.set_disk_links(next, v, Some(prev), Some(nn));
            }
            if let Some(vert) = self.vpool.get_mut(v) {
                if vert.edge == Some(e) {
                    vert.edge = Some(next);
                }
            }
        }
        self.set_disk_links(e, v, None, None);
    }

    /// Edges around `v` in disk-cycle order.
    pub fn vert_edges(&self, v: VertHandle) -> Vec<EdgeHandle> {
        let mut out = Vec::new();
        let Some(start) = self.vpool.get(v).and_then(|vert| vert.edge) else {
            return out;
        };
        let mut e = start;
        for _ in 0..CYCLE_GUARD {
            out.push(e);
            match self.disk_links(e, v) {
                Some((_, next)) if next != start => e = next,
                _ => break,
            }
        }
        out
    }

    /// Finds the edge joining `v1` and `v2` by walking the disk cycle of `v1` (`BM_edge_exists`).
    pub fn edge_exists(&self, v1: VertHandle, v2: VertHandle) -> Option<EdgeHandle> {
        self.vert_edges(v1).into_iter().find(|&e| {
            self.epool
                .get(e)
                .is_some_and(|ed| (ed.v1 == v1 && ed.v2 == v2) || (ed.v1 == v2 && ed.v2 == v1))
        })
    }

    /// Inserts loop `l` into the radial cycle of `e` (`bmesh_radial_loop_append`).
    pub fn radial_loop_append(&mut self, e: EdgeHandle, l: LoopHandle) {
        let first = self.epool.get(e).and_then(|ed| ed.loop_);
        match first {
            None => {
                if let Some(lp) = self.lpool.get_mut(l) {
                    lp.radial_next = l;
                    lp.radial_prev = l;
                }
                if let Some(ed) = self.epool.get_mut(e) {
                    ed.loop_ = Some(l);
                }
            }
            Some(first) => {
                let last = self
                    .lpool
                    .get(first)
                    .map(|lp| lp.radial_prev)
                    .unwrap_or(first);
                if let Some(lp) = self.lpool.get_mut(l) {
                    lp.radial_prev = last;
                    lp.radial_next = first;
                }
                if let Some(lp) = self.lpool.get_mut(last) {
                    lp.radial_next = l;
                }
                if let Some(lp) = self.lpool.get_mut(first) {
                    lp.radial_prev = l;
                }
            }
        }
    }

    /// Removes loop `l` from the radial cycle of its edge (`bmesh_radial_loop_remove`).
    pub fn radial_loop_remove(&mut self, l: LoopHandle) {
        let Some((e, next, prev)) = self.lpool.get(l).map(|lp| (lp.e, lp.radial_next, lp.radial_prev))
        else {
            return;
        };
        if next == l {
            if let Some(ed) = self.epool.get_mut(e) {
                if ed.loop_ == Some(l) {
                    ed.loop_ = None;
                }
            }
        } else {
            if let Some(lp) = self.lpool.get_mut(prev) {
                lp.radial_next = next;
            }
            if let Some(lp) = self.lpool.get_mut(next) {
                lp.radial_prev = prev;
            }
            if let Some(ed) = self.epool.get_mut(e) {
                if ed.loop_ == Some(l) {
                    ed.loop_ = Some(next);
                }
            }
        }
        if let Some(lp) = self.lpool.get_mut(l) {
            lp.radial_next = l;
            lp.radial_prev = l;
        }
    }

    /// Loops around edge `e` in radial order.
    pub fn edge_loops(&self, e: EdgeHandle) -> Vec<LoopHandle> {
        let mut out = Vec::new();
        let Some(start) = self.epool.get(e).and_then(|ed| ed.loop_) else {
            return out;
        };
        let mut l = start;
        for _ in 0..CYCLE_GUARD {
            out.push(l);
            match self.lpool.get(l) {
                Some(lp) if lp.radial_next != start => l = lp.radial_next,
                _ => break,
            }
        }
        out
    }

    /// Loops of face `f` in perimeter order.
    pub fn face_loops(&self, f: FaceHandle) -> Vec<LoopHandle> {
        let mut out = Vec::new();
        let Some(start) = self.fpool.get(f).map(|face| face.l_first) else {
            return out;
        };
        let mut l = start;
        for _ in 0..CYCLE_GUARD {
            out.push(l);
            match self.lpool.get(l) {
                Some(lp) if lp.next != start => l = lp.next,
                _ => break,
            }
        }
        out
    }
}

impl BMesh {
    /// Checks loop, radial and disk cycle consistency (cf. `BM_mesh_validate`).
    pub fn validate_topology(&self) -> Result<(), String> {
        for (fh, f) in self.fpool.iter() {
            let loops = self.face_loops(fh);
            if loops.len() != f.len as usize {
                return Err(format!("face {fh:?}: len {} != walked {}", f.len, loops.len()));
            }
            for &l in &loops {
                let lp = self.lpool.get(l).ok_or("dangling loop")?;
                if lp.f != fh {
                    return Err(format!("loop {l:?} owned by wrong face"));
                }
                if self.lpool.get(lp.next).map(|n| n.prev) != Some(l) {
                    return Err(format!("loop {l:?}: next.prev mismatch"));
                }
                if !self.edge_loops(lp.e).contains(&l) {
                    return Err(format!("loop {l:?} missing from radial cycle"));
                }
            }
        }
        for (eh, e) in self.epool.iter() {
            for v in [e.v1, e.v2] {
                if !self.vert_edges(v).contains(&eh) {
                    return Err(format!("edge {eh:?} missing from disk of {v:?}"));
                }
            }
            for l in self.edge_loops(eh) {
                if self.lpool.get(l).map(|lp| lp.e) != Some(eh) {
                    return Err(format!("radial loop {l:?} not on edge {eh:?}"));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disk_cycle_of_triangle_fan() {
        let mut bm = BMesh::new();
        let c = bm.vert_create(None, None);
        let vs: Vec<_> = (0..3).map(|_| bm.vert_create(None, None)).collect();
        let es: Vec<_> = vs
            .iter()
            .map(|&v| bm.edge_create(c, v, None).unwrap())
            .collect();
        assert_eq!(bm.vert_edges(c), es);
        bm.disk_edge_remove(c, es[1]);
        assert_eq!(bm.vert_edges(c), vec![es[0], es[2]]);
        bm.disk_edge_remove(c, es[0]);
        bm.disk_edge_remove(c, es[2]);
        assert!(bm.vert_edges(c).is_empty());
    }

    #[test]
    fn radial_cycle_shared_edge() {
        let mut bm = BMesh::new();
        let v: Vec<_> = (0..4).map(|_| bm.vert_create(None, None)).collect();
        bm.face_create(&[v[0], v[1], v[2]], None).unwrap();
        bm.face_create(&[v[0], v[2], v[3]], None).unwrap();
        let shared = bm.edge_exists(v[0], v[2]).unwrap();
        assert_eq!(bm.edge_loops(shared).len(), 2);
        let other = bm.edge_exists(v[0], v[1]).unwrap();
        assert_eq!(bm.edge_loops(other).len(), 1);
        bm.validate_topology().unwrap();
    }
}
