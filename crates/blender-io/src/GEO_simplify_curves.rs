//! Auto-transpiled C/C++ header module: GEO_simplify_curves
//! Source: upstream/source/blender/geometry/GEO_simplify_curves.hh

use blender_math::Vec3;
use blender_math::cpp_types::IndexMask;

extern "C" {
    pub fn simplify_curve_attribute(
        positions: *const Vec3,
        positions_len: usize,
        curves_selection: *const IndexMask,
        points_by_curve: *const usize,
        points_by_curve_len: usize,
        cyclic: *const bool,
        cyclic_len: usize,
        epsilon: f32,
        attribute_data: *const core::ffi::c_void,
        memory: *mut core::ffi::c_void,
    ) -> IndexMask;

    pub fn curve_simplify(
        positions: *const Vec3,
        positions_len: usize,
        cyclic: bool,
        epsilon: f32,
        attribute_data: *const core::ffi::c_void,
        points_to_delete: *mut bool,
    );
}
