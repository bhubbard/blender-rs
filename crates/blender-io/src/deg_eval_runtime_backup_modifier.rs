//! Auto-transpiled C/C++ header module: deg_eval_runtime_backup_modifier
//! Upstream: source/blender/depsgraph/intern/eval/deg_eval_runtime_backup_modifier.h

use crate::*;
use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ModifierDataBackup {
    pub type_: ModifierType,
    pub runtime: *mut c_void,
}
