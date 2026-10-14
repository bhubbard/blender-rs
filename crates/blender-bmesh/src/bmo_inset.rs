//! Auto-transpiled C/C++ header module: bmo_inset

use core::ffi::c_void;
use crate::*;

pub const ELE_NEW: i32 = 1;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct InterpFace {
    pub f: *mut BMFace,
    pub blocks_l: *mut *mut core::ffi::c_void,
    pub blocks_v: *mut *mut core::ffi::c_void,
    pub axis_mat: [[f32; 3]; 3],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SplitEdgeInfo {
    pub no: [f32; 3],
    pub length: f32,
    pub e_old: *mut BMEdge,
    pub e_new: *mut BMEdge,
    pub l: *mut BMLoop,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VertLengths {
    pub length_accum: f32,
    pub count: i32,
}
