//! Fast vector, matrix, quaternion, and geometric routines mirroring Blender's `BLI_math_*`.

pub use glam::{vec2, vec3, vec4, Mat3, Mat4, Quat, Vec2, Vec3, Vec4};

/// Tolerance used across Blender's geometric operations.
pub const BLENDER_EPSILON: f32 = 1e-6;

/// Checks if a 3D vector is approximately zero.
#[inline(always)]
pub fn is_zero_v3(v: Vec3) -> bool {
    v.length_squared() < (BLENDER_EPSILON * BLENDER_EPSILON)
}

/// Computes the normal of a triangle formed by 3 vertices.
#[inline]
pub fn normal_tri_v3(v1: Vec3, v2: Vec3, v3: Vec3) -> Option<Vec3> {
    let edge1 = v2 - v1;
    let edge2 = v3 - v1;
    let n = edge1.cross(edge2);
    let len_sq = n.length_squared();
    if len_sq > 0.0 {
        Some(n / len_sq.sqrt())
    } else {
        None
    }
}

/// Computes the area of a 3D triangle.
#[inline]
pub fn area_tri_v3(v1: Vec3, v2: Vec3, v3: Vec3) -> f32 {
    0.5 * (v2 - v1).cross(v3 - v1).length()
}

/// Axis-aligned bounding box.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl BoundingBox {
    pub const EMPTY: Self = Self {
        min: Vec3::splat(f32::INFINITY),
        max: Vec3::splat(f32::NEG_INFINITY),
    };

    #[inline]
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    #[inline]
    pub fn extend(&mut self, point: Vec3) {
        self.min = self.min.min(point);
        self.max = self.max.max(point);
    }

    #[inline]
    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    #[inline]
    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_zero_v3() {
        assert!(is_zero_v3(Vec3::ZERO));
        assert!(is_zero_v3(Vec3::new(1e-7, 1e-7, 1e-7)));
        assert!(!is_zero_v3(Vec3::new(1.0, 0.0, 0.0)));
    }

    #[test]
    fn test_triangle_normal_and_area() {
        let v1 = Vec3::new(0.0, 0.0, 0.0);
        let v2 = Vec3::new(1.0, 0.0, 0.0);
        let v3 = Vec3::new(0.0, 1.0, 0.0);
        let n = normal_tri_v3(v1, v2, v3).unwrap();
        assert!((n - Vec3::Z).length() < 1e-5);
        assert!((area_tri_v3(v1, v2, v3) - 0.5).abs() < 1e-5);
    }
}
