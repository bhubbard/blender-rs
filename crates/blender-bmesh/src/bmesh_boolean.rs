//! Auto-transpiled C/C++ header module: bmesh_boolean

use core::ffi::c_void;
use crate::bmesh_class::{BMesh, BMFace, BMLoop};

extern "C" {
    pub fn BM_mesh_boolean(
        bm: *mut BMesh,
        looptris: *const [*mut BMLoop; 3],
        looptris_len: usize,
        test_fn: Option<unsafe extern "C" fn(f: *mut BMFace, user_data: *mut c_void) -> i32>,
        user_data: *mut c_void,
        nshapes: i32,
        use_self: bool,
        keep_hidden: bool,
        hole_tolerant: bool,
        boolean_mode: i32,
    ) -> bool;

    pub fn BM_mesh_boolean_knife(
        bm: *mut BMesh,
        looptris: *const [*mut BMLoop; 3],
        looptris_len: usize,
        test_fn: Option<unsafe extern "C" fn(f: *mut BMFace, user_data: *mut c_void) -> i32>,
        user_data: *mut c_void,
        nshapes: i32,
        use_self: bool,
        use_separate_all: bool,
        hole_tolerant: bool,
        keep_hidden: bool,
    ) -> bool;
}
