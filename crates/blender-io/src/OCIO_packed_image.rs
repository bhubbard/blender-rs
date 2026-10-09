//! Auto-transpiled C/C++ header module: OCIO_packed_image

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BitDepth {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PackedImage {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitDepth_2 {
    BIT_DEPTH_UNKNOWN,
    BIT_DEPTH_F32,
}

impl Default for BitDepth_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BIT_DEPTH_UNKNOWN: i32 = BitDepth_2::BIT_DEPTH_UNKNOWN as i32;
pub const BIT_DEPTH_F32: i32 = BitDepth_2::BIT_DEPTH_F32 as i32;
