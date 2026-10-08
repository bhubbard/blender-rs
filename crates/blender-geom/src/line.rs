//! Point-to-line queries (`closest_to_line_v3`, `closest_to_line_segment_v3`, ...).

use glam::Vec3;

/// Closest point on the infinite line `l1..l2` to `p`, and its parameter `lambda`
/// (0 at `l1`, 1 at `l2`). A degenerate line returns `l1` with lambda 0.
pub fn closest_to_line_v3(p: Vec3, l1: Vec3, l2: Vec3) -> (Vec3, f32) {
    let u = l2 - l1;
    let len2 = u.length_squared();
    if len2 <= f32::MIN_POSITIVE {
        return (l1, 0.0);
    }
    let lambda = (p - l1).dot(u) / len2;
    (l1 + u * lambda, lambda)
}

/// Closest point on the segment `l1..l2` to `p`.
pub fn closest_to_line_segment_v3(p: Vec3, l1: Vec3, l2: Vec3) -> Vec3 {
    let (c, lambda) = closest_to_line_v3(p, l1, l2);
    if lambda <= 0.0 {
        l1
    } else if lambda >= 1.0 {
        l2
    } else {
        c
    }
}

/// Squared distance from `p` to the segment `l1..l2`.
pub fn dist_squared_to_line_segment_v3(p: Vec3, l1: Vec3, l2: Vec3) -> f32 {
    (p - closest_to_line_segment_v3(p, l1, l2)).length_squared()
}

/// Squared distance from `p` to the infinite line `l1..l2`.
pub fn dist_squared_to_line_v3(p: Vec3, l1: Vec3, l2: Vec3) -> f32 {
    (p - closest_to_line_v3(p, l1, l2).0).length_squared()
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    const B: Vec3 = Vec3::new(2.0, 0.0, 0.0);

    #[test]
    fn infinite_line_projection() {
        let (c, l) = closest_to_line_v3(Vec3::new(1.0, 3.0, 0.0), A, B);
        assert_eq!(c, Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(l, 0.5);
        let (c, l) = closest_to_line_v3(Vec3::new(5.0, 1.0, 0.0), A, B);
        assert_eq!(c, Vec3::new(5.0, 0.0, 0.0));
        assert_eq!(l, 2.5);
    }

    #[test]
    fn segment_clamps_to_endpoints() {
        assert_eq!(closest_to_line_segment_v3(Vec3::new(5.0, 1.0, 0.0), A, B), B);
        assert_eq!(closest_to_line_segment_v3(Vec3::new(-5.0, 1.0, 0.0), A, B), A);
        assert_eq!(dist_squared_to_line_segment_v3(Vec3::new(5.0, 4.0, 0.0), A, B), 25.0);
        assert_eq!(dist_squared_to_line_v3(Vec3::new(5.0, 4.0, 0.0), A, B), 16.0);
    }

    #[test]
    fn degenerate_line() {
        let (c, l) = closest_to_line_v3(Vec3::ONE, A, A);
        assert_eq!((c, l), (A, 0.0));
    }
}
