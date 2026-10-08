//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i8)]
pub enum eCacheFileType {
    #[default]
    CACHEFILE_TYPE_ALEMBIC = 1,
    CACHEFILE_TYPE_USD = 2,
    CACHE_FILE_TYPE_INVALID = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i16)]
pub enum eCacheFile_Flag {
    #[default]
    CACHEFILE_DS_EXPAND = (1 << 0),
    CACHEFILE_UNUSED_0 = (1 << 1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i16)]
pub enum eCacheFile_DrawFlag {
    #[default]
    CACHEFILE_KEYFRAME_DRAWN = (1 << 0),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum eCacheFileLayer_Flag {
    #[default]
    CACHEFILE_LAYER_HIDDEN = (1 << 0),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i8)]
pub enum eCacheFile_VelocityUnit {
    #[default]
    CACHEFILE_VELOCITY_UNIT_FRAME = 0,
    CACHEFILE_VELOCITY_UNIT_SECOND = 1,
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

