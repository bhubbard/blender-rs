//! Auto-transpiled C/C++ header module: GEO_smooth_curves
//! Source: upstream/source/blender/geometry/GEO_smooth_curves.hh

use blender_dna::DNA_curves_types::CurvesGeometry;
use blender_math::cpp_types::IndexMask;

extern "C" {
    pub fn gaussian_blur_1D(
        src: *const core::ffi::c_void,
        iterations: i32,
        influence_by_point: *const f32,
        influence_len: usize,
        smooth_ends: bool,
        keep_shape: bool,
        is_cyclic: bool,
        dst: *mut core::ffi::c_void,
    );

    pub fn smooth_curve_positions(
        curves: *mut CurvesGeometry,
        curves_to_smooth: *const IndexMask,
        point_selection: *const bool,
        point_selection_len: usize,
        iterations: i32,
        influence: f32,
        smooth_ends: bool,
        keep_shape: bool,
    );
}
