//! Auto-transpiled C/C++ header module: DNA_text_types

use core::ffi::c_void;
use crate::*;

pub const TXT_TABSIZE: i32 = 4;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TextLine {
    pub next: *mut TextLine,
    pub line: *mut i8,
    pub format: *mut i8,
    pub len: i32,
    pub _pad0: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Text {
    pub id: ID,
    pub _pad1: *mut core::ffi::c_void,
    pub filepath: *mut i8,
    pub compiled: *mut core::ffi::c_void,
    pub flags: eText_Flag,
    pub _pad0: [i8; 4],
    pub lines: ListBaseT<TextLine>,
    pub curl: *mut TextLine,
    pub curc: i32,
    pub mtime: f64,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eText_Flag {
    TXT_ISDIRTY = 1 << 0,
    TXT_ISMEM = 1 << 2,
    TXT_ISEXT = 1 << 3,
    TXT_ISSCRIPT = 1 << 4,
    TXT_FLAG_UNUSED_8 = 1 << 8,
    TXT_FLAG_UNUSED_9 = 1 << 9,
    TXT_TABSTOSPACES = 1 << 10,
}

impl Default for eText_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TXT_ISDIRTY: i32 = eText_Flag::TXT_ISDIRTY as i32;
pub const TXT_ISMEM: i32 = eText_Flag::TXT_ISMEM as i32;
pub const TXT_ISEXT: i32 = eText_Flag::TXT_ISEXT as i32;
pub const TXT_ISSCRIPT: i32 = eText_Flag::TXT_ISSCRIPT as i32;
pub const TXT_FLAG_UNUSED_8: i32 = eText_Flag::TXT_FLAG_UNUSED_8 as i32;
pub const TXT_FLAG_UNUSED_9: i32 = eText_Flag::TXT_FLAG_UNUSED_9 as i32;
pub const TXT_TABSTOSPACES: i32 = eText_Flag::TXT_TABSTOSPACES as i32;

