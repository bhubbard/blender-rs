//! Auto-transpiled C/C++ header module: BLO_undofile
//! Upstream: source/blender/blenloader/BLO_undofile.hh

use crate::*;
use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct MemFileChunk {
    pub next: *mut c_void,
    pub prev: *mut c_void,
    pub buf: *const u8,
    pub size: usize,
    pub is_identical: bool,
    pub is_identical_future: bool,
    pub id_session_uid: u32,
}

impl Default for MemFileChunk {
    fn default() -> Self {
        Self {
            next: core::ptr::null_mut(),
            prev: core::ptr::null_mut(),
            buf: core::ptr::null(),
            size: 0,
            is_identical: false,
            is_identical_future: false,
            id_session_uid: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MemFile {
    pub size: usize,
}

extern "C" {
    pub fn BLO_memfile_free(memfile: *mut MemFile);
    pub fn BLO_memfile_clear(memfile: *mut MemFile);
}
