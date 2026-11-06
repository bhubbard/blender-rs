//! Blender Upstream 3D Lattice Deformation Verification Suite
//! Ported directly from `source/blender/blenkernel/intern/lattice_deform_test.cc`.
//!
//! Validates:
//! 1. Undeformed identity lattice invariant: P' == P.
//! 2. Trilinear tensor-product volume interpolation across (u, v, w) grid cells.
//! 3. Local support property: Displacing a single control point restricts deformation to local cell neighborhood.
//! 4. Uniform scaling deformation.
//! 5. Bounding box normalization and clamping.
//! 6. Vertex weight group masking and linear falloff blending.

use blender_math::Vec3;

#[derive(Debug, Clone)]
pub struct Lattice3D {
    pub res_u: usize,
    pub res_v: usize,
    pub res_w: usize,
    pub min_bound: Vec3,
    pub max_bound: Vec3,
    pub points: Vec<Vec3>, // (res_u * res_v * res_w) deformed control points
}

impl Lattice3D {
    pub fn new(res_u: usize, res_v: usize, res_w: usize, min_bound: Vec3, max_bound: Vec3) -> Self {
        assert!(res_u >= 2 && res_v >= 2 && res_w >= 2);
        let mut points = Vec::with_capacity(res_u * res_v * res_w);

        for w in 0..res_w {
            let fw = w as f32 / (res_w - 1) as f32;
            let z = min_bound.z + (max_bound.z - min_bound.z) * fw;
            for v in 0..res_v {
                let fv = v as f32 / (res_v - 1) as f32;
                let y = min_bound.y + (max_bound.y - min_bound.y) * fv;
                for u in 0..res_u {
                    let fu = u as f32 / (res_u - 1) as f32;
                    let x = min_bound.x + (max_bound.x - min_bound.x) * fu;
                    points.push(Vec3::new(x, y, z));
                }
            }
        }

        Self {
            res_u,
            res_v,
            res_w,
            min_bound,
            max_bound,
            points,
        }
    }

    pub fn point_idx(&self, u: usize, v: usize, w: usize) -> usize {
        (w * self.res_v + v) * self.res_u + u
    }

    /// Evaluates trilinear lattice deformation for a point in world space
    pub fn deform_point(&self, pt: Vec3, weight: f32) -> Vec3 {
        let size = self.max_bound - self.min_bound;
        if size.x <= 1e-6 || size.y <= 1e-6 || size.z <= 1e-6 {
            return pt;
        }

        // Map pt to normalized [0, 1] lattice space
        let u_norm = ((pt.x - self.min_bound.x) / size.x).clamp(0.0, 1.0);
        let v_norm = ((pt.y - self.min_bound.y) / size.y).clamp(0.0, 1.0);
        let w_norm = ((pt.z - self.min_bound.z) / size.z).clamp(0.0, 1.0);

        // Find cell indices and local cell fractions [0, 1]
        let u_scaled = u_norm * (self.res_u - 1) as f32;
        let v_scaled = v_norm * (self.res_v - 1) as f32;
        let w_scaled = w_norm * (self.res_w - 1) as f32;

        let u0 = (u_scaled.floor() as usize).min(self.res_u - 2);
        let v0 = (v_scaled.floor() as usize).min(self.res_v - 2);
        let w0 = (w_scaled.floor() as usize).min(self.res_w - 2);

        let fu = u_scaled - u0 as f32;
        let fv = v_scaled - v0 as f32;
        let fw = w_scaled - w0 as f32;

        // Trilinear interpolation of 8 cell corners
        let c000 = self.points[self.point_idx(u0, v0, w0)];
        let c100 = self.points[self.point_idx(u0 + 1, v0, w0)];
        let c010 = self.points[self.point_idx(u0, v0 + 1, w0)];
        let c110 = self.points[self.point_idx(u0 + 1, v0 + 1, w0)];

        let c001 = self.points[self.point_idx(u0, v0, w0 + 1)];
        let c101 = self.points[self.point_idx(u0 + 1, v0, w0 + 1)];
        let c011 = self.points[self.point_idx(u0, v0 + 1, w0 + 1)];
        let c111 = self.points[self.point_idx(u0 + 1, v0 + 1, w0 + 1)];

        let c00 = c000.lerp(c100, fu);
        let c10 = c010.lerp(c110, fu);
        let c01 = c001.lerp(c101, fu);
        let c11 = c011.lerp(c111, fu);

        let c0 = c00.lerp(c10, fv);
        let c1 = c01.lerp(c11, fv);

        let deformed = c0.lerp(c1, fw);

        pt.lerp(deformed, weight.clamp(0.0, 1.0))
    }
}

#[test]
fn test_lattice_identity_preserves_coords() {
    let lattice = Lattice3D::new(3, 3, 3, Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));

    let test_points = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.5, -0.3, 0.7),
        Vec3::new(-0.8, 0.2, -0.5),
        Vec3::new(1.0, 1.0, 1.0),
    ];

    for &pt in &test_points {
        let deformed = lattice.deform_point(pt, 1.0);
        assert!((deformed.x - pt.x).abs() < 1e-5);
        assert!((deformed.y - pt.y).abs() < 1e-5);
        assert!((deformed.z - pt.z).abs() < 1e-5);
    }
}

#[test]
fn test_lattice_uniform_scaling() {
    let mut lattice = Lattice3D::new(2, 2, 2, Vec3::ZERO, Vec3::ONE);

    // Scale all control points by 2.0
    for p in &mut lattice.points {
        *p *= 2.0;
    }

    let pt = Vec3::new(0.5, 0.5, 0.5);
    let deformed = lattice.deform_point(pt, 1.0);

    // (0.5, 0.5, 0.5) scaled by 2 -> (1.0, 1.0, 1.0)
    assert!((deformed.x - 1.0).abs() < 1e-5);
    assert!((deformed.y - 1.0).abs() < 1e-5);
    assert!((deformed.z - 1.0).abs() < 1e-5);
}

#[test]
fn test_lattice_trilinear_interpolation_midpoint() {
    let mut lattice = Lattice3D::new(2, 2, 2, Vec3::ZERO, Vec3::ONE);

    // Displace top-right-front corner (1, 1, 1) along Z by +4.0
    let corner_idx = lattice.point_idx(1, 1, 1);
    lattice.points[corner_idx] += Vec3::new(0.0, 0.0, 4.0);

    // Midpoint (0.5, 0.5, 0.5) has trilinear weight 0.5 * 0.5 * 0.5 = 0.125 from this corner
    let pt = Vec3::new(0.5, 0.5, 0.5);
    let deformed = lattice.deform_point(pt, 1.0);

    assert!((deformed.z - (0.5 + 4.0 * 0.125)).abs() < 1e-5);
}

#[test]
fn test_lattice_local_control_point_displacement() {
    let mut lattice = Lattice3D::new(4, 4, 4, Vec3::ZERO, Vec3::splat(3.0));

    // Displace corner (0, 0, 0)
    let corner_idx = lattice.point_idx(0, 0, 0);
    lattice.points[corner_idx] += Vec3::new(10.0, 0.0, 0.0);

    // Point in distant cell (2.5, 2.5, 2.5) must NOT be affected by (0, 0, 0)
    let distant_pt = Vec3::new(2.5, 2.5, 2.5);
    let deformed = lattice.deform_point(distant_pt, 1.0);
    assert_eq!(deformed, distant_pt);
}

#[test]
fn test_lattice_outside_bounds_clamping() {
    let lattice = Lattice3D::new(2, 2, 2, Vec3::ZERO, Vec3::ONE);

    // Point far outside [0, 1] bounds
    let outside_pt = Vec3::new(5.0, 0.5, 0.5);
    let deformed = lattice.deform_point(outside_pt, 1.0);

    // X was clamped to 1.0
    assert_eq!(deformed, Vec3::new(1.0, 0.5, 0.5));
}

#[test]
fn test_lattice_vertex_weight_masking() {
    let mut lattice = Lattice3D::new(2, 2, 2, Vec3::ZERO, Vec3::ONE);
    for p in &mut lattice.points {
        *p += Vec3::new(0.0, 0.0, 10.0);
    }

    let pt = Vec3::new(0.5, 0.5, 0.5);

    // Weight = 0.0: no deformation
    assert_eq!(lattice.deform_point(pt, 0.0), pt);

    // Weight = 0.5: half deformation (Z offset = +5.0)
    let def_half = lattice.deform_point(pt, 0.5);
    assert_eq!(def_half, Vec3::new(0.5, 0.5, 5.5));

    // Weight = 1.0: full deformation (Z offset = +10.0)
    let def_full = lattice.deform_point(pt, 1.0);
    assert_eq!(def_full, Vec3::new(0.5, 0.5, 10.5));
}
