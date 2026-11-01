/* SPDX-FileCopyrightText: 2025-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Geometry Nodes closures: first-class callable node subgraphs.

use std::sync::Arc;
use crate::NOD_geometry_nodes_closure_signature::ClosureSignature;

/// Function index ranges for lazy evaluation of a closure.
#[derive(Debug, Clone, Default)]
pub struct ClosureFunctionIndices {
    pub main_inputs: (usize, usize),
    pub main_outputs: (usize, usize),
}

/// A first-class closure representing a callable node group.
#[derive(Debug, Clone)]
pub struct Closure {
    pub signature: Arc<ClosureSignature>,
    pub function_indices: ClosureFunctionIndices,
}

impl Closure {
    pub fn new(signature: Arc<ClosureSignature>) -> Self {
        Self {
            signature,
            function_indices: ClosureFunctionIndices::default(),
        }
    }
}
