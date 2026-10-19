//! Auto-transpiled C/C++ header module: deg_debug
//! Upstream: source/blender/depsgraph/intern/debug/deg_debug.h

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DepsgraphDebug {
    pub flags: i32,
    pub name: String,
    pub graph_evaluation_start_time: f64,
    pub graph_evaluation_total_time: f64,
}

impl DepsgraphDebug {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn total_evaluation_time(&self) -> f64 {
        self.graph_evaluation_total_time
    }
}
