//! Auto-transpiled C/C++ header module: deg_eval_runtime_backup_sequence
//! Upstream: source/blender/depsgraph/intern/eval/deg_eval_runtime_backup_sequence.h

use crate::*;
use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StripModifierDataBackup {
    pub sound_in: *mut c_void,
    pub sound_out: *mut c_void,
    pub flag: i32,
    pub params_hash: u64,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StripBackup {
    pub sound_time_stretch_fps: f32,
}

impl StripBackup {
    pub fn new() -> Self {
        Self::default()
    }
}
