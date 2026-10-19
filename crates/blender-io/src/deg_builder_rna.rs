//! Auto-transpiled C/C++ header module: deg_builder_rna
//! Upstream: source/blender/depsgraph/intern/builder/deg_builder_rna.h

use crate::*;
use std::ffi::c_char;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RNAPointerSource {
    ENTRY = 0,
    EXIT = 1,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct RNANodeIdentifier {
    pub id: *mut ID,
    pub type_: NodeType,
    pub component_name: *const c_char,
    pub operation_code: OperationCode,
    pub operation_name: *const c_char,
    pub operation_name_tag: i32,
}

impl Default for RNANodeIdentifier {
    fn default() -> Self {
        Self {
            id: core::ptr::null_mut(),
            type_: NodeType::UNDEFINED,
            component_name: core::ptr::null(),
            operation_code: OperationCode::OPERATION,
            operation_name: core::ptr::null(),
            operation_name_tag: -1,
        }
    }
}

impl RNANodeIdentifier {
    pub fn is_valid(&self) -> bool {
        !self.id.is_null() && self.type_ != NodeType::UNDEFINED
    }
}
