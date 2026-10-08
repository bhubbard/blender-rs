//! Triangle bounding-volume hierarchy, porting the ray-cast and nearest-point queries of
//! `BLI_bvhtree`. Built top-down by median split on the longest centroid axis.

use crate::bounds::Aabb;
use crate::tri::{closest_on_tri_to_point_v3, isect_ray_tri_v3};
use glam::Vec3;

const LEAF_SIZE: usize = 4;

#[derive(Debug, Clone, Copy)]
struct Node {
    bounds: Aabb,
    /// Internal node: child node indices. Leaf: `left == right == u32::MAX`.
    left: u32,
    right: u32,
    /// Leaf: range `start..start + count` into `Bvh::order`.
    start: u32,
    count: u32,
}

impl Node {
    fn is_leaf(&self) -> bool {
        self.left == u32::MAX
    }
}

/// Nearest ray hit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BvhRayHit {
    pub index: usize,
    pub t: f32,
    pub u: f32,
    pub v: f32,
}

/// Nearest point on any triangle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BvhNearest {
    pub index: usize,
    pub point: Vec3,
    pub dist_sq: f32,
}

#[derive(Debug, Clone, Default)]
pub struct Bvh {
    tris: Vec<[Vec3; 3]>,
    order: Vec<u32>,
    nodes: Vec<Node>,
    /// Per-triangle centroids; only populated during construction.
    centroids: Vec<Vec3>,
}

impl Bvh {
    /// Builds a BVH over `tris`; a triangle's position in the slice is its reported index.
    pub fn new(tris: &[[Vec3; 3]]) -> Self {
        let mut bvh = Bvh {
            tris: tris.to_vec(),
            order: (0..tris.len() as u32).collect(),
            nodes: Vec::new(),
            centroids: tris.iter().map(|t| (t[0] + t[1] + t[2]) / 3.0).collect(),
        };
        if !tris.is_empty() {
            let mut order = std::mem::take(&mut bvh.order);
            bvh.build(&mut order, 0);
            bvh.order = order;
        }
        bvh.centroids = Vec::new();
        bvh
    }

    pub fn len(&self) -> usize {
        self.tris.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tris.is_empty()
    }

    /// Bounds of the whole hierarchy.
    pub fn bounds(&self) -> Aabb {
        self.nodes.first().map_or(Aabb::EMPTY, |n| n.bounds)
    }

    fn tri_bounds(&self, i: u32) -> Aabb {
        Aabb::from_points(&self.tris[i as usize])
    }

    fn centroid(&self, i: u32) -> Vec3 {
        self.centroids[i as usize]
    }

    /// Builds the node for `order` (which covers `offset..offset + order.len()` globally).
    fn build(&mut self, order: &mut [u32], offset: usize) -> u32 {
        let id = self.nodes.len() as u32;
        let mut bounds = Aabb::EMPTY;
        let mut cb = Aabb::EMPTY;
        for &i in order.iter() {
            bounds = bounds.union(self.tri_bounds(i));
            cb = cb.expanded(self.centroid(i));
        }
        self.nodes.push(Node {
            bounds,
            left: u32::MAX,
            right: u32::MAX,
            start: offset as u32,
            count: order.len() as u32,
        });
        let ext = cb.size();
        let axis = if ext.x >= ext.y && ext.x >= ext.z {
            0
        } else if ext.y >= ext.z {
            1
        } else {
            2
        };
        if order.len() <= LEAF_SIZE || ext[axis] <= 0.0 {
            return id;
        }
        let mid = order.len() / 2;
        let cents = &self.centroids;
        order.select_nth_unstable_by(mid, |&a, &b| {
            cents[a as usize][axis].total_cmp(&cents[b as usize][axis])
        });
        let (lo, hi) = order.split_at_mut(mid);
        let left = self.build(lo, offset);
        let right = self.build(hi, offset + mid);
        let n = &mut self.nodes[id as usize];
        n.left = left;
        n.right = right;
        n.count = 0;
        id
    }

    /// Nearest hit of the ray (`t >= 0`, `dir` need not be normalised; `t` is in units of `dir`).
    pub fn ray_cast(&self, orig: Vec3, dir: Vec3) -> Option<BvhRayHit> {
        if self.nodes.is_empty() {
            return None;
        }
        let mut best: Option<BvhRayHit> = None;
        let mut stack = vec![0u32];
        while let Some(ni) = stack.pop() {
            let node = &self.nodes[ni as usize];
            let limit = best.map_or(f32::INFINITY, |b| b.t);
            match node.bounds.intersect_ray(orig, dir) {
                Some((t0, _)) if t0 <= limit => {}
                _ => continue,
            }
            if node.is_leaf() {
                for k in node.start..node.start + node.count {
                    let idx = self.order[k as usize] as usize;
                    let t = &self.tris[idx];
                    if let Some(h) = isect_ray_tri_v3(orig, dir, t[0], t[1], t[2]) {
                        if best.is_none_or(|b| h.t < b.t) {
                            best = Some(BvhRayHit { index: idx, t: h.t, u: h.u, v: h.v });
                        }
                    }
                }
            } else {
                stack.push(node.left);
                stack.push(node.right);
            }
        }
        best
    }

    /// Nearest point on any triangle to `p`.
    pub fn find_nearest(&self, p: Vec3) -> Option<BvhNearest> {
        if self.nodes.is_empty() {
            return None;
        }
        let mut best: Option<BvhNearest> = None;
        self.nearest_rec(0, p, &mut best);
        best
    }

    fn nearest_rec(&self, ni: u32, p: Vec3, best: &mut Option<BvhNearest>) {
        let node = &self.nodes[ni as usize];
        let limit = best.map_or(f32::INFINITY, |b| b.dist_sq);
        if node.bounds.dist_sq_to_point(p) > limit {
            return;
        }
        if node.is_leaf() {
            for k in node.start..node.start + node.count {
                let idx = self.order[k as usize] as usize;
                let t = &self.tris[idx];
                let q = closest_on_tri_to_point_v3(p, t[0], t[1], t[2]);
                let d = (q - p).length_squared();
                if best.is_none_or(|b| d < b.dist_sq) {
                    *best = Some(BvhNearest { index: idx, point: q, dist_sq: d });
                }
            }
            return;
        }
        let (l, r) = (node.left, node.right);
        let dl = self.nodes[l as usize].bounds.dist_sq_to_point(p);
        let dr = self.nodes[r as usize].bounds.dist_sq_to_point(p);
        let (first, second) = if dl <= dr { (l, r) } else { (r, l) };
        self.nearest_rec(first, p, best);
        self.nearest_rec(second, p, best);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Lcg(u64);
    impl Lcg {
        fn f(&mut self) -> f32 {
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            ((self.0 >> 40) as f32 / (1u64 << 24) as f32) * 20.0 - 10.0
        }
        fn v(&mut self) -> Vec3 {
            Vec3::new(self.f(), self.f(), self.f())
        }
        fn tri(&mut self) -> [Vec3; 3] {
            let c = self.v();
            [c, c + self.v() * 0.15, c + self.v() * 0.15]
        }
    }

    #[test]
    fn empty_bvh() {
        let b = Bvh::new(&[]);
        assert!(b.is_empty());
        assert!(b.bounds().is_empty());
        assert!(b.ray_cast(Vec3::ZERO, Vec3::Z).is_none());
        assert!(b.find_nearest(Vec3::ZERO).is_none());
    }

    #[test]
    fn single_triangle() {
        let tri = [Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 4.0, 0.0)];
        let b = Bvh::new(&[tri]);
        let h = b.ray_cast(Vec3::new(1.0, 1.0, 3.0), -Vec3::Z).unwrap();
        assert_eq!(h.index, 0);
        assert!((h.t - 3.0).abs() < 1e-6);
        let n = b.find_nearest(Vec3::new(1.0, 1.0, 2.0)).unwrap();
        assert_eq!(n.point, Vec3::new(1.0, 1.0, 0.0));
        assert_eq!(n.dist_sq, 4.0);
    }

    #[test]
    fn nearest_hit_wins_among_stacked_triangles() {
        let mk = |z: f32| [Vec3::new(0.0, 0.0, z), Vec3::new(4.0, 0.0, z), Vec3::new(0.0, 4.0, z)];
        let tris: Vec<_> = (0..20).map(|i| mk(i as f32)).collect();
        let b = Bvh::new(&tris);
        let h = b.ray_cast(Vec3::new(1.0, 1.0, 100.0), -Vec3::Z).unwrap();
        assert_eq!(h.index, 19);
        let h = b.ray_cast(Vec3::new(1.0, 1.0, -5.0), Vec3::Z).unwrap();
        assert_eq!(h.index, 0);
    }

    #[test]
    fn matches_brute_force() {
        let mut rng = Lcg(7);
        for &n in &[1usize, 5, 64, 500] {
            let tris: Vec<[Vec3; 3]> = (0..n).map(|_| rng.tri()).collect();
            let bvh = Bvh::new(&tris);
            assert_eq!(bvh.len(), n);
            for _ in 0..100 {
                let (o, d) = (rng.v(), rng.v().normalize());
                let brute = tris
                    .iter()
                    .enumerate()
                    .filter_map(|(i, t)| isect_ray_tri_v3(o, d, t[0], t[1], t[2]).map(|h| (i, h.t)))
                    .min_by(|a, b| a.1.total_cmp(&b.1));
                let got = bvh.ray_cast(o, d);
                match (brute, got) {
                    (None, None) => {}
                    (Some((_, bt)), Some(h)) => assert!((bt - h.t).abs() < 1e-4, "n={n}"),
                    other => panic!("ray mismatch n={n}: {other:?}"),
                }

                let p = rng.v();
                let want = tris
                    .iter()
                    .map(|t| (closest_on_tri_to_point_v3(p, t[0], t[1], t[2]) - p).length_squared())
                    .fold(f32::INFINITY, f32::min);
                let near = bvh.find_nearest(p).unwrap();
                assert!((near.dist_sq - want).abs() < 1e-4, "n={n}");
            }
        }
    }

    #[test]
    fn coincident_triangles_do_not_recurse_forever() {
        let t = [Vec3::ZERO, Vec3::X, Vec3::Y];
        let b = Bvh::new(&vec![t; 50]);
        assert!(b.ray_cast(Vec3::new(0.2, 0.2, 1.0), -Vec3::Z).is_some());
    }
}

#[cfg(test)]
mod perf {
    use super::*;

    #[test]
    fn builds_large_soup_quickly() {
        let n = 200_000usize;
        let tris: Vec<[Vec3; 3]> = (0..n)
            .map(|i| {
                let c = Vec3::new((i % 100) as f32, ((i / 100) % 100) as f32, (i / 10_000) as f32);
                [c, c + Vec3::X * 0.5, c + Vec3::Y * 0.5]
            })
            .collect();
        let start = std::time::Instant::now();
        let b = Bvh::new(&tris);
        assert!(start.elapsed().as_secs() < 5, "build took {:?}", start.elapsed());
        assert!(b.ray_cast(Vec3::new(10.2, 10.2, 100.0), -Vec3::Z).is_some());
    }
}
