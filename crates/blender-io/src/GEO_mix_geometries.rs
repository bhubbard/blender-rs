//! Auto-transpiled C/C++ header module: GEO_mix_geometries
//! Source: upstream/source/blender/geometry/GEO_mix_geometries.hh

use crate::ABC_alembic::GeometrySet;

extern "C" {
    pub fn mix_geometries(a: *mut GeometrySet, b: *const GeometrySet, factor: f32);
    pub fn mix_bundles(a: *mut core::ffi::c_void, b: *const core::ffi::c_void, factor: f32);
    pub fn mix_socket_values(a: *mut core::ffi::c_void, b: *const core::ffi::c_void, factor: f32);
}
