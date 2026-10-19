//! Auto-transpiled C/C++ header module: bmesh_mesh_debug

use core::ffi::c_char;
use crate::bmesh_class::BMesh;

extern "C" {
    pub fn BM_mesh_debug_info(bm: *mut BMesh) -> *mut c_char;
    pub fn BM_mesh_debug_print(bm: *mut BMesh);
}
