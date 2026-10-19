//! Auto-transpiled C/C++ header module: GEO_xpbd_constraint_damping_angular
//! Source: upstream/source/blender/geometry/xpbd/GEO_xpbd_constraint_damping_angular.hh

use blender_math::Vec3;
use crate::GEO_xpbd_constraint_set_params::ConstraintSetParams;
use crate::GEO_xpbd_updater_velocity::VelocityUpdater;
use crate::GEO_xpbd_constraint_set_templated::TemplatedVelocityConstraintSet;

#[derive(Debug, Clone)]
pub struct AngularDampingConstraintSet {
    pub geo_i: usize,
    pub points: Vec<usize>,
    pub angular_dampings: Vec<f32>,
    pub lambdas: Vec<f32>,
}

impl AngularDampingConstraintSet {
    pub const DEBUG_NAME: &'static str = "Angular Damping";

    pub fn new(geo_i: usize, points: Vec<usize>, angular_dampings: Vec<f32>) -> Self {
        let count = points.len();
        Self {
            geo_i,
            points,
            angular_dampings,
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
        let angular_velocity = params.angular_velocity(self.geo_i, point_i);
        let damping = self.angular_dampings[constraint_i];
        let damping_factor = damping * params.delta_time;
        let length = (angular_velocity.x * angular_velocity.x + angular_velocity.y * angular_velocity.y + angular_velocity.z * angular_velocity.z).sqrt();
        let gradient = if length > 1e-12 {
            angular_velocity * (1.0 / length)
        } else {
            Vec3::ZERO
        };
        let delta_lambda = -length * damping_factor - self.lambdas[constraint_i];
        let offset = gradient * delta_lambda;
        self.lambdas[constraint_i] += delta_lambda;
        updater.update_angular_velocity(self.geo_i, point_i, offset);
    }
}

impl TemplatedVelocityConstraintSet for AngularDampingConstraintSet {
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
