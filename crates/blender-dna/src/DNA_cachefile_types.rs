//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCacheFileType(pub i8);

impl eCacheFileType {
    pub const CACHEFILE_TYPE_ALEMBIC: Self = Self((1) as i8);
    pub const CACHEFILE_TYPE_USD: Self = Self((2) as i8);
    pub const CACHE_FILE_TYPE_INVALID: Self = Self((0) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCacheFile_Flag(pub i16);

impl eCacheFile_Flag {
    pub const CACHEFILE_DS_EXPAND: Self = Self(((1 << 0)) as i16);
    pub const CACHEFILE_UNUSED_0: Self = Self(((1 << 1)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCacheFile_DrawFlag(pub i16);

impl eCacheFile_DrawFlag {
    pub const CACHEFILE_KEYFRAME_DRAWN: Self = Self(((1 << 0)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCacheFileLayer_Flag(pub i32);

impl eCacheFileLayer_Flag {
    pub const CACHEFILE_LAYER_HIDDEN: Self = Self(((1 << 0)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCacheFile_VelocityUnit(pub i8);

impl eCacheFile_VelocityUnit {
    pub const CACHEFILE_VELOCITY_UNIT_FRAME: Self = Self((0) as i8);
    pub const CACHEFILE_VELOCITY_UNIT_SECOND: Self = Self((1) as i8);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CacheObjectPath {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub path: [u8; 4096],
}

impl Default for CacheObjectPath {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CacheFileLayer {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub filepath: [u8; 1024],
    pub flag: eCacheFileLayer_Flag,
}

impl Default for CacheFileLayer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CacheFile {
    pub adt: *mut core::ffi::c_void,
    pub object_paths: ListBaseT<CacheObjectPath>,
    pub nullptr: ListBaseT<CacheObjectPath>,
}

impl Default for CacheFile {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

