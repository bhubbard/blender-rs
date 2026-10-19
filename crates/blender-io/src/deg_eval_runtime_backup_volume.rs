//! Auto-transpiled C/C++ header module: deg_eval_runtime_backup_volume
//! Upstream: source/blender/depsgraph/intern/eval/deg_eval_runtime_backup_volume.h

use crate::*;
use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct VolumeBackup {
    pub grids: *mut c_void,
    pub filepath: [u8; 1024],
}

impl Default for VolumeBackup {
    fn default() -> Self {
        Self {
            grids: core::ptr::null_mut(),
            filepath: [0u8; 1024],
        }
    }
}
