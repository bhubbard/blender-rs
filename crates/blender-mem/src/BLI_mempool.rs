//! Auto-transpiled C/C++ header module: BLI_mempool

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BLI_mempool_chunk {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BLI_mempool {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BLI_mempool_iter {
    pub pool: *mut BLI_mempool,
    pub curchunk: *mut BLI_mempool_chunk,
    pub curindex: u32,
}

pub const BLI_MEMPOOL_NOP: i32 = 0;
pub const BLI_MEMPOOL_ALLOW_ITER: i32 = 1 << 0;
