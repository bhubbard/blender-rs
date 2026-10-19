//! Auto-transpiled C/C++ header module: deg_builder_pchanmap
//! Upstream: source/blender/depsgraph/intern/builder/deg_builder_pchanmap.hh

use crate::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct RootPChanMap {
    pub map: HashMap<String, HashSet<String>>,
}

impl RootPChanMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_bone(&mut self, bone: &str, root: &str) {
        self.map
            .entry(bone.to_string())
            .or_default()
            .insert(root.to_string());
    }

    pub fn has_common_root(&self, bone1: &str, bone2: &str) -> bool {
        if let (Some(set1), Some(set2)) = (self.map.get(bone1), self.map.get(bone2)) {
            !set1.is_disjoint(set2)
        } else {
            false
        }
    }
}
