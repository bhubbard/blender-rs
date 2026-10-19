//! Auto-transpiled C/C++ header module: GEO_xpbd_constraint_coloring_utils

use core::ffi::c_void;
use crate::GEO_xpbd_constraint_coloring::ConstraintColoring;

extern "C" {
    pub fn color_constraints__unary(
        affected_points: *const i32,
        affected_points_len: usize,
        memory: *mut c_void,
    ) -> ConstraintColoring;

    pub fn color_constraints__binary(
        affected_points: *const [i32; 2],
        affected_points_len: usize,
        memory: *mut c_void,
    ) -> ConstraintColoring;

    pub fn color_constraints__all_independent(constraints_num: i32) -> ConstraintColoring;
}
