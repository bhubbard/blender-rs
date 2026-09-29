//! Auto-transpiled C/C++ header module: BLI_ghash

use core::ffi::c_void;
use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GHashIterator {
    pub gh: *mut GHash,
    pub curEntry: *mut Entry,
    pub curBucket: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GHashIterState {
    pub curr_bucket: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct _gh_Entry {
    pub next: *mut core::ffi::c_void,
    pub key: *mut core::ffi::c_void,
    pub val: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GSetIterator {

}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GHashPair {
    pub first: *mut core::ffi::c_void,
    pub second: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GHash {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GSet {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Entry {
    pub _opaque: [u8; 0],
}

pub const GHASH_FLAG_ALLOW_DUPES: i32 = (1 << 0);
pub const GHASH_FLAG_ALLOW_SHRINK: i32 = (1 << 1);
pub const GHASH_FLAG_IS_GSET: i32 = (1 << 16);
