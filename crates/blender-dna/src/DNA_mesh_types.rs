//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[allow(non_camel_case_types)]
type int32_t = i32;
#[allow(non_camel_case_types)]
type uint32_t = u32;
#[allow(non_camel_case_types)]
type int16_t = i16;
#[allow(non_camel_case_types)]
type uint16_t = u16;
#[allow(non_camel_case_types)]
type int64_t = i64;
#[allow(non_camel_case_types)]
type uint64_t = u64;
#[allow(non_camel_case_types)]
type int8_t = i8;
#[allow(non_camel_case_types)]
type uint8_t = u8;
#[allow(non_camel_case_types)]
type uchar = u8;
#[allow(non_camel_case_types)]
type ushort = u16;
#[allow(non_camel_case_types)]
type uint = u32;
#[allow(non_camel_case_types)]
type ulong = u64;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMesh_TexSpaceFlag(pub i8);

impl eMesh_TexSpaceFlag {
    pub const ME_TEXSPACE_FLAG_AUTO: Self = Self((1 << 0) as i8);
    pub const ME_TEXSPACE_FLAG_AUTO_EVALUATED: Self = Self((1 << 1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMesh_EditFlag(pub i8);

impl eMesh_EditFlag {
    pub const ME_EDIT_MIRROR_VERTEX_GROUPS: Self = Self((1 << 0) as i8);
    pub const ME_EDIT_MIRROR_Y: Self = Self((1 << 1) as i8);
    pub const ME_EDIT_MIRROR_Z: Self = Self((1 << 2) as i8);
    pub const ME_EDIT_PAINT_FACE_SEL: Self = Self((1 << 3) as i8);
    pub const ME_EDIT_MIRROR_TOPO: Self = Self((1 << 4) as i8);
    pub const ME_EDIT_PAINT_VERT_SEL: Self = Self((1 << 5) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMesh_Flag(pub u16);

impl eMesh_Flag {
    pub const ME_FLAG_UNUSED_0: Self = Self((1 << 0) as u16);
    pub const ME_FLAG_UNUSED_1: Self = Self((1 << 1) as u16);
    pub const ME_FLAG_DEPRECATED_2: Self = Self((1 << 2) as u16);
    pub const ME_FLAG_UV_SELECT_SYNC_VALID: Self = Self((1 << 3) as u16);
    pub const ME_FLAG_UNUSED_4: Self = Self((1 << 4) as u16);
    pub const ME_AUTOSMOOTH_LEGACY: Self = Self((1 << 5) as u16);
    pub const ME_FLAG_UNUSED_6: Self = Self((1 << 6) as u16);
    pub const ME_FLAG_UNUSED_7: Self = Self((1 << 7) as u16);
    pub const ME_REMESH_REPROJECT_ATTRIBUTES: Self = Self((1 << 8) as u16);
    pub const ME_DS_EXPAND: Self = Self((1 << 9) as u16);
    pub const ME_SCULPT_DYNAMIC_TOPOLOGY: Self = Self((1 << 10) as u16);
    pub const ME_NO_OVERLAPPING_TOPOLOGY: Self = Self((1 << 11) as u16);
    pub const ME_FLAG_UNUSED_8: Self = Self((1 << 12) as u16);
    pub const ME_REMESH_FIX_POLES: Self = Self((1 << 13) as u16);
    pub const ME_REMESH_REPROJECT_VOLUME: Self = Self((1 << 14) as u16);
    pub const ME_FLAG_UNUSED_9: Self = Self((1 << 15) as u16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMesh_CDFlag(pub i8);

impl eMesh_CDFlag {
    pub const ME_CDFLAG_VERT_BWEIGHT: Self = Self((1 << 0) as i8);
    pub const ME_CDFLAG_EDGE_BWEIGHT: Self = Self((1 << 1) as i8);
    pub const ME_CDFLAG_EDGE_CREASE: Self = Self((1 << 2) as i8);
    pub const ME_CDFLAG_VERT_CREASE: Self = Self((1 << 3) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMesh_RemeshMode(pub i8);

impl eMesh_RemeshMode {
    pub const REMESH_VOXEL: Self = Self((0) as i8);
    pub const REMESH_QUAD: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MeshSubdivType(pub i32);

impl MeshSubdivType {
    pub const ME_CC_SUBSURF: Self = Self((0) as i32);
    pub const ME_SIMPLE_SUBSURF: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMeshSymmetryType(pub i8);

impl eMeshSymmetryType {
    pub const ME_SYMMETRY_X: Self = Self((1 << 0) as i8);
    pub const ME_SYMMETRY_Y: Self = Self((1 << 1) as i8);
    pub const ME_SYMMETRY_Z: Self = Self((1 << 2) as i8);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Mesh {
    pub adt: *mut core::ffi::c_void,
    pub key: *mut core::ffi::c_void,
    pub mat: *mut core::ffi::c_void,
    pub verts_num: i32,
    pub edges_num: i32,
    pub faces_num: i32,
    pub corners_num: i32,
    pub face_offset_indices: *mut core::ffi::c_void,
    pub attribute_storage: AttributeStorage,
    pub vert_data: CustomData,
    pub edge_data: CustomData,
    pub face_data: CustomData,
    pub corner_data: CustomData,
    pub vertex_group_names: ListBaseT<bDeformGroup>,
    pub nullptr: ListBaseT<bDeformGroup>,
}

impl Default for Mesh {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct TFace {
    pub uv: [[f32; 2]; 4],
}

impl Default for TFace {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

