//! Auto-transpiled C/C++ header module: deg_node_component
//! Upstream: source/blender/depsgraph/intern/node/deg_node_component.hh

use crate::*;
use std::collections::HashMap;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OperationIDKey {
    pub opcode: OperationCode,
    pub name_tag: i32,
    pub name: String,
}

impl Default for OperationIDKey {
    fn default() -> Self {
        Self {
            opcode: OperationCode::OPERATION,
            name_tag: -1,
            name: String::new(),
        }
    }
}

impl OperationIDKey {
    pub fn new(opcode: OperationCode, name: &str, name_tag: i32) -> Self {
        Self {
            opcode,
            name_tag,
            name: name.to_string(),
        }
    }

    pub fn identifier(&self) -> String {
        format!("{:?}:{}:{}", self.opcode, self.name, self.name_tag)
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentNode {
    pub base: DepsgraphNode,
    pub owner: *mut IDNode,
    pub operations: HashMap<OperationIDKey, Box<OperationNode>>,
}

impl Default for ComponentNode {
    fn default() -> Self {
        Self {
            base: DepsgraphNode {
                type_: NodeType::PARAMETERS,
                ..Default::default()
            },
            owner: core::ptr::null_mut(),
            operations: HashMap::new(),
        }
    }
}

impl ComponentNode {
    pub fn new(type_: NodeType, name: &str) -> Self {
        Self {
            base: DepsgraphNode {
                name: name.to_string(),
                type_,
                ..Default::default()
            },
            owner: core::ptr::null_mut(),
            operations: HashMap::new(),
        }
    }

    pub fn has_operation(&self, key: &OperationIDKey) -> bool {
        self.operations.contains_key(key)
    }

    pub fn find_operation(&self, key: &OperationIDKey) -> Option<&OperationNode> {
        self.operations.get(key).map(|b| b.as_ref())
    }

    pub fn find_operation_mut(&mut self, key: &OperationIDKey) -> Option<&mut OperationNode> {
        self.operations.get_mut(key).map(|b| b.as_mut())
    }

    pub fn add_operation(
        &mut self,
        opcode: OperationCode,
        name: &str,
        name_tag: i32,
        op_cb: DepsEvalOperationCb,
    ) -> &mut OperationNode {
        let key = OperationIDKey::new(opcode, name, name_tag);
        let node = Box::new(OperationNode {
            base: DepsgraphNode {
                name: name.to_string(),
                type_: NodeType::OPERATION,
                ..Default::default()
            },
            opcode,
            name_tag,
            flag: 0,
            operation: op_cb,
            owner: self as *mut Self as *mut std::ffi::c_void,
        });
        self.operations.entry(key.clone()).or_insert(node);
        self.operations.get_mut(&key).unwrap().as_mut()
    }
}
