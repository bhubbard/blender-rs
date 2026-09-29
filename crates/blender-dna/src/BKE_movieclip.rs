//! Auto-transpiled C/C++ header module: BKE_movieclip

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Depsgraph {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImBuf {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Main {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MovieDistortion {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovieClipCacheFlag {
    None = 0,
    SkipCache = 1 << 0,
}

impl Default for MovieClipCacheFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovieClipPostprocFlag {
    None = 0,
    DisableRed = (1 << 0),
    DisableGreen = (1 << 1),
    DisableBlue = (1 << 2),
    PreviewGray = (1 << 3),
}

impl Default for MovieClipPostprocFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
