//! Auto-transpiled C/C++ header module: DNA_effect_types

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Effect {
    pub next: *mut Effect,
    pub r#type: eEffect_Type,
    pub flag: eEffect_Flag,
    pub buttype: i16,
    pub _pad0: [i8; 2],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BuildEff {
    pub next: *mut BuildEff,
    pub r#type: eEffect_Type,
    pub flag: eEffect_Flag,
    pub buttype: i16,
    pub _pad0: [i8; 2],
    pub len: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Particle {
    pub co: [f32; 3],
    pub time: f32,
    pub mat_nr: i16,
    pub _pad0: [i8; 2],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct PartEff {
    pub next: *mut PartEff,
    pub r#type: eEffect_Type,
    pub flag: ePartEff_Flag,
    pub buttype: i16,
    pub stype: ePartEff_SType,
    pub vertgroup: i16,
    pub sta: f32,
    pub totpart: i32,
    pub normfac: f32,
    pub force: [f32; 3],
    pub damp: f32,
    pub nabla: f32,
    pub _pad: [i8; 4],
    pub mult: [f32; 4],
    pub child: [i16; 4],
    pub texmap: ePartEff_TexMap,
    pub curmult: i16,
    pub staticstep: i16,
    pub flag2: ePartEff_Flag2,
    pub disp: i16,
    pub vgroupname: [i8; 64],
    pub vgroupname_v: [i8; 64],
    pub imat: [[f32; 4]; 4],
    pub keys: *mut Particle,
    pub group: *mut Collection,
}

impl Default for PartEff {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WaveEff {
    pub next: *mut WaveEff,
    pub r#type: eEffect_Type,
    pub flag: eEffect_Flag,
    pub buttype: i16,
    pub startx: f32,
    pub narrow: f32,
    pub timeoffs: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Collection {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ePartEff_Flag {
    PAF_BSPLINE = 1 << 1,
    PAF_STATIC = 1 << 2,
    PAF_FACE = 1 << 3,
    PAF_ANIMATED = 1 << 4,
    PAF_UNBORN = 1 << 5,
    PAF_OFACE = 1 << 6,
    PAF_SHOWE = 1 << 7,
    PAF_TRAND = 1 << 8,
    PAF_EDISTR = 1 << 9,
    PAF_DIED = 1 << 11,
}

impl Default for ePartEff_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const PAF_BSPLINE: i32 = ePartEff_Flag::PAF_BSPLINE as i32;
pub const PAF_STATIC: i32 = ePartEff_Flag::PAF_STATIC as i32;
pub const PAF_FACE: i32 = ePartEff_Flag::PAF_FACE as i32;
pub const PAF_ANIMATED: i32 = ePartEff_Flag::PAF_ANIMATED as i32;
pub const PAF_UNBORN: i32 = ePartEff_Flag::PAF_UNBORN as i32;
pub const PAF_OFACE: i32 = ePartEff_Flag::PAF_OFACE as i32;
pub const PAF_SHOWE: i32 = ePartEff_Flag::PAF_SHOWE as i32;
pub const PAF_TRAND: i32 = ePartEff_Flag::PAF_TRAND as i32;
pub const PAF_EDISTR: i32 = ePartEff_Flag::PAF_EDISTR as i32;
pub const PAF_DIED: i32 = ePartEff_Flag::PAF_DIED as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ePartEff_Flag2 {
    PAF_TEXTIME = 1,
}

impl Default for ePartEff_Flag2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const PAF_TEXTIME: i32 = ePartEff_Flag2::PAF_TEXTIME as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eEffect_Type {
    EFF_BUILD = 0,
    EFF_PARTICLE = 1,
    EFF_WAVE = 2,
}

impl Default for eEffect_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const EFF_BUILD: i32 = eEffect_Type::EFF_BUILD as i32;
pub const EFF_PARTICLE: i32 = eEffect_Type::EFF_PARTICLE as i32;
pub const EFF_WAVE: i32 = eEffect_Type::EFF_WAVE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eEffect_Flag {
    EFF_SELECT = 1,
}

impl Default for eEffect_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const EFF_SELECT: i32 = eEffect_Flag::EFF_SELECT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ePartEff_SType {
    PAF_NORMAL = 0,
    PAF_VECT = 1,
}

impl Default for ePartEff_SType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const PAF_NORMAL: i32 = ePartEff_SType::PAF_NORMAL as i32;
pub const PAF_VECT: i32 = ePartEff_SType::PAF_VECT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ePartEff_TexMap {
    PAF_TEXINT = 0,
    PAF_TEXRGB = 1,
    PAF_TEXGRAD = 2,
}

impl Default for ePartEff_TexMap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const PAF_TEXINT: i32 = ePartEff_TexMap::PAF_TEXINT as i32;
pub const PAF_TEXRGB: i32 = ePartEff_TexMap::PAF_TEXRGB as i32;
pub const PAF_TEXGRAD: i32 = ePartEff_TexMap::PAF_TEXGRAD as i32;

