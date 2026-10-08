//! Topology queries, porting the iterator and query helpers from
//! `bmesh_iterators.cc` and `bmesh_query.cc` (`BM_iter_as_array` on faces/verts/edges,
//! `BM_edge_other_vert`, `BM_edge_is_boundary`, `BM_vert_in_face`, ...).

use crate::*;

impl BMesh {
    /// The vertex of `e` that is not `v` (`BM_edge_other_vert`).
    pub fn edge_other_vert(&self, e: EdgeHandle, v: VertHandle) -> Option<VertHandle> {
        let ed = self.epool.get(e)?;
        if ed.v1 == v {
            Some(ed.v2)
        } else if ed.v2 == v {
            Some(ed.v1)
        } else {
            None
        }
    }

    /// Vertices of `f` in perimeter order (`BM_ITER_ELEM(BM_VERTS_OF_FACE)`).
    pub fn face_verts(&self, f: FaceHandle) -> Vec<VertHandle> {
        self.face_loops(f)
            .into_iter()
            .filter_map(|l| self.lpool.get(l).map(|lp| lp.v))
            .collect()
    }

    /// Edges of `f` in perimeter order (`BM_ITER_ELEM(BM_EDGES_OF_FACE)`).
    pub fn face_edges(&self, f: FaceHandle) -> Vec<EdgeHandle> {
        self.face_loops(f)
            .into_iter()
            .filter_map(|l| self.lpool.get(l).map(|lp| lp.e))
            .collect()
    }

    /// Faces using `e`, in radial order (`BM_ITER_ELEM(BM_FACES_OF_EDGE)`).
    pub fn edge_faces(&self, e: EdgeHandle) -> Vec<FaceHandle> {
        self.edge_loops(e)
            .into_iter()
            .filter_map(|l| self.lpool.get(l).map(|lp| lp.f))
            .collect()
    }

    /// Distinct faces around `v` (`BM_ITER_ELEM(BM_FACES_OF_VERT)`).
    pub fn vert_faces(&self, v: VertHandle) -> Vec<FaceHandle> {
        let mut out: Vec<FaceHandle> = Vec::new();
        for e in self.vert_edges(v) {
            for l in self.edge_loops(e) {
                if let Some(lp) = self.lpool.get(l) {
                    if lp.v == v && !out.contains(&lp.f) {
                        out.push(lp.f);
                    }
                }
            }
        }
        out
    }

    /// Loops whose start vertex is `v` (`BM_ITER_ELEM(BM_LOOPS_OF_VERT)`).
    pub fn vert_loops(&self, v: VertHandle) -> Vec<LoopHandle> {
        let mut out = Vec::new();
        for e in self.vert_edges(v) {
            for l in self.edge_loops(e) {
                if self.lpool.get(l).is_some_and(|lp| lp.v == v) {
                    out.push(l);
                }
            }
        }
        out
    }

    /// `true` when `v` is used by no face (`BM_vert_is_wire` is edges-without-faces;
    /// this mirrors `BM_vert_is_edge_pair`'s complement for the common queries).
    pub fn vert_is_wire(&self, v: VertHandle) -> bool {
        self.vert_edges(v)
            .into_iter()
            .all(|e| self.epool.get(e).is_some_and(|ed| ed.loop_.is_none()))
    }

    /// `true` when `e` has no faces (`BM_edge_is_wire`).
    pub fn edge_is_wire(&self, e: EdgeHandle) -> bool {
        self.epool.get(e).is_some_and(|ed| ed.loop_.is_none())
    }

    /// `true` when `e` has exactly one face (`BM_edge_is_boundary`).
    pub fn edge_is_boundary(&self, e: EdgeHandle) -> bool {
        self.edge_loops(e).len() == 1
    }

    /// `true` when `e` has exactly two faces (`BM_edge_is_manifold`).
    pub fn edge_is_manifold(&self, e: EdgeHandle) -> bool {
        self.edge_loops(e).len() == 2
    }

    /// `true` when `v` is a corner of `f` (`BM_vert_in_face`).
    pub fn vert_in_face(&self, v: VertHandle, f: FaceHandle) -> bool {
        self.face_verts(f).contains(&v)
    }

    /// First face shared by the two edges, if any (`BM_edge_share_face_check`).
    pub fn edges_share_face(&self, a: EdgeHandle, b: EdgeHandle) -> Option<FaceHandle> {
        let fb = self.edge_faces(b);
        self.edge_faces(a).into_iter().find(|f| fb.contains(f))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn two_tris() -> (BMesh, Vec<VertHandle>, [FaceHandle; 2]) {
        let mut bm = BMesh::new();
        let v: Vec<_> = (0..4).map(|_| bm.vert_create(None, None)).collect();
        let f1 = bm.face_create(&[v[0], v[1], v[2]], None).unwrap();
        let f2 = bm.face_create(&[v[0], v[2], v[3]], None).unwrap();
        (bm, v, [f1, f2])
    }

    #[test]
    fn face_queries() {
        let (bm, v, f) = two_tris();
        assert_eq!(bm.face_verts(f[0]), vec![v[0], v[1], v[2]]);
        assert_eq!(bm.face_edges(f[0]).len(), 3);
        assert!(bm.vert_in_face(v[1], f[0]));
        assert!(!bm.vert_in_face(v[1], f[1]));
    }

    #[test]
    fn vertex_and_edge_queries() {
        let (bm, v, f) = two_tris();
        let shared = bm.edge_exists(v[0], v[2]).unwrap();
        let side = bm.edge_exists(v[0], v[1]).unwrap();
        assert!(bm.edge_is_manifold(shared));
        assert!(bm.edge_is_boundary(side));
        assert_eq!(bm.edge_other_vert(shared, v[0]), Some(v[2]));
        assert_eq!(bm.edge_other_vert(shared, v[1]), None);
        assert_eq!(bm.edges_share_face(shared, side), Some(f[0]));
        let mut vf = bm.vert_faces(v[0]);
        vf.sort_by_key(|h| h.index);
        assert_eq!(vf.len(), 2);
        assert_eq!(bm.vert_faces(v[1]), vec![f[0]]);
        assert_eq!(bm.vert_loops(v[0]).len(), 2);
        assert!(!bm.vert_is_wire(v[0]));
    }

    #[test]
    fn wire_queries() {
        let mut bm = BMesh::new();
        let a = bm.vert_create(None, None);
        let b = bm.vert_create(None, None);
        let e = bm.edge_create(a, b, None).unwrap();
        assert!(bm.edge_is_wire(e));
        assert!(bm.vert_is_wire(a));
    }
}
