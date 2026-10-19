//! Auto-transpiled C/C++ header module: bmesh_uvselect

use crate::bmesh_class::*;

pub const UV_IS_SELECTED: i32 = 1 << 0;
pub const UV_IS_UNSELECTED: i32 = 1 << 1;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct UVSelectValidateInfo_Sync {
    pub count_uv_vert_any_selected_with_vert_unselected: i32,
    pub count_uv_vert_none_selected_with_vert_selected: i32,
    pub count_uv_edge_any_selected_with_edge_unselected: i32,
    pub count_uv_edge_none_selected_with_edge_selected: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct UVSelectValidateInfo_Flush {
    pub count_uv_edge_selected_with_any_verts_unselected: i32,
    pub count_uv_edge_unselected_with_all_verts_selected: i32,
    pub count_uv_face_selected_with_any_verts_unselected: i32,
    pub count_uv_face_unselected_with_all_verts_selected: i32,
    pub count_uv_face_selected_with_any_edges_unselected: i32,
    pub count_uv_face_unselected_with_all_edges_selected: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct UVSelectValidateInfo_Contiguous {
    pub count_uv_vert_non_contiguous_selected: i32,
    pub count_uv_edge_non_contiguous_selected: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct UVSelectValidateInfo_FlushAndContiguous {
    pub count_uv_vert_isolated_in_edge_or_face_mode: i32,
    pub count_uv_vert_isolated_in_face_mode: i32,
    pub count_uv_edge_isolated_in_face_mode: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct UVSelectValidateInfo {
    pub sync: UVSelectValidateInfo_Sync,
    pub flush: UVSelectValidateInfo_Flush,
    pub contiguous: UVSelectValidateInfo_Contiguous,
    pub flush_contiguous: UVSelectValidateInfo_FlushAndContiguous,
}

extern "C" {
    pub fn BM_face_uvselect_test(f: *const BMFace) -> bool;
    pub fn BM_loop_vert_uvselect_test(l: *const BMLoop) -> bool;
    pub fn BM_loop_edge_uvselect_test(l: *const BMLoop) -> bool;

    pub fn BM_loop_vert_uvselect_check_other_loop_vert(l: *mut BMLoop, hflag: i8, cd_loop_uv_offset: i32) -> bool;
    pub fn BM_loop_vert_uvselect_check_other_loop_edge(l: *mut BMLoop, hflag: i8, cd_loop_uv_offset: i32) -> bool;
    pub fn BM_loop_vert_uvselect_check_other_edge(l: *mut BMLoop, hflag: i8, cd_loop_uv_offset: i32) -> bool;
    pub fn BM_loop_vert_uvselect_check_other_face(l: *mut BMLoop, hflag: i8, cd_loop_uv_offset: i32) -> bool;
    pub fn BM_loop_edge_uvselect_check_other_loop_edge(l: *mut BMLoop, hflag: i8, cd_loop_uv_offset: i32) -> bool;
    pub fn BM_loop_edge_uvselect_check_other_face(l: *mut BMLoop, hflag: i8, cd_loop_uv_offset: i32) -> bool;

    pub fn BM_face_uvselect_check_edges_all(f: *mut BMFace) -> bool;

    pub fn BM_face_uvselect_set_noflush(bm: *mut BMesh, f: *mut BMFace, select: bool);
    pub fn BM_face_uvselect_set(bm: *mut BMesh, f: *mut BMFace, select: bool);
    pub fn BM_loop_edge_uvselect_set_noflush(bm: *mut BMesh, l: *mut BMLoop, select: bool);
    pub fn BM_loop_edge_uvselect_set(bm: *mut BMesh, l: *mut BMLoop, select: bool);
    pub fn BM_loop_vert_uvselect_set_noflush(bm: *mut BMesh, l: *mut BMLoop, select: bool);

    pub fn BM_mesh_uvselect_clear(bm: *mut BMesh) -> bool;

    pub fn BM_loop_vert_uvselect_set_shared(bm: *mut BMesh, l: *mut BMLoop, select: bool, cd_loop_uv_offset: i32);
    pub fn BM_loop_edge_uvselect_set_shared(bm: *mut BMesh, l: *mut BMLoop, select: bool, cd_loop_uv_offset: i32);
    pub fn BM_face_uvselect_set_shared(bm: *mut BMesh, f: *mut BMFace, select: bool, cd_loop_uv_offset: i32);

    pub fn BM_vert_uvselect_set_pick(bm: *mut BMesh, l: *mut BMLoop, select: bool, cd_loop_uv_offset: i32);
    pub fn BM_edge_uvselect_set_pick(bm: *mut BMesh, l: *mut BMLoop, select: bool, cd_loop_uv_offset: i32);
    pub fn BM_face_uvselect_set_pick(bm: *mut BMesh, f: *mut BMFace, select: bool, cd_loop_uv_offset: i32);

    pub fn BM_mesh_uvselect_flush_from_loop_verts_only_select(bm: *mut BMesh);
    pub fn BM_mesh_uvselect_flush_from_loop_verts_only_deselect(bm: *mut BMesh);
    pub fn BM_mesh_uvselect_flush_from_loop_edges_only_select(bm: *mut BMesh);
    pub fn BM_mesh_uvselect_flush_from_loop_edges_only_deselect(bm: *mut BMesh);
    pub fn BM_mesh_uvselect_flush_from_faces_only_select(bm: *mut BMesh);
    pub fn BM_mesh_uvselect_flush_from_faces_only_deselect(bm: *mut BMesh);

    pub fn BM_mesh_uvselect_flush_shared_only_select(bm: *mut BMesh, cd_loop_uv_offset: i32);
    pub fn BM_mesh_uvselect_flush_shared_only_deselect(bm: *mut BMesh, cd_loop_uv_offset: i32);

    pub fn BM_mesh_uvselect_flush_from_loop_verts(bm: *mut BMesh);
    pub fn BM_mesh_uvselect_flush_from_loop_edges(bm: *mut BMesh, flush_down: bool);
    pub fn BM_mesh_uvselect_flush_from_faces(bm: *mut BMesh, flush_down: bool);
    pub fn BM_mesh_uvselect_flush_from_verts(bm: *mut BMesh, select: bool);

    pub fn BM_mesh_uvselect_mode_flush_ex(bm: *mut BMesh, selectmode: i16, flush_down: bool);
    pub fn BM_mesh_uvselect_mode_flush(bm: *mut BMesh);
    pub fn BM_mesh_uvselect_mode_flush_only_select(bm: *mut BMesh);

    pub fn BM_mesh_uvselect_flush_post_subdivide(bm: *mut BMesh, cd_loop_uv_offset: i32);
    pub fn BM_mesh_uvselect_sync_from_mesh_sticky_location(bm: *mut BMesh, cd_loop_uv_offset: i32);
    pub fn BM_mesh_uvselect_sync_from_mesh_sticky_disabled(bm: *mut BMesh);
    pub fn BM_mesh_uvselect_sync_from_mesh_sticky_vert(bm: *mut BMesh);
    pub fn BM_mesh_uvselect_sync_to_mesh(bm: *mut BMesh);

    pub fn BM_mesh_uvselect_is_valid(
        bm: *mut BMesh,
        cd_loop_uv_offset: i32,
        check_sync: bool,
        check_flush: bool,
        check_contiguous: bool,
        info: *mut UVSelectValidateInfo,
    ) -> bool;
}
