/* SPDX-FileCopyrightText: 2023-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Geometry Nodes evaluation logger and UI inspection cache.

use std::collections::HashMap;

/// Log of intermediate evaluated values for a single socket.
#[derive(Debug, Clone)]
pub struct SocketValueLog {
    pub socket_id: i32,
    pub name: String,
    pub value_summary: String,
}

/// Evaluation log for a specific node in a node tree.
#[derive(Debug, Clone)]
pub struct NodeLog {
    pub node_id: i32,
    pub execution_time_ns: u64,
    pub input_values: Vec<SocketValueLog>,
    pub output_values: Vec<SocketValueLog>,
    pub warnings: Vec<String>,
}

impl NodeLog {
    pub fn new(node_id: i32) -> Self {
        Self {
            node_id,
            execution_time_ns: 0,
            input_values: Vec::new(),
            output_values: Vec::new(),
            warnings: Vec::new(),
        }
    }
}

/// Full execution log for an evaluated node tree.
#[derive(Debug, Clone, Default)]
pub struct NodesEvalLog {
    pub node_logs: HashMap<i32, NodeLog>,
    pub total_execution_time_ns: u64,
}

impl NodesEvalLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn log_node(&mut self, log: NodeLog) {
        self.node_logs.insert(log.node_id, log);
    }

    pub fn find_node_log(&self, node_id: i32) -> Option<&NodeLog> {
        self.node_logs.get(&node_id)
    }
}
