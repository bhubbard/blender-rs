//! Auto-transpiled C/C++ header module: BLO_intern_readfile
//! Upstream: source/blender/blenloader/intern/readfile.hh

use crate::*;
use std::ffi::c_void;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFileDataFlag {
    FD_FLAGS_SWITCH_ENDIAN = 1 << 0,
    FD_FLAGS_FILE_POINTSIZE_IS_4 = 1 << 1,
    FD_FLAGS_POINTSIZE_DIFFERS = 1 << 2,
    FD_FLAGS_FILE_OK = 1 << 3,
    FD_FLAGS_IS_MEMFILE = 1 << 4,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BHead {
    pub code: i32,
    pub len: i32,
    pub old: *const c_void,
    pub SDNAnr: i32,
    pub nr: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FileData {
    pub flags: i32,
    pub filedes: i32,
    pub filesize: usize,
}
