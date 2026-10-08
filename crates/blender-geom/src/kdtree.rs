//! Static 3D k-d tree, porting `BLI_kdtree_3d` (`find_nearest`, `find_nearest_n`,
//! `range_search`). Built once from points, then queried; it is stored implicitly in a
//! single array (the median of each subrange is the subtree root).

use glam::Vec3;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy)]
struct Node {
    co: Vec3,
    index: usize,
    axis: u8,
}

/// A query result: the caller-supplied index and the squared distance to the query point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KdHit {
    pub index: usize,
    pub dist_sq: f32,
}

#[derive(Debug, Clone, Default)]
pub struct KdTree {
    nodes: Vec<Node>,
}

impl KdTree {
    /// Builds a balanced tree; `points[i]` is reported as index `i`.
    pub fn new(points: &[Vec3]) -> Self {
        let mut nodes: Vec<Node> = points
            .iter()
            .enumerate()
            .map(|(index, &co)| Node { co, index, axis: 0 })
            .collect();
        Self::build(&mut nodes);
        Self { nodes }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    fn build(nodes: &mut [Node]) {
        if nodes.len() <= 1 {
            return;
        }
        let (mut lo, mut hi) = (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY));
        for n in nodes.iter() {
            lo = lo.min(n.co);
            hi = hi.max(n.co);
        }
        let ext = hi - lo;
        let axis = if ext.x >= ext.y && ext.x >= ext.z {
            0
        } else if ext.y >= ext.z {
            1
        } else {
            2
        };
        let mid = nodes.len() / 2;
        nodes.select_nth_unstable_by(mid, |a, b| a.co[axis].total_cmp(&b.co[axis]));
        nodes[mid].axis = axis as u8;
        let (left, rest) = nodes.split_at_mut(mid);
        Self::build(left);
        Self::build(&mut rest[1..]);
    }

    /// Nearest point to `p`.
    pub fn find_nearest(&self, p: Vec3) -> Option<KdHit> {
        let mut best: Option<KdHit> = None;
        Self::nearest(&self.nodes, p, &mut best);
        best
    }

    fn nearest(nodes: &[Node], p: Vec3, best: &mut Option<KdHit>) {
        if nodes.is_empty() {
            return;
        }
        let mid = nodes.len() / 2;
        let n = &nodes[mid];
        let d = (n.co - p).length_squared();
        if best.is_none_or(|b| d < b.dist_sq) {
            *best = Some(KdHit { index: n.index, dist_sq: d });
        }
        if nodes.len() == 1 {
            return;
        }
        let diff = p[n.axis as usize] - n.co[n.axis as usize];
        let (near, far) = if diff < 0.0 {
            (&nodes[..mid], &nodes[mid + 1..])
        } else {
            (&nodes[mid + 1..], &nodes[..mid])
        };
        Self::nearest(near, p, best);
        if best.is_none_or(|b| diff * diff < b.dist_sq) {
            Self::nearest(far, p, best);
        }
    }

    /// Up to `k` nearest points to `p`, closest first.
    pub fn find_nearest_n(&self, p: Vec3, k: usize) -> Vec<KdHit> {
        if k == 0 {
            return Vec::new();
        }
        // Max-heap keyed by distance (non-negative f32 bit patterns order like the floats).
        let mut heap: BinaryHeap<(u32, usize)> = BinaryHeap::new();
        Self::nearest_n(&self.nodes, p, k, &mut heap);
        let mut out: Vec<KdHit> = heap
            .into_iter()
            .map(|(bits, index)| KdHit { index, dist_sq: f32::from_bits(bits) })
            .collect();
        out.sort_by(|a, b| a.dist_sq.total_cmp(&b.dist_sq).then(a.index.cmp(&b.index)));
        out
    }

    fn nearest_n(nodes: &[Node], p: Vec3, k: usize, heap: &mut BinaryHeap<(u32, usize)>) {
        if nodes.is_empty() {
            return;
        }
        let mid = nodes.len() / 2;
        let n = &nodes[mid];
        let d = (n.co - p).length_squared();
        if heap.len() < k {
            heap.push((d.to_bits(), n.index));
        } else if heap.peek().is_some_and(|&(w, _)| d.to_bits() < w) {
            heap.pop();
            heap.push((d.to_bits(), n.index));
        }
        if nodes.len() == 1 {
            return;
        }
        let diff = p[n.axis as usize] - n.co[n.axis as usize];
        let (near, far) = if diff < 0.0 {
            (&nodes[..mid], &nodes[mid + 1..])
        } else {
            (&nodes[mid + 1..], &nodes[..mid])
        };
        Self::nearest_n(near, p, k, heap);
        let worst = heap.peek().map_or(f32::INFINITY, |&(w, _)| f32::from_bits(w));
        if heap.len() < k || diff * diff < worst {
            Self::nearest_n(far, p, k, heap);
        }
    }

    /// All points within `radius` of `p` (inclusive), closest first.
    pub fn range_search(&self, p: Vec3, radius: f32) -> Vec<KdHit> {
        let mut out = Vec::new();
        Self::range(&self.nodes, p, radius * radius, &mut out);
        out.sort_by(|a, b| a.dist_sq.total_cmp(&b.dist_sq).then(a.index.cmp(&b.index)));
        out
    }

    fn range(nodes: &[Node], p: Vec3, r2: f32, out: &mut Vec<KdHit>) {
        if nodes.is_empty() {
            return;
        }
        let mid = nodes.len() / 2;
        let n = &nodes[mid];
        let d = (n.co - p).length_squared();
        if d <= r2 {
            out.push(KdHit { index: n.index, dist_sq: d });
        }
        if nodes.len() == 1 {
            return;
        }
        let diff = p[n.axis as usize] - n.co[n.axis as usize];
        if diff <= 0.0 || diff * diff <= r2 {
            Self::range(&nodes[..mid], p, r2, out);
        }
        if diff >= 0.0 || diff * diff <= r2 {
            Self::range(&nodes[mid + 1..], p, r2, out);
        }
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
    }

    fn brute(points: &[Vec3], p: Vec3) -> Vec<KdHit> {
        let mut all: Vec<KdHit> = points
            .iter()
            .enumerate()
            .map(|(index, c)| KdHit { index, dist_sq: (*c - p).length_squared() })
            .collect();
        all.sort_by(|a, b| a.dist_sq.total_cmp(&b.dist_sq).then(a.index.cmp(&b.index)));
        all
    }

    #[test]
    fn empty_and_single() {
        let t = KdTree::new(&[]);
        assert!(t.is_empty());
        assert_eq!(t.find_nearest(Vec3::ZERO), None);
        assert!(t.find_nearest_n(Vec3::ZERO, 3).is_empty());
        assert!(t.range_search(Vec3::ZERO, 5.0).is_empty());
        let t = KdTree::new(&[Vec3::X]);
        assert_eq!(t.find_nearest(Vec3::ZERO), Some(KdHit { index: 0, dist_sq: 1.0 }));
        assert_eq!(t.len(), 1);
    }

    #[test]
    fn matches_brute_force() {
        let mut rng = Lcg(42);
        for &n in &[2usize, 3, 17, 200, 1000] {
            let pts: Vec<Vec3> = (0..n).map(|_| rng.v()).collect();
            let tree = KdTree::new(&pts);
            for _ in 0..50 {
                let q = rng.v();
                let want = brute(&pts, q);
                assert_eq!(tree.find_nearest(q).unwrap().dist_sq, want[0].dist_sq);
                for k in [1usize, 4, 10] {
                    let got = tree.find_nearest_n(q, k);
                    let exp: Vec<f32> = want.iter().take(k).map(|h| h.dist_sq).collect();
                    let got_d: Vec<f32> = got.iter().map(|h| h.dist_sq).collect();
                    assert_eq!(got_d, exp, "n={n} k={k}");
                }
                let r = 4.0;
                let got = tree.range_search(q, r);
                let exp: Vec<_> = want.iter().filter(|h| h.dist_sq <= r * r).collect();
                assert_eq!(got.len(), exp.len(), "range n={n}");
                for (g, e) in got.iter().zip(&exp) {
                    assert_eq!(g.dist_sq, e.dist_sq);
                }
            }
        }
    }

    #[test]
    fn k_larger_than_len_returns_all() {
        let pts = [Vec3::ZERO, Vec3::X, Vec3::Y];
        let tree = KdTree::new(&pts);
        assert_eq!(tree.find_nearest_n(Vec3::ZERO, 10).len(), 3);
        assert!(tree.find_nearest_n(Vec3::ZERO, 0).is_empty());
    }

    #[test]
    fn duplicates_are_all_found() {
        let pts = vec![Vec3::ONE; 8];
        let tree = KdTree::new(&pts);
        assert_eq!(tree.range_search(Vec3::ONE, 0.0).len(), 8);
        let mut idx: Vec<_> = tree.find_nearest_n(Vec3::ONE, 8).iter().map(|h| h.index).collect();
        idx.sort();
        assert_eq!(idx, (0..8).collect::<Vec<_>>());
    }
}
