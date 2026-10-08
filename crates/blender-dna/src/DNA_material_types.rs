//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMaterialGPencilStyle_Flag(pub i16);

impl eMaterialGPencilStyle_Flag {
    pub const GP_MATERIAL_FILL_PATTERN: Self = Self(((1 << 0)) as i16);
    pub const GP_MATERIAL_HIDE: Self = Self(((1 << 1)) as i16);
    pub const GP_MATERIAL_LOCKED: Self = Self(((1 << 2)) as i16);
    pub const GP_MATERIAL_HIDE_ONIONSKIN: Self = Self(((1 << 3)) as i16);
    pub const GP_MATERIAL_TEX_CLAMP: Self = Self(((1 << 4)) as i16);
    pub const GP_MATERIAL_FILL_TEX_MIX: Self = Self(((1 << 5)) as i16);
    pub const GP_MATERIAL_FLIP_FILL: Self = Self(((1 << 6)) as i16);
    pub const GP_MATERIAL_STROKE_PATTERN: Self = Self(((1 << 7)) as i16);
    pub const GP_MATERIAL_STROKE_SHOW: Self = Self(((1 << 8)) as i16);
    pub const GP_MATERIAL_FILL_SHOW: Self = Self(((1 << 9)) as i16);
    pub const GP_MATERIAL_STROKE_TEX_MIX: Self = Self(((1 << 11)) as i16);
    pub const GP_MATERIAL_DISABLE_STENCIL: Self = Self(((1 << 12)) as i16);
    pub const GP_MATERIAL_IS_STROKE_HOLDOUT: Self = Self(((1 << 13)) as i16);
    pub const GP_MATERIAL_IS_FILL_HOLDOUT: Self = Self(((1 << 14)) as i16);
    pub const GP_MATERIAL_USE_DOTS_RANDOMIZATION: Self = Self(14 as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMaterialGPencilStyle_Mode(pub i32);

impl eMaterialGPencilStyle_Mode {
    pub const GP_MATERIAL_MODE_LINE: Self = Self((0) as i32);
    pub const GP_MATERIAL_MODE_DOT: Self = Self((1) as i32);
    pub const GP_MATERIAL_MODE_SQUARE: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMaterialLineArtFlags(pub i32);

impl eMaterialLineArtFlags {
    pub const LRT_MATERIAL_MASK_ENABLED: Self = Self(((1 << 0)) as i32);
    pub const LRT_MATERIAL_CUSTOM_OCCLUSION_EFFECTIVENESS: Self = Self(((1 << 1)) as i32);
    pub const LRT_MATERIAL_CUSTOM_INTERSECTION_PRIORITY: Self = Self(((1 << 2)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMaterial_Flag(pub i16);

impl eMaterial_Flag {
    pub const MA_IS_USED: Self = Self((1 << 0) as i16);
    pub const MA_DS_EXPAND: Self = Self((1 << 1) as i16);
    pub const MA_DS_SHOW_TEXS: Self = Self((1 << 2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMaterial_RampBlend(pub i32);

impl eMaterial_RampBlend {
    pub const MA_RAMP_BLEND: Self = Self((0) as i32);
    pub const MA_RAMP_ADD: Self = Self((1) as i32);
    pub const MA_RAMP_MULT: Self = Self((2) as i32);
    pub const MA_RAMP_SUB: Self = Self((3) as i32);
    pub const MA_RAMP_SCREEN: Self = Self((4) as i32);
    pub const MA_RAMP_DIV: Self = Self((5) as i32);
    pub const MA_RAMP_DIFF: Self = Self((6) as i32);
    pub const MA_RAMP_DARK: Self = Self((7) as i32);
    pub const MA_RAMP_LIGHT: Self = Self((8) as i32);
    pub const MA_RAMP_OVERLAY: Self = Self((9) as i32);
    pub const MA_RAMP_DODGE: Self = Self((10) as i32);
    pub const MA_RAMP_BURN: Self = Self((11) as i32);
    pub const MA_RAMP_HUE: Self = Self((12) as i32);
    pub const MA_RAMP_SAT: Self = Self((13) as i32);
    pub const MA_RAMP_VAL: Self = Self((14) as i32);
    pub const MA_RAMP_COLOR: Self = Self((15) as i32);
    pub const MA_RAMP_SOFT: Self = Self((16) as i32);
    pub const MA_RAMP_LINEAR: Self = Self((17) as i32);
    pub const MA_RAMP_EXCLUSION: Self = Self((18) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMTex_TexCo(pub i32);

impl eMTex_TexCo {
    pub const TEXCO_ORCO: Self = Self((1 << 0) as i32);
    pub const TEXCO_GLOB: Self = Self((1 << 3) as i32);
    pub const TEXCO_UV: Self = Self((1 << 4) as i32);
    pub const TEXCO_OBJECT: Self = Self((1 << 5) as i32);
    pub const TEXCO_WINDOW: Self = Self((1 << 10) as i32);
    pub const TEXCO_STRAND: Self = Self((1 << 13) as i32);
    pub const TEXCO_PARTICLE: Self = Self((1 << 13) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMTex_MapTo(pub i32);

impl eMTex_MapTo {
    pub const MAP_COL: Self = Self((1 << 0) as i32);
    pub const MAP_ALPHA: Self = Self((1 << 7) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePreviewType(pub i8);

impl ePreviewType {
    pub const MA_FLAT: Self = Self((0) as i8);
    pub const MA_SPHERE: Self = Self((1) as i8);
    pub const MA_CUBE: Self = Self((2) as i8);
    pub const MA_SHADERBALL: Self = Self((3) as i8);
    pub const MA_SPHERE_A: Self = Self((4) as i8);
    pub const MA_TEXTURE: Self = Self((5) as i8);
    pub const MA_LAMP: Self = Self((6) as i8);
    pub const MA_SKY: Self = Self((7) as i8);
    pub const MA_HAIR: Self = Self((10) as i8);
    pub const MA_ATMOS: Self = Self((11) as i8);
    pub const MA_CLOTH: Self = Self((12) as i8);
    pub const MA_FLUID: Self = Self((13) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMaterial_PreviewFlag(pub i16);

impl eMaterial_PreviewFlag {
    pub const MA_PREVIEW_WORLD: Self = Self((1 << 0) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMaterial_SurfaceRenderMethod(pub i8);

impl eMaterial_SurfaceRenderMethod {
    pub const MA_SURFACE_METHOD_DEFERRED: Self = Self((0) as i8);
    pub const MA_SURFACE_METHOD_FORWARD: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMaterial_VolumeIntersectionMethod(pub i8);

impl eMaterial_VolumeIntersectionMethod {
    pub const MA_VOLUME_ISECT_FAST: Self = Self((0) as i8);
    pub const MA_VOLUME_ISECT_ACCURATE: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMaterial_BlendMethod(pub i8);

impl eMaterial_BlendMethod {
    pub const MA_BM_SOLID: Self = Self((0) as i8);
    pub const MA_BM_CLIP: Self = Self((3) as i8);
    pub const MA_BM_HASHED: Self = Self((4) as i8);
    pub const MA_BM_BLEND: Self = Self((5) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMaterial_BlendFlag(pub i8);

impl eMaterial_BlendFlag {
    pub const MA_BL_HIDE_BACKFACE: Self = Self(((1 << 0)) as i8);
    pub const MA_BL_SS_REFRACTION: Self = Self(((1 << 1)) as i8);
    pub const MA_BL_CULL_BACKFACE: Self = Self(((1 << 2)) as i8);
    pub const MA_BL_TRANSLUCENCY: Self = Self(((1 << 3)) as i8);
    pub const MA_BL_LIGHTPROBE_VOLUME_DOUBLE_SIDED: Self = Self(((1 << 4)) as i8);
    pub const MA_BL_CULL_BACKFACE_SHADOW: Self = Self(((1 << 5)) as i8);
    pub const MA_BL_TRANSPARENT_SHADOW: Self = Self(((1 << 6)) as i8);
    pub const MA_BL_THICKNESS_FROM_SHADOW: Self = Self(7 as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMaterial_BlendShadow(pub i8);

impl eMaterial_BlendShadow {
    pub const MA_BS_NONE: Self = Self((0) as i8);
    pub const MA_BS_SOLID: Self = Self((1) as i8);
    pub const MA_BS_CLIP: Self = Self((2) as i8);
    pub const MA_BS_HASHED: Self = Self((3) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMaterial_DisplacementMethod(pub i8);

impl eMaterial_DisplacementMethod {
    pub const MA_DISPLACEMENT_BUMP: Self = Self((0) as i8);
    pub const MA_DISPLACEMENT_DISPLACE: Self = Self((1) as i8);
    pub const MA_DISPLACEMENT_BOTH: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMaterial_ThicknessMode(pub i8);

impl eMaterial_ThicknessMode {
    pub const MA_THICKNESS_SPHERE: Self = Self((0) as i8);
    pub const MA_THICKNESS_SLAB: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMaterialGPencilStyle_StrokeStyle(pub i16);

impl eMaterialGPencilStyle_StrokeStyle {
    pub const GP_MATERIAL_STROKE_STYLE_SOLID: Self = Self((0) as i16);
    pub const GP_MATERIAL_STROKE_STYLE_TEXTURE: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMaterialGPencilStyle_FillStyle(pub i16);

impl eMaterialGPencilStyle_FillStyle {
    pub const GP_MATERIAL_FILL_STYLE_SOLID: Self = Self((0) as i16);
    pub const GP_MATERIAL_FILL_STYLE_GRADIENT: Self = Self((1) as i16);
    pub const GP_MATERIAL_FILL_STYLE_CHECKER: Self = Self((2) as i16);
    pub const GP_MATERIAL_FILL_STYLE_TEXTURE: Self = Self((3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMaterialGPencilStyle_GradientType(pub i32);

impl eMaterialGPencilStyle_GradientType {
    pub const GP_MATERIAL_GRADIENT_LINEAR: Self = Self((0) as i32);
    pub const GP_MATERIAL_GRADIENT_RADIAL: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMaterialGPencilStyle_FollowMode(pub i32);

impl eMaterialGPencilStyle_FollowMode {
    pub const GP_MATERIAL_FOLLOW_PATH: Self = Self((0) as i32);
    pub const GP_MATERIAL_FOLLOW_OBJ: Self = Self((1) as i32);
    pub const GP_MATERIAL_FOLLOW_FIXED: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMaterialGPencilPlacementMode(pub i32);

impl eMaterialGPencilPlacementMode {
    pub const GP_MATERIAL_PLACEMENT_COUNT: Self = Self((0) as i32);
    pub const GP_MATERIAL_PLACEMENT_RADIUS: Self = Self((1) as i32);
    pub const GP_MATERIAL_PLACEMENT_DENSITY: Self = Self((2) as i32);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct TexPaintSlot {
    pub image_user: *mut core::ffi::c_void,
    pub uvname: *mut core::ffi::c_void,
    pub attribute_name: *mut core::ffi::c_void,
    pub valid: i32,
    pub interp: i32,
}

impl Default for TexPaintSlot {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MaterialGPencilStyle {
    pub ima: *mut core::ffi::c_void,
    pub stroke_rgba: [f32; 4],
}

impl Default for MaterialGPencilStyle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MaterialLineArt {
    pub flags: eMaterialLineArtFlags,
}

impl Default for MaterialLineArt {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Material {
    pub adt: *mut core::ffi::c_void,
    pub flag: eMaterial_Flag,
}

impl Default for Material {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

