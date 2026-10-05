use crate::*;

pub const SORT_AXIS: usize = 0;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VertOrder {
    pub angle: f32,
    pub v: *mut BMVert,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EdgeGroupIsland {
    pub edge_links: LinkNode,
    pub vert_len: u32,
    pub edge_len: u32,
    pub has_prev_edge: u32,
    pub min: *mut BMVert,
    pub max: *mut BMVert,
    pub min_axis: [f32; 2],
    pub max_axis: [f32; 2],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Edges_VertVert_BVHTreeTest {
    pub dist_orig: f32,
    pub edge_arr: *mut *mut BMEdge,
    pub v_origin: *mut BMVert,
    pub v_other: *mut BMVert,
    pub vert_range: *mut u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Edges_VertRay_BVHTreeTest {
    pub edge_arr: *mut *mut BMEdge,
    pub v_origin: *mut BMVert,
    pub vert_range: *mut u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EdgeGroup_FindConnection_Args {
    pub bvhtree: *mut BVHTree,
    pub edge_arr: *mut *mut BMEdge,
    pub edge_arr_len: u32,
    pub edge_arr_new: *mut *mut BMEdge,
    pub edge_arr_new_len: u32,
    pub vert_range: *mut u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TempVertPair {
    pub next: *mut TempVertPair,
    pub v_temp: *mut BMVert,
    pub v_orig: *mut BMVert,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LinkNode {
    pub next: *mut LinkNode,
    pub prev: *mut LinkNode,
    pub edge: *mut BMEdge,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BVHTree {
    pub root: *mut BMVertex,
    pub leafs: Vec<BMVertex>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMEdge {
    pub v1: *mut BMVertex,
    pub v2: *mut BMVertex,
}
