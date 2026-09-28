//! Fast vector, matrix, quaternion, and geometric routines mirroring Blender's `BLI_math_*`.

pub type float2 = glam::Vec2;
pub type float3 = glam::Vec3;
pub type float4 = glam::Vec4;
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

/// Axis-aligned 2D bounding box mirroring Blender's `Bounds<float2>`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds2D {
    pub min: Vec2,
    pub max: Vec2,
}

impl Bounds2D {
    #[inline]
    pub const fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    #[inline]
    pub fn from_point(p: Vec2) -> Self {
        Self { min: p, max: p }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.min.x >= self.max.x || self.min.y >= self.max.y
    }

    #[inline]
    pub fn center(&self) -> Vec2 {
        (self.min + self.max) * 0.5
    }

    #[inline]
    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    #[inline]
    pub fn translate(&mut self, offset: Vec2) {
        self.min += offset;
        self.max += offset;
    }

    #[inline]
    pub fn recenter(&mut self, new_center: Vec2) {
        let half = self.size() * 0.5;
        self.min = new_center - half;
        self.max = new_center + half;
    }

    #[inline]
    pub fn scale_from_center(&mut self, scale: Vec2) {
        let c = self.center();
        let half = (self.size() * scale) * 0.5;
        self.min = c - half;
        self.max = c + half;
    }

    #[inline]
    pub fn pad(&mut self, offset: Vec2) {
        self.min -= offset;
        self.max += offset;
    }
}

/// Axis-aligned 3D bounding box mirroring Blender's `Bounds<float3>`.
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
    pub const fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    #[inline]
    pub fn from_point(p: Vec3) -> Self {
        Self { min: p, max: p }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.min.x >= self.max.x || self.min.y >= self.max.y || self.min.z >= self.max.z
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

    #[inline]
    pub fn translate(&mut self, offset: Vec3) {
        self.min += offset;
        self.max += offset;
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
    #[test]
    fn test_upstream_bounds_center_and_size() {
        let b1 = Bounds2D::from_point(Vec2::ZERO);
        assert_eq!(b1.center(), Vec2::ZERO);
        assert_eq!(b1.size(), Vec2::ZERO);

        let b3 = Bounds2D::new(Vec2::new(-3.0, -5.0), Vec2::new(2.0, 4.0));
        assert_eq!(b3.center(), Vec2::new(-0.5, -0.5));
        assert_eq!(b3.size(), Vec2::new(5.0, 9.0));
    }

    #[test]
    fn test_upstream_bounds_translate_and_recenter() {
        let mut b = Bounds2D::new(Vec2::new(-3.0, -5.0), Vec2::new(2.0, 4.0));
        b.translate(Vec2::new(2.0, 2.0));
        assert_eq!(b.min, Vec2::new(-1.0, -3.0));
        assert_eq!(b.max, Vec2::new(4.0, 6.0));

        b.recenter(Vec2::new(2.0, 3.0));
        assert_eq!(b.center(), Vec2::new(2.0, 3.0));
    }
}





pub mod BLI_generic_vector_array;









pub mod eevee_material_shared;





pub mod VecMat;

pub mod BPy_FrsMaterial;





















pub mod GPU_matrix;



pub mod gpu_shader_math_angle_bsl;

pub mod gpu_shader_math_axis_angle_bsl;

pub mod gpu_shader_math_euler_bsl;



















pub mod GEO_transform;





