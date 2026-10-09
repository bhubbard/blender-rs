//! Auto-transpiled C/C++ header module: IMB_filetype

use crate::*;

pub const IM_FTYPE_FLOAT: i32 = 1;
pub const IM_MAX_SPACE: usize = 64;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum eImFileTypeCapability {
    #[default]
    Zero = 0,
    File = 1 << 0,
    Memory = 1 << 1,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum eImbFileType {
    #[default]
    IMB_FTYPE_NONE = 0,
    IMB_FTYPE_TARGA = 1,
    IMB_FTYPE_IRIS = 2,
    IMB_FTYPE_HAMX = 3,
    IMB_FTYPE_FTYPE = 4,
    IMB_FTYPE_JPEG = 5,
    IMB_FTYPE_PNG = 6,
    IMB_FTYPE_BMP = 7,
    IMB_FTYPE_RADHDR = 9,
    IMB_FTYPE_TIF = 10,
    IMB_FTYPE_OPENEXR = 11,
    IMB_FTYPE_DDS = 13,
    IMB_FTYPE_WEBP = 14,
    IMB_FTYPE_AVIF = 15,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImFileType {
    pub r_height: *mut usize,
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
