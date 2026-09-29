//! Auto-transpiled C/C++ header module: bmesh_mesh_convert

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMeshFromMeshParams {
    pub calc_face_normal: bool,
    pub calc_vert_normal: bool,
    pub add_key_index: bool,
    pub use_shapekey: bool,
    pub active_shapekey: i32,
    pub cd_mask_extra: CustomData_MeshMasks,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMeshToMeshParams {
    pub calc_object_remap: bool,
    pub update_shapekey_indices: bool,
    pub active_shapekey_to_mvert: bool,
    pub cd_mask_extra: CustomData_MeshMasks,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CustomData_MeshMasks {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Main {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Mesh {
    pub _opaque: [u8; 0],
}
