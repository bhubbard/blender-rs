//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCustomDataType(pub i32);

impl eCustomDataType {
    pub const CD_AUTO_FROM_NAME: Self = Self((-1) as i32);
    pub const CD_MVERT: Self = Self((0) as i32);
    pub const CD_MSTICKY: Self = Self((1) as i32);
    pub const CD_MDEFORMVERT: Self = Self((2) as i32);
    pub const CD_MEDGE: Self = Self((3) as i32);
    pub const CD_MFACE: Self = Self((4) as i32);
    pub const CD_MTFACE: Self = Self((5) as i32);
    pub const CD_MCOL: Self = Self((6) as i32);
    pub const CD_ORIGINDEX: Self = Self((7) as i32);
    pub const CD_NORMAL: Self = Self((8) as i32);
    pub const CD_FACEMAP: Self = Self((9) as i32);
    pub const CD_PROP_FLOAT: Self = Self((10) as i32);
    pub const CD_PROP_INT32: Self = Self((11) as i32);
    pub const CD_PROP_STRING: Self = Self((12) as i32);
    pub const CD_ORIGSPACE: Self = Self((13) as i32);
    pub const CD_ORCO: Self = Self((14) as i32);
    pub const CD_MTEXPOLY: Self = Self((15) as i32);
    pub const CD_MLOOPUV: Self = Self((16) as i32);
    pub const CD_PROP_BYTE_COLOR: Self = Self((17) as i32);
    pub const CD_TANGENT: Self = Self((18) as i32);
    pub const CD_MDISPS: Self = Self((19) as i32);
    pub const CD_PROP_FLOAT4X4: Self = Self((20) as i32);
    pub const CD_PROP_INT16_2D: Self = Self((22) as i32);
    pub const CD_CLOTH_ORCO: Self = Self((23) as i32);
    pub const CD_PROP_FLOAT4: Self = Self((24) as i32);
    pub const CD_MPOLY: Self = Self((25) as i32);
    pub const CD_MLOOP: Self = Self((26) as i32);
    pub const CD_SHAPE_KEYINDEX: Self = Self((27) as i32);
    pub const CD_SHAPEKEY: Self = Self((28) as i32);
    pub const CD_BWEIGHT: Self = Self((29) as i32);
    pub const CD_CREASE: Self = Self((30) as i32);
    pub const CD_ORIGSPACE_MLOOP: Self = Self((31) as i32);
    pub const CD_BM_ELEM_PYPTR: Self = Self((33) as i32);
    pub const CD_PAINT_MASK: Self = Self((34) as i32);
    pub const CD_GRID_PAINT_MASK: Self = Self((35) as i32);
    pub const CD_MVERT_SKIN: Self = Self((36) as i32);
    pub const CD_FREESTYLE_EDGE: Self = Self((37) as i32);
    pub const CD_FREESTYLE_FACE: Self = Self((38) as i32);
    pub const CD_MLOOPTANGENT: Self = Self((39) as i32);
    pub const CD_TESSLOOPNORMAL: Self = Self((40) as i32);
    pub const CD_CUSTOMLOOPNORMAL: Self = Self((41) as i32);
    pub const CD_SCULPT_FACE_SETS: Self = Self((42) as i32);
    pub const CD_PROP_INT8: Self = Self((45) as i32);
    pub const CD_PROP_INT32_2D: Self = Self((46) as i32);
    pub const CD_PROP_COLOR: Self = Self((47) as i32);
    pub const CD_PROP_FLOAT3: Self = Self((48) as i32);
    pub const CD_PROP_FLOAT2: Self = Self((49) as i32);
    pub const CD_PROP_BOOL: Self = Self((50) as i32);
    pub const CD_PROP_QUATERNION: Self = Self((52) as i32);
    pub const CD_NUMTYPES: Self = Self((53) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCustomDataLayer_Flag(pub i32);

impl eCustomDataLayer_Flag {
    pub const CD_FLAG_NOCOPY: Self = Self(((1 << 0)) as i32);
    pub const CD_FLAG_UNUSED: Self = Self(((1 << 1)) as i32);
    pub const CD_FLAG_TEMPORARY: Self = Self(2 as i32);
    pub const CD_FLAG_EXTERNAL: Self = Self(((1 << 3)) as i32);
    pub const CD_FLAG_IN_MEMORY: Self = Self(((1 << 4)) as i32);
    pub const CD_FLAG_COLOR_ACTIVE: Self = Self(((1 << 5)) as i32);
    pub const CD_FLAG_COLOR_RENDER: Self = Self(((1 << 6)) as i32);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CustomDataLayer {
    pub r#type: eCustomDataType,
}

impl Default for CustomDataLayer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CustomDataExternal {
    pub filepath: [u8; 1024],
}

impl Default for CustomDataExternal {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CustomData {
    pub layers: *mut core::ffi::c_void,
    pub typemap: [i32; 53],
}

impl Default for CustomData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CustomData_MeshMasks {
    pub vmask: usize,
    pub emask: usize,
    pub fmask: usize,
    pub pmask: usize,
    pub lmask: usize,
}

impl Default for CustomData_MeshMasks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

