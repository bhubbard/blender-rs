//! Auto-transpiled C/C++ header module: BLI_set_slots

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DefaultSetSlot {

}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SimpleSetSlot {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct HashedSetSlot {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IntrusiveSetSlot {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Empty = 0,
    Occupied = 1,
    Removed = 2,
}

impl Default for State {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const Empty: i32 = State::Empty as i32;
pub const Occupied: i32 = State::Occupied as i32;
pub const Removed: i32 = State::Removed as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State_2 {
    Empty = 0,
    Occupied = 1,
    Removed = 2,
}

impl Default for State_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
