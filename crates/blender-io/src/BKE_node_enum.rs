//! Auto-transpiled C/C++ header module: BKE_node_enum

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RuntimeNodeEnumItem {
    pub identifier: i32,
    pub icon: i32,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeSocketValueMenuRuntimeFlag {
    NODE_MENU_ITEMS_CONFLICT = (1 << 0),
}

impl Default for NodeSocketValueMenuRuntimeFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
