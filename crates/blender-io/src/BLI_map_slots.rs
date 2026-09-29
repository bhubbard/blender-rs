//! Auto-transpiled C/C++ header module: BLI_map_slots

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DefaultMapSlot {

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
