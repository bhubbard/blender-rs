use crate::*;
pub const MAX_LIBJPEG_MARKER_LENGTH: i32 = 65533;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct my_error_mgr {
    pub r#pub: jpeg_error_mgr,
    pub setjmp_buffer: jmp_buf,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct my_source_mgr {
    pub r#pub: jpeg_source_mgr,
    pub buffer: *mut u8,
    pub size: i32,
    pub terminal: [JOCTET; 2],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NeoGeo_Word {
    pub pad1: u8,
    pub pad2: u8,
    pub pad3: u8,
    pub quality: u8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct JOCTET {
    pub byte1: u8,
    pub byte2: u8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct jpeg_error_mgr {
    pub error_code: i32,
    pub error_message: String,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct jmp_buf {
    pub buffer: *mut u8,
    pub size: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct jpeg_source_mgr {
    pub buffer: *mut u8,
    pub size: i32,
    pub terminal: [JOCTET; 2],
}
