//! Auto-transpiled C/C++ header module: DNA_shader_fx_types

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct ShaderFxData {
    pub next: *mut ShaderFxData,
    pub r#type: ShaderFxType,
    pub mode: ShaderFxMode,
    pub _pad0: [i8; 4],
    pub flag: ShaderFxFlag,
    pub ui_expand_flag: i16,
    pub name: [i8; 64],
    pub error: *mut i8,
}

impl Default for ShaderFxData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ShaderFxData_Runtime {
    pub loc: [f32; 3],
    pub _pad: [i8; 4],
    pub fx_sh: *mut DRWShadingGroup,
    pub fx_sh_b: *mut DRWShadingGroup,
    pub fx_sh_c: *mut DRWShadingGroup,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BlurShaderFxData {
    pub shaderfx: ShaderFxData,
    pub radius: [f32; 2],
    pub flag: eBlurShaderFx_Flag,
    pub samples: i32,
    pub rotation: f32,
    pub _pad: [i8; 4],
    pub runtime: ShaderFxData_Runtime,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ColorizeShaderFxData {
    pub shaderfx: ShaderFxData,
    pub mode: ColorizeShaderFxModes,
    pub low_color: [f32; 4],
    pub high_color: [f32; 4],
    pub factor: f32,
    pub flag: i32,
    pub _pad: [i8; 4],
    pub runtime: ShaderFxData_Runtime,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FlipShaderFxData {
    pub shaderfx: ShaderFxData,
    pub flag: eFlipShaderFx_Flag,
    pub flipmode: i32,
    pub runtime: ShaderFxData_Runtime,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GlowShaderFxData {
    pub shaderfx: ShaderFxData,
    pub glow_color: [f32; 4],
    pub select_color: [f32; 3],
    pub threshold: f32,
    pub flag: eGlowShaderFx_Flag,
    pub mode: GlowShaderFxModes,
    pub blur: [f32; 2],
    pub samples: i32,
    pub rotation: f32,
    pub blend_mode: i32,
    pub _pad: [i8; 4],
    pub runtime: ShaderFxData_Runtime,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PixelShaderFxData {
    pub shaderfx: ShaderFxData,
    pub size: [i32; 3],
    pub flag: ePixelShaderFx_Flag,
    pub rgba: [f32; 4],
    pub runtime: ShaderFxData_Runtime,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RimShaderFxData {
    pub shaderfx: ShaderFxData,
    pub offset: [i32; 2],
    pub flag: i32,
    pub rim_rgb: [f32; 3],
    pub mask_rgb: [f32; 3],
    pub mode: RimShaderFxModes,
    pub blur: [i32; 2],
    pub samples: i32,
    pub _pad: [i8; 4],
    pub runtime: ShaderFxData_Runtime,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ShadowShaderFxData {
    pub shaderfx: ShaderFxData,
    pub object: *mut Object,
    pub offset: [i32; 2],
    pub flag: eShadowShaderFx_Flag,
    pub shadow_rgba: [f32; 4],
    pub amplitude: f32,
    pub period: f32,
    pub phase: f32,
    pub orientation: i32,
    pub scale: [f32; 2],
    pub rotation: f32,
    pub blur: [i32; 2],
    pub samples: i32,
    pub _pad: [i8; 4],
    pub runtime: ShaderFxData_Runtime,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SwirlShaderFxData {
    pub shaderfx: ShaderFxData,
    pub object: *mut Object,
    pub flag: eSwirlShaderFx_Flag,
    pub radius: i32,
    pub angle: f32,
    pub transparent: i32,
    pub runtime: ShaderFxData_Runtime,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WaveShaderFxData {
    pub shaderfx: ShaderFxData,
    pub amplitude: f32,
    pub period: f32,
    pub phase: f32,
    pub orientation: i32,
    pub flag: i32,
    pub _pad: [i8; 4],
    pub runtime: ShaderFxData_Runtime,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DRWShadingGroup {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Object {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderFxMode {
    eShaderFxMode_Realtime = (1 << 0),
    eShaderFxMode_Render = (1 << 1),
    eShaderFxMode_Editmode = (1 << 2),
    eShaderFxMode_Expanded_DEPRECATED = (1 << 3),
}

impl Default for ShaderFxMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const eShaderFxMode_Realtime: i32 = ShaderFxMode::eShaderFxMode_Realtime as i32;
pub const eShaderFxMode_Render: i32 = ShaderFxMode::eShaderFxMode_Render as i32;
pub const eShaderFxMode_Editmode: i32 = ShaderFxMode::eShaderFxMode_Editmode as i32;
pub const eShaderFxMode_Expanded_DEPRECATED: i32 = ShaderFxMode::eShaderFxMode_Expanded_DEPRECATED as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderFxFlag {
    eShaderFxFlag_OverrideLibrary_Local = (1 << 0),
}

impl Default for ShaderFxFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const eShaderFxFlag_OverrideLibrary_Local: i32 = ShaderFxFlag::eShaderFxFlag_OverrideLibrary_Local as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBlurShaderFx_Flag {
    FX_BLUR_DOF_MODE = (1 << 0),
}

impl Default for eBlurShaderFx_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FX_BLUR_DOF_MODE: i32 = eBlurShaderFx_Flag::FX_BLUR_DOF_MODE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorizeShaderFxModes {
    eShaderFxColorizeMode_GrayScale = 0,
    eShaderFxColorizeMode_Sepia = 1,
    eShaderFxColorizeMode_Duotone = 2,
    eShaderFxColorizeMode_Custom = 3,
    eShaderFxColorizeMode_Transparent = 4,
}

impl Default for ColorizeShaderFxModes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const eShaderFxColorizeMode_GrayScale: i32 = ColorizeShaderFxModes::eShaderFxColorizeMode_GrayScale as i32;
pub const eShaderFxColorizeMode_Sepia: i32 = ColorizeShaderFxModes::eShaderFxColorizeMode_Sepia as i32;
pub const eShaderFxColorizeMode_Duotone: i32 = ColorizeShaderFxModes::eShaderFxColorizeMode_Duotone as i32;
pub const eShaderFxColorizeMode_Custom: i32 = ColorizeShaderFxModes::eShaderFxColorizeMode_Custom as i32;
pub const eShaderFxColorizeMode_Transparent: i32 = ColorizeShaderFxModes::eShaderFxColorizeMode_Transparent as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFlipShaderFx_Flag {
    FX_FLIP_HORIZONTAL = (1 << 0),
    FX_FLIP_VERTICAL = (1 << 1),
}

impl Default for eFlipShaderFx_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FX_FLIP_HORIZONTAL: i32 = eFlipShaderFx_Flag::FX_FLIP_HORIZONTAL as i32;
pub const FX_FLIP_VERTICAL: i32 = eFlipShaderFx_Flag::FX_FLIP_VERTICAL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlowShaderFxModes {
    eShaderFxGlowMode_Luminance = 0,
    eShaderFxGlowMode_Color = 1,
}

impl Default for GlowShaderFxModes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const eShaderFxGlowMode_Luminance: i32 = GlowShaderFxModes::eShaderFxGlowMode_Luminance as i32;
pub const eShaderFxGlowMode_Color: i32 = GlowShaderFxModes::eShaderFxGlowMode_Color as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGlowShaderFx_Flag {
    FX_GLOW_USE_ALPHA = (1 << 0),
}

impl Default for eGlowShaderFx_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FX_GLOW_USE_ALPHA: i32 = eGlowShaderFx_Flag::FX_GLOW_USE_ALPHA as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ePixelShaderFx_Flag {
    FX_PIXEL_FILTER_NEAREST = (1 << 0),
}

impl Default for ePixelShaderFx_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FX_PIXEL_FILTER_NEAREST: i32 = ePixelShaderFx_Flag::FX_PIXEL_FILTER_NEAREST as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RimShaderFxModes {
    eShaderFxRimMode_Normal = 0,
    eShaderFxRimMode_Overlay = 1,
    eShaderFxRimMode_Add = 2,
    eShaderFxRimMode_Subtract = 3,
    eShaderFxRimMode_Multiply = 4,
    eShaderFxRimMode_Divide = 5,
}

impl Default for RimShaderFxModes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const eShaderFxRimMode_Normal: i32 = RimShaderFxModes::eShaderFxRimMode_Normal as i32;
pub const eShaderFxRimMode_Overlay: i32 = RimShaderFxModes::eShaderFxRimMode_Overlay as i32;
pub const eShaderFxRimMode_Add: i32 = RimShaderFxModes::eShaderFxRimMode_Add as i32;
pub const eShaderFxRimMode_Subtract: i32 = RimShaderFxModes::eShaderFxRimMode_Subtract as i32;
pub const eShaderFxRimMode_Multiply: i32 = RimShaderFxModes::eShaderFxRimMode_Multiply as i32;
pub const eShaderFxRimMode_Divide: i32 = RimShaderFxModes::eShaderFxRimMode_Divide as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eShadowShaderFx_Flag {
    FX_SHADOW_USE_OBJECT = (1 << 0),
    FX_SHADOW_USE_WAVE = (1 << 1),
}

impl Default for eShadowShaderFx_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FX_SHADOW_USE_OBJECT: i32 = eShadowShaderFx_Flag::FX_SHADOW_USE_OBJECT as i32;
pub const FX_SHADOW_USE_WAVE: i32 = eShadowShaderFx_Flag::FX_SHADOW_USE_WAVE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eSwirlShaderFx_Flag {
    FX_SWIRL_MAKE_TRANSPARENT = (1 << 0),
}

impl Default for eSwirlShaderFx_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FX_SWIRL_MAKE_TRANSPARENT: i32 = eSwirlShaderFx_Flag::FX_SWIRL_MAKE_TRANSPARENT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderFxType {
    eShaderFxType_None = 0,
    eShaderFxType_Blur = 1,
    eShaderFxType_Flip = 2,
    eShaderFxType_Light_deprecated = 3,
    eShaderFxType_Pixel = 4,
    eShaderFxType_Swirl = 5,
    eShaderFxType_Wave = 6,
    eShaderFxType_Rim = 7,
    eShaderFxType_Colorize = 8,
    eShaderFxType_Shadow = 9,
    eShaderFxType_Glow = 10,
    NUM_SHADER_FX_TYPES,
}

impl Default for ShaderFxType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const eShaderFxType_None: i32 = ShaderFxType::eShaderFxType_None as i32;
pub const eShaderFxType_Blur: i32 = ShaderFxType::eShaderFxType_Blur as i32;
pub const eShaderFxType_Flip: i32 = ShaderFxType::eShaderFxType_Flip as i32;
pub const eShaderFxType_Light_deprecated: i32 = ShaderFxType::eShaderFxType_Light_deprecated as i32;
pub const eShaderFxType_Pixel: i32 = ShaderFxType::eShaderFxType_Pixel as i32;
pub const eShaderFxType_Swirl: i32 = ShaderFxType::eShaderFxType_Swirl as i32;
pub const eShaderFxType_Wave: i32 = ShaderFxType::eShaderFxType_Wave as i32;
pub const eShaderFxType_Rim: i32 = ShaderFxType::eShaderFxType_Rim as i32;
pub const eShaderFxType_Colorize: i32 = ShaderFxType::eShaderFxType_Colorize as i32;
pub const eShaderFxType_Shadow: i32 = ShaderFxType::eShaderFxType_Shadow as i32;
pub const eShaderFxType_Glow: i32 = ShaderFxType::eShaderFxType_Glow as i32;
pub const NUM_SHADER_FX_TYPES: i32 = ShaderFxType::NUM_SHADER_FX_TYPES as i32;

