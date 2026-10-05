//! Auto-transpiled C/C++ header module: DNA_light_types

use crate::*;

pub const MAX_MTEX: i32 = 18;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Light {
    pub id: ID,
    pub adt: *mut AnimData,
    pub r#type: eLightType,
    pub flag: eLight_Flag,
    pub mode: eLight_Mode,
    pub r: f32,
    pub temperature: f32,
    pub energy: f32,
    pub exposure: f32,
    pub radius: f32,
    pub spotblend: f32,
    pub area_shape: eLightAreaShape,
    pub _pad1: i16,
    pub area_size: f32,
    pub area_sizey: f32,
    pub area_sizez: f32,
    pub pr_texture: i16,
    pub use_nodes: i16,
    pub clipsta: f32,
    pub clipend_deprecated: f32,
    pub cascade_max_dist: f32,
    pub cascade_exponent: f32,
    pub cascade_fade: f32,
    pub cascade_count: i32,
    pub diff_fac: f32,
    pub spec_fac: f32,
    pub transmission_fac: f32,
    pub volume_fac: f32,
    pub att_dist: f32,
    pub shadow_filter_radius: f32,
    pub shadow_maximum_resolution: f32,
    pub shadow_jitter_overblur: f32,
    pub preview: *mut PreviewImage,
    pub nodetree: *mut bNodeTree,
    pub energy_deprecated: f32,
    pub _pad2: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnimData {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bNodeTree {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PreviewImage {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLight_Flag {
    LA_DS_EXPAND = 1 << 0,
    LA_DS_SHOW_TEXS = 1 << 2,
}

impl Default for eLight_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LA_DS_EXPAND: i32 = eLight_Flag::LA_DS_EXPAND as i32;
pub const LA_DS_SHOW_TEXS: i32 = eLight_Flag::LA_DS_SHOW_TEXS as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLightType {
    LA_LOCAL = 0,
    LA_SUN = 1,
    LA_SPOT = 2,
    LA_AREA = 4,
}

impl Default for eLightType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LA_LOCAL: i32 = eLightType::LA_LOCAL as i32;
pub const LA_SUN: i32 = eLightType::LA_SUN as i32;
pub const LA_SPOT: i32 = eLightType::LA_SPOT as i32;
pub const LA_AREA: i32 = eLightType::LA_AREA as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLight_Mode {
    LA_SHADOW = 1 << 0,
    LA_SQUARE = 1 << 7,
    LA_SHAD_RAY = 1 << 13,
    LA_SHOW_CONE = 1 << 17,
    LA_CUSTOM_ATTENUATION = 1 << 20,
    LA_USE_SOFT_FALLOFF = 1 << 21,
    LA_SHAD_RES_ABSOLUTE = 1 << 22,
    LA_SHADOW_JITTER = 1 << 23,
    LA_USE_TEMPERATURE = 1 << 24,
    LA_UNNORMALIZED = 1 << 25,
}

impl Default for eLight_Mode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LA_SHADOW: i32 = eLight_Mode::LA_SHADOW as i32;
pub const LA_SQUARE: i32 = eLight_Mode::LA_SQUARE as i32;
pub const LA_SHAD_RAY: i32 = eLight_Mode::LA_SHAD_RAY as i32;
pub const LA_SHOW_CONE: i32 = eLight_Mode::LA_SHOW_CONE as i32;
pub const LA_CUSTOM_ATTENUATION: i32 = eLight_Mode::LA_CUSTOM_ATTENUATION as i32;
pub const LA_USE_SOFT_FALLOFF: i32 = eLight_Mode::LA_USE_SOFT_FALLOFF as i32;
pub const LA_SHAD_RES_ABSOLUTE: i32 = eLight_Mode::LA_SHAD_RES_ABSOLUTE as i32;
pub const LA_SHADOW_JITTER: i32 = eLight_Mode::LA_SHADOW_JITTER as i32;
pub const LA_USE_TEMPERATURE: i32 = eLight_Mode::LA_USE_TEMPERATURE as i32;
pub const LA_UNNORMALIZED: i32 = eLight_Mode::LA_UNNORMALIZED as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLightFalloffType {
    LA_FALLOFF_CONSTANT = 0,
    LA_FALLOFF_INVLINEAR = 1,
    LA_FALLOFF_INVSQUARE = 2,
    LA_FALLOFF_CURVE = 3,
    LA_FALLOFF_SLIDERS = 4,
    LA_FALLOFF_INVCOEFFICIENTS = 5,
}

impl Default for eLightFalloffType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LA_FALLOFF_CONSTANT: i32 = eLightFalloffType::LA_FALLOFF_CONSTANT as i32;
pub const LA_FALLOFF_INVLINEAR: i32 = eLightFalloffType::LA_FALLOFF_INVLINEAR as i32;
pub const LA_FALLOFF_INVSQUARE: i32 = eLightFalloffType::LA_FALLOFF_INVSQUARE as i32;
pub const LA_FALLOFF_CURVE: i32 = eLightFalloffType::LA_FALLOFF_CURVE as i32;
pub const LA_FALLOFF_SLIDERS: i32 = eLightFalloffType::LA_FALLOFF_SLIDERS as i32;
pub const LA_FALLOFF_INVCOEFFICIENTS: i32 = eLightFalloffType::LA_FALLOFF_INVCOEFFICIENTS as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLightAreaShape {
    LA_AREA_SQUARE = 0,
    LA_AREA_RECT = 1,
    LA_AREA_DISK = 4,
    LA_AREA_ELLIPSE = 5,
}

impl Default for eLightAreaShape {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LA_AREA_SQUARE: i32 = eLightAreaShape::LA_AREA_SQUARE as i32;
pub const LA_AREA_RECT: i32 = eLightAreaShape::LA_AREA_RECT as i32;
pub const LA_AREA_DISK: i32 = eLightAreaShape::LA_AREA_DISK as i32;
pub const LA_AREA_ELLIPSE: i32 = eLightAreaShape::LA_AREA_ELLIPSE as i32;

