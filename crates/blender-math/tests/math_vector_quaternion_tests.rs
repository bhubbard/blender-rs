//! Tests for vectors, quaternions, Euler rotations, and 3D bounding boxes,
//! porting upstream Blender's BLI_math_vector_test.cc and BLI_math_rotation_test.cc.

use blender_math::{area_tri_v3, is_zero_v3, normal_tri_v3, BoundingBox, Bounds2D};
use glam::{EulerRot, Mat4, Quat, Vec2, Vec3};

#[test]
fn test_vector_zero_and_precision() {
    assert!(is_zero_v3(Vec3::ZERO));
    assert!(is_zero_v3(Vec3::new(1e-7, -1e-7, 0.0)));
    assert!(!is_zero_v3(Vec3::new(1e-4, 0.0, 0.0)));
}

#[test]
fn test_triangle_normals_and_degenerate_handling() {
    // CCW triangle in XY plane
    let v0 = Vec3::new(0.0, 0.0, 0.0);
    let v1 = Vec3::new(1.0, 0.0, 0.0);
    let v2 = Vec3::new(0.0, 1.0, 0.0);
    let n = normal_tri_v3(v0, v1, v2).expect("Normal should exist");
    assert!((n - Vec3::Z).length() < 1e-5);
    assert!((area_tri_v3(v0, v1, v2) - 0.5).abs() < 1e-5);

    // CW triangle has flipped normal
    let n_cw = normal_tri_v3(v0, v2, v1).expect("Normal should exist");
    assert!((n_cw - (-Vec3::Z)).length() < 1e-5);

    // Degenerate collinear points
    let v_collinear = Vec3::new(2.0, 0.0, 0.0);
    let n_degen = normal_tri_v3(v0, v1, v_collinear);
    assert!(n_degen.is_none());
    assert!(area_tri_v3(v0, v1, v_collinear) < 1e-5);
}

#[test]
fn test_quaternion_rotation_and_slerp() {
    // 90 degrees around Z axis
    let q_z90 = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);
    let rotated = q_z90 * Vec3::X;
    assert!((rotated.x - 0.0).abs() < 1e-5);
    assert!((rotated.y - 1.0).abs() < 1e-5);
    assert!((rotated.z - 0.0).abs() < 1e-5);

    // SLERP halfway should yield 45 degrees
    let q_ident = Quat::IDENTITY;
    let q_half = q_ident.slerp(q_z90, 0.5);
    let rot_half = q_half * Vec3::X;
    let expected = Vec3::new(2.0f32.sqrt() * 0.5, 2.0f32.sqrt() * 0.5, 0.0);
    assert!((rot_half - expected).length() < 1e-5);
}

#[test]
fn test_euler_to_quaternion_to_matrix_consistency() {
    let roll = 0.3f32;
    let pitch = -0.5f32;
    let yaw = 1.2f32;

    // XYZ Euler order
    let q = Quat::from_euler(EulerRot::XYZ, roll, pitch, yaw);
    let mat_from_q = Mat4::from_quat(q);

    let mat_rot_x = Mat4::from_rotation_x(roll);
    let mat_rot_y = Mat4::from_rotation_y(pitch);
    let mat_rot_z = Mat4::from_rotation_z(yaw);
    let mat_euler = mat_rot_x * mat_rot_y * mat_rot_z;

    let test_pt = Vec3::new(1.0, 2.0, -3.0);
    let res_q = mat_from_q.transform_point3(test_pt);
    let res_euler = mat_euler.transform_point3(test_pt);

    assert!((res_q - res_euler).length() < 1e-4);
}

#[test]
fn test_bounds_2d_and_3d_expansion() {
    let mut b2 = Bounds2D::from_point(Vec2::new(1.0, 2.0));
    assert!(b2.is_empty()); // Point has zero area
    assert_eq!(b2.center(), Vec2::new(1.0, 2.0));
    assert_eq!(b2.size(), Vec2::ZERO);

    b2.pad(Vec2::new(2.0, 3.0));
    assert!(!b2.is_empty()); // Padded bounds now has non-zero area
    assert_eq!(b2.min, Vec2::new(-1.0, -1.0));
    assert_eq!(b2.max, Vec2::new(3.0, 5.0));
    assert_eq!(b2.size(), Vec2::new(4.0, 6.0));
    assert_eq!(b2.center(), Vec2::new(1.0, 2.0));

    let mut b3 = BoundingBox::EMPTY;
    assert!(b3.is_empty());
    b3.extend(Vec3::new(-1.0, 0.0, 5.0));
    b3.extend(Vec3::new(3.0, -4.0, 1.0));
    assert_eq!(b3.min, Vec3::new(-1.0, -4.0, 1.0));
    assert_eq!(b3.max, Vec3::new(3.0, 0.0, 5.0));
    assert_eq!(b3.center(), Vec3::new(1.0, -2.0, 3.0));
    assert_eq!(b3.size(), Vec3::new(4.0, 4.0, 4.0));
}

#[test]
fn test_orthonormal_basis_construction() {
    let normal = Vec3::new(0.0, 1.0, 0.0);
    let (tangent, bitangent) = normal.any_orthonormal_pair();

    // Check mutual orthogonality
    assert!(normal.dot(tangent).abs() < 1e-5);
    assert!(normal.dot(bitangent).abs() < 1e-5);
    assert!(tangent.dot(bitangent).abs() < 1e-5);

    // Check unit length
    assert!((tangent.length() - 1.0).abs() < 1e-5);
    assert!((bitangent.length() - 1.0).abs() < 1e-5);
}
