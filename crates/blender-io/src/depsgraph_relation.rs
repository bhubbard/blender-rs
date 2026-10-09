//! Auto-transpiled C/C++ header module: depsgraph_relation

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Relation {
    pub from: *mut Node,
    pub to: *mut Node,
    pub name: *mut i8,
    pub flag: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Node {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationFlag {
    RELATION_FLAG_CYCLIC = (1 << 0),
    RELATION_FLAG_NO_FLUSH = (1 << 1),
    RELATION_FLAG_FLUSH_USER_EDIT_ONLY = (1 << 2),
    RELATION_FLAG_GODMODE = (1 << 4),
    RELATION_CHECK_BEFORE_ADD = (1 << 5),
    RELATION_NO_VISIBILITY_CHANGE = (1 << 6),
}

impl Default for RelationFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const RELATION_FLAG_CYCLIC: i32 = RelationFlag::RELATION_FLAG_CYCLIC as i32;
pub const RELATION_FLAG_NO_FLUSH: i32 = RelationFlag::RELATION_FLAG_NO_FLUSH as i32;
pub const RELATION_FLAG_FLUSH_USER_EDIT_ONLY: i32 = RelationFlag::RELATION_FLAG_FLUSH_USER_EDIT_ONLY as i32;
pub const RELATION_FLAG_GODMODE: i32 = RelationFlag::RELATION_FLAG_GODMODE as i32;
pub const RELATION_CHECK_BEFORE_ADD: i32 = RelationFlag::RELATION_CHECK_BEFORE_ADD as i32;
pub const RELATION_NO_VISIBILITY_CHANGE: i32 = RelationFlag::RELATION_NO_VISIBILITY_CHANGE as i32;
