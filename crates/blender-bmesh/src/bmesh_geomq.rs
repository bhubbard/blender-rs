//! Geometric and adjacency queries from `bmesh_query.cc` / `bmesh_polygon.cc`:
//! lengths, areas, normals, centres, share/exists checks, double finders, convexity and
//! face angles.
//!
//! Upstream reads cached face normals (`f->no`). This kernel does not keep those current
//! after topology edits, so normals are recomputed from vertex positions (Newell), and
//! "equal normals" uses a small tolerance instead of exact equality.

use crate::*;

const NORMAL_EQ: f32 = 1e-6;

impl BMesh {
    fn vert_co(&self, v: VertHandle) -> Vec3 {
        self.vpool.get(v).map_or(Vec3::ZERO, |x| x.co)
    }

    /// Vertex positions of a face in perimeter order.
    pub fn face_coords(&self, f: FaceHandle) -> Vec<Vec3> {
        self.face_verts(f).into_iter().map(|v| self.vert_co(v)).collect()
    }

    pub fn edge_calc_length_squared(&self, e: EdgeHandle) -> f32 {
        self.epool
            .get(e)
            .map_or(0.0, |ed| (self.vert_co(ed.v1) - self.vert_co(ed.v2)).length_squared())
    }

    pub fn edge_calc_length(&self, e: EdgeHandle) -> f32 {
        self.edge_calc_length_squared(e).sqrt()
    }

    /// Un-normalised Newell normal of a face (length = 2 × area).
    fn face_newell(&self, f: FaceHandle) -> Vec3 {
        let pts = self.face_coords(f);
        let Some(&last) = pts.last() else { return Vec3::ZERO };
        let mut prev = last;
        let mut n = Vec3::ZERO;
        for &cur in &pts {
            n += prev.cross(cur);
            prev = cur;
        }
        n
    }

    /// Unit face normal from positions (zero for a degenerate face) (`BM_face_calc_normal`).
    pub fn face_calc_normal(&self, f: FaceHandle) -> Vec3 {
        self.face_newell(f).normalize_or_zero()
    }

    /// Face area (`BM_face_calc_area`).
    pub fn face_calc_area(&self, f: FaceHandle) -> f32 {
        0.5 * self.face_newell(f).length()
    }

    /// Sum of edge lengths around the face (`BM_face_calc_perimeter`).
    pub fn face_calc_perimeter(&self, f: FaceHandle) -> f32 {
        self.face_edges(f).into_iter().map(|e| self.edge_calc_length(e)).sum()
    }

    /// Average of the vertex positions (`BM_face_calc_center_median`).
    pub fn face_calc_center_median(&self, f: FaceHandle) -> Vec3 {
        let pts = self.face_coords(f);
        if pts.is_empty() {
            return Vec3::ZERO;
        }
        pts.iter().copied().sum::<Vec3>() / pts.len() as f32
    }

    /// Centre of the bounding box (`BM_face_calc_center_bounds`).
    pub fn face_calc_center_bounds(&self, f: FaceHandle) -> Vec3 {
        let pts = self.face_coords(f);
        if pts.is_empty() {
            return Vec3::ZERO;
        }
        let (mut lo, mut hi) = (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY));
        for p in pts {
            lo = lo.min(p);
            hi = hi.max(p);
        }
        (lo + hi) * 0.5
    }

    /// The vertex shared by two edges, if any (`BM_edge_share_vert`).
    pub fn edge_share_vert(&self, a: EdgeHandle, b: EdgeHandle) -> Option<VertHandle> {
        let (ea, eb) = (self.epool.get(a)?, self.epool.get(b)?);
        [ea.v1, ea.v2].into_iter().find(|&v| v == eb.v1 || v == eb.v2)
    }

    pub fn edge_share_vert_check(&self, a: EdgeHandle, b: EdgeHandle) -> bool {
        self.edge_share_vert(a, b).is_some()
    }

    /// Number of edges two faces have in common (`BM_face_share_edge_count`).
    pub fn face_share_edge_count(&self, a: FaceHandle, b: FaceHandle) -> usize {
        let eb = self.face_edges(b);
        self.face_edges(a).into_iter().filter(|e| eb.contains(e)).count()
    }

    pub fn face_share_edge_check(&self, a: FaceHandle, b: FaceHandle) -> bool {
        self.face_share_edge_count(a, b) > 0
    }

    /// Number of vertices two faces have in common (`BM_face_share_vert_count`).
    pub fn face_share_vert_count(&self, a: FaceHandle, b: FaceHandle) -> usize {
        let vb = self.face_verts(b);
        self.face_verts(a).into_iter().filter(|v| vb.contains(v)).count()
    }

    pub fn face_share_vert_check(&self, a: FaceHandle, b: FaceHandle) -> bool {
        self.face_share_vert_count(a, b) > 0
    }

    /// Finds an existing face with exactly these vertices in order, in either winding,
    /// starting from `verts[0]` (`BM_face_exists`).
    pub fn face_exists(&self, verts: &[VertHandle]) -> Option<FaceHandle> {
        let len = verts.len();
        if len < 3 {
            return None;
        }
        for l in self.vert_loops(verts[0]) {
            let lp = self.lpool.get(l)?;
            if self.fpool.get(lp.f).map(|f| f.len as usize) != Some(len) {
                continue;
            }
            let walk = |forward: bool| -> bool {
                let mut cur = l;
                (0..len).all(|i| {
                    let Some(c) = self.lpool.get(cur) else { return false };
                    let ok = c.v == verts[i];
                    cur = if forward { c.next } else { c.prev };
                    ok
                })
            };
            if walk(true) || walk(false) {
                return Some(lp.f);
            }
        }
        None
    }

    /// Another face using exactly the same vertex set (`BM_face_find_double`).
    pub fn face_find_double(&self, f: FaceHandle) -> Option<FaceHandle> {
        let verts = self.face_verts(f);
        let e = *self.face_edges(f).first()?;
        self.edge_faces(e).into_iter().find(|&g| {
            g != f && {
                let gv = self.face_verts(g);
                gv.len() == verts.len() && verts.iter().all(|v| gv.contains(v))
            }
        })
    }

    /// Another edge joining the same two vertices (`BM_edge_find_double`).
    pub fn edge_find_double(&self, e: EdgeHandle) -> Option<EdgeHandle> {
        let ed = self.epool.get(e)?;
        self.vert_edges(ed.v1)
            .into_iter()
            .find(|&o| o != e && self.edge_other_vert(o, ed.v1) == Some(ed.v2))
    }

    /// Loop whose edge is longest / shortest in the face (`BM_face_find_longest_loop`).
    pub fn face_find_longest_loop(&self, f: FaceHandle) -> Option<LoopHandle> {
        self.face_loops(f).into_iter().max_by(|&a, &b| {
            self.loop_edge_len_sq(a).total_cmp(&self.loop_edge_len_sq(b))
        })
    }

    pub fn face_find_shortest_loop(&self, f: FaceHandle) -> Option<LoopHandle> {
        self.face_loops(f).into_iter().min_by(|&a, &b| {
            self.loop_edge_len_sq(a).total_cmp(&self.loop_edge_len_sq(b))
        })
    }

    fn loop_edge_len_sq(&self, l: LoopHandle) -> f32 {
        self.lpool.get(l).map_or(0.0, |lp| self.edge_calc_length_squared(lp.e))
    }

    /// Angle between the normals of the two faces on `e`, or `fallback` if not manifold
    /// (`BM_edge_calc_face_angle_ex`).
    pub fn edge_calc_face_angle_ex(&self, e: EdgeHandle, fallback: f32) -> f32 {
        match self.edge_manifold_normals(e) {
            Some((n1, n2, _)) => n1.dot(n2).clamp(-1.0, 1.0).acos(),
            None => fallback,
        }
    }

    /// As [`Self::edge_calc_face_angle_ex`] with the upstream 90° fallback.
    pub fn edge_calc_face_angle(&self, e: EdgeHandle) -> f32 {
        self.edge_calc_face_angle_ex(e, 90f32.to_radians())
    }

    /// Face angle, negative for concave edges (`BM_edge_calc_face_angle_signed_ex`).
    pub fn edge_calc_face_angle_signed_ex(&self, e: EdgeHandle, fallback: f32) -> f32 {
        match self.edge_manifold_normals(e) {
            Some((n1, n2, _)) => {
                let angle = n1.dot(n2).clamp(-1.0, 1.0).acos();
                if self.edge_is_convex(e) { angle } else { -angle }
            }
            None => fallback,
        }
    }

    pub fn edge_calc_face_angle_signed(&self, e: EdgeHandle) -> f32 {
        self.edge_calc_face_angle_signed_ex(e, 90f32.to_radians())
    }

    /// `true` unless `e` is a manifold edge folding inward (`BM_edge_is_convex`).
    pub fn edge_is_convex(&self, e: EdgeHandle) -> bool {
        let Some((n1, n2, l1)) = self.edge_manifold_normals(e) else { return true };
        if (n1 - n2).length() <= NORMAL_EQ {
            return true;
        }
        let Some(lp) = self.lpool.get(l1) else { return true };
        let next_v = self.lpool.get(lp.next).map_or(Vec3::ZERO, |n| self.vert_co(n.v));
        let dir = next_v - self.vert_co(lp.v);
        dir.dot(n1.cross(n2)) > 0.0
    }

    /// Normals of the two faces on a manifold edge plus the first radial loop.
    fn edge_manifold_normals(&self, e: EdgeHandle) -> Option<(Vec3, Vec3, LoopHandle)> {
        let loops = self.edge_loops(e);
        if loops.len() != 2 {
            return None;
        }
        let f1 = self.lpool.get(loops[0])?.f;
        let f2 = self.lpool.get(loops[1])?.f;
        Some((self.face_calc_normal(f1), self.face_calc_normal(f2), loops[0]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::FRAC_PI_2;

    fn quad(bm: &mut BMesh, pts: [[f32; 3]; 4]) -> (FaceHandle, [VertHandle; 4]) {
        let v = pts.map(|p| bm.vert_create(Some(Vec3::from(p)), None));
        (bm.face_create(&v, None).unwrap(), v)
    }

    /// Unit cube with outward-facing quads.
    fn cube() -> (BMesh, Vec<FaceHandle>) {
        let mut bm = BMesh::new();
        let p = [
            [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0],
        ];
        let v: Vec<_> = p.iter().map(|&c| bm.vert_create(Some(Vec3::from(c)), None)).collect();
        let faces = [[0, 3, 2, 1], [4, 5, 6, 7], [0, 1, 5, 4], [1, 2, 6, 5], [2, 3, 7, 6], [3, 0, 4, 7]]
            .iter()
            .map(|f| bm.face_create(&f.map(|i| v[i]), None).unwrap())
            .collect();
        (bm, faces)
    }

    #[test]
    fn lengths_areas_and_centres() {
        let (bm, faces) = cube();
        for &f in &faces {
            assert!((bm.face_calc_area(f) - 1.0).abs() < 1e-6);
            assert!((bm.face_calc_perimeter(f) - 4.0).abs() < 1e-6);
            assert!((bm.face_calc_normal(f).length() - 1.0).abs() < 1e-6);
        }
        // Bottom face points down, top face up; normals point away from the centre.
        assert_eq!(bm.face_calc_normal(faces[0]), -Vec3::Z);
        assert_eq!(bm.face_calc_normal(faces[1]), Vec3::Z);
        for &f in &faces {
            let out = bm.face_calc_center_median(f) - Vec3::splat(0.5);
            assert!(bm.face_calc_normal(f).dot(out) > 0.0);
        }
        assert_eq!(bm.face_calc_center_median(faces[1]), Vec3::new(0.5, 0.5, 1.0));
        assert_eq!(bm.face_calc_center_bounds(faces[2]), Vec3::new(0.5, 0.0, 0.5));
        let e = bm.epool.iter().next().unwrap().0;
        assert_eq!(bm.edge_calc_length(e), 1.0);
        assert_eq!(bm.edge_calc_length_squared(e), 1.0);
    }

    #[test]
    fn cube_edges_are_convex_right_angles() {
        let (bm, _) = cube();
        for (e, _) in bm.epool.iter() {
            assert!(bm.edge_is_convex(e));
            assert!((bm.edge_calc_face_angle(e) - FRAC_PI_2).abs() < 1e-5);
            assert!((bm.edge_calc_face_angle_signed(e) - FRAC_PI_2).abs() < 1e-5);
        }
    }

    /// Two quads folded along the Y axis; wings go up (`h = 1`, a valley) or down (ridge).
    fn fold(h: f32) -> (BMesh, EdgeHandle) {
        let mut bm = BMesh::new();
        let v0 = bm.vert_create(Some(Vec3::new(0.0, 0.0, 0.0)), None);
        let v1 = bm.vert_create(Some(Vec3::new(0.0, 1.0, 0.0)), None);
        let a1 = bm.vert_create(Some(Vec3::new(-1.0, 1.0, h)), None);
        let a0 = bm.vert_create(Some(Vec3::new(-1.0, 0.0, h)), None);
        let b0 = bm.vert_create(Some(Vec3::new(1.0, 0.0, h)), None);
        let b1 = bm.vert_create(Some(Vec3::new(1.0, 1.0, h)), None);
        bm.face_create(&[v0, v1, a1, a0], None).unwrap();
        bm.face_create(&[v1, v0, b0, b1], None).unwrap();
        let e = bm.edge_exists(v0, v1).unwrap();
        (bm, e)
    }

    #[test]
    fn ridge_is_convex_and_valley_is_concave() {
        let (ridge, er) = fold(-1.0);
        assert!(ridge.edge_is_convex(er));
        assert!((ridge.edge_calc_face_angle_signed(er) - FRAC_PI_2).abs() < 1e-5);
        let (valley, ev) = fold(1.0);
        assert!(!valley.edge_is_convex(ev));
        assert!((valley.edge_calc_face_angle_signed(ev) + FRAC_PI_2).abs() < 1e-5);
        assert!((valley.edge_calc_face_angle(ev) - FRAC_PI_2).abs() < 1e-5);
    }

    #[test]
    fn flat_and_boundary_edges() {
        let (bm, e) = fold(0.0);
        assert!(bm.edge_is_convex(e)); // coplanar
        assert!(bm.edge_calc_face_angle(e).abs() < 1e-3);
        let mut bm = BMesh::new();
        let (f, _) = quad(&mut bm, [[0.0; 3], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]]);
        let boundary = bm.face_edges(f)[0];
        assert!(bm.edge_is_convex(boundary));
        assert_eq!(bm.edge_calc_face_angle_ex(boundary, 1.25), 1.25);
        assert_eq!(bm.edge_calc_face_angle_signed_ex(boundary, -2.0), -2.0);
        assert!((bm.edge_calc_face_angle(boundary) - FRAC_PI_2).abs() < 1e-6);
    }

    #[test]
    fn share_queries() {
        let (bm, faces) = cube();
        // Adjacent faces share one edge and two vertices; opposite faces share nothing.
        assert_eq!(bm.face_share_edge_count(faces[0], faces[2]), 1);
        assert_eq!(bm.face_share_vert_count(faces[0], faces[2]), 2);
        assert!(bm.face_share_edge_check(faces[0], faces[2]));
        assert!(!bm.face_share_edge_check(faces[0], faces[1]));
        assert!(!bm.face_share_vert_check(faces[0], faces[1]));
        let e0 = bm.face_edges(faces[0]);
        assert_eq!(bm.edge_share_vert(e0[0], e0[1]), Some(bm.face_verts(faces[0])[1]));
        assert!(bm.edge_share_vert_check(e0[0], e0[1]));
        assert!(!bm.edge_share_vert_check(e0[0], e0[2]));
    }

    #[test]
    fn face_exists_matches_either_winding_and_rotation() {
        let (bm, faces) = cube();
        let v = bm.face_verts(faces[0]);
        assert_eq!(bm.face_exists(&v), Some(faces[0]));
        let mut rev = v.clone();
        rev.reverse();
        assert_eq!(bm.face_exists(&rev), Some(faces[0]));
        // Starting from a different vertex of the same face (rotation) still matches.
        let rot = [v[1], v[2], v[3], v[0]];
        assert_eq!(bm.face_exists(&rot), Some(faces[0]));
        // Wrong order (a bow-tie) or wrong length does not.
        assert_eq!(bm.face_exists(&[v[0], v[2], v[1], v[3]]), None);
        assert_eq!(bm.face_exists(&v[..3]), None);
        assert_eq!(bm.face_exists(&[]), None);
    }

    #[test]
    fn doubles_are_found() {
        let (mut bm, faces) = cube();
        assert_eq!(bm.face_find_double(faces[0]), None);
        let v = bm.face_verts(faces[0]);
        let a = bm.vert_create(None, None);
        let b = bm.vert_create(None, None);
        let e1 = bm.edge_create(a, b, None).unwrap();
        assert_eq!(bm.edge_find_double(e1), None);
        let e2 = bm.edge_create(a, b, None).unwrap();
        assert_eq!(bm.edge_find_double(e1), Some(e2));
        // A duplicate face on the same edges is found from either side.
        let dup = bm.face_create(&[v[3], v[2], v[1], v[0]], None).unwrap();
        assert_eq!(bm.face_find_double(faces[0]), Some(dup));
        assert_eq!(bm.face_find_double(dup), Some(faces[0]));
    }

    #[test]
    fn longest_and_shortest_loops() {
        let mut bm = BMesh::new();
        let (f, v) = quad(&mut bm, [[0.0; 3], [4.0, 0.0, 0.0], [4.0, 1.0, 0.0], [0.0, 1.0, 0.0]]);
        let longest = bm.face_find_longest_loop(f).unwrap();
        let shortest = bm.face_find_shortest_loop(f).unwrap();
        assert_eq!(bm.edge_calc_length(bm.lpool.get(longest).unwrap().e), 4.0);
        assert_eq!(bm.edge_calc_length(bm.lpool.get(shortest).unwrap().e), 1.0);
        let _ = v;
        assert_eq!(bm.face_calc_area(f), 4.0);
        assert_eq!(bm.face_calc_perimeter(f), 10.0);
    }

    #[test]
    fn degenerate_and_dead_handles() {
        let mut bm = BMesh::new();
        let v: Vec<_> = (0..3).map(|i| bm.vert_create(Some(Vec3::X * i as f32), None)).collect();
        let f = bm.face_create(&v, None).unwrap();
        assert_eq!(bm.face_calc_normal(f), Vec3::ZERO);
        assert_eq!(bm.face_calc_area(f), 0.0);
        bm.face_kill(f).unwrap();
        assert_eq!(bm.face_calc_area(f), 0.0);
        assert_eq!(bm.face_calc_center_median(f), Vec3::ZERO);
        assert!(bm.face_find_longest_loop(f).is_none());
    }
}
