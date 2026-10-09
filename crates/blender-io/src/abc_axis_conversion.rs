//! Auto-transpiled C/C++ header module: abc_axis_conversion

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Object {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbcAxisSwapMode {
    ABC_ZUP_FROM_YUP = 1,
    ABC_YUP_FROM_ZUP = 2,
}

impl Default for AbcAxisSwapMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const ABC_ZUP_FROM_YUP: i32 = AbcAxisSwapMode::ABC_ZUP_FROM_YUP as i32;
pub const ABC_YUP_FROM_ZUP: i32 = AbcAxisSwapMode::ABC_YUP_FROM_ZUP as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbcMatrixMode {
    ABC_MATRIX_WORLD = 1,
    ABC_MATRIX_LOCAL = 2,
}

impl Default for AbcMatrixMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const ABC_MATRIX_WORLD: i32 = AbcMatrixMode::ABC_MATRIX_WORLD as i32;
pub const ABC_MATRIX_LOCAL: i32 = AbcMatrixMode::ABC_MATRIX_LOCAL as i32;
