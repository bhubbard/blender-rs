//! Auto-transpiled C/C++ header module: GEO_xpbd_constraint_rod_bend_twist
//! Source: upstream/source/blender/geometry/xpbd/GEO_xpbd_constraint_rod_bend_twist.hh

use blender_math::{Vec4, Quat};
use crate::GEO_xpbd_constraint_coloring::ConstraintColoring;
use crate::GEO_xpbd_constraint_coloring_utils::color_constraints__all_independent;
use crate::GEO_xpbd_constraint_set_templated::TemplatedConstraintSet;

/// Aligns rotations of two consecutive rods based on a rest rotation.
#[derive(Debug, Clone)]
pub struct RodBendAndTwistConstraintSet {
    pub geo_i: usize,
    pub curves_range: std::ops::Range<usize>,
    pub points_by_curve: Vec<std::ops::Range<usize>>,
    pub rest_rotations: Vec<Quat>,
    pub compliances: Vec<f32>,
    pub error_scale: f32,
    pub lambdas: Vec<Vec4>,
}

impl RodBendAndTwistConstraintSet {
    pub const DEBUG_NAME: &'static str = "Rod Bend and Twist";

    pub fn new(
        geo_i: usize,
        curves_range: std::ops::Range<usize>,
        points_by_curve: Vec<std::ops::Range<usize>>,
        rest_rotations: Vec<Quat>,
        compliances: Vec<f32>,
        error_scale: f32,
        total_points: usize,
    ) -> Self {
        Self {
            geo_i,
            curves_range,
            points_by_curve,
            rest_rotations,
            compliances,
            error_scale,
            lambdas: vec![Vec4::ZERO; total_points],
        }
    }

    pub fn reset_force(&mut self, constraint_i: usize) {
        if let Some(range) = self.points_by_curve.get(constraint_i) {
            for idx in range.clone() {
                if idx < self.lambdas.len() {
                    self.lambdas[idx] = Vec4::ZERO;
                }
            }
        }
    }

    pub fn color_constraints(&self) -> ConstraintColoring {
        unsafe {
            color_constraints__all_independent(self.curves_range.len() as i32)
        }
    }
}

impl TemplatedConstraintSet for RodBendAndTwistConstraintSet {
    fn constraints_num(&self) -> usize {
        self.curves_range.len()
    }

    fn reset_forces(&mut self) {
        for lambda in &mut self.lambdas {
            *lambda = Vec4::ZERO;
        }
    }

    fn debug_name(&self) -> &'static str {
        Self::DEBUG_NAME
    }
}
