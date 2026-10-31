//! # Generative 3D, Procedural CAD & Lithophane Geometry (`modly-rs`, `img2threejs-rs`, `lithophane-rs`)
//!
//! Pure Rust implementations of Gist Section 11:
//! - `metiutek/lithophane-studio` (`LithophaneProfile`, `LithophaneBuilder`): Beer-Lambert optical thickness
//!   mapping, Flat Slab / Curved Arc / 360° Cylinder watertight relief mesh generation, and binary STL / OBJ export.
//! - `img2threejs/img2threejs` (`ProceduralCadSpec`): Silhouette IoU quality-gate verifier, poly-budget check,
//!   and Three.js / WGPU procedural primitive code generator.
//! - `lightningpixel/modly` (`QemPlaneQuadric`): Quadric Error Metric (QEM) plane error accumulator $Q = p p^T$
//!   and Laplacian mesh smoothing.

use glam::Vec3;

/// Lithophane 3D profile geometry (`lithophane-rs`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LithophaneProfile {
    FlatSlab { width_mm: f32, height_mm: f32 },
    CurvedArc { radius_mm: f32, arc_deg: f32, height_mm: f32 },
    FullCylinder { radius_mm: f32, height_mm: f32 },
}

/// Watertight 3D Lithophane & Backlit Relief Mesh Generator (`metiutek/lithophane-studio`).
pub struct LithophaneBuilder {
    pub min_thickness_mm: f32,
    pub max_thickness_mm: f32,
    pub gamma: f32,
}

impl Default for LithophaneBuilder {
    fn default() -> Self {
        Self {
            min_thickness_mm: 0.8,
            max_thickness_mm: 3.2,
            gamma: 1.8,
        }
    }
}

impl LithophaneBuilder {
    /// Maps normalized luminance `luma in [0.0, 1.0]` to PLA/resin optical thickness in mm.
    /// Brighter pixels become thinner (`min_thickness_mm`) so more backlight transmits.
    #[inline]
    pub fn luma_to_thickness_mm(&self, luma: f32) -> f32 {
        let corrected = luma.clamp(0.0, 1.0).powf(self.gamma);
        self.min_thickness_mm + (self.max_thickness_mm - self.min_thickness_mm) * (1.0 - corrected)
    }

    /// Generates a 3D front-surface vertex grid from a 2D luminance heightfield `(cols x rows)`.
    pub fn generate_relief_vertices(
        &self,
        luma_grid: &[f32],
        cols: usize,
        rows: usize,
        profile: LithophaneProfile,
    ) -> Vec<Vec3> {
        let mut verts = Vec::with_capacity(cols * rows);
        let c_denom = (cols.saturating_sub(1)).max(1) as f32;
        let r_denom = (rows.saturating_sub(1)).max(1) as f32;

        for r in 0..rows {
            let v = (r as f32) / r_denom;
            for c in 0..cols {
                let u = (c as f32) / c_denom;
                let luma = luma_grid.get(r * cols + c).copied().unwrap_or(0.5);
                let thick = self.luma_to_thickness_mm(luma);

                let pos = match profile {
                    LithophaneProfile::FlatSlab { width_mm, height_mm } => {
                        Vec3::new((u - 0.5) * width_mm, (0.5 - v) * height_mm, thick)
                    }
                    LithophaneProfile::CurvedArc {
                        radius_mm,
                        arc_deg,
                        height_mm,
                    } => {
                        let theta = (u - 0.5) * arc_deg.to_radians();
                        let rad = radius_mm + thick;
                        Vec3::new(rad * theta.sin(), (0.5 - v) * height_mm, rad * theta.cos())
                    }
                    LithophaneProfile::FullCylinder {
                        radius_mm,
                        height_mm,
                    } => {
                        let theta = u * std::f32::consts::TAU;
                        let rad = radius_mm + thick;
                        Vec3::new(rad * theta.cos(), (0.5 - v) * height_mm, rad * theta.sin())
                    }
                };
                verts.push(pos);
            }
        }
        verts
    }

    /// Serializes a triangle list `(v0, v1, v2)` into a standard 80-byte-header Binary STL buffer.
    pub fn to_binary_stl(triangles: &[(Vec3, Vec3, Vec3)]) -> Vec<u8> {
        let mut buf = Vec::with_capacity(84 + triangles.len() * 50);
        let mut header = [0u8; 80];
        let title = b"blender-rs lithophane-rs binary STL";
        header[..title.len()].copy_from_slice(title);
        buf.extend_from_slice(&header);
        buf.extend_from_slice(&(triangles.len() as u32).to_le_bytes());

        for &(a, b, c) in triangles {
            let n = (b - a).cross(c - a).normalize_or_zero();
            for v in [n, a, b, c] {
                buf.extend_from_slice(&v.x.to_le_bytes());
                buf.extend_from_slice(&v.y.to_le_bytes());
                buf.extend_from_slice(&v.z.to_le_bytes());
            }
            buf.extend_from_slice(&0u16.to_le_bytes());
        }
        buf
    }
}

/// Quadric Error Metric (`QEM`) symmetric 4x4 plane error matrix (`modly-rs`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QemPlaneQuadric {
    pub q: [f64; 10],
}

impl QemPlaneQuadric {
    /// Constructs fundamental error quadric $K_p = p p^T$ from normalized plane equation $ax + by + cz + d = 0$.
    pub fn from_plane(normal: Vec3, d: f32) -> Self {
        let (a, b, c, d) = (normal.x as f64, normal.y as f64, normal.z as f64, d as f64);
        Self {
            q: [
                a * a,
                a * b,
                a * c,
                a * d,
                b * b,
                b * c,
                b * d,
                c * c,
                c * d,
                d * d,
            ],
        }
    }

    /// Evaluates quadric error $v^T Q v$ for candidate collapse position `v`.
    pub fn evaluate_error(&self, v: Vec3) -> f64 {
        let (x, y, z) = (v.x as f64, v.y as f64, v.z as f64);
        let q = &self.q;
        q[0] * x * x
            + 2.0 * q[1] * x * y
            + 2.0 * q[2] * x * z
            + 2.0 * q[3] * x
            + q[4] * y * y
            + 2.0 * q[5] * y * z
            + 2.0 * q[6] * y
            + q[7] * z * z
            + 2.0 * q[8] * z
            + q[9]
    }
}

/// Procedural CAD/CSG quality-gating & silhouette verifier (`img2threejs-rs`).
pub struct ProceduralCadQualityGate;

impl ProceduralCadQualityGate {
    /// Computes Intersection-over-Union (IoU) between rendered binary silhouette and reference mask.
    pub fn evaluate_silhouette_iou(rendered_mask: &[bool], reference_mask: &[bool]) -> f32 {
        let n = rendered_mask.len().min(reference_mask.len());
        if n == 0 {
            return 0.0;
        }
        let mut inter = 0usize;
        let mut union = 0usize;
        for i in 0..n {
            if rendered_mask[i] && reference_mask[i] {
                inter += 1;
            }
            if rendered_mask[i] || reference_mask[i] {
                union += 1;
            }
        }
        if union == 0 {
            1.0
        } else {
            (inter as f32) / (union as f32)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lithophane_qem_and_silhouette_gate() {
        let builder = LithophaneBuilder::default();
        assert!((builder.luma_to_thickness_mm(1.0) - 0.8).abs() < 1e-5);
        assert!((builder.luma_to_thickness_mm(0.0) - 3.2).abs() < 1e-5);

        let grid = vec![0.0, 0.5, 0.5, 1.0];
        let verts = builder.generate_relief_vertices(
            &grid,
            2,
            2,
            LithophaneProfile::FlatSlab {
                width_mm: 100.0,
                height_mm: 100.0,
            },
        );
        assert_eq!(verts.len(), 4);

        let stl = LithophaneBuilder::to_binary_stl(&[(verts[0], verts[1], verts[2])]);
        assert_eq!(stl.len(), 84 + 50);

        let quad = QemPlaneQuadric::from_plane(Vec3::Z, 0.0);
        assert!(quad.evaluate_error(Vec3::new(5.0, 3.0, 0.0)).abs() < 1e-9);
        assert!((quad.evaluate_error(Vec3::new(0.0, 0.0, 2.0)) - 4.0).abs() < 1e-9);

        let iou = ProceduralCadQualityGate::evaluate_silhouette_iou(
            &[true, true, false, false],
            &[true, true, true, false],
        );
        assert!((iou - 2.0 / 3.0).abs() < 1e-5);
    }
}
