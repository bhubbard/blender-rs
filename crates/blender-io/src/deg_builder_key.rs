//! Auto-transpiled C/C++ header module: deg_builder_key
//! Upstream: source/blender/depsgraph/intern/builder/deg_builder_key.hh

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TimeSourceKey;

impl TimeSourceKey {
    pub fn identifier(&self) -> &'static str {
        "TimeSource"
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentKey {
    pub id: *const ID,
    pub type_: NodeType,
    pub name: String,
}

impl Default for ComponentKey {
    fn default() -> Self {
        Self {
            id: core::ptr::null(),
            type_: NodeType::UNDEFINED,
            name: String::new(),
        }
    }
}

impl ComponentKey {
    pub fn new(id: *const ID, type_: NodeType, name: &str) -> Self {
        Self {
            id,
            type_,
            name: name.to_string(),
        }
    }

    pub fn identifier(&self) -> String {
        format!("{:p}:{:?}:{}", self.id, self.type_, self.name)
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OperationKey {
    pub id: *const ID,
    pub component_type: NodeType,
    pub component_name: String,
    pub opcode: OperationCode,
    pub name: String,
    pub name_tag: i32,
}

impl Default for OperationKey {
    fn default() -> Self {
        Self {
            id: core::ptr::null(),
            component_type: NodeType::UNDEFINED,
            component_name: String::new(),
            opcode: OperationCode::OPERATION,
            name: String::new(),
            name_tag: -1,
        }
    }
}

impl OperationKey {
    pub fn identifier(&self) -> String {
        format!(
            "{:p}:{:?}:{}:{:?}:{}:{}",
            self.id, self.component_type, self.component_name, self.opcode, self.name, self.name_tag
        )
    }
}
