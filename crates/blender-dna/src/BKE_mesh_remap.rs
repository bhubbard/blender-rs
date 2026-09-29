//! Auto-transpiled C/C++ header module: BKE_mesh_remap

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MeshPairRemapItem {
    pub sources_num: i32,
    pub indices_src: *mut i32,
    pub weights_src: *mut f32,
    pub island: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MeshPairRemap {
    pub items_num: i32,
    pub items: *mut MeshPairRemapItem,
    pub mem: *mut MemArena,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MemArena {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Mesh {
    pub _opaque: [u8; 0],
}
