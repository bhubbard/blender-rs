//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eWorld_Mode(pub i16);

impl eWorld_Mode {
    pub const WO_MIST: Self = Self((1 << 0) as i16);
    pub const WO_MODE_UNUSED_1: Self = Self((1 << 1) as i16);
    pub const WO_MODE_UNUSED_2: Self = Self((1 << 2) as i16);
    pub const WO_MODE_UNUSED_3: Self = Self((1 << 3) as i16);
    pub const WO_MODE_UNUSED_4: Self = Self((1 << 4) as i16);
    pub const WO_MODE_UNUSED_5: Self = Self((1 << 5) as i16);
    pub const WO_MODE_UNUSED_6: Self = Self((1 << 6) as i16);
    pub const WO_MODE_UNUSED_7: Self = Self((1 << 7) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eWorld_MistType(pub i16);

impl eWorld_MistType {
    pub const WO_MIST_QUADRATIC: Self = Self((0) as i16);
    pub const WO_MIST_LINEAR: Self = Self((1) as i16);
    pub const WO_MIST_INVERSE_QUADRATIC: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eWorld_Flag(pub i16);

impl eWorld_Flag {
    pub const WO_DS_EXPAND: Self = Self((1 << 0) as i16);
    pub const WO_DS_SHOW_TEXS: Self = Self((1 << 2) as i16);
    pub const WO_USE_EEVEE_FINITE_VOLUME: Self = Self((1 << 3) as i16);
    pub const WO_USE_SUN_SHADOW: Self = Self((1 << 4) as i16);
    pub const WO_USE_SUN_SHADOW_JITTER: Self = Self((1 << 5) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eLightProbeResolution(pub i32);

impl eLightProbeResolution {
    pub const LIGHT_PROBE_RESOLUTION_128: Self = Self((7) as i32);
    pub const LIGHT_PROBE_RESOLUTION_256: Self = Self((8) as i32);
    pub const LIGHT_PROBE_RESOLUTION_512: Self = Self((9) as i32);
    pub const LIGHT_PROBE_RESOLUTION_1024: Self = Self((10) as i32);
    pub const LIGHT_PROBE_RESOLUTION_2048: Self = Self((11) as i32);
    pub const LIGHT_PROBE_RESOLUTION_4096: Self = Self((12) as i32);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct World {
    pub adt: *mut core::ffi::c_void,
    pub _pad0: [u8; 4],
}

impl Default for World {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

