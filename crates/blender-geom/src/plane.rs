//! Plane helpers (`plane_from_point_normal_v3`, `dist_signed_to_plane_v3`,
//! `isect_line_plane_v3`).

use glam::{Vec3, Vec4};

/// Plane `(nx, ny, nz, d)` with `n·x + d = 0`, built from a point and (unit) normal.
pub fn plane_from_point_normal_v3(co: Vec3, no: Vec3) -> Vec4 {
    no.extend(-no.dot(co))
}

/// Signed distance from `p` to `plane` (positive on the normal side, for a unit normal).
pub fn dist_signed_to_plane_v3(p: Vec3, plane: Vec4) -> f32 {
    plane.truncate().dot(p) + plane.w
}

/// Intersects the line `l1..l2` with the plane through `plane_co` with normal `plane_no`.
/// Returns the parameter `lambda` (0 at `l1`, 1 at `l2`), or `None` when parallel.
pub fn isect_line_plane_v3(l1: Vec3, l2: Vec3, plane_co: Vec3, plane_no: Vec3) -> Option<f32> {
    let dir = l2 - l1;
    let dot = dir.dot(plane_no);
    if dot.abs() < f32::EPSILON {
        return None;
    }
    Some((plane_co - l1).dot(plane_no) / dot)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plane_distance() {
        let pl = plane_from_point_normal_v3(Vec3::new(0.0, 0.0, 1.0), Vec3::Z);
        assert_eq!(dist_signed_to_plane_v3(Vec3::new(3.0, 2.0, 4.0), pl), 3.0);
        assert_eq!(dist_signed_to_plane_v3(Vec3::new(0.0, 0.0, -1.0), pl), -2.0);
    }

    #[test]
    fn line_plane_intersection() {
        let t = isect_line_plane_v3(Vec3::ZERO, Vec3::new(0.0, 0.0, 4.0), Vec3::Z, Vec3::Z);
        assert_eq!(t, Some(0.25));
        let parallel = isect_line_plane_v3(Vec3::ZERO, Vec3::X, Vec3::Z, Vec3::Z);
        assert_eq!(parallel, None);
    }
}
