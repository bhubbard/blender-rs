//! Auto-transpiled C/C++ header module: COM_domain

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RealizationOptions {
    pub interpolation: Interpolation,
    pub extension_x: Extension,
    pub extension_y: Extension,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Interpolation {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Extension {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Domain {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interpolation_2 {
    Nearest,
    Bilinear,
    Bicubic,
    Anisotropic,
}

impl Default for Interpolation_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const Nearest: i32 = Interpolation_2::Nearest as i32;
pub const Bilinear: i32 = Interpolation_2::Bilinear as i32;
pub const Bicubic: i32 = Interpolation_2::Bicubic as i32;
pub const Anisotropic: i32 = Interpolation_2::Anisotropic as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Extension_2 {
    Extend,
    Repeat,
    Clip,
}

impl Default for Extension_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const Extend: i32 = Extension_2::Extend as i32;
pub const Repeat: i32 = Extension_2::Repeat as i32;
pub const Clip: i32 = Extension_2::Clip as i32;
