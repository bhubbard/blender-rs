//! Auto-transpiled C/C++ header module: OCIO_view

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ScopeInfo {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ColorSpace {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Gamut {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TransferFunction {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct View {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gamut_2 {
    Unknown,
    Rec709,
    P3D65,
    Rec2020,
}

impl Default for Gamut_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const Unknown: i32 = Gamut_2::Unknown as i32;
pub const Rec709: i32 = Gamut_2::Rec709 as i32;
pub const P3D65: i32 = Gamut_2::P3D65 as i32;
pub const Rec2020: i32 = Gamut_2::Rec2020 as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferFunction_2 {
    Unknown,
    sRGB,
    ExtendedsRGB,
    Gamma18,
    Gamma22,
    Gamma24,
    Gamma26,
    PQ,
    HLG,
}

impl Default for TransferFunction_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const sRGB: i32 = TransferFunction_2::sRGB as i32;
pub const ExtendedsRGB: i32 = TransferFunction_2::ExtendedsRGB as i32;
pub const Gamma18: i32 = TransferFunction_2::Gamma18 as i32;
pub const Gamma22: i32 = TransferFunction_2::Gamma22 as i32;
pub const Gamma24: i32 = TransferFunction_2::Gamma24 as i32;
pub const Gamma26: i32 = TransferFunction_2::Gamma26 as i32;
pub const PQ: i32 = TransferFunction_2::PQ as i32;
pub const HLG: i32 = TransferFunction_2::HLG as i32;
