//! Auto-transpiled C/C++ header module: GEO_foreach_geometry
//! Source: upstream/source/blender/geometry/GEO_foreach_geometry.hh

use core::ffi::c_void;
use crate::ABC_alembic::GeometrySet;

extern "C" {
    pub fn foreach_real_geometry(
        geometry: *mut GeometrySet,
        fn_callback: extern "C" fn(*mut GeometrySet, *mut c_void),
        user_data: *mut c_void,
    );
}
