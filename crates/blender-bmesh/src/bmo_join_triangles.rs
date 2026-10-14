//! Auto-transpiled C/C++ header module: bmo_join_triangles

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct JoinEdgesState {
    pub edge_queue: *mut Heap,
    pub edge_queue_nodes: *mut *mut HeapNode,
    pub use_topo_influence: bool,
    pub topo_influnce: f32,
    pub select_tris_only: bool,
    pub debug_bm: *mut BMesh,
    pub debug_merge_limit: i32,
    pub debug_merge_count: i32,
    pub debug_neighbor: i32,
    pub debug_neighbor_global_count: i32,
    pub debug_this_step: bool,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DelimitData_CD {
    pub cd_type: i32,
    pub cd_size: i32,
    pub cd_offset: i32,
    pub cd_offset_end: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DelimitData {
    pub do_seam: u32,
    pub do_sharp: u32,
    pub do_mat: u32,
    pub do_angle_face: u32,
    pub do_angle_shape: u32,
    pub angle_face: f32,
    pub angle_face__cos: f32,
    pub angle_shape: f32,
    pub cdata: [DelimitData_CD; 4],
    pub cdata_len: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct JoinEdgesNeighborItem {
    pub e: *mut BMEdge,
    pub l: *mut BMLoop,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct JoinEdgesNeighborInfo {
    pub items: [JoinEdgesNeighborItem; 8],
    pub items_num: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct to {
    pub _opaque: [u8; 0],
}
