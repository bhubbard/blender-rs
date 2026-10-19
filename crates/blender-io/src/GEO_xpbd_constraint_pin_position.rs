//! Auto-transpiled C/C++ header module: GEO_xpbd_constraint_pin_position
//! Source: upstream/source/blender/geometry/xpbd/GEO_xpbd_constraint_pin_position.hh

use blender_math::Vec3;
use crate::GEO_xpbd_constraint_coloring::ConstraintColoring;
use crate::GEO_xpbd_constraint_coloring_utils::color_constraints__unary;
use crate::GEO_xpbd_constraint_set_templated::TemplatedConstraintSet;

#[derive(Debug, Clone)]
pub struct PinPositionConstraintSet {
    pub geo_i: usize,
    pub point_indices: Vec<i32>,
    pub pin_positions: Vec<Vec3>,
    pub compliances: Vec<f32>,
    pub error_scale: f32,
    pub lambdas: Vec<f32>,
}

impl PinPositionConstraintSet {
    pub const DEBUG_NAME: &'static str = "Pinned Position";

    pub fn new(
        geo_i: usize,
        point_indices: Vec<i32>,
        pin_positions: Vec<Vec3>,
        compliances: Vec<f32>,
        error_scale: f32,
    ) -> Self {
        let count = point_indices.len();
        Self {
            geo_i,
            point_indices,
            pin_positions,
            compliances,
            error_scale,
            lambdas: vec![0.0; count],
        }
    }

    pub fn reset_force(&mut self, constraint_i: usize) {
        if constraint_i < self.lambdas.len() {
            self.lambdas[constraint_i] = 0.0;
        }
    }

    pub fn color_constraints(&self) -> ConstraintColoring {
        unsafe {
            color_constraints__unary(self.point_indices.as_ptr(), self.point_indices.len(), core::ptr::null_mut())
        }
    }
}

impl TemplatedConstraintSet for PinPositionConstraintSet {
    fn constraints_num(&self) -> usize {
        self.point_indices.len()
    }

    fn reset_forces(&mut self) {
        for lambda in &mut self.lambdas {
            *lambda = 0.0;
        }
    }

    fn debug_name(&self) -> &'static str {
        Self::DEBUG_NAME
    }
}
