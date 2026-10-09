//! Auto-transpiled C/C++ header module: IO_path_util_types

use crate::*;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ePathReferenceMode {
    PATH_REFERENCE_AUTO = 0,
    PATH_REFERENCE_ABSOLUTE = 1,
    PATH_REFERENCE_RELATIVE = 2,
    PATH_REFERENCE_MATCH = 3,
    PATH_REFERENCE_STRIP = 4,
    PATH_REFERENCE_COPY = 5,
}

impl Default for ePathReferenceMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const PATH_REFERENCE_AUTO: i32 = ePathReferenceMode::PATH_REFERENCE_AUTO as i32;
pub const PATH_REFERENCE_ABSOLUTE: i32 = ePathReferenceMode::PATH_REFERENCE_ABSOLUTE as i32;
pub const PATH_REFERENCE_RELATIVE: i32 = ePathReferenceMode::PATH_REFERENCE_RELATIVE as i32;
pub const PATH_REFERENCE_MATCH: i32 = ePathReferenceMode::PATH_REFERENCE_MATCH as i32;
pub const PATH_REFERENCE_STRIP: i32 = ePathReferenceMode::PATH_REFERENCE_STRIP as i32;
pub const PATH_REFERENCE_COPY: i32 = ePathReferenceMode::PATH_REFERENCE_COPY as i32;
