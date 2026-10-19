//! Auto-transpiled C/C++ header module: DNA_sdna_types

use crate::*;

pub const SDNA_RAW_DATA_STRUCT_INDEX: i32 = 0;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SDNA_StructMember {
    pub type_index: i16,
    pub member_index: i16,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SDNA_Struct {
    pub type_index: i16,
    pub members_num: i16,
    pub members: [SDNA_StructMember; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SDNA_Alias {
    pub types: Vector<String>,
    pub members: Vector<String>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SDNA {
    pub data: *const i8,
    pub data_size: i32,
    pub data_alloc: bool,
    pub pointer_size: i32,
    pub types_num: i32,
    pub types: Vector<String>,
    pub types_size: *mut i16,
    pub types_alignment: Vector<i32>,
    pub structs: Vector<*mut SDNA_Struct>,
    pub members_num: i32,
    pub members: Vector<String>,
    pub members_array_num: Vector<i16>,
    pub alias: SDNA_Alias,
}
