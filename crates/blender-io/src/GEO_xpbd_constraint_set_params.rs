//! Auto-transpiled C/C++ header module: GEO_xpbd_constraint_set_params
//! Source: upstream/source/blender/geometry/xpbd/GEO_xpbd_constraint_set_params.hh

use blender_math::{Vec3, Quat};
use crate::GEO_xpbd_geometry_ref::GeometryRef;

/// Provides access to the input data that should be considered by a constraint.
#[derive(Debug, Clone)]
pub struct ConstraintSetParams<'a> {
    pub geometry_refs: &'a [GeometryRef],
    pub delta_time: f32,
    pub compliance_term_factor: f32,
    pub dynamic_friction_factor: f32,
}

impl<'a> ConstraintSetParams<'a> {
    pub fn new(geometry_refs: &'a [GeometryRef], delta_time: f32) -> Self {
        let dt_sq = delta_time * delta_time;
        let compliance_term_factor = if dt_sq > 0.0 { 1.0 / dt_sq } else { 0.0 };
        let dynamic_friction_factor = if delta_time > 0.0 { 1.0 / delta_time } else { 0.0 };
        Self {
            geometry_refs,
            delta_time,
            compliance_term_factor,
            dynamic_friction_factor,
        }
    }

    pub fn geometry_refs(&self) -> &'a [GeometryRef] {
        self.geometry_refs
    }

    pub fn position(&self, geo_i: usize, point_i: usize) -> Vec3 {
        self.geometry_refs[geo_i].positions[point_i]
    }

    pub fn rotation(&self, geo_i: usize, point_i: usize) -> Quat {
        self.geometry_refs[geo_i].rotations[point_i]
    }

    pub fn prev_position(&self, geo_i: usize, point_i: usize) -> Vec3 {
        self.geometry_refs[geo_i].prev_positions[point_i]
    }

    pub fn prev_rotation(&self, geo_i: usize, point_i: usize) -> Quat {
        self.geometry_refs[geo_i].prev_rotations[point_i]
    }

    pub fn velocity(&self, geo_i: usize, point_i: usize) -> Vec3 {
        self.geometry_refs[geo_i].velocities[point_i]
    }

    pub fn angular_velocity(&self, geo_i: usize, point_i: usize) -> Vec3 {
        self.geometry_refs[geo_i].angular_velocities[point_i]
    }

    pub fn inv_mass(&self, geo_i: usize, point_i: usize) -> f32 {
        self.geometry_refs[geo_i].inv_masses[point_i]
    }

    pub fn moment_of_inertia(&self, geo_i: usize, point_i: usize) -> Vec3 {
        self.geometry_refs[geo_i].moments_of_inertia[point_i]
    }

    pub fn inv_moment_of_inertia(&self, geo_i: usize, point_i: usize) -> Vec3 {
        self.geometry_refs[geo_i].inv_moments_of_inertia[point_i]
    }
}
