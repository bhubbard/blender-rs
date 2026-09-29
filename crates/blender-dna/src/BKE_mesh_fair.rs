//! Auto-transpiled C/C++ header module: BKE_mesh_fair

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Mesh {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMeshFairingDepth {
    MESH_FAIRING_DEPTH_POSITION = 1,
    MESH_FAIRING_DEPTH_TANGENCY = 2,
}

impl Default for eMeshFairingDepth {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
