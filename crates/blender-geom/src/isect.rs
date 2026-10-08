//! Line and segment intersections (`isect_seg_seg_v2_point`, `isect_line_line_v2_point`,
//! `isect_line_line_v3`).

use glam::{Vec2, Vec3};

/// Intersection of two infinite 2D lines. Returns the point, or `None` when parallel.
pub fn isect_line_line_v2(a1: Vec2, a2: Vec2, b1: Vec2, b2: Vec2) -> Option<Vec2> {
    let r = a2 - a1;
    let s = b2 - b1;
    let denom = r.perp_dot(s);
    if denom.abs() < f32::EPSILON {
        return None;
    }
    let t = (b1 - a1).perp_dot(s) / denom;
    Some(a1 + r * t)
}

/// Result of a segment/segment hit: the point and the parameters along each segment.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SegHit {
    pub point: Vec2,
    pub t_a: f32,
    pub t_b: f32,
}

/// Intersection of 2D segments `a1..a2` and `b1..b2` (endpoints inclusive).
/// Parallel and collinear segments return `None`.
pub fn isect_seg_seg_v2(a1: Vec2, a2: Vec2, b1: Vec2, b2: Vec2) -> Option<SegHit> {
    let r = a2 - a1;
    let s = b2 - b1;
    let denom = r.perp_dot(s);
    if denom.abs() < f32::EPSILON {
        return None;
    }
    let d = b1 - a1;
    let t_a = d.perp_dot(s) / denom;
    let t_b = d.perp_dot(r) / denom;
    if (0.0..=1.0).contains(&t_a) && (0.0..=1.0).contains(&t_b) {
        Some(SegHit { point: a1 + r * t_a, t_a, t_b })
    } else {
        None
    }
}

/// Closest points between the infinite 3D lines `a1..a2` and `b1..b2`
/// (one point on each line); `None` when the lines are parallel.
pub fn isect_line_line_v3(a1: Vec3, a2: Vec3, b1: Vec3, b2: Vec3) -> Option<(Vec3, Vec3)> {
    let d1 = a2 - a1;
    let d2 = b2 - b1;
    let r = a1 - b1;
    let a = d1.dot(d1);
    let e = d2.dot(d2);
    let b = d1.dot(d2);
    let denom = a * e - b * b;
    if a <= f32::MIN_POSITIVE || e <= f32::MIN_POSITIVE || denom.abs() <= f32::EPSILON * a * e {
        return None;
    }
    let c = d1.dot(r);
    let f = d2.dot(r);
    let s = (b * f - c * e) / denom;
    let t = (a * f - b * c) / denom;
    Some((a1 + d1 * s, b1 + d2 * t))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crossing_segments() {
        let h = isect_seg_seg_v2(
            Vec2::new(0.0, 0.0),
            Vec2::new(4.0, 4.0),
            Vec2::new(0.0, 4.0),
            Vec2::new(4.0, 0.0),
        )
        .unwrap();
        assert_eq!(h.point, Vec2::new(2.0, 2.0));
        assert_eq!((h.t_a, h.t_b), (0.5, 0.5));
    }

    #[test]
    fn segment_misses() {
        // Lines cross at (2,2) but the second segment stops short.
        let miss = isect_seg_seg_v2(
            Vec2::new(0.0, 0.0),
            Vec2::new(4.0, 4.0),
            Vec2::new(0.0, 4.0),
            Vec2::new(1.0, 3.0),
        );
        assert!(miss.is_none());
        // Parallel and collinear.
        assert!(isect_seg_seg_v2(Vec2::ZERO, Vec2::X, Vec2::Y, Vec2::new(1.0, 1.0)).is_none());
        assert!(isect_seg_seg_v2(Vec2::ZERO, Vec2::X, Vec2::X, Vec2::new(2.0, 0.0)).is_none());
    }

    #[test]
    fn touching_endpoint_counts() {
        let h = isect_seg_seg_v2(Vec2::ZERO, Vec2::X, Vec2::X, Vec2::new(1.0, 1.0)).unwrap();
        assert_eq!(h.point, Vec2::X);
    }

    #[test]
    fn infinite_lines() {
        let p = isect_line_line_v2(Vec2::ZERO, Vec2::X, Vec2::new(3.0, -1.0), Vec2::new(3.0, 5.0));
        assert_eq!(p, Some(Vec2::new(3.0, 0.0)));
        assert!(isect_line_line_v2(Vec2::ZERO, Vec2::X, Vec2::Y, Vec2::new(1.0, 1.0)).is_none());
    }

    #[test]
    fn skew_lines_3d() {
        // X axis and a line parallel to Y at x=2, z=3: closest points (2,0,0) and (2,0,3).
        let (p, q) = isect_line_line_v3(
            Vec3::ZERO,
            Vec3::X,
            Vec3::new(2.0, -1.0, 3.0),
            Vec3::new(2.0, 1.0, 3.0),
        )
        .unwrap();
        assert!((p - Vec3::new(2.0, 0.0, 0.0)).length() < 1e-5);
        assert!((q - Vec3::new(2.0, 0.0, 3.0)).length() < 1e-5);
        assert!(isect_line_line_v3(Vec3::ZERO, Vec3::X, Vec3::Y, Vec3::new(1.0, 1.0, 0.0)).is_none());
    }
}
