//! Auto-transpiled C/C++ header module: BKE_main_idmap

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Library {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ID {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDNameLib_Map {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Main {
    pub _opaque: [u8; 0],
}

pub const MAIN_IDMAP_TYPE_NAME: i32 = 1 << 0;
pub const MAIN_IDMAP_TYPE_UID: i32 = 1 << 1;
