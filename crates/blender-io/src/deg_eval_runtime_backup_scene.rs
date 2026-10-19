//! Auto-transpiled C/C++ header module: deg_eval_runtime_backup_scene
//! Upstream: source/blender/depsgraph/intern/eval/deg_eval_runtime_backup_scene.h

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SceneBackup {
    pub rigidbody_last_time: f32,
    pub sequencer_backup: SequencerBackup,
}

impl SceneBackup {
    pub fn new() -> Self {
        Self::default()
    }
}
