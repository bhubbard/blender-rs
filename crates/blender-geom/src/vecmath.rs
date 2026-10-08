//! Vector helpers from `BLI_math_vector`: angles, projection, reflection, orthonormal
//! bases, axis rotation and comparison.

use glam::Vec3;

/// Angle between two unit vectors, clamped against rounding error (`angle_normalized_v3v3`).
pub fn angle_normalized_v3v3(a: Vec3, b: Vec3) -> f32 {
    a.dot(b).clamp(-1.0, 1.0).acos()
}

/// Angle between two arbitrary vectors; a zero vector gives π/2 as in Blender.
pub fn angle_v3v3(a: Vec3, b: Vec3) -> f32 {
    angle_normalized_v3v3(a.normalize_or_zero(), b.normalize_or_zero())
}

/// Angle at `b` of the corner `a-b-c`.
pub fn angle_v3v3v3(a: Vec3, b: Vec3, c: Vec3) -> f32 {
    angle_v3v3(a - b, c - b)
}

/// Projection of `p` onto `n` (zero when `n` is zero).
pub fn project_v3_v3(p: Vec3, n: Vec3) -> Vec3 {
    let d = n.length_squared();
    if d == 0.0 {
        Vec3::ZERO
    } else {
        n * (p.dot(n) / d)
    }
}

/// Reflection of `v` about the plane with normal `n` (need not be unit length).
pub fn reflect_v3_v3v3(v: Vec3, n: Vec3) -> Vec3 {
    let n = n.normalize_or_zero();
    v - n * (2.0 * v.dot(n))
}

/// Two unit vectors completing `n` to an orthonormal basis (Duff et al., 2017).
/// `n` must be unit length.
pub fn ortho_basis_v3(n: Vec3) -> (Vec3, Vec3) {
    let sign = if n.z >= 0.0 { 1.0 } else { -1.0 };
    let a = -1.0 / (sign + n.z);
    let b = n.x * n.y * a;
    (
        Vec3::new(1.0 + sign * n.x * n.x * a, sign * b, -sign * n.x),
        Vec3::new(b, sign + n.y * n.y * a, -n.y),
    )
}

/// Rotates `v` about the (unit or non-unit) `axis` by `angle` radians (Rodrigues).
pub fn rotate_v3_v3v3fl(v: Vec3, axis: Vec3, angle: f32) -> Vec3 {
    let k = axis.normalize_or_zero();
    if k == Vec3::ZERO {
        return v;
    }
    let (s, c) = angle.sin_cos();
    v * c + k.cross(v) * s + k * (k.dot(v) * (1.0 - c))
}

/// Index (0, 1, 2) of the axis with the largest absolute component (`axis_dominant_v3_single`).
pub fn closest_axis_v3(v: Vec3) -> usize {
    let a = v.abs();
    if a.x >= a.y && a.x >= a.z {
        0
    } else if a.y >= a.z {
        1
    } else {
        2
    }
}

pub fn mid_v3_v3v3(a: Vec3, b: Vec3) -> Vec3 {
    (a + b) * 0.5
}

/// `true` when every component differs by at most `limit` (`compare_v3v3`).
pub fn compare_v3v3(a: Vec3, b: Vec3, limit: f32) -> bool {
    (a - b).abs().max_element() <= limit
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::{FRAC_PI_2, PI};

    fn close(a: Vec3, b: Vec3) -> bool {
        (a - b).length() < 1e-5
    }

    #[test]
    fn angles() {
        assert!((angle_v3v3(Vec3::X, Vec3::Y) - FRAC_PI_2).abs() < 1e-6);
        assert!((angle_v3v3(Vec3::X, -Vec3::X * 3.0) - PI).abs() < 1e-6);
        assert_eq!(angle_v3v3(Vec3::X, Vec3::X * 2.0), 0.0);
        assert!((angle_v3v3(Vec3::ZERO, Vec3::X) - FRAC_PI_2).abs() < 1e-6);
        let a = angle_v3v3v3(Vec3::X, Vec3::ZERO, Vec3::new(1.0, 1.0, 0.0));
        assert!((a - PI / 4.0).abs() < 1e-6);
        // Rounding that pushes the dot product above 1 must not produce NaN.
        let v = Vec3::new(0.577_350_3, 0.577_350_3, 0.577_350_3);
        assert!(!angle_normalized_v3v3(v, v).is_nan());
    }

    #[test]
    fn projection_and_reflection() {
        assert_eq!(project_v3_v3(Vec3::new(3.0, 4.0, 0.0), Vec3::X * 2.0), Vec3::new(3.0, 0.0, 0.0));
        assert_eq!(project_v3_v3(Vec3::ONE, Vec3::ZERO), Vec3::ZERO);
        assert!(close(reflect_v3_v3v3(Vec3::new(1.0, -1.0, 0.0), Vec3::Y * 5.0), Vec3::new(1.0, 1.0, 0.0)));
    }

    #[test]
    fn orthonormal_basis_for_many_normals() {
        for n in [
            Vec3::X,
            -Vec3::X,
            Vec3::Y,
            Vec3::Z,
            -Vec3::Z,
            Vec3::new(1.0, 2.0, 3.0).normalize(),
            Vec3::new(-0.3, 0.2, -0.9).normalize(),
        ] {
            let (t, b) = ortho_basis_v3(n);
            assert!((t.length() - 1.0).abs() < 1e-5 && (b.length() - 1.0).abs() < 1e-5);
            assert!(t.dot(b).abs() < 1e-5 && t.dot(n).abs() < 1e-5 && b.dot(n).abs() < 1e-5);
        }
    }

    #[test]
    fn axis_rotation() {
        assert!(close(rotate_v3_v3v3fl(Vec3::X, Vec3::Z * 4.0, FRAC_PI_2), Vec3::Y));
        assert!(close(rotate_v3_v3v3fl(Vec3::X, Vec3::X, 1.0), Vec3::X));
        assert_eq!(rotate_v3_v3v3fl(Vec3::X, Vec3::ZERO, 1.0), Vec3::X);
        let v = Vec3::new(1.0, 2.0, 3.0);
        let r = rotate_v3_v3v3fl(v, Vec3::new(1.0, 1.0, 0.0), 0.7);
        assert!((r.length() - v.length()).abs() < 1e-5);
    }

    #[test]
    fn misc() {
        assert_eq!(closest_axis_v3(Vec3::new(-5.0, 2.0, 3.0)), 0);
        assert_eq!(closest_axis_v3(Vec3::new(1.0, -4.0, 3.0)), 1);
        assert_eq!(closest_axis_v3(Vec3::new(1.0, 2.0, -9.0)), 2);
        assert_eq!(mid_v3_v3v3(Vec3::ZERO, Vec3::splat(2.0)), Vec3::ONE);
        assert!(compare_v3v3(Vec3::ZERO, Vec3::splat(0.001), 0.01));
        assert!(!compare_v3v3(Vec3::ZERO, Vec3::new(0.0, 0.5, 0.0), 0.01));
    }
}
