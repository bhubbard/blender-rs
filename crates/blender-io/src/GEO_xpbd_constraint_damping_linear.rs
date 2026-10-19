//! Auto-transpiled C/C++ header module: GEO_xpbd_constraint_damping_linear
//! Source: upstream/source/blender/geometry/xpbd/GEO_xpbd_constraint_damping_linear.hh

use blender_math::Vec3;
use crate::GEO_xpbd_constraint_set_params::ConstraintSetParams;
use crate::GEO_xpbd_updater_velocity::VelocityUpdater;
use crate::GEO_xpbd_constraint_set_templated::TemplatedVelocityConstraintSet;

#[derive(Debug, Clone)]
pub struct LinearDampingConstraintSet {
    pub geo_i: usize,
    pub points: Vec<usize>,
    pub linear_dampings: Vec<f32>,
    pub lambdas: Vec<f32>,
}

impl LinearDampingConstraintSet {
    pub const DEBUG_NAME: &'static str = "Linear Damping";

    pub fn new(geo_i: usize, points: Vec<usize>, linear_dampings: Vec<f32>) -> Self {
        let count = points.len();
        Self {
            geo_i,
            points,
            linear_dampings,
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
        let velocity = params.velocity(self.geo_i, point_i);
        let damping = self.linear_dampings[constraint_i];
        let damping_factor = (params.delta_time * damping).clamp(0.0, 1.0);
        let length = (velocity.x * velocity.x + velocity.y * velocity.y + velocity.z * velocity.z).sqrt();
        let gradient = if length > 1e-12 {
            velocity * (1.0 / length)
        } else {
            Vec3::ZERO
        };
        let delta_lambda = -length * damping_factor - self.lambdas[constraint_i];
        let offset = gradient * delta_lambda;
        self.lambdas[constraint_i] += delta_lambda;
        updater.update_velocity(self.geo_i, point_i, offset);
    }
}

impl TemplatedVelocityConstraintSet for LinearDampingConstraintSet {
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
