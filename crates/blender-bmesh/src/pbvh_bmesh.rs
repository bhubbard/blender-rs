use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EdgeQueue {
    pub heap: *mut HeapSimple,
    pub center: [f32; 3],
    pub center_proj: [f32; 3],
    pub radius_squared: f32,
    pub limit_len_squared: f32,
    pub limit_len: f32,
    pub use_front_face: bool,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EdgeQueueContext {
    pub queue: *mut EdgeQueue,
    pub pool: *mut BLI_mempool,
    pub bm: *mut BMesh,
    pub cd_vert_mask_offset: i32,
    pub cd_vert_node_offset: i32,
    pub cd_face_node_offset: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FastNodeBuildInfo {
    pub totface: i32,
    pub start: i32,
    pub child1: *mut FastNodeBuildInfo,
    pub child2: *mut FastNodeBuildInfo,
}

// Placeholder for HeapSimple
pub struct HeapSimple {
    // Implement heap operations here
}
