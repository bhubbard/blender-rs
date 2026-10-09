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
#[allow(non_camel_case_types)]
type int = i32;
#[allow(non_camel_case_types)]
type UString = String;
#[allow(non_camel_case_types)]
type PropertyFlag = u32;
#[allow(non_camel_case_types)]
type PropertyOverrideFlag = u32;
#[allow(non_camel_case_types)]
type ParameterFlag = u32;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct transfer(pub i32);

impl transfer {
    pub const transfer_PrintingDensity: Self = Self((1) as i32);
    pub const transfer_Linear: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct descriptor(pub i32);

impl descriptor {
    pub const descriptor_Red: Self = Self((1) as i32);
    pub const descriptor_Green: Self = Self((2) as i32);
    pub const descriptor_Blue: Self = Self((3) as i32);
    pub const descriptor_Luminance: Self = Self((6) as i32);
    pub const descriptor_RGB: Self = Self((50) as i32);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct LogImageElement {
    pub depth: i32,
    pub bitsPerSample: i32,
    pub dataOffset: i32,
    pub packing: i32,
    pub transfer: i32,
    pub descriptor: i32,
    pub refLowData: u32,
    pub refHighData: u32,
    pub refLowQuantity: f32,
    pub refHighQuantity: f32,
    pub maxValue: f32,
}

impl Default for LogImageElement {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct LogImageFile {
    pub width: i32,
    pub height: i32,
    pub numElements: i32,
    pub depth: i32,
    pub element: [LogImageElement; 8],
    pub referenceBlack: f32,
    pub referenceWhite: f32,
    pub gamma: f32,
    pub file: *mut core::ffi::c_void,
    pub memBuffer: *mut core::ffi::c_void,
    pub memBufferSize: usize,
    pub memCursor: *mut core::ffi::c_void,
    pub isMSB: i32,
}

impl Default for LogImageFile {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

