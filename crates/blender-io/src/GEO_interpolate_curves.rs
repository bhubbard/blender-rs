//! Auto-transpiled C/C++ header module: GEO_interpolate_curves

use core::ffi::c_void;
use blender_dna::DNA_curves_types::CurvesGeometry;
use blender_math::cpp_types::IndexMask;

extern "C" {
    pub fn sample_curve_padded(
        positions: *const [f32; 3],
        positions_len: usize,
        cyclic: bool,
        r_indices: *mut i32,
        r_factors: *mut f32,
    );

    pub fn sample_curve_padded_geom(
        curves: *const CurvesGeometry,
        curve_index: i32,
        cyclic: bool,
        reverse: bool,
        r_indices: *mut i32,
        r_factors: *mut f32,
    );

    pub fn interpolate_curves(
        from_curves: *const CurvesGeometry,
        to_curves: *const CurvesGeometry,
        from_curve_indices: *const i32,
        from_curve_indices_len: usize,
        to_curve_indices: *const i32,
        to_curve_indices_len: usize,
        dst_curve_mask: *const IndexMask,
        dst_curve_flip_direction: *const bool,
        mix_factor: f32,
        dst_curves: *mut CurvesGeometry,
        memory: *mut c_void,
    );

    pub fn interpolate_curves_with_samples(
        from_curves: *const CurvesGeometry,
        to_curves: *const CurvesGeometry,
        from_curve_indices: *const i32,
        from_curve_indices_len: usize,
        to_curve_indices: *const i32,
        to_curve_indices_len: usize,
        from_sample_indices: *const i32,
        to_sample_indices: *const i32,
        from_sample_factors: *const f32,
        to_sample_factors: *const f32,
        dst_curve_mask: *const IndexMask,
        mix_factor: f32,
        dst_curves: *mut CurvesGeometry,
        memory: *mut c_void,
    );
}
