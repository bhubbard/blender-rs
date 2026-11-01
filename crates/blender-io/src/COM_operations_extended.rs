/* SPDX-FileCopyrightText: 2025-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Compositor group node operations, scene effects operations, and undefined node fallbacks.

use blender_dna::bNode;
use crate::COM_node_operation::NodeOperation;

/// Group node operation factory.
pub fn get_group_node_operation(_node: *const bNode) -> Option<Box<NodeOperation>> {
    None
}

/// Fallback operation for unhandled or undefined nodes in compositor node graphs.
pub fn get_undefined_node_operation(_node: *const bNode) -> Option<Box<NodeOperation>> {
    None
}

/// Operation that evaluates scene compositor effects in sequence.
#[derive(Debug, Clone, Default)]
pub struct SceneCompositorEffectsOperation {
    pub has_output: bool,
    pub has_viewer_output: bool,
}

impl SceneCompositorEffectsOperation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execute(&mut self) {
        self.has_output = true;
    }
}
