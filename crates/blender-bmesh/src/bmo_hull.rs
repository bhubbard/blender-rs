//! Auto-transpiled C/C++ header module: bmo_hull

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct HullTriangle {
    pub v: [*mut BMVert; 3],
    pub no: [f32; 3],
    pub skip: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct HullFinalEdges {
    pub ListBaseT<LinkData> *> *edges: Map<BMVert,
    pub base_pool: *mut BLI_mempool,
    pub link_pool: *mut BLI_mempool,
}

pub const HULL_FLAG_INPUT: i32 = (1 << 0);
pub const HULL_FLAG_INTERIOR_ELE: i32 = (1 << 1);
pub const HULL_FLAG_OUTPUT_GEOM: i32 = (1 << 2);
pub const HULL_FLAG_DEL: i32 = (1 << 3);
pub const HULL_FLAG_HOLE: i32 = (1 << 4);
