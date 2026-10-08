//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCurveProfilePoint_Flag(pub i16);

impl eCurveProfilePoint_Flag {
    pub const PROF_SELECT: Self = Self(((1 << 0)) as i16);
    pub const PROF_H1_SELECT: Self = Self(((1 << 1)) as i16);
    pub const PROF_H2_SELECT: Self = Self(((1 << 2)) as i16);
    pub const PROF_ACTIVE: Self = Self(((1 << 3)) as i16);
    pub const PROF_H1_ACTIVE: Self = Self(((1 << 4)) as i16);
    pub const PROF_H2_ACTIVE: Self = Self(((1 << 5)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCurveProfile_Flag(pub i32);

impl eCurveProfile_Flag {
    pub const PROF_USE_CLIP: Self = Self(((1 << 0)) as i32);
    pub const PROF_SAMPLE_STRAIGHT_EDGES: Self = Self(((1 << 2)) as i32);
    pub const PROF_SAMPLE_EVEN_LENGTHS: Self = Self(((1 << 3)) as i32);
    pub const PROF_DIRTY_PRESET: Self = Self(((1 << 4)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCurveProfilePresets(pub i32);

impl eCurveProfilePresets {
    pub const PROF_PRESET_LINE: Self = Self((0) as i32);
    pub const PROF_PRESET_SUPPORTS: Self = Self((1) as i32);
    pub const PROF_PRESET_CORNICE: Self = Self((2) as i32);
    pub const PROF_PRESET_CROWN: Self = Self((3) as i32);
    pub const PROF_PRESET_STEPS: Self = Self((4) as i32);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CurveProfilePoint {
    pub x: f32,
    pub y: f32,
    pub flag: eCurveProfilePoint_Flag,
}

impl Default for CurveProfilePoint {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

impl Default for CurveProfile {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

