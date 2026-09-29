//! Auto-transpiled C/C++ header module: BKE_mesh_iterators

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Mesh {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshForeachFlag {
    MESH_FOREACH_NOP = 0,
    MESH_FOREACH_USE_NORMAL = (1 << 0),
}

impl Default for MeshForeachFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
