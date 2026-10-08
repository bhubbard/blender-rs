//! Matrix helpers from `BLI_math_matrix`: composition and decomposition, safe inversion,
//! orthogonalisation and OpenGL-style projections.

use glam::{Mat3, Mat4, Quat, Vec3};

/// `loc_rot_size_to_mat4`: scale, then rotate, then translate.
pub fn loc_rot_size_to_mat4(loc: Vec3, rot: Quat, size: Vec3) -> Mat4 {
    Mat4::from_scale_rotation_translation(size, rot, loc)
}

/// `mat4_to_loc_rot_size`. For a negative-determinant matrix the x scale is negative.
pub fn mat4_to_loc_rot_size(m: Mat4) -> (Vec3, Quat, Vec3) {
    let (scale, rot, loc) = m.to_scale_rotation_translation();
    (loc, rot, scale)
}

/// Inverse, or `None` when the matrix is singular (`invert_m4`).
pub fn invert_m4(m: Mat4) -> Option<Mat4> {
    let det = m.determinant();
    if !det.is_finite() || det.abs() < f32::MIN_POSITIVE * 16.0 {
        return None;
    }
    let inv = m.inverse();
    inv.is_finite().then_some(inv)
}

/// Transforms a point (w = 1) (`mul_m4_v3`).
pub fn mul_m4_v3(m: &Mat4, p: Vec3) -> Vec3 {
    m.transform_point3(p)
}

/// Transforms a direction, ignoring translation (`mul_mat3_m4_v3`).
pub fn mul_mat3_m4_v3(m: &Mat4, d: Vec3) -> Vec3 {
    m.transform_vector3(d)
}

/// Gram-Schmidt orthonormalisation of the columns, keeping the first column's direction
/// (`orthogonalize_m3` with the x axis as the anchor). Degenerate columns become zero.
pub fn orthogonalize_m3(m: Mat3) -> Mat3 {
    let x = m.x_axis.normalize_or_zero();
    let y = (m.y_axis - x * m.y_axis.dot(x)).normalize_or_zero();
    let z = (m.z_axis - x * m.z_axis.dot(x) - y * m.z_axis.dot(y)).normalize_or_zero();
    Mat3::from_cols(x, y, z)
}

/// `true` when the columns are orthonormal within `eps`.
pub fn is_orthonormal_m3(m: Mat3, eps: f32) -> bool {
    let p = m.transpose() * m;
    (p.x_axis - Vec3::X).abs().max_element() <= eps
        && (p.y_axis - Vec3::Y).abs().max_element() <= eps
        && (p.z_axis - Vec3::Z).abs().max_element() <= eps
}

/// OpenGL orthographic projection (`orthographic_m4`), clip depth in [-1, 1].
pub fn orthographic_m4(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
    Mat4::orthographic_rh_gl(left, right, bottom, top, near, far)
}

/// OpenGL perspective frustum (`perspective_m4`), clip depth in [-1, 1].
pub fn perspective_m4(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
    let (dx, dy, dz) = (right - left, top - bottom, far - near);
    Mat4::from_cols(
        glam::Vec4::new(2.0 * near / dx, 0.0, 0.0, 0.0),
        glam::Vec4::new(0.0, 2.0 * near / dy, 0.0, 0.0),
        glam::Vec4::new((right + left) / dx, (top + bottom) / dy, -(far + near) / dz, -1.0),
        glam::Vec4::new(0.0, 0.0, -2.0 * far * near / dz, 0.0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: Vec3, b: Vec3) -> bool {
        (a - b).length() < 1e-4
    }

    #[test]
    fn compose_decompose_roundtrip() {
        let loc = Vec3::new(1.0, -2.0, 3.0);
        let rot = Quat::from_euler(glam::EulerRot::ZYX, 0.4, -0.3, 1.0);
        let size = Vec3::new(2.0, 0.5, 3.0);
        let m = loc_rot_size_to_mat4(loc, rot, size);
        let (l, r, s) = mat4_to_loc_rot_size(m);
        assert!(close(l, loc) && close(s, size));
        assert!(close(r * Vec3::X, rot * Vec3::X) && close(r * Vec3::Y, rot * Vec3::Y));
        // Recomposing gives back the same matrix.
        let m2 = loc_rot_size_to_mat4(l, r, s);
        assert!(m.abs_diff_eq(m2, 1e-4));
    }

    #[test]
    fn point_versus_direction() {
        let m = loc_rot_size_to_mat4(Vec3::new(5.0, 0.0, 0.0), Quat::IDENTITY, Vec3::splat(2.0));
        assert_eq!(mul_m4_v3(&m, Vec3::X), Vec3::new(7.0, 0.0, 0.0));
        assert_eq!(mul_mat3_m4_v3(&m, Vec3::X), Vec3::new(2.0, 0.0, 0.0));
    }

    #[test]
    fn inversion() {
        let m = loc_rot_size_to_mat4(Vec3::new(1.0, 2.0, 3.0), Quat::from_rotation_y(0.9), Vec3::splat(2.0));
        let inv = invert_m4(m).unwrap();
        assert!((m * inv).abs_diff_eq(Mat4::IDENTITY, 1e-4));
        assert!(invert_m4(Mat4::ZERO).is_none());
        let flat = Mat4::from_scale(Vec3::new(1.0, 1.0, 0.0));
        assert!(invert_m4(flat).is_none());
        assert!(invert_m4(Mat4::from_scale(Vec3::splat(f32::NAN))).is_none());
    }

    #[test]
    fn orthogonalise() {
        let skew = Mat3::from_cols(Vec3::new(1.0, 0.1, 0.0), Vec3::new(0.2, 1.0, 0.1), Vec3::new(0.0, 0.3, 1.0));
        assert!(!is_orthonormal_m3(skew, 1e-3));
        let o = orthogonalize_m3(skew);
        assert!(is_orthonormal_m3(o, 1e-5));
        assert!(close(o.x_axis, skew.x_axis.normalize()));
        let degenerate = orthogonalize_m3(Mat3::from_cols(Vec3::X, Vec3::X, Vec3::Z));
        assert_eq!(degenerate.y_axis, Vec3::ZERO);
    }

    #[test]
    fn projections_map_frustum_to_clip_cube() {
        let o = orthographic_m4(-2.0, 2.0, -1.0, 1.0, 1.0, 5.0);
        assert!(close(o.project_point3(Vec3::new(-2.0, -1.0, -1.0)), Vec3::new(-1.0, -1.0, -1.0)));
        assert!(close(o.project_point3(Vec3::new(2.0, 1.0, -5.0)), Vec3::ONE));
        let p = perspective_m4(-1.0, 1.0, -1.0, 1.0, 1.0, 10.0);
        assert!(close(p.project_point3(Vec3::new(0.0, 0.0, -1.0)), Vec3::new(0.0, 0.0, -1.0)));
        assert!(close(p.project_point3(Vec3::new(10.0, 10.0, -10.0)), Vec3::ONE));
    }
}
