//! Auto-transpiled C/C++ header module: openimageio_support

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ReadContext {
    pub mem_start: *mut u8,
    pub mem_size: usize,
    pub file_format: *mut i8,
    pub file_type: eImbFileType,
    pub flags: ImBufFlags,
    pub use_all_planes: bool,
    pub use_metadata_colorspace: bool,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WriteContext {
    pub file_format: *mut i8,
    pub ibuf: *mut ImBuf,
    pub flags: ImBufFlags,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImFileColorSpace {
    pub _opaque: [u8; 0],
}
