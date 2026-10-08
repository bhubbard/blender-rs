//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eVGroupSelect(pub i8);

impl eVGroupSelect {
    pub const WT_VGROUP_ALL: Self = Self((0) as i8);
    pub const WT_VGROUP_ACTIVE: Self = Self((1) as i8);
    pub const WT_VGROUP_BONE_SELECT: Self = Self((2) as i8);
    pub const WT_VGROUP_BONE_DEFORM: Self = Self((3) as i8);
    pub const WT_VGROUP_BONE_DEFORM_OFF: Self = Self((4) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSeqImageFitMethod(pub i32);

impl eSeqImageFitMethod {
    pub const SEQ_SCALE_TO_FIT: Self = Self(0 as i32);
    pub const SEQ_SCALE_TO_FILL: Self = Self(1 as i32);
    pub const SEQ_STRETCH_TO_FILL: Self = Self(2 as i32);
    pub const SEQ_USE_ORIGINAL_SIZE: Self = Self(3 as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePaintSymmetryFlags(pub i32);

impl ePaintSymmetryFlags {
    pub const PAINT_SYMM_NONE: Self = Self((0) as i32);
    pub const PAINT_SYMM_X: Self = Self(((1 << 0)) as i32);
    pub const PAINT_SYMM_Y: Self = Self(((1 << 1)) as i32);
    pub const PAINT_SYMM_Z: Self = Self(((1 << 2)) as i32);
    pub const PAINT_SYMMETRY_FEATHER: Self = Self(((1 << 3)) as i32);
    pub const PAINT_TILE_X: Self = Self(((1 << 4)) as i32);
    pub const PAINT_TILE_Y: Self = Self(((1 << 5)) as i32);
    pub const PAINT_TILE_Z: Self = Self(((1 << 6)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eUnifiedPaintSettingsFlags(pub i32);

impl eUnifiedPaintSettingsFlags {
    pub const UNIFIED_PAINT_SIZE_DEPRECATED: Self = Self(((1 << 0)) as i32);
    pub const UNIFIED_PAINT_ALPHA_DEPRECATED: Self = Self(((1 << 1)) as i32);
    pub const UNIFIED_PAINT_BRUSH_LOCK_SIZE: Self = Self(((1 << 2)) as i32);
    pub const UNIFIED_PAINT_FLAG_UNUSED_0: Self = Self(((1 << 3)) as i32);
    pub const UNIFIED_PAINT_FLAG_UNUSED_1: Self = Self(((1 << 4)) as i32);
    pub const UNIFIED_PAINT_WEIGHT_DEPRECATED: Self = Self(((1 << 5)) as i32);
    pub const UNIFIED_PAINT_COLOR_DEPRECATED: Self = Self(((1 << 6)) as i32);
    pub const UNIFIED_PAINT_INPUT_SAMPLES_DEPRECATED: Self = Self(((1 << 7)) as i32);
    pub const UNIFIED_PAINT_COLOR_JITTER: Self = Self(((1 << 8)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PaintCurveVisibilityFlags(pub i32);

impl PaintCurveVisibilityFlags {
    pub const PAINT_CURVE_SHOW_STRENGTH: Self = Self(((1 << 0)) as i32);
    pub const PAINT_CURVE_SHOW_SIZE: Self = Self(((1 << 1)) as i32);
    pub const PAINT_CURVE_SHOW_JITTER: Self = Self(((1 << 2)) as i32);
    pub const PAINT_CURVE_SHOW_HARDNESS: Self = Self(((1 << 3)) as i32);
    pub const PAINT_CURVE_SHOW_AUTO_SMOOTH: Self = Self(((1 << 4)) as i32);
    pub const PAINT_CURVE_SHOW_SPACING: Self = Self(((1 << 5)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eScenePassType(pub u32);

impl eScenePassType {
    pub const SCE_PASS_COMBINED: Self = Self(((1 << 0)) as u32);
    pub const SCE_PASS_DEPTH: Self = Self(((1 << 1)) as u32);
    pub const SCE_PASS_UNUSED_1: Self = Self(((1 << 2)) as u32);
    pub const SCE_PASS_UNUSED_2: Self = Self(((1 << 3)) as u32);
    pub const SCE_PASS_UNUSED_3: Self = Self(((1 << 4)) as u32);
    pub const SCE_PASS_SHADOW: Self = Self(((1 << 5)) as u32);
    pub const SCE_PASS_AO: Self = Self(((1 << 6)) as u32);
    pub const SCE_PASS_POSITION: Self = Self(((1 << 7)) as u32);
    pub const SCE_PASS_NORMAL: Self = Self(((1 << 8)) as u32);
    pub const SCE_PASS_VECTOR: Self = Self(((1 << 9)) as u32);
    pub const SCE_PASS_UNUSED_5: Self = Self(((1 << 10)) as u32);
    pub const SCE_PASS_INDEXOB: Self = Self(((1 << 11)) as u32);
    pub const SCE_PASS_UV: Self = Self(((1 << 12)) as u32);
    pub const SCE_PASS_UNUSED_6: Self = Self(((1 << 13)) as u32);
    pub const SCE_PASS_MIST: Self = Self(((1 << 14)) as u32);
    pub const SCE_PASS_UNUSED_7: Self = Self(((1 << 15)) as u32);
    pub const SCE_PASS_EMIT: Self = Self(((1 << 16)) as u32);
    pub const SCE_PASS_ENVIRONMENT: Self = Self(((1 << 17)) as u32);
    pub const SCE_PASS_INDEXMA: Self = Self(((1 << 18)) as u32);
    pub const SCE_PASS_DIFFUSE_DIRECT: Self = Self(((1 << 19)) as u32);
    pub const SCE_PASS_DIFFUSE_INDIRECT: Self = Self(((1 << 20)) as u32);
    pub const SCE_PASS_DIFFUSE_COLOR: Self = Self(((1 << 21)) as u32);
    pub const SCE_PASS_GLOSSY_DIRECT: Self = Self(((1 << 22)) as u32);
    pub const SCE_PASS_GLOSSY_INDIRECT: Self = Self(((1 << 23)) as u32);
    pub const SCE_PASS_GLOSSY_COLOR: Self = Self(((1 << 24)) as u32);
    pub const SCE_PASS_TRANSM_DIRECT: Self = Self(((1 << 25)) as u32);
    pub const SCE_PASS_TRANSM_INDIRECT: Self = Self(((1 << 26)) as u32);
    pub const SCE_PASS_TRANSM_COLOR: Self = Self(((1 << 27)) as u32);
    pub const SCE_PASS_SUBSURFACE_DIRECT: Self = Self(((1 << 28)) as u32);
    pub const SCE_PASS_SUBSURFACE_INDIRECT: Self = Self(((1 << 29)) as u32);
    pub const SCE_PASS_SUBSURFACE_COLOR: Self = Self(((1 << 30)) as u32);
    pub const SCE_PASS_ROUGHNESS: Self = Self(31 as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSceneLayer_Flag(pub i32);

impl eSceneLayer_Flag {
    pub const SCE_LAY_SOLID: Self = Self((1 << 0) as i32);
    pub const SCE_LAY_UNUSED_1: Self = Self((1 << 1) as i32);
    pub const SCE_LAY_UNUSED_2: Self = Self((1 << 2) as i32);
    pub const SCE_LAY_UNUSED_3: Self = Self((1 << 3) as i32);
    pub const SCE_LAY_SKY: Self = Self((1 << 4) as i32);
    pub const SCE_LAY_STRAND: Self = Self((1 << 5) as i32);
    pub const SCE_LAY_FRS: Self = Self((1 << 6) as i32);
    pub const SCE_LAY_AO: Self = Self((1 << 7) as i32);
    pub const SCE_LAY_VOLUMES: Self = Self((1 << 8) as i32);
    pub const SCE_LAY_MOTION_BLUR: Self = Self((1 << 9) as i32);
    pub const SCE_LAY_GREASE_PENCIL: Self = Self((1 << 10) as i32);
    pub const SCE_LAY_FLAG_DEFAULT: Self = Self((((1 << 15) - 1)) as i32);
    pub const SCE_LAY_UNUSED_4: Self = Self((1 << 15) as i32);
    pub const SCE_LAY_UNUSED_5: Self = Self((1 << 16) as i32);
    pub const SCE_LAY_DISABLE: Self = Self((1 << 17) as i32);
    pub const SCE_LAY_UNUSED_6: Self = Self((1 << 18) as i32);
    pub const SCE_LAY_UNUSED_7: Self = Self((1 << 19) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSceneView_Flag(pub i32);

impl eSceneView_Flag {
    pub const SCE_VIEW_DISABLE: Self = Self((1 << 0) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSceneViews_Format(pub i8);

impl eSceneViews_Format {
    pub const SCE_VIEWS_FORMAT_STEREO_3D: Self = Self((0) as i8);
    pub const SCE_VIEWS_FORMAT_MULTIVIEW: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eImageFormat_ViewsFormat(pub i8);

impl eImageFormat_ViewsFormat {
    pub const R_IMF_VIEWS_INDIVIDUAL: Self = Self((0) as i8);
    pub const R_IMF_VIEWS_STEREO_3D: Self = Self((1) as i8);
    pub const R_IMF_VIEWS_MULTIVIEW: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eStereoDisplayMode(pub i8);

impl eStereoDisplayMode {
    pub const S3D_DISPLAY_ANAGLYPH: Self = Self((0) as i8);
    pub const S3D_DISPLAY_INTERLACE: Self = Self((1) as i8);
    pub const S3D_DISPLAY_PAGEFLIP: Self = Self((2) as i8);
    pub const S3D_DISPLAY_SIDEBYSIDE: Self = Self((3) as i8);
    pub const S3D_DISPLAY_TOPBOTTOM: Self = Self((4) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eStereo3dFlag(pub i16);

impl eStereo3dFlag {
    pub const S3D_INTERLACE_SWAP: Self = Self(((1 << 0)) as i16);
    pub const S3D_SIDEBYSIDE_CROSSEYED: Self = Self(((1 << 1)) as i16);
    pub const S3D_SQUEEZED_FRAME: Self = Self(((1 << 2)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eStereo3dAnaglyphType(pub i8);

impl eStereo3dAnaglyphType {
    pub const S3D_ANAGLYPH_REDCYAN: Self = Self((0) as i8);
    pub const S3D_ANAGLYPH_GREENMAGENTA: Self = Self((1) as i8);
    pub const S3D_ANAGLYPH_YELLOWBLUE: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eStereo3dInterlaceType(pub i8);

impl eStereo3dInterlaceType {
    pub const S3D_INTERLACE_ROW: Self = Self((0) as i8);
    pub const S3D_INTERLACE_COLUMN: Self = Self((1) as i8);
    pub const S3D_INTERLACE_CHECKERBOARD: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eStereoViews(pub i8);

impl eStereoViews {
    pub const STEREO_LEFT_ID: Self = Self((0) as i8);
    pub const STEREO_RIGHT_ID: Self = Self((1) as i8);
    pub const STEREO_3D_ID: Self = Self((2) as i8);
    pub const STEREO_MONO_ID: Self = Self((3) as i8);
}

