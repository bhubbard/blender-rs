//! Auto-transpiled C/C++ header module: DNA_lightprobe_types

use core::ffi::c_void;
use crate::*;

pub const LIGHTCACHE_STATIC_VERSION: i32 = 2;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LightProbe {
    pub id: ID,
    pub adt: *mut AnimData,
    pub r#type: eLightProbeType,
    pub flag: eLightProbe_Flag,
    pub attenuation_type: eLightProbeShape,
    pub parallax_type: eLightProbeShape,
    pub _pad0: [i8; 3],
    pub distinf: f32,
    pub distpar: f32,
    pub falloff: f32,
    pub clipsta: f32,
    pub vis_bias: f32,
    pub vis_blur: f32,
    pub intensity: f32,
    pub grid_resolution_x: i32,
    pub grid_resolution_y: i32,
    pub grid_resolution_z: i32,
    pub grid_bake_samples: i32,
    pub grid_surface_bias: f32,
    pub grid_escape_bias: f32,
    pub grid_normal_bias: f32,
    pub grid_view_bias: f32,
    pub grid_facing_bias: f32,
    pub grid_validity_threshold: f32,
    pub grid_dilation_threshold: f32,
    pub grid_dilation_radius: f32,
    pub grid_clamp_direct: f32,
    pub grid_clamp_indirect: f32,
    pub grid_surfel_density: i32,
    pub visibility_grp: *mut Collection,
    pub data_display_size: f32,
    pub _pad1: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LightProbeCache {
    pub position: [f32; 3],
    pub attenuation_fac: f32,
    pub attenuation_type: f32,
    pub _pad3: [f32; 2],
    pub attenuationmat: [[f32; 4]; 4],
    pub parallaxmat: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LightGridCache {
    pub mat: [[f32; 4]; 4],
    pub resolution: [i32; 3],
    pub corner: [f32; 3],
    pub increment_x: [f32; 3],
    pub increment_y: [f32; 3],
    pub increment_z: [f32; 3],
    pub visibility_bias: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LightCacheTexture {
    pub data: *mut i8,
    pub tex_size: [i32; 3],
    pub data_type: eLightCacheTexture_DataType,
    pub components: i8,
    pub _pad: [i8; 2],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LightCache {
    pub flag: eLightCache_Flag,
    pub version: i32,
    pub r#type: eLightCacheType,
    pub cube_len: i32,
    pub mips_len: i32,
    pub vis_res: i32,
    pub _pad: [[i8; 2]; 4],
    pub grid_tx: LightCacheTexture,
    pub cube_tx: LightCacheTexture,
    pub cube_mips: *mut LightCacheTexture,
    pub cube_data: *mut LightProbeCache,
    pub grid_data: *mut LightGridCache,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LightProbeBakingData {
    pub validity: *mut f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LightProbeIrradianceData {

}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LightProbeVisibilityData {
    pub L0: *mut f32,
    pub L1_a: *mut f32,
    pub L1_b: *mut f32,
    pub L1_c: *mut f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LightProbeConnectivityData {
    pub validity: *mut u8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LightProbeBlockData {
    pub offset: [i32; 3],
    pub level: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LightProbeGridCacheFrame {
    pub size: [i32; 3],
    pub data_layout: eLightProbeGridCacheLayout,
    pub block_len: i32,
    pub block_size: i32,
    pub block_infos: *mut LightProbeBlockData,
    pub baking: LightProbeBakingData,
    pub irradiance: LightProbeIrradianceData,
    pub visibility: LightProbeVisibilityData,
    pub connectivity: LightProbeConnectivityData,
    pub _pad: [i8; 4],
    pub surfels_len: i32,
    pub surfels: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LightProbeObjectCache {
    pub cache_type: eLightProbeObjectCacheType,
    pub shared: i8,
    pub dirty: i8,
    pub _pad0: [i8; 2],
    pub grid_static_cache: *mut LightProbeGridCacheFrame,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnimData {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Object {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Texture {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Collection {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct definition {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLightProbeType {
    LIGHTPROBE_TYPE_SPHERE = 0,
    LIGHTPROBE_TYPE_PLANE = 1,
    LIGHTPROBE_TYPE_VOLUME = 2,
}

impl Default for eLightProbeType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LIGHTPROBE_TYPE_SPHERE: i32 = eLightProbeType::LIGHTPROBE_TYPE_SPHERE as i32;
pub const LIGHTPROBE_TYPE_PLANE: i32 = eLightProbeType::LIGHTPROBE_TYPE_PLANE as i32;
pub const LIGHTPROBE_TYPE_VOLUME: i32 = eLightProbeType::LIGHTPROBE_TYPE_VOLUME as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLightProbe_Flag {
    LIGHTPROBE_FLAG_CUSTOM_PARALLAX = (1 << 0),
    LIGHTPROBE_FLAG_SHOW_INFLUENCE = (1 << 1),
    LIGHTPROBE_FLAG_SHOW_PARALLAX = (1 << 2),
    LIGHTPROBE_FLAG_SHOW_CLIP_DIST = (1 << 3),
    LIGHTPROBE_FLAG_SHOW_DATA = (1 << 4),
    LIGHTPROBE_FLAG_INVERT_GROUP = (1 << 5),
    LIGHTPROBE_DS_EXPAND = (1 << 6),
}

impl Default for eLightProbe_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LIGHTPROBE_FLAG_CUSTOM_PARALLAX: i32 = eLightProbe_Flag::LIGHTPROBE_FLAG_CUSTOM_PARALLAX as i32;
pub const LIGHTPROBE_FLAG_SHOW_INFLUENCE: i32 = eLightProbe_Flag::LIGHTPROBE_FLAG_SHOW_INFLUENCE as i32;
pub const LIGHTPROBE_FLAG_SHOW_PARALLAX: i32 = eLightProbe_Flag::LIGHTPROBE_FLAG_SHOW_PARALLAX as i32;
pub const LIGHTPROBE_FLAG_SHOW_CLIP_DIST: i32 = eLightProbe_Flag::LIGHTPROBE_FLAG_SHOW_CLIP_DIST as i32;
pub const LIGHTPROBE_FLAG_SHOW_DATA: i32 = eLightProbe_Flag::LIGHTPROBE_FLAG_SHOW_DATA as i32;
pub const LIGHTPROBE_FLAG_INVERT_GROUP: i32 = eLightProbe_Flag::LIGHTPROBE_FLAG_INVERT_GROUP as i32;
pub const LIGHTPROBE_DS_EXPAND: i32 = eLightProbe_Flag::LIGHTPROBE_DS_EXPAND as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLightProbe_GridFlag {
    LIGHTPROBE_GRID_CAPTURE_WORLD = (1 << 0),
    LIGHTPROBE_GRID_CAPTURE_INDIRECT = (1 << 1),
    LIGHTPROBE_GRID_CAPTURE_EMISSION = (1 << 2),
}

impl Default for eLightProbe_GridFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LIGHTPROBE_GRID_CAPTURE_WORLD: i32 = eLightProbe_GridFlag::LIGHTPROBE_GRID_CAPTURE_WORLD as i32;
pub const LIGHTPROBE_GRID_CAPTURE_INDIRECT: i32 = eLightProbe_GridFlag::LIGHTPROBE_GRID_CAPTURE_INDIRECT as i32;
pub const LIGHTPROBE_GRID_CAPTURE_EMISSION: i32 = eLightProbe_GridFlag::LIGHTPROBE_GRID_CAPTURE_EMISSION as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLightProbeDisplay {
    LIGHTPROBE_DISP_WIRE = 0,
    LIGHTPROBE_DISP_SHADED = 1,
    LIGHTPROBE_DISP_DIFFUSE = 2,
    LIGHTPROBE_DISP_REFLECTIVE = 3,
}

impl Default for eLightProbeDisplay {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LIGHTPROBE_DISP_WIRE: i32 = eLightProbeDisplay::LIGHTPROBE_DISP_WIRE as i32;
pub const LIGHTPROBE_DISP_SHADED: i32 = eLightProbeDisplay::LIGHTPROBE_DISP_SHADED as i32;
pub const LIGHTPROBE_DISP_DIFFUSE: i32 = eLightProbeDisplay::LIGHTPROBE_DISP_DIFFUSE as i32;
pub const LIGHTPROBE_DISP_REFLECTIVE: i32 = eLightProbeDisplay::LIGHTPROBE_DISP_REFLECTIVE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLightProbeShape {
    LIGHTPROBE_SHAPE_ELIPSOID = 0,
    LIGHTPROBE_SHAPE_BOX = 1,
}

impl Default for eLightProbeShape {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LIGHTPROBE_SHAPE_ELIPSOID: i32 = eLightProbeShape::LIGHTPROBE_SHAPE_ELIPSOID as i32;
pub const LIGHTPROBE_SHAPE_BOX: i32 = eLightProbeShape::LIGHTPROBE_SHAPE_BOX as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLightCacheType {
    LIGHTCACHE_TYPE_STATIC = 0,
}

impl Default for eLightCacheType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LIGHTCACHE_TYPE_STATIC: i32 = eLightCacheType::LIGHTCACHE_TYPE_STATIC as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLightCache_Flag {
    LIGHTCACHE_BAKED = (1 << 0),
    LIGHTCACHE_BAKING = (1 << 1),
    LIGHTCACHE_CUBE_READY = (1 << 2),
    LIGHTCACHE_GRID_READY = (1 << 3),
    LIGHTCACHE_UPDATE_CUBE = (1 << 4),
    LIGHTCACHE_UPDATE_GRID = (1 << 5),
    LIGHTCACHE_UPDATE_WORLD = (1 << 6),
    LIGHTCACHE_UPDATE_AUTO = (1 << 7),
    LIGHTCACHE_INVALID = (1 << 8),
    LIGHTCACHE_NOT_USABLE = (1 << 9),
}

impl Default for eLightCache_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LIGHTCACHE_BAKED: i32 = eLightCache_Flag::LIGHTCACHE_BAKED as i32;
pub const LIGHTCACHE_BAKING: i32 = eLightCache_Flag::LIGHTCACHE_BAKING as i32;
pub const LIGHTCACHE_CUBE_READY: i32 = eLightCache_Flag::LIGHTCACHE_CUBE_READY as i32;
pub const LIGHTCACHE_GRID_READY: i32 = eLightCache_Flag::LIGHTCACHE_GRID_READY as i32;
pub const LIGHTCACHE_UPDATE_CUBE: i32 = eLightCache_Flag::LIGHTCACHE_UPDATE_CUBE as i32;
pub const LIGHTCACHE_UPDATE_GRID: i32 = eLightCache_Flag::LIGHTCACHE_UPDATE_GRID as i32;
pub const LIGHTCACHE_UPDATE_WORLD: i32 = eLightCache_Flag::LIGHTCACHE_UPDATE_WORLD as i32;
pub const LIGHTCACHE_UPDATE_AUTO: i32 = eLightCache_Flag::LIGHTCACHE_UPDATE_AUTO as i32;
pub const LIGHTCACHE_INVALID: i32 = eLightCache_Flag::LIGHTCACHE_INVALID as i32;
pub const LIGHTCACHE_NOT_USABLE: i32 = eLightCache_Flag::LIGHTCACHE_NOT_USABLE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLightCacheTexture_DataType {
    LIGHTCACHETEX_BYTE = (1 << 0),
    LIGHTCACHETEX_FLOAT = (1 << 1),
    LIGHTCACHETEX_UINT = (1 << 2),
}

impl Default for eLightCacheTexture_DataType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LIGHTCACHETEX_BYTE: i32 = eLightCacheTexture_DataType::LIGHTCACHETEX_BYTE as i32;
pub const LIGHTCACHETEX_FLOAT: i32 = eLightCacheTexture_DataType::LIGHTCACHETEX_FLOAT as i32;
pub const LIGHTCACHETEX_UINT: i32 = eLightCacheTexture_DataType::LIGHTCACHETEX_UINT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLightProbeGridCacheLayout {
    LIGHTPROBE_CACHE_UNIFORM_GRID = 0,
    LIGHTPROBE_CACHE_ADAPTIVE_RESOLUTION = 1,
}

impl Default for eLightProbeGridCacheLayout {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LIGHTPROBE_CACHE_UNIFORM_GRID: i32 = eLightProbeGridCacheLayout::LIGHTPROBE_CACHE_UNIFORM_GRID as i32;
pub const LIGHTPROBE_CACHE_ADAPTIVE_RESOLUTION: i32 = eLightProbeGridCacheLayout::LIGHTPROBE_CACHE_ADAPTIVE_RESOLUTION as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLightProbeObjectCacheType {
    LIGHTPROBE_CACHE_TYPE_NONE = 0,
    LIGHTPROBE_CACHE_TYPE_STATIC = 1,
}

impl Default for eLightProbeObjectCacheType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LIGHTPROBE_CACHE_TYPE_NONE: i32 = eLightProbeObjectCacheType::LIGHTPROBE_CACHE_TYPE_NONE as i32;
pub const LIGHTPROBE_CACHE_TYPE_STATIC: i32 = eLightProbeObjectCacheType::LIGHTPROBE_CACHE_TYPE_STATIC as i32;

