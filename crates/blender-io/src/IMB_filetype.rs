//! Auto-transpiled C/C++ header module: IMB_filetype

use crate::*;

pub const IM_FTYPE_FLOAT: i32 = 1;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImFileType {
    pub r_height): *mut usize,
    pub flag: i32,
    pub capability_read: eImFileTypeCapability,
    pub capability_write: eImFileTypeCapability,
    pub filetype: eImbFileType,
    pub filetype_id: *mut i8,
    pub file_extensions: *mut *mut i8,
    pub default_save_role: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct ImFileColorSpace {
    pub metadata_colorspace: [i8; IM_MAX_SPACE],
    pub is_hdr_float: bool,
}

impl Default for ImFileColorSpace {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImBuf {
    pub _opaque: [u8; 0],
}
