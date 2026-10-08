//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i16)]
pub enum eCurveMapPoint_Flag {
    #[default]
    CUMA_SELECT = (1 << 0),
    CUMA_HANDLE_VECTOR = (1 << 1),
    CUMA_HANDLE_AUTO_ANIM = (1 << 2),
    CUMA_REMOVE = (1 << 3),
    CUMA_ACTIVE = (1 << 4),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum eCurveMappingFlags {
    #[default]
    CUMA_DO_CLIP = (1 << 0),
    CUMA_PREMULLED = (1 << 1),
    CUMA_DRAW_CFRA = (1 << 2),
    CUMA_DRAW_SAMPLE = (1 << 3),
    CUMA_EXTEND_EXTRAPOLATE = (1 << 4),
    CUMA_USE_WRAPPING = (1 << 5),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum eCurveMappingPreset {
    #[default]
    CURVE_PRESET_LINE = 0,
    CURVE_PRESET_SHARP = 1,
    CURVE_PRESET_SMOOTH = 2,
    CURVE_PRESET_MAX = 3,
    CURVE_PRESET_MID8 = 4,
    CURVE_PRESET_ROUND = 5,
    CURVE_PRESET_ROOT = 6,
    CURVE_PRESET_GAUSS = 7,
    CURVE_PRESET_BELL = 8,
    CURVE_PRESET_CONSTANT_MEDIAN = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i16)]
pub enum eCurveMappingTone {
    #[default]
    CURVE_TONE_STANDARD = 0,
    CURVE_TONE_FILMLIKE = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i16)]
pub enum eHistogram_Mode {
    #[default]
    HISTO_MODE_LUMA = 0,
    HISTO_MODE_RGB = 1,
    HISTO_MODE_R = 2,
    HISTO_MODE_G = 3,
    HISTO_MODE_B = 4,
    HISTO_MODE_ALPHA = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i16)]
pub enum eHistogram_Flag {
    #[default]
    HISTO_FLAG_LINE = (1 << 0),
    HISTO_FLAG_SAMPLELINE = (1 << 1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum eScopes_WaveformMode {
    #[default]
    SCOPES_WAVEFRM_LUMA = 0,
    SCOPES_WAVEFRM_RGB_PARADE = 1,
    SCOPES_WAVEFRM_YCC_601 = 2,
    SCOPES_WAVEFRM_YCC_709 = 3,
    SCOPES_WAVEFRM_YCC_JPEG = 4,
    SCOPES_WAVEFRM_RGB = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum eScopes_VecscopeMode {
    #[default]
    SCOPES_VECSCOPE_RGB = 0,
    SCOPES_VECSCOPE_LUMA = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i8)]
pub enum eColorManageDisplay_Emulation {
    #[default]
    COLORMANAGE_DISPLAY_EMULATION_AUTO = 0,
    COLORMANAGE_DISPLAY_EMULATION_OFF = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum eColorManageView_Flag {
    #[default]
    COLORMANAGE_VIEW_USE_CURVES = (1 << 0),
    COLORMANAGE_VIEW_USE_DEPRECATED = (1 << 1),
    COLORMANAGE_VIEW_USE_WHITE_BALANCE = (1 << 2),
    COLORMANAGE_VIEW_ONLY_VIEW_LOOK = (1 << 3),
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CurveMapPoint {
    pub x: f32,
    pub y: f32,
    pub flag: eCurveMapPoint_Flag,
}

impl Default for CurveMapPoint {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CurveMapping {

}

impl Default for CurveMapping {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Histogram {
    pub channels: i32,
    pub x_resolution: i32,
    pub data_luma: [f32; 256],
}

impl Default for Histogram {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Scopes {
    pub ok: i32,
    pub sample_full: i32,
    pub sample_lines: i32,
    pub wavefrm_mode: eScopes_WaveformMode,
}

impl Default for Scopes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ColorManagedViewSettings {
    pub flag: eColorManageView_Flag,
}

impl Default for ColorManagedViewSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ColorManagedDisplaySettings {
    pub display_device: [u8; 64],
    pub emulation: eColorManageDisplay_Emulation,
}

impl Default for ColorManagedDisplaySettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ColorManagedColorspaceSettings {
    pub name: [u8; 64],
    pub interop_id: [u8; 64],
}

impl Default for ColorManagedColorspaceSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

