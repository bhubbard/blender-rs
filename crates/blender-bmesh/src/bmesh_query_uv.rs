//! Auto-transpiled C/C++ header module: bmesh_query_uv

use crate::bmesh_class::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BMUVOffsets {
    pub uv: i32,
    pub pin: i32,
}

pub const BMUVOFFSETS_NONE: BMUVOffsets = BMUVOffsets { uv: -1, pin: -1 };

extern "C" {
    pub fn BM_uv_map_offsets_from_layer(bm: *const BMesh, layer: i32) -> BMUVOffsets;
    pub fn BM_uv_map_offsets_get(bm: *const BMesh) -> BMUVOffsets;

    pub fn BM_loop_uv_calc_edge_length_squared(l: *const BMLoop, cd_loop_uv_offset: i32) -> f32;
    pub fn BM_loop_uv_calc_edge_length(l: *const BMLoop, cd_loop_uv_offset: i32) -> f32;

    pub fn BM_face_uv_calc_center_median_weighted(
        f: *const BMFace,
        aspect: *const [f32; 2],
        cd_loop_uv_offset: i32,
        r_cent: *mut [f32; 2],
    );
    pub fn BM_face_uv_calc_center_median(
        f: *const BMFace,
        cd_loop_uv_offset: i32,
        r_cent: *mut [f32; 2],
    );

    pub fn BM_face_uv_calc_cross(f: *const BMFace, cd_loop_uv_offset: i32) -> f32;
    pub fn BM_face_uv_minmax(
        f: *const BMFace,
        min: *mut [f32; 2],
        max: *mut [f32; 2],
        cd_loop_uv_offset: i32,
    );

    pub fn BM_loop_uv_share_edge_check_with_limit(
        l_a: *const BMLoop,
        l_b: *const BMLoop,
        limit: *const [f32; 2],
        cd_loop_uv_offset: i32,
    ) -> bool;

    pub fn BM_loop_uv_share_edge_check(
        l_a: *const BMLoop,
        l_b: *const BMLoop,
        cd_loop_uv_offset: i32,
    ) -> bool;

    pub fn BM_edge_uv_share_vert_check(
        e: *const BMEdge,
        l_a: *const BMLoop,
        l_b: *const BMLoop,
        cd_loop_uv_offset: i32,
    ) -> bool;

    pub fn BM_loop_uv_share_vert_check(
        l_a: *const BMLoop,
        l_b: *const BMLoop,
        cd_loop_uv_offset: i32,
    ) -> bool;

    pub fn BM_face_uv_point_inside_test(
        f: *const BMFace,
        co: *const [f32; 2],
        cd_loop_uv_offset: i32,
    ) -> bool;
}
