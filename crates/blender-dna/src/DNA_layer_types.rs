//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eViewLayerEEVEEPassType(pub i32);

impl eViewLayerEEVEEPassType {
    pub const EEVEE_RENDER_PASS_COMBINED: Self = Self(((1 << 0)) as i32);
    pub const EEVEE_RENDER_PASS_DEPTH: Self = Self(((1 << 1)) as i32);
    pub const EEVEE_RENDER_PASS_MIST: Self = Self(((1 << 2)) as i32);
    pub const EEVEE_RENDER_PASS_NORMAL: Self = Self(((1 << 3)) as i32);
    pub const EEVEE_RENDER_PASS_DIFFUSE_LIGHT: Self = Self(((1 << 4)) as i32);
    pub const EEVEE_RENDER_PASS_DIFFUSE_COLOR: Self = Self(((1 << 5)) as i32);
    pub const EEVEE_RENDER_PASS_SPECULAR_LIGHT: Self = Self(((1 << 6)) as i32);
    pub const EEVEE_RENDER_PASS_SPECULAR_COLOR: Self = Self(((1 << 7)) as i32);
    pub const EEVEE_RENDER_PASS_UNUSED_8: Self = Self(((1 << 8)) as i32);
    pub const EEVEE_RENDER_PASS_VOLUME_LIGHT: Self = Self(((1 << 9)) as i32);
    pub const EEVEE_RENDER_PASS_EMIT: Self = Self(((1 << 10)) as i32);
    pub const EEVEE_RENDER_PASS_ENVIRONMENT: Self = Self(((1 << 11)) as i32);
    pub const EEVEE_RENDER_PASS_SHADOW: Self = Self(((1 << 12)) as i32);
    pub const EEVEE_RENDER_PASS_AO: Self = Self(((1 << 13)) as i32);
    pub const EEVEE_RENDER_PASS_UNUSED_14: Self = Self(((1 << 14)) as i32);
    pub const EEVEE_RENDER_PASS_AOV: Self = Self(((1 << 15)) as i32);
    pub const EEVEE_RENDER_PASS_CRYPTOMATTE: Self = Self(((1 << 16)) as i32);
    pub const EEVEE_RENDER_PASS_CRYPTOMATTE_OBJECT: Self = Self(((1 << 16)) as i32);
    pub const EEVEE_RENDER_PASS_CRYPTOMATTE_ASSET: Self = Self(((1 << 17)) as i32);
    pub const EEVEE_RENDER_PASS_CRYPTOMATTE_MATERIAL: Self = Self(((1 << 18)) as i32);
    pub const EEVEE_RENDER_PASS_VECTOR: Self = Self(((1 << 19)) as i32);
    pub const EEVEE_RENDER_PASS_TRANSPARENT: Self = Self(((1 << 20)) as i32);
    pub const EEVEE_RENDER_PASS_POSITION: Self = Self(((1 << 21)) as i32);
    pub const EEVEE_RENDER_PASS_DENOISING_DEPTH: Self = Self(((1 << 22)) as i32);
    pub const EEVEE_RENDER_PASS_DENOISING_NORMAL: Self = Self(((1 << 23)) as i32);
    pub const EEVEE_RENDER_PASS_DENOISING_ROUGHNESS: Self = Self(((1 << 24)) as i32);
    pub const EEVEE_RENDER_PASS_DENOISING_DIFFUSE_ALBEDO: Self = Self(((1 << 25)) as i32);
    pub const EEVEE_RENDER_PASS_DENOISING_SPECULAR_ALBEDO: Self = Self(((1 << 26)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eViewLayerEEVEEDenoisingPassFlag(pub u32);

impl eViewLayerEEVEEDenoisingPassFlag {
    pub const EEVEE_DENOISING_PASS_STORE: Self = Self(((1 << 0)) as u32);
    pub const EEVEE_DENOISING_PASS_USE_ALBEDO_ROUGHNESS_WEIGHTING: Self = Self(((1 << 1)) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eViewLayerGreasePencilFlags(pub i32);

impl eViewLayerGreasePencilFlags {
    pub const GREASE_PENCIL_AS_SEPARATE_PASS: Self = Self(((1 << 0)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eViewLayerAOVType(pub i32);

impl eViewLayerAOVType {
    pub const AOV_TYPE_VALUE: Self = Self((0) as i32);
    pub const AOV_TYPE_COLOR: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eViewLayerAOVFlag(pub i32);

impl eViewLayerAOVFlag {
    pub const AOV_CONFLICT: Self = Self(((1 << 0)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eViewLayerCryptomatteFlags(pub i16);

impl eViewLayerCryptomatteFlags {
    pub const VIEW_LAYER_CRYPTOMATTE_OBJECT: Self = Self(((1 << 0)) as i16);
    pub const VIEW_LAYER_CRYPTOMATTE_MATERIAL: Self = Self(((1 << 1)) as i16);
    pub const VIEW_LAYER_CRYPTOMATTE_ASSET: Self = Self(((1 << 2)) as i16);
    pub const VIEW_LAYER_CRYPTOMATTE_ACCURATE: Self = Self(((1 << 3)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBase_Flag(pub i16);

impl eBase_Flag {
    pub const BASE_SELECTED: Self = Self(((1 << 0)) as i16);
    pub const BASE_HIDDEN: Self = Self(((1 << 8)) as i16);
    pub const BASE_ENABLED_AND_MAYBE_VISIBLE_IN_VIEWPORT: Self = Self(((1 << 1)) as i16);
    pub const BASE_SELECTABLE: Self = Self(((1 << 2)) as i16);
    pub const BASE_FROM_DUPLI: Self = Self(((1 << 3)) as i16);
    pub const BASE_ENABLED_AND_VISIBLE_IN_DEFAULT_VIEWPORT: Self = Self(((1 << 4)) as i16);
    pub const BASE_FROM_SET: Self = Self(((1 << 5)) as i16);
    pub const BASE_ENABLED_VIEWPORT: Self = Self(((1 << 6)) as i16);
    pub const BASE_ENABLED_RENDER: Self = Self(((1 << 7)) as i16);
    pub const BASE_HOLDOUT: Self = Self(((1 << 10)) as i16);
    pub const BASE_INDIRECT_ONLY: Self = Self(((1 << 11)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eLayerCollection_Flag(pub i16);

impl eLayerCollection_Flag {
    pub const LAYER_COLLECTION_EXCLUDE: Self = Self(((1 << 4)) as i16);
    pub const LAYER_COLLECTION_HOLDOUT: Self = Self(((1 << 5)) as i16);
    pub const LAYER_COLLECTION_INDIRECT_ONLY: Self = Self(((1 << 6)) as i16);
    pub const LAYER_COLLECTION_HIDE: Self = Self(((1 << 7)) as i16);
    pub const LAYER_COLLECTION_PREVIOUSLY_EXCLUDED: Self = Self(((1 << 8)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eLayerCollection_RuntimeFlag(pub i16);

impl eLayerCollection_RuntimeFlag {
    pub const LAYER_COLLECTION_HAS_OBJECTS: Self = Self(((1 << 0)) as i16);
    pub const LAYER_COLLECTION_HIDE_VIEWPORT: Self = Self(((1 << 2)) as i16);
    pub const LAYER_COLLECTION_VISIBLE_VIEW_LAYER: Self = Self(((1 << 4)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eViewLayer_Flag(pub i16);

impl eViewLayer_Flag {
    pub const VIEW_LAYER_RENDER: Self = Self(((1 << 0)) as i16);
    pub const VIEW_LAYER_FREESTYLE: Self = Self(((1 << 2)) as i16);
    pub const VIEW_LAYER_OUT_OF_SYNC: Self = Self(((1 << 3)) as i16);
    pub const VIEW_LAYER_HAS_EXPORT_COLLECTIONS: Self = Self(((1 << 4)) as i16);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Base {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub object: *mut core::ffi::c_void,
    pub base_orig: *mut core::ffi::c_void,
    pub flag: eBase_Flag,
}

impl Default for Base {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct LayerCollection {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub collection: *mut core::ffi::c_void,
    pub _pad1: *mut core::ffi::c_void,
    pub flag: eLayerCollection_Flag,
}

impl Default for LayerCollection {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ViewLayerEEVEE {
    pub render_passes: eViewLayerEEVEEPassType,
}

impl Default for ViewLayerEEVEE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ViewLayerAOV {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub name: [u8; 64],
    pub flag: eViewLayerAOVFlag,
}

impl Default for ViewLayerAOV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ViewLayerLightgroup {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub name: [u8; 64],
}

impl Default for ViewLayerLightgroup {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct LightgroupMembership {
    pub name: [u8; 64],
}

impl Default for LightgroupMembership {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ViewLayer {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub name: [u8; 64],
    pub flag: eViewLayer_Flag,
    pub _pad: [u8; 6],
}

impl Default for ViewLayer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

