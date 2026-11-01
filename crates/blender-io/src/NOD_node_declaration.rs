/* SPDX-FileCopyrightText: 2023-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Declarative node socket and interface definitions.

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositorInputRealizationMode {
    None = 0,
    Transforms = 1,
    OperationDomain = 2,
}

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketInOut {
    In = 0,
    Out = 1,
}

/// Declaration of a single socket on a node.
#[derive(Debug, Clone)]
pub struct SocketDeclaration {
    pub name: String,
    pub identifier: String,
    pub in_out: SocketInOut,
    pub type_id: i32,
    pub is_available: bool,
}

impl SocketDeclaration {
    pub fn new(name: &str, identifier: &str, in_out: SocketInOut, type_id: i32) -> Self {
        Self {
            name: name.to_string(),
            identifier: identifier.to_string(),
            in_out,
            type_id,
            is_available: true,
        }
    }
}

/// Complete declarative specification of a node's sockets and panels.
#[derive(Debug, Clone, Default)]
pub struct NodeDeclaration {
    pub inputs: Vec<SocketDeclaration>,
    pub outputs: Vec<SocketDeclaration>,
}

impl NodeDeclaration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_input(&mut self, name: &str, identifier: &str, type_id: i32) {
        self.inputs.push(SocketDeclaration::new(
            name,
            identifier,
            SocketInOut::In,
            type_id,
        ));
    }

    pub fn add_output(&mut self, name: &str, identifier: &str, type_id: i32) {
        self.outputs.push(SocketDeclaration::new(
            name,
            identifier,
            SocketInOut::Out,
            type_id,
        ));
    }
}
