//! Auto-transpiled C/C++ header module: GEO_xpbd_constraint_friction_face
//! Source: upstream/source/blender/geometry/xpbd/GEO_xpbd_constraint_friction_face.hh

use blender_math::Vec3;
use crate::GEO_xpbd_constraint_set_params::ConstraintSetParams;
use crate::GEO_xpbd_updater_velocity::VelocityUpdater;
use crate::GEO_xpbd_constraint_set_templated::TemplatedVelocityConstraintSet;

#[derive(Debug, Clone)]
pub struct FrictionFaceConstraintSet {
    pub geo_i: usize,
    pub points: Vec<usize>,
    pub separating_axes: Vec<Vec3>,
    pub contact_velocities: Vec<Vec3>,
    pub dynamic_friction_terms: Vec<f32>,
    pub lambdas_normal: Vec<f32>,
    pub lambdas: Vec<f32>,
}

impl FrictionFaceConstraintSet {
    pub const DEBUG_NAME: &'static str = "Friction";

    pub fn new(
        geo_i: usize,
        points: Vec<usize>,
        separating_axes: Vec<Vec3>,
        contact_velocities: Vec<Vec3>,
        dynamic_friction_terms: Vec<f32>,
        lambdas_normal: Vec<f32>,
    ) -> Self {
        let count = points.len();
        Self {
            geo_i,
            points,
            separating_axes,
            contact_velocities,
            dynamic_friction_terms,
            lambdas_normal,
            lambdas: vec![0.0; count],
        }
    }

    pub fn reset_force(&mut self, constraint_i: usize) {
        if constraint_i < self.lambdas.len() {
            self.lambdas[constraint_i] = 0.0;
        }
    }

    pub fn solve_single(&mut self, params: &ConstraintSetParams, updater: &mut VelocityUpdater, constraint_i: usize) {
        let point_i = self.points[constraint_i];
        let inv_m = params.inv_mass(self.geo_i, point_i);
        if inv_m <= 0.0 {
            return;
        }

        let dynamic_friction = self.dynamic_friction_terms[constraint_i] * params.dynamic_friction_factor;
        let lambda_normal = self.lambdas_normal[constraint_i];
        let axis = self.separating_axes[constraint_i];
        let contact_velocity = self.contact_velocities[constraint_i];
        let velocity = params.velocity(self.geo_i, point_i) - contact_velocity;

        let dot = velocity.x * axis.x + velocity.y * axis.y + velocity.z * axis.z;
        let velocity_tangent = velocity - axis * dot;

        let residual = (velocity_tangent.x * velocity_tangent.x + velocity_tangent.y * velocity_tangent.y + velocity_tangent.z * velocity_tangent.z).sqrt();
        let gradient = if residual > 1e-12 {
            velocity_tangent * (1.0 / residual)
        } else {
            Vec3::ZERO
        };

        let delta_lambda = (dynamic_friction * lambda_normal).min(residual / inv_m);
        self.lambdas[constraint_i] += delta_lambda;

        updater.update_velocity(self.geo_i, point_i, gradient * (-inv_m * delta_lambda));
    }
}

impl TemplatedVelocityConstraintSet for FrictionFaceConstraintSet {
    fn constraints_num(&self) -> usize {
        self.points.len()
    }

    fn reset_forces(&mut self) {
        for lambda in &mut self.lambdas {
            *lambda = 0.0;
        }
    }

    fn solve_sequential(&mut self, params: &ConstraintSetParams, updater: &mut VelocityUpdater) {
        for i in 0..self.points.len() {
            self.solve_single(params, updater, i);
        }
    }

    fn debug_name(&self) -> &'static str {
        Self::DEBUG_NAME
    }
}
