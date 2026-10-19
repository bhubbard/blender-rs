//! Auto-transpiled C/C++ header module: deg_eval_runtime_backup_movieclip
//! Upstream: source/blender/depsgraph/intern/eval/deg_eval_runtime_backup_movieclip.h

use crate::*;
use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MovieClipBackup {
    pub anim: *mut c_void,
    pub cache: *mut c_void,
}

impl MovieClipBackup {
    pub fn new() -> Self {
        Self::default()
    }
}
