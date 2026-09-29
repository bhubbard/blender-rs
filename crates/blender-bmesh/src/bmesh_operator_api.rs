//! Auto-transpiled C/C++ header module: bmesh_operator_api

use core::ffi::c_void;
use crate::*;

pub const BMO_OP_SLOT_TOTAL_TYPES: usize = 11;
pub const BMO_OP_MAX_SLOTS: usize = 21;
pub const MAX_SLOTNAME: usize = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub union eBMOpSlotSubType_Union {
    pub elem: eBMOpSlotSubType_Elem,
    pub ptr: eBMOpSlotSubType_Ptr,
    pub map: eBMOpSlotSubType_Map,
    pub intg: eBMOpSlotSubType_Int,
}

impl Default for eBMOpSlotSubType_Union {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

impl PartialEq for eBMOpSlotSubType_Union {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl core::fmt::Debug for eBMOpSlotSubType_Union {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "eBMOpSlotSubType_Union")
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMO_FlagSet {
    pub value: i32,
    pub identifier: *mut i8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMOpSlot {
    pub slot_name: *mut i8,
    pub slot_type: eBMOpSlotType,
    pub slot_subtype: eBMOpSlotSubType_Union,
    pub len: i32,
    pub i: i32,
    pub f: f32,
    pub p: *mut core::ffi::c_void,
    pub vec: [f32; 3],
    pub buf: *mut *mut core::ffi::c_void,
    pub ghash: *mut GHash,
    pub _i: i32,
    pub flags: *mut BMO_FlagSet,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct BMOperator {
    pub slots_in: [BMOpSlot; BMO_OP_MAX_SLOTS],
    pub slots_out: [BMOpSlot; BMO_OP_MAX_SLOTS],
    pub arena: *mut MemArena,
    pub r#type: i32,
    pub type_flag: BMOpTypeFlag,
    pub flag: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct BMOSlotType {
    pub name: [i8; MAX_SLOTNAME],
    pub r#type: eBMOpSlotType,
    pub subtype: eBMOpSlotSubType_Union,
    pub enum_flags: *mut BMO_FlagSet,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct BMOpDefine {
    pub opname: *mut i8,
    pub slot_types_in: [BMOSlotType; BMO_OP_MAX_SLOTS],
    pub slot_types_out: [BMOSlotType; BMO_OP_MAX_SLOTS],
    pub type_flag: BMOpTypeFlag,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMOIter {
    pub slot: *mut BMOpSlot,
    pub giter: GHashIterator,
    pub val: *mut *mut core::ffi::c_void,
    pub restrictmask: i8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GHashIterator {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MemArena {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBMOpSlotType {
    BMO_OP_SLOT_BOOL = 1,
    BMO_OP_SLOT_INT = 2,
    BMO_OP_SLOT_FLT = 3,
    BMO_OP_SLOT_PTR = 4,
    BMO_OP_SLOT_MAT = 5,
    BMO_OP_SLOT_VEC = 8,
    BMO_OP_SLOT_ELEMENT_BUF = 9,
    BMO_OP_SLOT_MAPPING = 10,
}

impl Default for eBMOpSlotType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBMOpSlotSubType_Elem {
    BMO_OP_SLOT_SUBTYPE_ELEM_VERT = BM_VERT,
    BMO_OP_SLOT_SUBTYPE_ELEM_EDGE = BM_EDGE,
    BMO_OP_SLOT_SUBTYPE_ELEM_FACE = BM_FACE,
    BMO_OP_SLOT_SUBTYPE_ELEM_IS_SINGLE = (BM_FACE << 1),
}

impl Default for eBMOpSlotSubType_Elem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBMOpSlotSubType_Map {
    BMO_OP_SLOT_SUBTYPE_MAP_EMPTY = 64,
    BMO_OP_SLOT_SUBTYPE_MAP_ELEM = 65,
    BMO_OP_SLOT_SUBTYPE_MAP_FLT = 66,
    BMO_OP_SLOT_SUBTYPE_MAP_INT = 67,
    BMO_OP_SLOT_SUBTYPE_MAP_BOOL = 68,
    BMO_OP_SLOT_SUBTYPE_MAP_INTERNAL = 69,
}

impl Default for eBMOpSlotSubType_Map {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBMOpSlotSubType_Ptr {
    BMO_OP_SLOT_SUBTYPE_PTR_BMESH = 100,
    BMO_OP_SLOT_SUBTYPE_PTR_SCENE = 101,
    BMO_OP_SLOT_SUBTYPE_PTR_OBJECT = 102,
    BMO_OP_SLOT_SUBTYPE_PTR_MESH = 103,
    BMO_OP_SLOT_SUBTYPE_PTR_STRUCT = 104,
}

impl Default for eBMOpSlotSubType_Ptr {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBMOpSlotSubType_Int {
    BMO_OP_SLOT_SUBTYPE_INT_ENUM = 200,
    BMO_OP_SLOT_SUBTYPE_INT_FLAG = 201,
}

impl Default for eBMOpSlotSubType_Int {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BMOpTypeFlag {
    BMO_OPTYPE_FLAG_NOP = 0,
    BMO_OPTYPE_FLAG_UNTAN_MULTIRES = (1 << 0),
    BMO_OPTYPE_FLAG_NORMALS_CALC = (1 << 1),
    BMO_OPTYPE_FLAG_SELECT_FLUSH = (1 << 2),
    BMO_OPTYPE_FLAG_SELECT_VALIDATE = (1 << 3),
    BMO_OPTYPE_FLAG_INVALIDATE_CLNOR_ALL = (1 << 4),
}

impl Default for BMOpTypeFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BMO_FLAG_RESPECT_HIDE: i32 = 1;

pub const DEL_VERTS: i32 = 1;
pub const DEL_EDGES: i32 = 0;
pub const DEL_ONLYFACES: i32 = 1;
pub const DEL_EDGESFACES: i32 = 2;
pub const DEL_FACES: i32 = 3;
pub const DEL_FACES_KEEP_BOUNDARY: i32 = 4;
pub const DEL_ONLYTAGGED: i32 = 5;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BMO_SymmDirection {
    BMO_SYMMETRIZE_NEGATIVE_X,
    BMO_SYMMETRIZE_NEGATIVE_Y,
    BMO_SYMMETRIZE_NEGATIVE_Z,
    BMO_SYMMETRIZE_POSITIVE_X,
    BMO_SYMMETRIZE_POSITIVE_Y,
    BMO_SYMMETRIZE_POSITIVE_Z,
}

impl Default for BMO_SymmDirection {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BMO_Delimit {
    BMO_DELIM_NORMAL = 1 << 0,
    BMO_DELIM_MATERIAL = 1 << 1,
    BMO_DELIM_SEAM = 1 << 2,
    BMO_DELIM_SHARP = 1 << 3,
    BMO_DELIM_UV = 1 << 4,
}

impl Default for BMO_Delimit {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
