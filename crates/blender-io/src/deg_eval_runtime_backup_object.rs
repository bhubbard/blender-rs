//! Auto-transpiled C/C++ header module: deg_eval_runtime_backup_object
//! Upstream: source/blender/depsgraph/intern/eval/deg_eval_runtime_backup_object.h

use crate::*;
use std::collections::HashMap;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ObjectRuntimeBackup {
    pub base_flag: i16,
    pub base_local_view_bits: u16,
    pub modifier_runtime_data: HashMap<i32, ModifierDataBackup>,
}

impl ObjectRuntimeBackup {
    pub fn new() -> Self {
        Self::default()
    }
}
