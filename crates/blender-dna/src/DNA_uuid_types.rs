//! Auto-transpiled C/C++ header module: DNA_uuid_types

use crate::*;

pub const UUID_STRING_SIZE: i32 = 37;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bUUID {
    pub time_low: u32,
    pub time_mid: u16,
    pub time_hi_and_version: u16,
    pub clock_seq_hi_and_reserved: u8,
    pub clock_seq_low: u8,
    pub node: [u8; 6],
}

