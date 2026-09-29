//! Auto-transpiled C/C++ header module: eevee_shadow_shared

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ShadowTileData {
    pub page: [u32; 3],
    pub cache_index: u32,
    pub is_used: bool,
    pub do_update: bool,
    pub is_allocated: bool,
    pub is_rendered: bool,
    pub is_cached: bool,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ShadowSamplingTile {
    pub page: [u32; 3],
    pub lod: u32,
    pub lod_offset: [u32; 2],
    pub is_valid: bool,
}
