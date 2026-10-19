//! Auto-transpiled C/C++ header module: BLO_read_write
//! Upstream: source/blender/blenloader/BLO_read_write.hh

use crate::*;
use crate::DEG_depsgraph::Main;
use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BlendWriter {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BlendDataReader {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BlendLibReader {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BlendExpander {
    pub _opaque: [u8; 0],
}

extern "C" {
    pub fn BLO_write_struct_by_name(writer: *mut BlendWriter, struct_name: *const i8, data: *const c_void);
    pub fn BLO_write_raw(writer: *mut BlendWriter, size: usize, data: *const c_void);
    pub fn BLO_read_data_address(reader: *mut BlendDataReader, ptr: *mut *mut c_void);
    pub fn BLO_read_struct_by_name(reader: *mut BlendDataReader, struct_name: *const i8, ptr: *mut *mut c_void);
}
