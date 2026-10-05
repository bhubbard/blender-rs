//! Auto-transpiled C/C++ header module: IMB_partial_update

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Changes {
    pub kind: Kind,
    pub buffer_width: i32,
    pub buffer_height: i32,
    pub chunk_x_len: i32,
    pub chunk_y_len: i32,
    pub modified_chunks: BitVector<>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImBuf {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    None,
    Partial,
    Full,
    Resized,
}

impl Default for Kind {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const None: i32 = Kind::None as i32;
pub const Partial: i32 = Kind::Partial as i32;
pub const Full: i32 = Kind::Full as i32;
pub const Resized: i32 = Kind::Resized as i32;
