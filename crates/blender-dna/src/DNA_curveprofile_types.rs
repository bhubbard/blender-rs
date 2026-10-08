//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i16)]
pub enum eCurveProfilePoint_Flag {
    #[default]
    PROF_SELECT = (1 << 0),
    PROF_H1_SELECT = (1 << 1),
    PROF_H2_SELECT = (1 << 2),
    PROF_ACTIVE = (1 << 3),
    PROF_H1_ACTIVE = (1 << 4),
    PROF_H2_ACTIVE = (1 << 5),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum eCurveProfile_Flag {
    #[default]
    PROF_USE_CLIP = (1 << 0),
    PROF_SAMPLE_STRAIGHT_EDGES = (1 << 2),
    PROF_SAMPLE_EVEN_LENGTHS = (1 << 3),
    PROF_DIRTY_PRESET = (1 << 4),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum eCurveProfilePresets {
    #[default]
    PROF_PRESET_LINE = 0,
    PROF_PRESET_SUPPORTS = 1,
    PROF_PRESET_CORNICE = 2,
    PROF_PRESET_CROWN = 3,
    PROF_PRESET_STEPS = 4,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct CurveProfilePoint {
    pub x: f32,
    pub y: f32,
    pub flag: eCurveProfilePoint_Flag,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct CurveProfile {
    pub path_len: i16,
    pub segments_len: i16,
    pub preset: eCurveProfilePresets,
    pub path: *mut core::ffi::c_void,
    pub table: *mut core::ffi::c_void,
    pub segments: *mut core::ffi::c_void,
    pub flag: eCurveProfile_Flag,
}

