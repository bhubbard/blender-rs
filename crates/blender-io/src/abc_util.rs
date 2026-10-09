//! Auto-transpiled C/C++ header module: abc_util

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SampleInterpolationSettings {
    pub weight: f64,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Object {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AbcReaderConstructorArgs {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AbcObjectReader {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct holds {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TContainer {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AbcFaceVaryingInterpolateBoundary {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AbcInterpolateBoundary {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbcFaceVaryingInterpolateBoundary_2 {
    ALL = 0,
    EDGE_AND_CORNERS = 1,
    NONE = 2,
    BOUNDARIES = 3,
}

impl Default for AbcFaceVaryingInterpolateBoundary_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const ALL: i32 = AbcFaceVaryingInterpolateBoundary_2::ALL as i32;
pub const EDGE_AND_CORNERS: i32 = AbcFaceVaryingInterpolateBoundary_2::EDGE_AND_CORNERS as i32;
pub const NONE: i32 = AbcFaceVaryingInterpolateBoundary_2::NONE as i32;
pub const BOUNDARIES: i32 = AbcFaceVaryingInterpolateBoundary_2::BOUNDARIES as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbcInterpolateBoundary_2 {
    NONE = 0,
    EDGE_AND_CORNERS = 1,
    EDGE_ONLY = 2,
}

impl Default for AbcInterpolateBoundary_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const EDGE_ONLY: i32 = AbcInterpolateBoundary_2::EDGE_ONLY as i32;
