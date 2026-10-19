//! Auto-transpiled C/C++ header module: bmesh_structure

use crate::bmesh_class::*;

extern "C" {
    pub fn bmesh_loop_validate(f: *mut BMFace) -> bool;

    pub fn bmesh_disk_edge_append(e: *mut BMEdge, v: *mut BMVert);
    pub fn bmesh_disk_edge_remove(e: *mut BMEdge, v: *mut BMVert);

    pub fn bmesh_disk_facevert_count_at_most(v: *const BMVert, count_max: i32) -> i32;
    pub fn bmesh_disk_facevert_count(v: *const BMVert) -> i32;

    pub fn bmesh_disk_faceedge_find_first(e: *const BMEdge, v: *const BMVert) -> *mut BMEdge;
    pub fn bmesh_disk_faceloop_find_first(e: *const BMEdge, v: *const BMVert) -> *mut BMLoop;
    pub fn bmesh_disk_faceloop_find_first_visible(e: *const BMEdge, v: *const BMVert) -> *mut BMLoop;
    pub fn bmesh_disk_faceedge_find_next(e: *const BMEdge, v: *const BMVert) -> *mut BMEdge;

    pub fn bmesh_radial_loop_append(e: *mut BMEdge, l: *mut BMLoop);
    pub fn bmesh_radial_loop_remove(e: *mut BMEdge, l: *mut BMLoop);
    pub fn bmesh_radial_loop_unlink(l: *mut BMLoop);

    pub fn bmesh_radial_facevert_count_at_most(l: *const BMLoop, v: *const BMVert, count_max: i32) -> i32;
    pub fn bmesh_radial_facevert_count(l: *const BMLoop, v: *const BMVert) -> i32;
    pub fn bmesh_radial_facevert_check(l: *const BMLoop, v: *const BMVert) -> bool;

    pub fn bmesh_radial_faceloop_find_first(l: *const BMLoop, v: *const BMVert) -> *mut BMLoop;
    pub fn bmesh_radial_faceloop_find_next(l: *const BMLoop, v: *const BMVert) -> *mut BMLoop;
    pub fn bmesh_radial_faceloop_find_vert(f: *const BMFace, v: *const BMVert) -> *mut BMLoop;

    pub fn bmesh_radial_validate(radlen: i32, l: *mut BMLoop) -> bool;

    pub fn bmesh_disk_vert_swap(e: *mut BMEdge, v_dst: *mut BMVert, v_src: *mut BMVert);
    pub fn bmesh_edge_vert_swap(e: *mut BMEdge, v_dst: *mut BMVert, v_src: *mut BMVert);
    pub fn bmesh_disk_vert_replace(e: *mut BMEdge, v_dst: *mut BMVert, v_src: *mut BMVert);
    pub fn bmesh_disk_edge_exists(v1: *const BMVert, v2: *const BMVert) -> *mut BMEdge;
    pub fn bmesh_disk_validate(len: i32, e: *mut BMEdge, v: *mut BMVert) -> bool;
}
