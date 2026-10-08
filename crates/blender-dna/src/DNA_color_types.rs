//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCurveMapPoint_Flag(pub i16);

impl eCurveMapPoint_Flag {
    pub const CUMA_SELECT: Self = Self(((1 << 0)) as i16);
    pub const CUMA_HANDLE_VECTOR: Self = Self(((1 << 1)) as i16);
    pub const CUMA_HANDLE_AUTO_ANIM: Self = Self(((1 << 2)) as i16);
    pub const CUMA_REMOVE: Self = Self(((1 << 3)) as i16);
    pub const CUMA_ACTIVE: Self = Self(((1 << 4)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCurveMappingFlags(pub i32);

impl eCurveMappingFlags {
    pub const CUMA_DO_CLIP: Self = Self(((1 << 0)) as i32);
    pub const CUMA_PREMULLED: Self = Self(((1 << 1)) as i32);
    pub const CUMA_DRAW_CFRA: Self = Self(((1 << 2)) as i32);
    pub const CUMA_DRAW_SAMPLE: Self = Self(((1 << 3)) as i32);
    pub const CUMA_EXTEND_EXTRAPOLATE: Self = Self(((1 << 4)) as i32);
    pub const CUMA_USE_WRAPPING: Self = Self(((1 << 5)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCurveMappingPreset(pub i32);

impl eCurveMappingPreset {
    pub const CURVE_PRESET_LINE: Self = Self((0) as i32);
    pub const CURVE_PRESET_SHARP: Self = Self((1) as i32);
    pub const CURVE_PRESET_SMOOTH: Self = Self((2) as i32);
    pub const CURVE_PRESET_MAX: Self = Self((3) as i32);
    pub const CURVE_PRESET_MID8: Self = Self((4) as i32);
    pub const CURVE_PRESET_ROUND: Self = Self((5) as i32);
    pub const CURVE_PRESET_ROOT: Self = Self((6) as i32);
    pub const CURVE_PRESET_GAUSS: Self = Self((7) as i32);
    pub const CURVE_PRESET_BELL: Self = Self((8) as i32);
    pub const CURVE_PRESET_CONSTANT_MEDIAN: Self = Self((9) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCurveMappingTone(pub i16);

impl eCurveMappingTone {
    pub const CURVE_TONE_STANDARD: Self = Self((0) as i16);
    pub const CURVE_TONE_FILMLIKE: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eHistogram_Mode(pub i16);

impl eHistogram_Mode {
    pub const HISTO_MODE_LUMA: Self = Self((0) as i16);
    pub const HISTO_MODE_RGB: Self = Self((1) as i16);
    pub const HISTO_MODE_R: Self = Self((2) as i16);
    pub const HISTO_MODE_G: Self = Self((3) as i16);
    pub const HISTO_MODE_B: Self = Self((4) as i16);
    pub const HISTO_MODE_ALPHA: Self = Self((5) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eHistogram_Flag(pub i16);

impl eHistogram_Flag {
    pub const HISTO_FLAG_LINE: Self = Self(((1 << 0)) as i16);
    pub const HISTO_FLAG_SAMPLELINE: Self = Self(((1 << 1)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eScopes_WaveformMode(pub i32);

impl eScopes_WaveformMode {
    pub const SCOPES_WAVEFRM_LUMA: Self = Self((0) as i32);
    pub const SCOPES_WAVEFRM_RGB_PARADE: Self = Self((1) as i32);
    pub const SCOPES_WAVEFRM_YCC_601: Self = Self((2) as i32);
    pub const SCOPES_WAVEFRM_YCC_709: Self = Self((3) as i32);
    pub const SCOPES_WAVEFRM_YCC_JPEG: Self = Self((4) as i32);
    pub const SCOPES_WAVEFRM_RGB: Self = Self((5) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eScopes_VecscopeMode(pub i32);

impl eScopes_VecscopeMode {
    pub const SCOPES_VECSCOPE_RGB: Self = Self((0) as i32);
    pub const SCOPES_VECSCOPE_LUMA: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eColorManageDisplay_Emulation(pub i8);

impl eColorManageDisplay_Emulation {
    pub const COLORMANAGE_DISPLAY_EMULATION_AUTO: Self = Self((0) as i8);
    pub const COLORMANAGE_DISPLAY_EMULATION_OFF: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eColorManageView_Flag(pub i32);

impl eColorManageView_Flag {
    pub const COLORMANAGE_VIEW_USE_CURVES: Self = Self(((1 << 0)) as i32);
    pub const COLORMANAGE_VIEW_USE_DEPRECATED: Self = Self(((1 << 1)) as i32);
    pub const COLORMANAGE_VIEW_USE_WHITE_BALANCE: Self = Self(((1 << 2)) as i32);
    pub const COLORMANAGE_VIEW_ONLY_VIEW_LOOK: Self = Self(((1 << 3)) as i32);
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

