/* SPDX-FileCopyrightText: 2025-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Closure signature types: input/output keys, structure types, and linked signatures.

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeSocketInterfaceStructureType {
    Single = 0,
    Multi = 1,
    Dynamic = 2,
    Auto = 3,
}

/// A single input or output declaration within a closure signature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureSignatureItem {
    pub key: String,
    pub type_name: String,
    pub structure_type: NodeSocketInterfaceStructureType,
}

impl ClosureSignatureItem {
    pub fn new(key: &str, type_name: &str, structure_type: NodeSocketInterfaceStructureType) -> Self {
        Self {
            key: key.to_string(),
            type_name: type_name.to_string(),
            structure_type,
        }
    }
}

/// Complete signature defining the interface of a closure.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ClosureSignature {
    pub inputs: Vec<ClosureSignatureItem>,
    pub outputs: Vec<ClosureSignatureItem>,
}

impl ClosureSignature {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn find_input_index(&self, key: &str) -> Option<usize> {
        self.inputs.iter().position(|item| item.key == key)
    }

    pub fn find_output_index(&self, key: &str) -> Option<usize> {
        self.outputs.iter().position(|item| item.key == key)
    }
}
