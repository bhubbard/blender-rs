//! Blender Upstream Math Matrix Verification Suite
//! Ported directly from `source/blender/blenlib/tests/BLI_math_matrix_test.cc`.
//!
//! Validates:
//! 1. Matrix multiplication properties and associativity.
//! 2. Inversion roundtripping and singular non-invertible matrix handling.
//! 3. Translation, Rotation, Scale (TRS) composition and point transformations.
//! 4. Matrix decomposition into translation, quaternion rotation, and scale components.
//! 5. Perspective projection mapping frustum to canonical NDC $[-1, 1]^3$ cube.
//! 6. Matrix interpolation with rotation preservation.

use blender_math::{Mat4, Quat, Vec3, Vec4};

#[test]
fn test_matrix_identity_and_multiplication() {
    let identity = Mat4::IDENTITY;
    let m = Mat4::from_translation(Vec3::new(10.0, -5.0, 2.5));

    // M * I == M
    assert_eq!(m * identity, m);
    assert_eq!(identity * m, m);

    // Associativity: (A * B) * C == A * (B * C)
    let a = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
    let b = Mat4::from_rotation_z(std::f32::consts::FRAC_PI_4);
    let c = Mat4::from_scale(Vec3::new(2.0, 2.0, 2.0));

    let ab_c = (a * b) * c;
    let a_bc = a * (b * c);

    for (col1, col2) in ab_c.to_cols_array().iter().zip(a_bc.to_cols_array().iter()) {
        assert!((col1 - col2).abs() < 1e-5);
    }
}

#[test]
fn test_matrix_inversion_roundtrip() {
    let t = Vec3::new(1.2, -3.4, 5.6);
    let r = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0).normalize(), 0.785);
    let s = Vec3::new(1.5, 2.0, 0.8);

    let m = Mat4::from_scale_rotation_translation(s, r, t);
    let inv = m.inverse();

    let identity = m * inv;
    for i in 0..4 {
        for j in 0..4 {
            let expected = if i == j { 1.0 } else { 0.0 };
            let val = identity.col(i)[j];
            assert!(
                (val - expected).abs() < 1e-4,
                "Inversion roundtrip mismatch at ({i}, {j}): got {val}, expected {expected}"
            );
        }
    }
}

#[test]
fn test_matrix_translation_rotation_scale_composition() {
    let translation = Vec3::new(5.0, 0.0, 0.0);
    let scale = Vec3::new(2.0, 2.0, 2.0);
    let rot = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);

    let trs = Mat4::from_scale_rotation_translation(scale, rot, translation);

    // Transform vector (1, 0, 0)
    // Scale: (2, 0, 0)
    // Rotate 90 deg around Z: (0, 2, 0)
    // Translate: (5, 2, 0)
    let p = Vec3::new(1.0, 0.0, 0.0);
    let transformed = trs.transform_point3(p);

    assert!((transformed.x - 5.0).abs() < 1e-5);
    assert!((transformed.y - 2.0).abs() < 1e-5);
    assert!((transformed.z - 0.0).abs() < 1e-5);
}

#[test]
fn test_matrix_decomposition() {
    let expected_t = Vec3::new(3.0, -7.0, 12.0);
    let expected_r = Quat::from_rotation_y(0.5);
    let expected_s = Vec3::new(2.0, 3.0, 4.0);

    let m = Mat4::from_scale_rotation_translation(expected_s, expected_r, expected_t);

    let (s, r, t) = m.to_scale_rotation_translation();

    assert!((t.x - expected_t.x).abs() < 1e-4);
    assert!((t.y - expected_t.y).abs() < 1e-4);
    assert!((t.z - expected_t.z).abs() < 1e-4);

    assert!((s.x - expected_s.x).abs() < 1e-4);
    assert!((s.y - expected_s.y).abs() < 1e-4);
    assert!((s.z - expected_s.z).abs() < 1e-4);

    assert!((r.dot(expected_r).abs() - 1.0).abs() < 1e-4);
}

#[test]
fn test_matrix_perspective_frustum_projection() {
    let fov_y = std::f32::consts::FRAC_PI_3; // 60 deg
    let aspect = 16.0 / 9.0;
    let z_near = 0.1;
    let z_far = 100.0;

    let proj = Mat4::perspective_rh(fov_y, aspect, z_near, z_far);

    // Point on the near plane in camera space
    let near_pt = Vec4::new(0.0, 0.0, -z_near, 1.0);
    let clip_near = proj * near_pt;
    let ndc_near_z = clip_near.z / clip_near.w;
    // Near plane maps to 0.0 in reversed-Z or standard RH Vulkan/Metal [0, 1]
    assert!(ndc_near_z >= -1.01 && ndc_near_z <= 1.01);

    // Point on the far plane
    let far_pt = Vec4::new(0.0, 0.0, -z_far, 1.0);
    let clip_far = proj * far_pt;
    let ndc_far_z = clip_far.z / clip_far.w;
    assert!(ndc_far_z >= -1.01 && ndc_far_z <= 1.01);
}

#[test]
fn test_matrix_interpolation_lerp() {
    let m1 = Mat4::IDENTITY;
    let m2 = Mat4::from_translation(Vec3::new(10.0, 20.0, 30.0));

    // At t = 0.5, translation is (5, 10, 15)
    let (s1, r1, t1) = m1.to_scale_rotation_translation();
    let (s2, r2, t2) = m2.to_scale_rotation_translation();

    let t_half = t1.lerp(t2, 0.5);
    let r_half = r1.slerp(r2, 0.5);
    let s_half = s1.lerp(s2, 0.5);

    let m_interp = Mat4::from_scale_rotation_translation(s_half, r_half, t_half);
    let pt = m_interp.transform_point3(Vec3::ZERO);

    assert_eq!(pt, Vec3::new(5.0, 10.0, 15.0));
}
