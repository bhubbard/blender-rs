//! Auto-transpiled C/C++ header module: DNA_colorband_types

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CBData {
    pub r: f32,
    pub cur: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ColorBand {
    pub tot: i16,
    pub ipotype: eColorBand_Interp,
    pub ipotype_hue: eColorBand_HueInterp,
    pub color_mode: eColorBand_ColorMode,
    pub _pad: [i8; 1],
    pub data: [CBData; 32],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eColorBand_ColorMode {
    COLBAND_BLEND_RGB = 0,
    COLBAND_BLEND_HSV = 1,
    COLBAND_BLEND_HSL = 2,
}

impl Default for eColorBand_ColorMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const COLBAND_BLEND_RGB: i32 = eColorBand_ColorMode::COLBAND_BLEND_RGB as i32;
pub const COLBAND_BLEND_HSV: i32 = eColorBand_ColorMode::COLBAND_BLEND_HSV as i32;
pub const COLBAND_BLEND_HSL: i32 = eColorBand_ColorMode::COLBAND_BLEND_HSL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eColorBand_Interp {
    COLBAND_INTERP_LINEAR = 0,
    COLBAND_INTERP_EASE = 1,
    COLBAND_INTERP_B_SPLINE = 2,
    COLBAND_INTERP_CARDINAL = 3,
    COLBAND_INTERP_CONSTANT = 4,
}

impl Default for eColorBand_Interp {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const COLBAND_INTERP_LINEAR: i32 = eColorBand_Interp::COLBAND_INTERP_LINEAR as i32;
pub const COLBAND_INTERP_EASE: i32 = eColorBand_Interp::COLBAND_INTERP_EASE as i32;
pub const COLBAND_INTERP_B_SPLINE: i32 = eColorBand_Interp::COLBAND_INTERP_B_SPLINE as i32;
pub const COLBAND_INTERP_CARDINAL: i32 = eColorBand_Interp::COLBAND_INTERP_CARDINAL as i32;
pub const COLBAND_INTERP_CONSTANT: i32 = eColorBand_Interp::COLBAND_INTERP_CONSTANT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eColorBand_HueInterp {
    COLBAND_HUE_NEAR = 0,
    COLBAND_HUE_FAR = 1,
    COLBAND_HUE_CW = 2,
    COLBAND_HUE_CCW = 3,
}

impl Default for eColorBand_HueInterp {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const COLBAND_HUE_NEAR: i32 = eColorBand_HueInterp::COLBAND_HUE_NEAR as i32;
pub const COLBAND_HUE_FAR: i32 = eColorBand_HueInterp::COLBAND_HUE_FAR as i32;
pub const COLBAND_HUE_CW: i32 = eColorBand_HueInterp::COLBAND_HUE_CW as i32;
pub const COLBAND_HUE_CCW: i32 = eColorBand_HueInterp::COLBAND_HUE_CCW as i32;

