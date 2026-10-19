//! Auto-transpiled C/C++ header module: GEO_volume_grid_resample
//! Source: upstream/source/blender/geometry/GEO_volume_grid_resample.hh

use blender_math::Mat4;

extern "C" {
    pub fn resample_sdf_grid_if_necessary(
        volume_grid: *mut core::ffi::c_void,
        tree_token: *mut core::ffi::c_void,
        transform: *const Mat4,
        storage: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
}
