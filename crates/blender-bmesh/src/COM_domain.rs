//! Auto-transpiled C/C++ header module: COM_domain

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RealizationOptions {
    pub interpolation: Interpolation,
    pub extension_x: Extension,
    pub extension_y: Extension,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interpolation {
    Nearest,
    Bilinear,
    Bicubic,
    Anisotropic,
}

impl Default for Interpolation {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const Nearest: i32 = Interpolation::Nearest as i32;
pub const Bilinear: i32 = Interpolation::Bilinear as i32;
pub const Bicubic: i32 = Interpolation::Bicubic as i32;
pub const Anisotropic: i32 = Interpolation::Anisotropic as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Extension {
    Extend,
    Repeat,
    Clip,
}

impl Default for Extension {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const Extend: i32 = Extension::Extend as i32;
pub const Repeat: i32 = Extension::Repeat as i32;
pub const Clip: i32 = Extension::Clip as i32;
