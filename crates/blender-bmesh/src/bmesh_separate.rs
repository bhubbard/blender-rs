//! `BM_vert_separate`: split a vertex shared by face fans that are not connected through
//! an edge into one vertex per fan.

use crate::*;

impl BMesh {
    /// Separates `v` so every group of faces connected around it through shared edges gets
    /// its own vertex. Returns the vertices holding each group; the first is `v` itself.
    /// Wire edges stay on `v`. A vertex whose faces already form one fan (or that has no
    /// faces) is left untouched and returned alone.
    pub fn vert_separate(&mut self, v: VertHandle) -> Result<Vec<VertHandle>, BMeshError> {
        let Some(vert) = self.vpool.get(v) else {
            return Err(BMeshError::InvalidVert(v));
        };
        let co = vert.co;
        let edges = self.vert_edges(v);

        // Union-find over the faces around `v`: faces on the same edge are one group.
        let mut faces: Vec<FaceHandle> = Vec::new();
        let edge_faces: Vec<Vec<usize>> = edges
            .iter()
            .map(|&e| {
                self.edge_faces(e)
                    .into_iter()
                    .map(|f| match faces.iter().position(|&x| x == f) {
                        Some(i) => i,
                        None => {
                            faces.push(f);
                            faces.len() - 1
                        }
                    })
                    .collect()
            })
            .collect();
        let mut parent: Vec<usize> = (0..faces.len()).collect();
        fn find(parent: &mut [usize], mut i: usize) -> usize {
            while parent[i] != i {
                parent[i] = parent[parent[i]];
                i = parent[i];
            }
            i
        }
        for fs in &edge_faces {
            for w in fs.windows(2) {
                let (a, b) = (find(&mut parent, w[0]), find(&mut parent, w[1]));
                if a != b {
                    parent[b] = a;
                }
            }
        }
        // Number the groups by first appearance so the original vertex keeps the first one.
        let mut roots: Vec<usize> = Vec::new();
        let mut groups: Vec<Vec<FaceHandle>> = Vec::new();
        for (i, &f) in faces.iter().enumerate() {
            let r = find(&mut parent, i);
            let g = match roots.iter().position(|&x| x == r) {
                Some(g) => g,
                None => {
                    roots.push(r);
                    groups.push(Vec::new());
                    roots.len() - 1
                }
            };
            groups[g].push(f);
        }
        let group_of_edge: Vec<Option<usize>> = edge_faces
            .iter()
            .map(|fs| {
                fs.first().map(|&i| {
                    let r = find(&mut parent, i);
                    roots.iter().position(|&x| x == r).unwrap()
                })
            })
            .collect();

        if groups.len() <= 1 {
            return Ok(vec![v]);
        }

        let mut out = vec![v];
        for g in 1..groups.len() {
            let nv = self.vert_create(Some(co), Some(v));
            out.push(nv);
            for (i, &e) in edges.iter().enumerate() {
                if group_of_edge[i] != Some(g) {
                    continue;
                }
                self.disk_edge_remove(v, e);
                if let Some(ed) = self.epool.get_mut(e) {
                    if ed.v1 == v {
                        ed.v1 = nv;
                    } else if ed.v2 == v {
                        ed.v2 = nv;
                    }
                }
                self.disk_edge_append(nv, e);
            }
            for &f in &groups[g] {
                for l in self.face_loops(f) {
                    if let Some(lp) = self.lpool.get_mut(l) {
                        if lp.v == v {
                            lp.v = nv;
                        }
                    }
                }
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bowtie() -> (BMesh, VertHandle, [FaceHandle; 2]) {
        let mut bm = BMesh::new();
        let c = bm.vert_create(Some(Vec3::ZERO), None);
        let p: Vec<_> = [(1.0, 1.0), (1.0, -1.0), (-1.0, -1.0), (-1.0, 1.0)]
            .iter()
            .map(|&(x, y)| bm.vert_create(Some(Vec3::new(x, y, 0.0)), None))
            .collect();
        let f1 = bm.face_create(&[c, p[0], p[1]], None).unwrap();
        let f2 = bm.face_create(&[c, p[2], p[3]], None).unwrap();
        (bm, c, [f1, f2])
    }

    #[test]
    fn bowtie_splits_in_two() {
        let (mut bm, c, [f1, f2]) = bowtie();
        let out = bm.vert_separate(c).unwrap();
        assert_eq!(out.len(), 2);
        assert_eq!(out[0], c);
        assert_eq!(bm.totvert(), 6);
        assert_eq!(bm.totedge(), 6);
        assert_eq!(bm.totface(), 2);
        for &v in &out {
            assert_eq!(bm.vpool.get(v).unwrap().co, Vec3::ZERO);
            assert_eq!(bm.vert_edges(v).len(), 2);
            assert_eq!(bm.vert_faces(v).len(), 1);
        }
        assert!(bm.vert_in_face(out[0], f1) && !bm.vert_in_face(out[0], f2));
        assert!(bm.vert_in_face(out[1], f2) && !bm.vert_in_face(out[1], f1));
        bm.validate_topology().unwrap();
    }

    #[test]
    fn connected_fan_is_untouched() {
        let mut bm = BMesh::new();
        let c = bm.vert_create(None, None);
        let r: Vec<_> = (0..4).map(|_| bm.vert_create(None, None)).collect();
        for i in 0..3 {
            bm.face_create(&[c, r[i], r[i + 1]], None).unwrap();
        }
        assert_eq!(bm.vert_separate(c).unwrap(), vec![c]);
        assert_eq!(bm.totvert(), 5);
        bm.validate_topology().unwrap();
    }

    #[test]
    fn fan_plus_isolated_face_and_wire_edge() {
        let mut bm = BMesh::new();
        let c = bm.vert_create(None, None);
        let r: Vec<_> = (0..3).map(|_| bm.vert_create(None, None)).collect();
        bm.face_create(&[c, r[0], r[1]], None).unwrap();
        bm.face_create(&[c, r[1], r[2]], None).unwrap(); // shares edge c-r1: one fan
        let q: Vec<_> = (0..2).map(|_| bm.vert_create(None, None)).collect();
        bm.face_create(&[c, q[0], q[1]], None).unwrap(); // separate fan
        let w = bm.vert_create(None, None);
        let wire = bm.edge_create(c, w, None).unwrap();
        let out = bm.vert_separate(c).unwrap();
        assert_eq!(out.len(), 2);
        // The wire edge and the first fan (2 faces) stay on `c`.
        assert!(bm.vert_edges(c).contains(&wire));
        assert_eq!(bm.vert_faces(c).len(), 2);
        assert_eq!(bm.vert_faces(out[1]).len(), 1);
        bm.validate_topology().unwrap();
    }

    #[test]
    fn three_separate_fans_and_data_copy() {
        let mut bm = BMesh::new();
        let c = bm.vert_create(Some(Vec3::ONE), None);
        bm.elem_float_data_set(c, CustomDataType::PropFloat, 3.5);
        for _ in 0..3 {
            let a = bm.vert_create(None, None);
            let b = bm.vert_create(None, None);
            bm.face_create(&[c, a, b], None).unwrap();
        }
        let out = bm.vert_separate(c).unwrap();
        assert_eq!(out.len(), 3);
        for &v in &out {
            assert_eq!(bm.elem_float_data_get(v, CustomDataType::PropFloat), Some(3.5));
            assert_eq!(bm.vert_faces(v).len(), 1);
        }
        bm.validate_topology().unwrap();
    }

    #[test]
    fn creation_order_does_not_change_grouping() {
        // Two fans around `c` (5 triangles, one 3-triangle open fan), built in many orders so
        // the disk cycle visits edges of different fans interleaved.
        let tris_a: Vec<[usize; 2]> = vec![[0, 1], [1, 2], [2, 3], [3, 4], [4, 0]]; // closed ring
        let tris_b: Vec<[usize; 2]> = vec![[5, 6], [6, 7], [7, 8]]; // open fan
        let mut seed = 12345u64;
        for _ in 0..40 {
            let mut all: Vec<[usize; 2]> = tris_a.iter().chain(tris_b.iter()).copied().collect();
            for i in (1..all.len()).rev() {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                all.swap(i, (seed >> 33) as usize % (i + 1));
            }
            let mut bm = BMesh::new();
            let c = bm.vert_create(None, None);
            let r: Vec<_> = (0..9).map(|_| bm.vert_create(None, None)).collect();
            for [x, y] in all {
                bm.face_create(&[c, r[x], r[y]], None).unwrap();
            }
            let out = bm.vert_separate(c).unwrap();
            assert_eq!(out.len(), 2);
            let mut counts: Vec<usize> = out.iter().map(|&v| bm.vert_faces(v).len()).collect();
            counts.sort();
            assert_eq!(counts, vec![3, 5]);
            bm.validate_topology().unwrap();
        }
    }

    #[test]
    fn loose_and_invalid_vertices() {
        let mut bm = BMesh::new();
        let v = bm.vert_create(None, None);
        assert_eq!(bm.vert_separate(v).unwrap(), vec![v]);
        bm.vert_kill(v).unwrap();
        assert_eq!(bm.vert_separate(v), Err(BMeshError::InvalidVert(v)));
    }
}
