//! Blender DNA structure definition types mirroring `DNA_genfile.h`.

#![allow(non_snake_case, non_camel_case_types, dead_code)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum eSDNA_Type {
    #[default]
    SDNA_TYPE_CHAR = 0,
    SDNA_TYPE_UCHAR = 1,
    SDNA_TYPE_SHORT = 2,
    SDNA_TYPE_USHORT = 3,
    SDNA_TYPE_INT = 4,
    SDNA_TYPE_FLOAT = 7,
    SDNA_TYPE_DOUBLE = 8,
    SDNA_TYPE_INT64 = 9,
    SDNA_TYPE_UINT64 = 10,
    SDNA_TYPE_VOID = 11,
    SDNA_TYPE_INT8 = 12,
    SDNA_TYPE_RAW_DATA = 13,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SDNA {
    pub data: *mut core::ffi::c_void,
    pub datalen: i32,
    pub nr_names: i32,
    pub nr_types: i32,
    pub nr_structs: i32,
}
