//! Auto-transpiled C/C++ header module: DNA_image_enums

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImColorMode {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImColorMode_2 {
    BW = 8,
    BW_A = 16,
    RGB = 24,
    RGBA = 32,
}

impl Default for ImColorMode_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BW: i32 = ImColorMode_2::BW as i32;
pub const BW_A: i32 = ImColorMode_2::BW_A as i32;
pub const RGB: i32 = ImColorMode_2::RGB as i32;
pub const RGBA: i32 = ImColorMode_2::RGBA as i32;

