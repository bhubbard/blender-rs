//! Auto-transpiled C/C++ header module: deg_node_time
//! Upstream: source/blender/depsgraph/intern/node/deg_node_time.hh

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct TimeSourceNode {
    pub base: DepsgraphNode,
    pub tagged_for_update: bool,
}

impl Default for TimeSourceNode {
    fn default() -> Self {
        Self {
            base: DepsgraphNode {
                type_: NodeType::TIMESOURCE,
                name: "TimeSource".to_string(),
                ..Default::default()
            },
            tagged_for_update: false,
        }
    }
}

impl TimeSourceNode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tag_update(&mut self) {
        self.tagged_for_update = true;
    }

    pub fn flush_update_tag(&mut self) {
        self.tagged_for_update = false;
    }
}
