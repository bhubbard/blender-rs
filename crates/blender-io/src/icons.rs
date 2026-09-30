//! Auto-transpiled C/C++ header module: icons

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DeferredIconDeleteNode {
    pub next: *mut DeferredIconDeleteNode,
    pub icon_id: i32,
}

pub const ICON_FLAG_MANAGED: i32 = (1 << 0);
