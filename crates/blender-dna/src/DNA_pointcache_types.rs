//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePointCache_DataType(pub i16);

impl ePointCache_DataType {
    pub const BPHYS_DATA_INDEX: Self = Self((0) as i16);
    pub const BPHYS_DATA_LOCATION: Self = Self((1) as i16);
    pub const BPHYS_DATA_SMOKE_LOW: Self = Self((1) as i16);
    pub const BPHYS_DATA_VELOCITY: Self = Self((2) as i16);
    pub const BPHYS_DATA_SMOKE_HIGH: Self = Self((2) as i16);
    pub const BPHYS_DATA_ROTATION: Self = Self((3) as i16);
    pub const BPHYS_DATA_DYNAMICPAINT: Self = Self((3) as i16);
    pub const BPHYS_DATA_AVELOCITY: Self = Self((4) as i16);
    pub const BPHYS_DATA_XCONST: Self = Self((4) as i16);
    pub const BPHYS_DATA_SIZE: Self = Self((5) as i16);
    pub const BPHYS_DATA_TIMES: Self = Self((6) as i16);
    pub const BPHYS_DATA_BOIDS: Self = Self((7) as i16);
    pub const BPHYS_TOT_DATA: Self = Self((8) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePointCache_ExtraDataType(pub i16);

impl ePointCache_ExtraDataType {
    pub const BPHYS_EXTRA_FLUID_SPRINGS: Self = Self((1) as i16);
    pub const BPHYS_EXTRA_CLOTH_ACCELERATION: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePointCache_Flag(pub i32);

impl ePointCache_Flag {
    pub const PTCACHE_BAKED: Self = Self((1 << 0) as i32);
    pub const PTCACHE_OUTDATED: Self = Self((1 << 1) as i32);
    pub const PTCACHE_SIMULATION_VALID: Self = Self((1 << 2) as i32);
    pub const PTCACHE_BAKING: Self = Self((1 << 3) as i32);
    pub const PTCACHE_DISK_CACHE: Self = Self((1 << 6) as i32);
    pub const PTCACHE_FRAMES_SKIPPED: Self = Self((1 << 8) as i32);
    pub const PTCACHE_EXTERNAL: Self = Self((1 << 9) as i32);
    pub const PTCACHE_READ_INFO: Self = Self((1 << 10) as i32);
    pub const PTCACHE_IGNORE_LIBPATH: Self = Self((1 << 11) as i32);
    pub const PTCACHE_FAKE_SMOKE: Self = Self((1 << 12) as i32);
    pub const PTCACHE_IGNORE_CLEAR: Self = Self((1 << 13) as i32);
    pub const PTCACHE_FLAG_INFO_DIRTY: Self = Self((1 << 14) as i32);
    pub const PTCACHE_REDO_NEEDED: Self = Self(12 as i32);
    pub const PTCACHE_FLAGS_COPY: Self = Self(13 as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PointCacheCompression(pub i16);

impl PointCacheCompression {
    pub const PTCACHE_COMPRESS_NO: Self = Self((0) as i16);
    pub const PTCACHE_COMPRESS_LZO_DEPRECATED: Self = Self((1) as i16);
    pub const PTCACHE_COMPRESS_LZMA_DEPRECATED: Self = Self((2) as i16);
    pub const PTCACHE_COMPRESS_ZSTD_FILTERED: Self = Self((3) as i16);
    pub const PTCACHE_COMPRESS_ZSTD_FAST_DEPRECATED: Self = Self((4) as i16);
    pub const PTCACHE_COMPRESS_ZSTD_SLOW_DEPRECATED: Self = Self((8) as i16);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct PTCacheExtra {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub r#type: ePointCache_ExtraDataType,
}

impl Default for PTCacheExtra {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct PTCacheMem {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub frame: u32,
    pub totpoint: u32,
    pub data_types: u32,
    pub flag: u32,
    pub data: [(); 8],
}

impl Default for PTCacheMem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct PointCache {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub flag: ePointCache_Flag,
}

impl Default for PointCache {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

