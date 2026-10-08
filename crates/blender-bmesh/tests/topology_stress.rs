//! Randomised (deterministic-seed) stress test: every operator must leave the
//! mesh with consistent loop, radial and disk cycles.

use blender_bmesh::{BMesh, EdgeHandle, FaceHandle};
use blender_math::Vec3;

struct Lcg(u64);
impl Lcg {
    fn next(&mut self, n: usize) -> usize {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 33) as usize) % n.max(1)
    }
}

fn grid(n: usize) -> BMesh {
    let mut bm = BMesh::new();
    let mut v = Vec::new();
    for y in 0..=n {
        for x in 0..=n {
            v.push(bm.vert_create(Some(Vec3::new(x as f32, y as f32, 0.0)), None));
        }
    }
    let w = n + 1;
    for y in 0..n {
        for x in 0..n {
            let i = y * w + x;
            bm.face_create(&[v[i], v[i + 1], v[i + w + 1], v[i + w]], None)
                .unwrap();
        }
    }
    bm
}

fn edges(bm: &BMesh) -> Vec<EdgeHandle> {
    bm.epool.iter().map(|(h, _)| h).collect()
}
fn faces(bm: &BMesh) -> Vec<FaceHandle> {
    bm.fpool.iter().map(|(h, _)| h).collect()
}

#[test]
fn random_operations_keep_topology_valid() {
    for seed in 0..20u64 {
        let mut rng = Lcg(seed * 7919 + 1);
        let mut bm = grid(4);
        bm.validate_topology().unwrap();
        for step in 0..60 {
            let op = rng.next(7);
            let es = edges(&bm);
            let fs = faces(&bm);
            if es.is_empty() {
                break;
            }
            let e = es[rng.next(es.len())];
            match op {
                0 => {
                    bm.edge_split(e, 0.5).unwrap();
                }
                1 => {
                    let _ = bm.faces_join_pair(e);
                }
                2 => {
                    let ed = bm.epool.get(e).unwrap();
                    let kill = if rng.next(2) == 0 { ed.v1 } else { ed.v2 };
                    bm.edge_collapse(e, kill).unwrap();
                }
                3 if !fs.is_empty() => {
                    let f = fs[rng.next(fs.len())];
                    let vs = bm.face_verts(f);
                    if vs.len() >= 4 {
                        let _ = bm.face_split(f, vs[0], vs[2]);
                    }
                }
                4 => {
                    let _ = bm.edge_rotate(e);
                }
                5 => {
                    let vs: Vec<_> = bm.vpool.iter().map(|(h, _)| h).collect();
                    if !vs.is_empty() {
                        let _ = bm.vert_dissolve_pair(vs[rng.next(vs.len())]);
                    }
                }
                _ => {
                    if rng.next(4) == 0 && !fs.is_empty() {
                        let _ = bm.face_kill(fs[rng.next(fs.len())]);
                    }
                }
            }
            if let Err(msg) = bm.validate_topology() {
                panic!("seed {seed} step {step} op {op}: {msg}");
            }
        }
    }
}
