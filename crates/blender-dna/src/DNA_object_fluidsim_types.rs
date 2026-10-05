//! Auto-transpiled C/C++ header module: DNA_object_fluidsim_types

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FluidVertexVelocity {
    pub vel: [f32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct FluidsimSettings {
    pub fmd: *mut FluidsimModifierData,
    pub threads: i32,
    pub _pad1: [i8; 4],
    pub r#type: eFluidsim_Type,
    pub show_advancedoptions: i16,
    pub resolutionxyz: i16,
    pub previewresxyz: i16,
    pub realsize: f32,
    pub guiDisplayMode: i16,
    pub renderDisplayMode: i16,
    pub viscosityValue: f32,
    pub viscosityMode: i16,
    pub viscosityExponent: i16,
    pub grav: [f32; 3],
    pub animStart: f32,
    pub bakeStart: i32,
    pub frameOffset: i32,
    pub _pad2: [i8; 4],
    pub gstar: f32,
    pub maxRefine: i32,
    pub iniVelx: f32,
    pub surfdataPath: [i8; 1024],
    pub bbStart: [f32; 3],
    pub typeFlags: i16,
    pub domainNovecgen: i8,
    pub partSlipValue: f32,
    pub generateTracers: i32,
    pub generateParticles: f32,
    pub surfaceSmoothing: f32,
    pub surfaceSubdivs: i32,
    pub flag: eFluidsim_Flag,
    pub particleInfSize: f32,
    pub farFieldSize: f32,
    pub meshVelocities: *mut FluidVertexVelocity,
    pub totvert: i32,
    pub cpsTimeStart: f32,
    pub cpsTimeEnd: f32,
    pub cpsQuality: f32,
    pub attractforceStrength: f32,
    pub attractforceRadius: f32,
    pub velocityforceStrength: f32,
    pub velocityforceRadius: f32,
    pub lastgoodframe: i32,
    pub animRate: f32,
}

impl Default for FluidsimSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FluidsimModifierData {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidsim_Type {
    OB_FLUIDSIM_ENABLE = 1,
    OB_FLUIDSIM_DOMAIN = 1 << 1,
    OB_FLUIDSIM_FLUID = 1 << 2,
    OB_FLUIDSIM_OBSTACLE = 1 << 3,
    OB_FLUIDSIM_INFLOW = 1 << 4,
    OB_FLUIDSIM_OUTFLOW = 1 << 5,
    OB_FLUIDSIM_PARTICLE = 1 << 6,
    OB_FLUIDSIM_CONTROL = 1 << 7,
}

impl Default for eFluidsim_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const OB_FLUIDSIM_ENABLE: i32 = eFluidsim_Type::OB_FLUIDSIM_ENABLE as i32;
pub const OB_FLUIDSIM_DOMAIN: i32 = eFluidsim_Type::OB_FLUIDSIM_DOMAIN as i32;
pub const OB_FLUIDSIM_FLUID: i32 = eFluidsim_Type::OB_FLUIDSIM_FLUID as i32;
pub const OB_FLUIDSIM_OBSTACLE: i32 = eFluidsim_Type::OB_FLUIDSIM_OBSTACLE as i32;
pub const OB_FLUIDSIM_INFLOW: i32 = eFluidsim_Type::OB_FLUIDSIM_INFLOW as i32;
pub const OB_FLUIDSIM_OUTFLOW: i32 = eFluidsim_Type::OB_FLUIDSIM_OUTFLOW as i32;
pub const OB_FLUIDSIM_PARTICLE: i32 = eFluidsim_Type::OB_FLUIDSIM_PARTICLE as i32;
pub const OB_FLUIDSIM_CONTROL: i32 = eFluidsim_Type::OB_FLUIDSIM_CONTROL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidsim_Flag {
    OB_FLUIDSIM_REVERSE = 1 << 0,
    OB_FLUIDSIM_ACTIVE = 1 << 1,
    OB_FLUIDSIM_OVERRIDE_TIME = 1 << 2,
}

impl Default for eFluidsim_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const OB_FLUIDSIM_REVERSE: i32 = eFluidsim_Flag::OB_FLUIDSIM_REVERSE as i32;
pub const OB_FLUIDSIM_ACTIVE: i32 = eFluidsim_Flag::OB_FLUIDSIM_ACTIVE as i32;
pub const OB_FLUIDSIM_OVERRIDE_TIME: i32 = eFluidsim_Flag::OB_FLUIDSIM_OVERRIDE_TIME as i32;

