//! Auto-transpiled C/C++ header module: bmesh_decimate

use crate::bmesh_class::{BMesh, BMVert, BMEdge};
use crate::bmesh_operator_api::BMO_Delimit;

extern "C" {
    pub fn BM_mesh_decimate_collapse(
        bm: *mut BMesh,
        factor: f32,
        vweights: *mut f32,
        vweight_factor: f32,
        do_triangulate: bool,
        symmetry_axis: i32,
        symmetry_eps: f32,
    );

    pub fn BM_mesh_decimate_unsubdivide_ex(bm: *mut BMesh, iterations: i32, tag_only: bool);
    pub fn BM_mesh_decimate_unsubdivide(bm: *mut BMesh, iterations: i32);

    pub fn BM_mesh_decimate_dissolve_ex(
        bm: *mut BMesh,
        angle_limit: f32,
        do_dissolve_boundaries: bool,
        delimit: BMO_Delimit,
        vinput_arr: *mut *mut BMVert,
        vinput_len: i32,
        einput_arr: *mut *mut BMEdge,
        einput_len: i32,
        oflag_out: i16,
    );

    pub fn BM_mesh_decimate_dissolve(
        bm: *mut BMesh,
        angle_limit: f32,
        do_dissolve_boundaries: bool,
        delimit: BMO_Delimit,
    );
}
