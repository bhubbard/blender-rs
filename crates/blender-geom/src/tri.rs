//! Triangle queries: closest point, barycentric weights, ray and 2D point tests
//! (`closest_on_tri_to_point_v3`, `barycentric_weights_v3`, `isect_ray_tri_v3`,
//! `isect_point_tri_v2`).

use glam::{Vec2, Vec3};

/// Closest point on triangle `abc` to `p` (Ericson, *Real-Time Collision Detection* 5.1.5).
pub fn closest_on_tri_to_point_v3(p: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    let ab = b - a;
    let ac = c - a;
    let ap = p - a;
    let d1 = ab.dot(ap);
    let d2 = ac.dot(ap);
    if d1 <= 0.0 && d2 <= 0.0 {
        return a;
    }
    let bp = p - b;
    let d3 = ab.dot(bp);
    let d4 = ac.dot(bp);
    if d3 >= 0.0 && d4 <= d3 {
        return b;
    }
    let vc = d1 * d4 - d3 * d2;
    if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        return a + ab * (d1 / (d1 - d3));
    }
    let cp = p - c;
    let d5 = ab.dot(cp);
    let d6 = ac.dot(cp);
    if d6 >= 0.0 && d5 <= d6 {
        return c;
    }
    let vb = d5 * d2 - d1 * d6;
    if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        return a + ac * (d2 / (d2 - d6));
    }
    let va = d3 * d6 - d5 * d4;
    if va <= 0.0 && (d4 - d3) >= 0.0 && (d5 - d6) >= 0.0 {
        let w = (d4 - d3) / ((d4 - d3) + (d5 - d6));
        return b + (c - b) * w;
    }
    let denom = 1.0 / (va + vb + vc);
    a + ab * (vb * denom) + ac * (vc * denom)
}

/// Barycentric weights `(wa, wb, wc)` of the projection of `p` onto triangle `abc`.
/// `None` for a degenerate triangle.
pub fn barycentric_weights_v3(p: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Option<Vec3> {
    let v0 = b - a;
    let v1 = c - a;
    let v2 = p - a;
    let d00 = v0.dot(v0);
    let d01 = v0.dot(v1);
    let d11 = v1.dot(v1);
    let d20 = v2.dot(v0);
    let d21 = v2.dot(v1);
    let denom = d00 * d11 - d01 * d01;
    if denom.abs() <= f32::MIN_POSITIVE {
        return None;
    }
    let wb = (d11 * d20 - d01 * d21) / denom;
    let wc = (d00 * d21 - d01 * d20) / denom;
    Some(Vec3::new(1.0 - wb - wc, wb, wc))
}

/// Hit of a ray against a triangle: distance `t` along the ray and barycentric `(u, v)`
/// of the hit relative to `v1 - v0` and `v2 - v0`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RayTriHit {
    pub t: f32,
    pub u: f32,
    pub v: f32,
}

/// Möller–Trumbore ray/triangle intersection (ray only, `t >= 0`; double-sided).
pub fn isect_ray_tri_v3(orig: Vec3, dir: Vec3, v0: Vec3, v1: Vec3, v2: Vec3) -> Option<RayTriHit> {
    let e1 = v1 - v0;
    let e2 = v2 - v0;
    let p = dir.cross(e2);
    let det = e1.dot(p);
    if det.abs() < f32::EPSILON {
        return None;
    }
    let inv = 1.0 / det;
    let tv = orig - v0;
    let u = tv.dot(p) * inv;
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = tv.cross(e1);
    let v = dir.dot(q) * inv;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let t = e2.dot(q) * inv;
    if t < 0.0 {
        return None;
    }
    Some(RayTriHit { t, u, v })
}

/// 2D point-in-triangle test, independent of winding; edges count as inside.
pub fn isect_point_tri_v2(pt: Vec2, a: Vec2, b: Vec2, c: Vec2) -> bool {
    let side = |p1: Vec2, p2: Vec2| (pt - p2).perp_dot(p1 - p2);
    let d1 = side(a, b);
    let d2 = side(b, c);
    let d3 = side(c, a);
    let has_neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let has_pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
    !(has_neg && has_pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    const B: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    const C: Vec3 = Vec3::new(0.0, 4.0, 0.0);

    #[test]
    fn closest_point_regions() {
        // Above the interior: projects straight down.
        assert_eq!(closest_on_tri_to_point_v3(Vec3::new(1.0, 1.0, 3.0), A, B, C), Vec3::new(1.0, 1.0, 0.0));
        // Vertex regions.
        assert_eq!(closest_on_tri_to_point_v3(Vec3::new(-1.0, -1.0, 0.0), A, B, C), A);
        assert_eq!(closest_on_tri_to_point_v3(Vec3::new(9.0, -1.0, 0.0), A, B, C), B);
        assert_eq!(closest_on_tri_to_point_v3(Vec3::new(-1.0, 9.0, 0.0), A, B, C), C);
        // Edge regions.
        assert_eq!(closest_on_tri_to_point_v3(Vec3::new(2.0, -3.0, 0.0), A, B, C), Vec3::new(2.0, 0.0, 0.0));
        assert_eq!(closest_on_tri_to_point_v3(Vec3::new(-3.0, 2.0, 0.0), A, B, C), Vec3::new(0.0, 2.0, 0.0));
        assert_eq!(closest_on_tri_to_point_v3(Vec3::new(4.0, 4.0, 0.0), A, B, C), Vec3::new(2.0, 2.0, 0.0));
    }

    #[test]
    fn closest_point_is_never_farther_than_vertices() {
        let p = Vec3::new(1.3, -2.1, 0.7);
        let q = closest_on_tri_to_point_v3(p, A, B, C);
        for v in [A, B, C] {
            assert!((p - q).length() <= (p - v).length() + 1e-5);
        }
    }

    #[test]
    fn barycentric() {
        let w = barycentric_weights_v3(Vec3::new(1.0, 1.0, 0.0), A, B, C).unwrap();
        assert!((w - Vec3::new(0.5, 0.25, 0.25)).length() < 1e-6);
        assert!((w.x + w.y + w.z - 1.0).abs() < 1e-6);
        let at_b = barycentric_weights_v3(B, A, B, C).unwrap();
        assert!((at_b - Vec3::Y).length() < 1e-6);
        assert!(barycentric_weights_v3(Vec3::ZERO, A, A, A).is_none());
    }

    #[test]
    fn ray_hits_and_misses() {
        let hit = isect_ray_tri_v3(Vec3::new(1.0, 1.0, 5.0), -Vec3::Z, A, B, C).unwrap();
        assert!((hit.t - 5.0).abs() < 1e-6);
        assert!((hit.u - 0.25).abs() < 1e-6 && (hit.v - 0.25).abs() < 1e-6);
        // Back-face hit works too.
        assert!(isect_ray_tri_v3(Vec3::new(1.0, 1.0, -5.0), Vec3::Z, A, B, C).is_some());
        // Misses: outside, behind the origin, parallel.
        assert!(isect_ray_tri_v3(Vec3::new(5.0, 5.0, 5.0), -Vec3::Z, A, B, C).is_none());
        assert!(isect_ray_tri_v3(Vec3::new(1.0, 1.0, 5.0), Vec3::Z, A, B, C).is_none());
        assert!(isect_ray_tri_v3(Vec3::new(1.0, 1.0, 5.0), Vec3::X, A, B, C).is_none());
    }

    #[test]
    fn point_in_triangle_2d() {
        let (a, b, c) = (Vec2::ZERO, Vec2::new(4.0, 0.0), Vec2::new(0.0, 4.0));
        assert!(isect_point_tri_v2(Vec2::new(1.0, 1.0), a, b, c));
        assert!(isect_point_tri_v2(Vec2::new(2.0, 2.0), a, b, c)); // on hypotenuse
        assert!(!isect_point_tri_v2(Vec2::new(3.0, 3.0), a, b, c));
        // Winding independent.
        assert!(isect_point_tri_v2(Vec2::new(1.0, 1.0), a, c, b));
    }
}
