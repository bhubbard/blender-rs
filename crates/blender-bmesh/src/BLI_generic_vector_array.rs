//! Auto-transpiled C/C++ header module: BLI_generic_vector_array

use core::ffi::c_void;
use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Item {
    pub start: *mut core::ffi::c_void,
    pub length: i64,
    pub capacity: i64,
}
