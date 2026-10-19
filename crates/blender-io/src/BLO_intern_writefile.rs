//! Auto-transpiled C/C++ header module: BLO_intern_writefile
//! Upstream: source/blender/blenloader/intern/writefile.hh

use crate::*;
use std::collections::HashMap;
use std::ffi::c_void;

#[derive(Debug, Default)]
pub struct WriteDataStableAddressIDs {
    pub pointer_map: HashMap<*const c_void, u64>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WriteData {
    pub current_chunk_size: usize,
}
