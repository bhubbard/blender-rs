//! Auto-transpiled C/C++ header module: DNA_view3d_enums

use crate::*;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eV3DOffscreenDrawFlag {
    V3D_OFSDRAW_NONE = (0),
    V3D_OFSDRAW_SHOW_ANNOTATION = (1 << 0),
    V3D_OFSDRAW_OVERRIDE_SCENE_SETTINGS = (1 << 1),
    V3D_OFSDRAW_SHOW_GRIDFLOOR = (1 << 2),
    V3D_OFSDRAW_SHOW_SELECTION = (1 << 3),
    V3D_OFSDRAW_XR_SHOW_CONTROLLERS = (1 << 4),
    V3D_OFSDRAW_XR_SHOW_CUSTOM_OVERLAYS = (1 << 5),
    V3D_OFSDRAW_SHOW_OBJECT_EXTRAS = (1 << 6),
    V3D_OFSDRAW_XR_SHOW_PASSTHROUGH = (1 << 7),
    V3D_OFSDRAW_NO_WORLD_BACKGROUND_OVERRIDE = (1 << 8),
}

impl Default for eV3DOffscreenDrawFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const V3D_OFSDRAW_NONE: i32 = eV3DOffscreenDrawFlag::V3D_OFSDRAW_NONE as i32;
pub const V3D_OFSDRAW_SHOW_ANNOTATION: i32 = eV3DOffscreenDrawFlag::V3D_OFSDRAW_SHOW_ANNOTATION as i32;
pub const V3D_OFSDRAW_OVERRIDE_SCENE_SETTINGS: i32 = eV3DOffscreenDrawFlag::V3D_OFSDRAW_OVERRIDE_SCENE_SETTINGS as i32;
pub const V3D_OFSDRAW_SHOW_GRIDFLOOR: i32 = eV3DOffscreenDrawFlag::V3D_OFSDRAW_SHOW_GRIDFLOOR as i32;
pub const V3D_OFSDRAW_SHOW_SELECTION: i32 = eV3DOffscreenDrawFlag::V3D_OFSDRAW_SHOW_SELECTION as i32;
pub const V3D_OFSDRAW_XR_SHOW_CONTROLLERS: i32 = eV3DOffscreenDrawFlag::V3D_OFSDRAW_XR_SHOW_CONTROLLERS as i32;
pub const V3D_OFSDRAW_XR_SHOW_CUSTOM_OVERLAYS: i32 = eV3DOffscreenDrawFlag::V3D_OFSDRAW_XR_SHOW_CUSTOM_OVERLAYS as i32;
pub const V3D_OFSDRAW_SHOW_OBJECT_EXTRAS: i32 = eV3DOffscreenDrawFlag::V3D_OFSDRAW_SHOW_OBJECT_EXTRAS as i32;
pub const V3D_OFSDRAW_XR_SHOW_PASSTHROUGH: i32 = eV3DOffscreenDrawFlag::V3D_OFSDRAW_XR_SHOW_PASSTHROUGH as i32;
pub const V3D_OFSDRAW_NO_WORLD_BACKGROUND_OVERRIDE: i32 = eV3DOffscreenDrawFlag::V3D_OFSDRAW_NO_WORLD_BACKGROUND_OVERRIDE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eV3DShadingLightingMode {
    V3D_LIGHTING_FLAT = 0,
    V3D_LIGHTING_STUDIO = 1,
    V3D_LIGHTING_MATCAP = 2,
}

impl Default for eV3DShadingLightingMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const V3D_LIGHTING_FLAT: i32 = eV3DShadingLightingMode::V3D_LIGHTING_FLAT as i32;
pub const V3D_LIGHTING_STUDIO: i32 = eV3DShadingLightingMode::V3D_LIGHTING_STUDIO as i32;
pub const V3D_LIGHTING_MATCAP: i32 = eV3DShadingLightingMode::V3D_LIGHTING_MATCAP as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eV3DShadingColorType {
    V3D_SHADING_MATERIAL_COLOR = 0,
    V3D_SHADING_RANDOM_COLOR = 1,
    V3D_SHADING_SINGLE_COLOR = 2,
    V3D_SHADING_TEXTURE_COLOR = 3,
    V3D_SHADING_OBJECT_COLOR = 4,
    V3D_SHADING_VERTEX_COLOR = 5,
}

impl Default for eV3DShadingColorType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const V3D_SHADING_MATERIAL_COLOR: i32 = eV3DShadingColorType::V3D_SHADING_MATERIAL_COLOR as i32;
pub const V3D_SHADING_RANDOM_COLOR: i32 = eV3DShadingColorType::V3D_SHADING_RANDOM_COLOR as i32;
pub const V3D_SHADING_SINGLE_COLOR: i32 = eV3DShadingColorType::V3D_SHADING_SINGLE_COLOR as i32;
pub const V3D_SHADING_TEXTURE_COLOR: i32 = eV3DShadingColorType::V3D_SHADING_TEXTURE_COLOR as i32;
pub const V3D_SHADING_OBJECT_COLOR: i32 = eV3DShadingColorType::V3D_SHADING_OBJECT_COLOR as i32;
pub const V3D_SHADING_VERTEX_COLOR: i32 = eV3DShadingColorType::V3D_SHADING_VERTEX_COLOR as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eV3DShadingBackgroundType {
    V3D_SHADING_BACKGROUND_THEME = 0,
    V3D_SHADING_BACKGROUND_WORLD = 1,
    V3D_SHADING_BACKGROUND_VIEWPORT = 2,
}

impl Default for eV3DShadingBackgroundType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const V3D_SHADING_BACKGROUND_THEME: i32 = eV3DShadingBackgroundType::V3D_SHADING_BACKGROUND_THEME as i32;
pub const V3D_SHADING_BACKGROUND_WORLD: i32 = eV3DShadingBackgroundType::V3D_SHADING_BACKGROUND_WORLD as i32;
pub const V3D_SHADING_BACKGROUND_VIEWPORT: i32 = eV3DShadingBackgroundType::V3D_SHADING_BACKGROUND_VIEWPORT as i32;

