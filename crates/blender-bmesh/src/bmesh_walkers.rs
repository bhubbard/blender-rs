//! Auto-transpiled C/C++ header module: bmesh_walkers

use core::ffi::c_void;
use crate::bmesh_class::*;
use crate::bmesh_walkers_private::BMwGenericWalker;
use crate::{ListBaseT, BLI_mempool};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BMWOrder {
    BMW_DEPTH_FIRST = 0,
    BMW_BREADTH_FIRST = 1,
}

pub const BMW_FLAG_NOP: i32 = 0;
pub const BMW_FLAG_TEST_HIDDEN: i32 = 1 << 0;

pub const BMW_DELIMIT_NONE: i32 = 0;
pub const BMW_DELIMIT_EDGE_LOOP_INNER_CORNERS: i32 = 1 << 0;
pub const BMW_DELIMIT_EDGE_LOOP_OUTER_CORNERS: i32 = 1 << 1;
pub const BMW_DELIMIT_EDGE_LOOP_NGONS: i32 = 1 << 2;
pub const BMW_DELIMIT_EDGE_RING_NGONS: i32 = 1 << 3;
pub const BMW_DELIMIT_EDGE_MARK_SEAM: i32 = 1 << 4;
pub const BMW_DELIMIT_EDGE_MARK_SHARP: i32 = 1 << 5;
pub const BMW_DELIMIT_FACE_MARK_MATERIAL: i32 = 1 << 6;

#[repr(C)]
#[derive(Debug)]
pub struct BMWalker {
    pub begin_htype: i8,
    pub begin: Option<unsafe extern "C" fn(walker: *mut BMWalker, start: *mut c_void)>,
    pub step: Option<unsafe extern "C" fn(walker: *mut BMWalker) -> *mut c_void>,
    pub yield_fn: Option<unsafe extern "C" fn(walker: *mut BMWalker) -> *mut c_void>,
    pub structsize: i32,
    pub order: BMWOrder,
    pub valid_mask: i32,
    pub delimit_supported: i32,
    pub layer: i32,
    pub bm: *mut BMesh,
    pub worklist: *mut BLI_mempool,
    pub states: ListBaseT<BMwGenericWalker>,
    pub mask_vert: i16,
    pub mask_edge: i16,
    pub mask_face: i16,
    pub flag: i32,
    pub delimit: i32,
    pub visit_set: *mut c_void,
    pub visit_set_alt: *mut c_void,
    pub depth: i32,
}

pub const BMW_VERT_SHELL: i32 = 0;
pub const BMW_LOOP_SHELL: i32 = 1;
pub const BMW_LOOP_SHELL_WIRE: i32 = 2;
pub const BMW_FACE_SHELL: i32 = 3;
pub const BMW_EDGELOOP: i32 = 4;
pub const BMW_FACELOOP: i32 = 5;
pub const BMW_EDGERING: i32 = 6;
pub const BMW_EDGEBOUNDARY: i32 = 7;
pub const BMW_EDGELOOP_NONMANIFOLD: i32 = 8;
pub const BMW_LOOPDATA_ISLAND: i32 = 9;
pub const BMW_ISLANDBOUND: i32 = 10;
pub const BMW_ISLAND: i32 = 11;
pub const BMW_ISLAND_MANIFOLD: i32 = 12;
pub const BMW_CONNECTED_VERTEX: i32 = 13;
pub const BMW_CUSTOM: i32 = 14;
pub const BMW_MAXWALKERS: i32 = 15;

pub const BMW_NIL_LAY: i32 = 0;

extern "C" {
    pub fn BMW_init(
        walker: *mut BMWalker,
        bm: *mut BMesh,
        r#type: i32,
        mask_vert: i16,
        mask_edge: i16,
        mask_face: i16,
        flag: i32,
        layer: i32,
        delimit: i32,
    );
    pub fn BMW_begin(walker: *mut BMWalker, start: *mut c_void) -> *mut c_void;
    pub fn BMW_step(walker: *mut BMWalker) -> *mut c_void;
    pub fn BMW_end(walker: *mut BMWalker);
    pub fn BMW_current_depth(walker: *mut BMWalker) -> i32;
    pub fn BMW_current_state(walker: *mut BMWalker) -> *mut c_void;
    pub fn BMW_state_add(walker: *mut BMWalker) -> *mut c_void;
    pub fn BMW_state_remove(walker: *mut BMWalker);
    pub fn BMW_walk(walker: *mut BMWalker) -> *mut c_void;
    pub fn BMW_reset(walker: *mut BMWalker);
}
