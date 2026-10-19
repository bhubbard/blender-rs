//! Auto-transpiled C/C++ header module: deg_eval_runtime_backup_sound
//! Upstream: source/blender/depsgraph/intern/eval/deg_eval_runtime_backup_sound.h

use crate::*;
use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SoundBackup {
    pub cache: *mut c_void,
    pub playback_handle: *mut c_void,
    pub waveform: *mut Vec<f32>,
}

impl SoundBackup {
    pub fn new() -> Self {
        Self::default()
    }
}
