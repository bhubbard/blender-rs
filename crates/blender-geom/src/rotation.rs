//! Rotation conversions, porting `BLI_math_rotation` (`eul_to_quat`, `quat_to_eul`,
//! `axis_angle_to_quat`, `quat_to_axis_angle`, `rotation_between_vecs_to_quat`).
//!
//! Blender's Euler order name lists the axes in the order the rotations are *applied*:
//! `XYZ` means X first, then Y, then Z, i.e. `R = Rz * Ry * Rx`. `glam::EulerRot` names the
//! product left to right instead, so each order maps to the reversed glam sequence.

use glam::{EulerRot, Mat3, Quat, Vec3};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EulerOrder {
    Xyz,
    Xzy,
    Yxz,
    Yzx,
    Zxy,
    Zyx,
}

impl EulerOrder {
    /// The glam sequence and, for it, which of (x, y, z) fills each of its three slots.
    fn glam(self) -> (EulerRot, [usize; 3]) {
        match self {
            EulerOrder::Xyz => (EulerRot::ZYX, [2, 1, 0]),
            EulerOrder::Xzy => (EulerRot::YZX, [1, 2, 0]),
            EulerOrder::Yxz => (EulerRot::ZXY, [2, 0, 1]),
            EulerOrder::Yzx => (EulerRot::XZY, [0, 2, 1]),
            EulerOrder::Zxy => (EulerRot::YXZ, [1, 0, 2]),
            EulerOrder::Zyx => (EulerRot::XYZ, [0, 1, 2]),
        }
    }
}

/// Euler angles `(x, y, z)` in radians with an application order.
pub fn euler_to_quat(angles: Vec3, order: EulerOrder) -> Quat {
    let (rot, slots) = order.glam();
    Quat::from_euler(rot, angles[slots[0]], angles[slots[1]], angles[slots[2]])
}

/// Inverse of [`euler_to_quat`]; returns `(x, y, z)` angles in radians.
pub fn quat_to_euler(q: Quat, order: EulerOrder) -> Vec3 {
    let (rot, slots) = order.glam();
    let (a, b, c) = q.normalize().to_euler(rot);
    let mut out = Vec3::ZERO;
    for (v, slot) in [a, b, c].into_iter().zip(slots) {
        out[slot] = v;
    }
    out
}

pub fn euler_to_mat3(angles: Vec3, order: EulerOrder) -> Mat3 {
    Mat3::from_quat(euler_to_quat(angles, order))
}

/// Quaternion for a rotation of `angle` radians about `axis` (need not be normalised;
/// a zero axis gives the identity, as in Blender).
pub fn axis_angle_to_quat(axis: Vec3, angle: f32) -> Quat {
    let a = axis.normalize_or_zero();
    if a == Vec3::ZERO {
        return Quat::IDENTITY;
    }
    Quat::from_axis_angle(a, angle)
}

/// Axis and angle of a quaternion; the identity yields the Z axis and angle 0 (Blender's choice).
pub fn quat_to_axis_angle(q: Quat) -> (Vec3, f32) {
    let q = q.normalize();
    let s = (q.x * q.x + q.y * q.y + q.z * q.z).sqrt();
    if s < 1e-6 {
        return (Vec3::Z, 0.0);
    }
    (Vec3::new(q.x, q.y, q.z) / s, 2.0 * s.atan2(q.w))
}

/// Shortest-arc rotation taking direction `a` onto direction `b`.
pub fn rotation_between_vecs_to_quat(a: Vec3, b: Vec3) -> Quat {
    let (a, b) = (a.normalize_or_zero(), b.normalize_or_zero());
    if a == Vec3::ZERO || b == Vec3::ZERO {
        return Quat::IDENTITY;
    }
    Quat::from_rotation_arc(a, b)
}

/// Spherical interpolation along the shorter arc (`interp_qt_qtqt`).
pub fn quat_interp(a: Quat, b: Quat, t: f32) -> Quat {
    a.slerp(b, t)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ORDERS: [EulerOrder; 6] = [
        EulerOrder::Xyz,
        EulerOrder::Xzy,
        EulerOrder::Yxz,
        EulerOrder::Yzx,
        EulerOrder::Zxy,
        EulerOrder::Zyx,
    ];

    fn close(a: Vec3, b: Vec3) -> bool {
        (a - b).length() < 1e-4
    }

    #[test]
    fn order_matches_explicit_matrix_product() {
        let (x, y, z) = (0.3f32, -0.7f32, 1.1f32);
        let (rx, ry, rz) = (Mat3::from_rotation_x(x), Mat3::from_rotation_y(y), Mat3::from_rotation_z(z));
        let expected = [
            (EulerOrder::Xyz, rz * ry * rx),
            (EulerOrder::Xzy, ry * rz * rx),
            (EulerOrder::Yxz, rz * rx * ry),
            (EulerOrder::Yzx, rx * rz * ry),
            (EulerOrder::Zxy, ry * rx * rz),
            (EulerOrder::Zyx, rx * ry * rz),
        ];
        let v = Vec3::new(1.0, 2.0, 3.0);
        for (order, m) in expected {
            let got = euler_to_mat3(Vec3::new(x, y, z), order);
            assert!(close(got * v, m * v), "{order:?}");
        }
    }

    #[test]
    fn euler_roundtrip_all_orders() {
        for order in ORDERS {
            for angles in [
                Vec3::new(0.3, -0.7, 1.1),
                Vec3::new(-1.2, 0.4, -0.5),
                Vec3::new(0.0, 0.0, 0.0),
            ] {
                let q = euler_to_quat(angles, order);
                let back = quat_to_euler(q, order);
                assert!(close(back, angles), "{order:?}: {angles} -> {back}");
            }
        }
    }

    #[test]
    fn single_axis_rotation() {
        let q = euler_to_quat(Vec3::new(0.0, 0.0, std::f32::consts::FRAC_PI_2), EulerOrder::Xyz);
        assert!(close(q * Vec3::X, Vec3::Y));
    }

    #[test]
    fn axis_angle_roundtrip() {
        let axis = Vec3::new(1.0, 2.0, -0.5).normalize();
        let q = axis_angle_to_quat(axis * 5.0, 1.2);
        let (a, ang) = quat_to_axis_angle(q);
        assert!(close(a, axis));
        assert!((ang - 1.2).abs() < 1e-4);
        assert_eq!(axis_angle_to_quat(Vec3::ZERO, 1.0), Quat::IDENTITY);
        assert_eq!(quat_to_axis_angle(Quat::IDENTITY), (Vec3::Z, 0.0));
    }

    #[test]
    fn rotation_between_vectors() {
        let q = rotation_between_vecs_to_quat(Vec3::X, Vec3::new(0.0, 3.0, 0.0));
        assert!(close(q * Vec3::X, Vec3::Y));
        // Opposite vectors still give a valid half-turn.
        let q = rotation_between_vecs_to_quat(Vec3::X, -Vec3::X);
        assert!(close(q * Vec3::X, -Vec3::X));
        assert_eq!(rotation_between_vecs_to_quat(Vec3::ZERO, Vec3::X), Quat::IDENTITY);
    }

    #[test]
    fn interpolation_endpoints_and_midpoint() {
        let a = Quat::IDENTITY;
        let b = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);
        assert!(close(quat_interp(a, b, 0.0) * Vec3::X, Vec3::X));
        assert!(close(quat_interp(a, b, 1.0) * Vec3::X, Vec3::Y));
        let mid = quat_interp(a, b, 0.5) * Vec3::X;
        assert!(close(mid, Vec3::new(0.5f32.sqrt(), 0.5f32.sqrt(), 0.0)));
    }
}
