//! Auto-transpiled C/C++ header module: BKE_curveprofile

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BlendDataReader {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BlendWriter {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CurveProfile {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CurveProfilePoint {
    pub _opaque: [u8; 0],
}

pub const PROF_UPDATE_NONE: i32 = 0;
pub const PROF_UPDATE_REMOVE_DOUBLES: i32 = (1 << 0);
pub const PROF_UPDATE_CLIP: i32 = (1 << 1);
