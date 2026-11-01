/* SPDX-FileCopyrightText: 2011-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! MaterialX graph exporter wrapper.

use std::collections::HashMap;
use blender_dna::{bNode, Material};
use crate::DEG_depsgraph::Depsgraph;

/// Wrapper around a MaterialX procedural shading node graph.
pub struct MaterialXNodeGraph {
    pub depsgraph: *const Depsgraph,
    pub material: *const Material,
    pub node_names: HashMap<usize, String>,
}

impl MaterialXNodeGraph {
    pub fn new(depsgraph: *const Depsgraph, material: *const Material) -> Self {
        Self {
            depsgraph,
            material,
            node_names: HashMap::new(),
        }
    }

    pub fn unique_node_name(&mut self, node: *const bNode, socket_out_name: &str) -> String {
        let key = node as usize;
        let count = self.node_names.len();
        let name = format!("node_{}_{}_{}", key, socket_out_name, count);
        self.node_names.insert(key, name.clone());
        name
    }
}
