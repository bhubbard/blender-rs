//! Auto-transpiled C/C++ header module: bmesh_path_region_uv

use core::ffi::c_void;
use crate::bmesh_class::{BMesh, BMElem, BMFace, BMLoop};
use crate::bmesh_path::LinkNode;

extern "C" {
    pub fn BM_mesh_calc_path_uv_region_vert(
        bm: *mut BMesh,
        ele_src: *mut BMElem,
        ele_dst: *mut BMElem,
        cd_loop_uv_offset: i32,
        filter_fn: Option<unsafe extern "C" fn(l: *mut BMLoop, user_data: *mut c_void) -> bool>,
        user_data: *mut c_void,
    ) -> *mut LinkNode;

    pub fn BM_mesh_calc_path_uv_region_edge(
        bm: *mut BMesh,
        ele_src: *mut BMElem,
        ele_dst: *mut BMElem,
        cd_loop_uv_offset: i32,
        filter_fn: Option<unsafe extern "C" fn(l: *mut BMLoop, user_data: *mut c_void) -> bool>,
        user_data: *mut c_void,
    ) -> *mut LinkNode;

    pub fn BM_mesh_calc_path_uv_region_face(
        bm: *mut BMesh,
        ele_src: *mut BMElem,
        ele_dst: *mut BMElem,
        cd_loop_uv_offset: i32,
        filter_fn: Option<unsafe extern "C" fn(f: *mut BMFace, user_data: *mut c_void) -> bool>,
        user_data: *mut c_void,
    ) -> *mut LinkNode;
}
