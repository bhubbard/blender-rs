//! Auto-transpiled C/C++ header module: deg_eval_runtime_backup_sequencer
//! Upstream: source/blender/depsgraph/intern/eval/deg_eval_runtime_backup_sequencer.h

use crate::*;
use std::collections::HashMap;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SequencerBackup {
    pub strips_backup: HashMap<u64, StripBackup>,
}

impl SequencerBackup {
    pub fn new() -> Self {
        Self::default()
    }
}
