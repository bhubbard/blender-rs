//! Auto-transpiled C/C++ header module: BLI_task_size_hints

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TaskSizeHints {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Type {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TaskSizeHints_Static {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TaskSizeHints_IndividualLookup {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TaskSizeHints_AccumulatedLookup {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TaskSizeHints_IndividualLookupFn {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TaskSizeHints_AccumulatedLookupFn {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type_2 {
    Static,
    IndividualLookup,
    AccumulatedLookup,
}

impl Default for Type_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const Static: i32 = Type_2::Static as i32;
pub const IndividualLookup: i32 = Type_2::IndividualLookup as i32;
pub const AccumulatedLookup: i32 = Type_2::AccumulatedLookup as i32;
