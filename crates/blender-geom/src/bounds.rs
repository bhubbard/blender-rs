//! Axis-aligned bounding boxes (`BLI_bounds`, `isect_aabb_aabb_v3`, ray/slab test).

use glam::Vec3;

/// An axis-aligned box. The empty box has `min > max` so that any point expands it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Default for Aabb {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Aabb {
    pub const EMPTY: Aabb = Aabb {
        min: Vec3::splat(f32::INFINITY),
        max: Vec3::splat(f32::NEG_INFINITY),
    };

    pub fn from_points(points: &[Vec3]) -> Self {
        points.iter().fold(Self::EMPTY, |b, &p| b.expanded(p))
    }

    pub fn is_empty(&self) -> bool {
        self.min.cmpgt(self.max).any()
    }

    pub fn expanded(self, p: Vec3) -> Self {
        Self { min: self.min.min(p), max: self.max.max(p) }
    }

    pub fn union(self, o: Self) -> Self {
        Self { min: self.min.min(o.min), max: self.max.max(o.max) }
    }

    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    pub fn contains(&self, p: Vec3) -> bool {
        p.cmpge(self.min).all() && p.cmple(self.max).all()
    }

    /// Squared distance from `p` to the box (0 inside); infinite for the empty box.
    pub fn dist_sq_to_point(&self, p: Vec3) -> f32 {
        if self.is_empty() {
            return f32::INFINITY;
        }
        (p.clamp(self.min, self.max) - p).length_squared()
    }

    /// Boxes that merely touch count as intersecting (`isect_aabb_aabb_v3`).
    pub fn intersects(&self, o: &Self) -> bool {
        !self.is_empty()
            && !o.is_empty()
            && self.min.cmple(o.max).all()
            && o.min.cmple(self.max).all()
    }

    /// Slab test. Returns the entry/exit distances `(t_min, t_max)` along the ray, with
    /// `t_min` clamped to 0 when the origin is inside, or `None` on a miss.
    pub fn intersect_ray(&self, orig: Vec3, dir: Vec3) -> Option<(f32, f32)> {
        if self.is_empty() {
            return None;
        }
        let mut t0 = 0.0f32;
        let mut t1 = f32::INFINITY;
        for i in 0..3 {
            let (o, d, lo, hi) = (orig[i], dir[i], self.min[i], self.max[i]);
            if d.abs() < f32::EPSILON {
                if o < lo || o > hi {
                    return None;
                }
                continue;
            }
            let inv = 1.0 / d;
            let (a, b) = ((lo - o) * inv, (hi - o) * inv);
            let (near, far) = if a < b { (a, b) } else { (b, a) };
            t0 = t0.max(near);
            t1 = t1.min(far);
            if t0 > t1 {
                return None;
            }
        }
        Some((t0, t1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit() -> Aabb {
        Aabb::from_points(&[Vec3::ZERO, Vec3::ONE])
    }

    #[test]
    fn build_and_query() {
        let b = Aabb::from_points(&[Vec3::new(1.0, 5.0, -2.0), Vec3::new(-1.0, 2.0, 4.0), Vec3::ZERO]);
        assert_eq!(b.min, Vec3::new(-1.0, 0.0, -2.0));
        assert_eq!(b.max, Vec3::new(1.0, 5.0, 4.0));
        assert_eq!(b.center(), Vec3::new(0.0, 2.5, 1.0));
        assert_eq!(b.size(), Vec3::new(2.0, 5.0, 6.0));
        assert!(b.contains(Vec3::ZERO) && !b.contains(Vec3::splat(9.0)));
    }

    #[test]
    fn empty_box_behaviour() {
        let e = Aabb::EMPTY;
        assert!(e.is_empty());
        assert!(!e.contains(Vec3::ZERO));
        assert!(!e.intersects(&unit()));
        assert_eq!(e.union(unit()), unit());
        assert!(e.intersect_ray(Vec3::ZERO, Vec3::X).is_none());
    }

    #[test]
    fn box_intersection() {
        let a = unit();
        let touching = Aabb { min: Vec3::ONE, max: Vec3::splat(2.0) };
        let apart = Aabb { min: Vec3::splat(1.5), max: Vec3::splat(2.0) };
        assert!(a.intersects(&touching));
        assert!(!a.intersects(&apart));
    }

    #[test]
    fn point_distance() {
        let b = unit();
        assert_eq!(b.dist_sq_to_point(Vec3::splat(0.5)), 0.0);
        assert_eq!(b.dist_sq_to_point(Vec3::new(3.0, 0.5, 0.5)), 4.0);
        assert_eq!(b.dist_sq_to_point(Vec3::new(2.0, 2.0, 0.5)), 2.0);
        assert_eq!(Aabb::EMPTY.dist_sq_to_point(Vec3::ZERO), f32::INFINITY);
    }

    #[test]
    fn ray_slab() {
        let b = unit();
        let (t0, t1) = b.intersect_ray(Vec3::new(-1.0, 0.5, 0.5), Vec3::X).unwrap();
        assert_eq!((t0, t1), (1.0, 2.0));
        // Origin inside: entry clamps to 0.
        let (t0, _) = b.intersect_ray(Vec3::splat(0.5), Vec3::X).unwrap();
        assert_eq!(t0, 0.0);
        // Pointing away, passing beside, and parallel-outside all miss.
        assert!(b.intersect_ray(Vec3::new(-1.0, 0.5, 0.5), -Vec3::X).is_none());
        assert!(b.intersect_ray(Vec3::new(-1.0, 2.0, 0.5), Vec3::X).is_none());
        assert!(b.intersect_ray(Vec3::new(0.5, 2.0, 0.5), Vec3::X).is_none());
        // Diagonal ray through the corner region.
        assert!(b.intersect_ray(Vec3::splat(-1.0), Vec3::ONE).is_some());
    }
}
