//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i16)]
pub enum eWorld_Mode {
    #[default]
    WO_MIST = 1 << 0,
    WO_MODE_UNUSED_1 = 1 << 1,
    WO_MODE_UNUSED_2 = 1 << 2,
    WO_MODE_UNUSED_3 = 1 << 3,
    WO_MODE_UNUSED_4 = 1 << 4,
    WO_MODE_UNUSED_5 = 1 << 5,
    WO_MODE_UNUSED_6 = 1 << 6,
    WO_MODE_UNUSED_7 = 1 << 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i16)]
pub enum eWorld_MistType {
    #[default]
    WO_MIST_QUADRATIC = 0,
    WO_MIST_LINEAR = 1,
    WO_MIST_INVERSE_QUADRATIC = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i16)]
pub enum eWorld_Flag {
    #[default]
    WO_DS_EXPAND = 1 << 0,
    WO_DS_SHOW_TEXS = 1 << 2,
    WO_USE_EEVEE_FINITE_VOLUME = 1 << 3,
    WO_USE_SUN_SHADOW = 1 << 4,
    WO_USE_SUN_SHADOW_JITTER = 1 << 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum eLightProbeResolution {
    #[default]
    LIGHT_PROBE_RESOLUTION_128 = 7,
    LIGHT_PROBE_RESOLUTION_256 = 8,
    LIGHT_PROBE_RESOLUTION_512 = 9,
    LIGHT_PROBE_RESOLUTION_1024 = 10,
    LIGHT_PROBE_RESOLUTION_2048 = 11,
    LIGHT_PROBE_RESOLUTION_4096 = 12,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct World {
    pub adt: *mut core::ffi::c_void,
    pub _pad0: [u8; 4],
}

