//! Auto-transpiled C/C++ header module: eevee_sync

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BaseHandle {
    pub recalc: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct HairParticleInfo {
    pub recalc_flags: u32,
    pub sub_key: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Material {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MaterialPass {
    pub _opaque: [u8; 0],
}
