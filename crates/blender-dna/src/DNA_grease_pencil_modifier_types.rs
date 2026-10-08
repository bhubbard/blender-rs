//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GpencilModifierMode(pub i32);

impl GpencilModifierMode {
    pub const eGpencilModifierMode_Realtime: Self = Self(((1 << 0)) as i32);
    pub const eGpencilModifierMode_Render: Self = Self(((1 << 1)) as i32);
    pub const eGpencilModifierMode_Editmode: Self = Self(((1 << 2)) as i32);
    pub const eGpencilModifierMode_Expanded_DEPRECATED: Self = Self(((1 << 3)) as i32);
    pub const eGpencilModifierMode_Virtual: Self = Self(((1 << 4)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GpencilModifierFlag(pub i16);

impl GpencilModifierFlag {
    pub const eGpencilModifierFlag_OverrideLibrary_Local: Self = Self(((1 << 0)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eGpencilModifierSpace(pub i32);

impl eGpencilModifierSpace {
    pub const GP_SPACE_LOCAL: Self = Self((0) as i32);
    pub const GP_SPACE_WORLD: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GpencilModifierType(pub i32);

impl GpencilModifierType {
    pub const eGpencilModifierType_None: Self = Self((0) as i32);
    pub const eGpencilModifierType_Noise: Self = Self((1) as i32);
    pub const eGpencilModifierType_Subdiv: Self = Self((2) as i32);
    pub const eGpencilModifierType_Thick: Self = Self((3) as i32);
    pub const eGpencilModifierType_Tint: Self = Self((4) as i32);
    pub const eGpencilModifierType_Array: Self = Self((5) as i32);
    pub const eGpencilModifierType_Build: Self = Self((6) as i32);
    pub const eGpencilModifierType_Opacity: Self = Self((7) as i32);
    pub const eGpencilModifierType_Color: Self = Self((8) as i32);
    pub const eGpencilModifierType_Lattice: Self = Self((9) as i32);
    pub const eGpencilModifierType_Simplify: Self = Self((10) as i32);
    pub const eGpencilModifierType_Smooth: Self = Self((11) as i32);
    pub const eGpencilModifierType_Hook: Self = Self((12) as i32);
    pub const eGpencilModifierType_Offset: Self = Self((13) as i32);
    pub const eGpencilModifierType_Mirror: Self = Self((14) as i32);
    pub const eGpencilModifierType_Armature: Self = Self((15) as i32);
    pub const eGpencilModifierType_Time: Self = Self((16) as i32);
    pub const eGpencilModifierType_Multiply: Self = Self((17) as i32);
    pub const eGpencilModifierType_Texture: Self = Self((18) as i32);
    pub const eGpencilModifierType_Lineart: Self = Self((19) as i32);
    pub const eGpencilModifierType_Length: Self = Self((20) as i32);
    pub const eGpencilModifierType_WeightProximity: Self = Self((21) as i32);
    pub const eGpencilModifierType_Dash: Self = Self((22) as i32);
    pub const eGpencilModifierType_WeightAngle: Self = Self((23) as i32);
    pub const eGpencilModifierType_Shrinkwrap: Self = Self((24) as i32);
    pub const eGpencilModifierType_Envelope: Self = Self((25) as i32);
    pub const eGpencilModifierType_Outline: Self = Self((26) as i32);
    pub const NUM_GREASEPENCIL_MODIFIER_TYPES: Self = Self(27 as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSubdivGpencil_Flag(pub i32);

impl eSubdivGpencil_Flag {
    pub const GP_SUBDIV_INVERT_LAYER: Self = Self(((1 << 1)) as i32);
    pub const GP_SUBDIV_INVERT_PASS: Self = Self(((1 << 2)) as i32);
    pub const GP_SUBDIV_INVERT_LAYERPASS: Self = Self(((1 << 3)) as i32);
    pub const GP_SUBDIV_INVERT_MATERIAL: Self = Self(((1 << 4)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSubdivGpencil_Type(pub i16);

impl eSubdivGpencil_Type {
    pub const GP_SUBDIV_CATMULL: Self = Self((0) as i16);
    pub const GP_SUBDIV_SIMPLE: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eThickGpencil_Flag(pub i32);

impl eThickGpencil_Flag {
    pub const GP_THICK_INVERT_LAYER: Self = Self(((1 << 0)) as i32);
    pub const GP_THICK_INVERT_PASS: Self = Self(((1 << 1)) as i32);
    pub const GP_THICK_INVERT_VGROUP: Self = Self(((1 << 2)) as i32);
    pub const GP_THICK_CUSTOM_CURVE: Self = Self(((1 << 3)) as i32);
    pub const GP_THICK_NORMALIZE: Self = Self(((1 << 4)) as i32);
    pub const GP_THICK_INVERT_LAYERPASS: Self = Self(((1 << 5)) as i32);
    pub const GP_THICK_INVERT_MATERIAL: Self = Self(((1 << 6)) as i32);
    pub const GP_THICK_WEIGHT_FACTOR: Self = Self(((1 << 7)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eTimeGpencil_Flag(pub i32);

impl eTimeGpencil_Flag {
    pub const GP_TIME_INVERT_LAYER: Self = Self(((1 << 0)) as i32);
    pub const GP_TIME_KEEP_LOOP: Self = Self(((1 << 1)) as i32);
    pub const GP_TIME_INVERT_LAYERPASS: Self = Self(((1 << 2)) as i32);
    pub const GP_TIME_CUSTOM_RANGE: Self = Self(((1 << 3)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eTimeGpencil_Mode(pub i32);

impl eTimeGpencil_Mode {
    pub const GP_TIME_MODE_NORMAL: Self = Self((0) as i32);
    pub const GP_TIME_MODE_REVERSE: Self = Self((1) as i32);
    pub const GP_TIME_MODE_FIX: Self = Self((2) as i32);
    pub const GP_TIME_MODE_PINGPONG: Self = Self((3) as i32);
    pub const GP_TIME_MODE_CHAIN: Self = Self((4) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eTimeGpencil_Seg_Mode(pub i32);

impl eTimeGpencil_Seg_Mode {
    pub const GP_TIME_SEG_MODE_NORMAL: Self = Self((0) as i32);
    pub const GP_TIME_SEG_MODE_REVERSE: Self = Self((1) as i32);
    pub const GP_TIME_SEG_MODE_PINGPONG: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eModifyColorGpencil_Flag(pub i8);

impl eModifyColorGpencil_Flag {
    pub const GP_MODIFY_COLOR_BOTH: Self = Self((0) as i8);
    pub const GP_MODIFY_COLOR_STROKE: Self = Self((1) as i8);
    pub const GP_MODIFY_COLOR_FILL: Self = Self((2) as i8);
    pub const GP_MODIFY_COLOR_HARDNESS: Self = Self((3) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eOpacityModesGpencil_Flag(pub i32);

impl eOpacityModesGpencil_Flag {
    pub const GP_OPACITY_MODE_MATERIAL: Self = Self((0) as i32);
    pub const GP_OPACITY_MODE_STRENGTH: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eColorGpencil_Flag(pub i32);

impl eColorGpencil_Flag {
    pub const GP_COLOR_INVERT_LAYER: Self = Self(((1 << 1)) as i32);
    pub const GP_COLOR_INVERT_PASS: Self = Self(((1 << 2)) as i32);
    pub const GP_COLOR_INVERT_LAYERPASS: Self = Self(((1 << 3)) as i32);
    pub const GP_COLOR_INVERT_MATERIAL: Self = Self(((1 << 4)) as i32);
    pub const GP_COLOR_CUSTOM_CURVE: Self = Self(((1 << 5)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eOpacityGpencil_Flag(pub i32);

impl eOpacityGpencil_Flag {
    pub const GP_OPACITY_INVERT_LAYER: Self = Self(((1 << 0)) as i32);
    pub const GP_OPACITY_INVERT_PASS: Self = Self(((1 << 1)) as i32);
    pub const GP_OPACITY_INVERT_VGROUP: Self = Self(((1 << 2)) as i32);
    pub const GP_OPACITY_INVERT_LAYERPASS: Self = Self(((1 << 4)) as i32);
    pub const GP_OPACITY_INVERT_MATERIAL: Self = Self(((1 << 5)) as i32);
    pub const GP_OPACITY_CUSTOM_CURVE: Self = Self(((1 << 6)) as i32);
    pub const GP_OPACITY_NORMALIZE: Self = Self(((1 << 7)) as i32);
    pub const GP_OPACITY_WEIGHT_FACTOR: Self = Self(((1 << 8)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eOutlineGpencil_Flag(pub i32);

impl eOutlineGpencil_Flag {
    pub const GP_OUTLINE_INVERT_LAYER: Self = Self(((1 << 0)) as i32);
    pub const GP_OUTLINE_INVERT_PASS: Self = Self(((1 << 1)) as i32);
    pub const GP_OUTLINE_INVERT_LAYERPASS: Self = Self(((1 << 2)) as i32);
    pub const GP_OUTLINE_INVERT_MATERIAL: Self = Self(((1 << 3)) as i32);
    pub const GP_OUTLINE_KEEP_SHAPE: Self = Self(((1 << 4)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBuildGpencil_Mode(pub i16);

impl eBuildGpencil_Mode {
    pub const GP_BUILD_MODE_SEQUENTIAL: Self = Self((0) as i16);
    pub const GP_BUILD_MODE_CONCURRENT: Self = Self((1) as i16);
    pub const GP_BUILD_MODE_ADDITIVE: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBuildGpencil_Transition(pub i16);

impl eBuildGpencil_Transition {
    pub const GP_BUILD_TRANSITION_GROW: Self = Self((0) as i16);
    pub const GP_BUILD_TRANSITION_SHRINK: Self = Self((1) as i16);
    pub const GP_BUILD_TRANSITION_VANISH: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBuildGpencil_TimeAlignment(pub i16);

impl eBuildGpencil_TimeAlignment {
    pub const GP_BUILD_TIMEALIGN_START: Self = Self((0) as i16);
    pub const GP_BUILD_TIMEALIGN_END: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBuildGpencil_TimeMode(pub i16);

impl eBuildGpencil_TimeMode {
    pub const GP_BUILD_TIMEMODE_FRAMES: Self = Self((0) as i16);
    pub const GP_BUILD_TIMEMODE_PERCENTAGE: Self = Self((1) as i16);
    pub const GP_BUILD_TIMEMODE_DRAWSPEED: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBuildGpencil_Flag(pub i16);

impl eBuildGpencil_Flag {
    pub const GP_BUILD_INVERT_LAYER: Self = Self(((1 << 0)) as i16);
    pub const GP_BUILD_INVERT_PASS: Self = Self(((1 << 1)) as i16);
    pub const GP_BUILD_RESTRICT_TIME: Self = Self(((1 << 2)) as i16);
    pub const GP_BUILD_INVERT_LAYERPASS: Self = Self(((1 << 3)) as i16);
    pub const GP_BUILD_USE_FADING: Self = Self(((1 << 4)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eLatticeGpencil_Flag(pub i32);

impl eLatticeGpencil_Flag {
    pub const GP_LATTICE_INVERT_LAYER: Self = Self(((1 << 0)) as i32);
    pub const GP_LATTICE_INVERT_PASS: Self = Self(((1 << 1)) as i32);
    pub const GP_LATTICE_INVERT_VGROUP: Self = Self(((1 << 2)) as i32);
    pub const GP_LATTICE_INVERT_LAYERPASS: Self = Self(((1 << 3)) as i32);
    pub const GP_LATTICE_INVERT_MATERIAL: Self = Self(((1 << 4)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eDashGpencil_Flag(pub i32);

impl eDashGpencil_Flag {
    pub const GP_DASH_INVERT_LAYER: Self = Self(((1 << 0)) as i32);
    pub const GP_DASH_INVERT_PASS: Self = Self(((1 << 1)) as i32);
    pub const GP_DASH_INVERT_LAYERPASS: Self = Self(((1 << 2)) as i32);
    pub const GP_DASH_INVERT_MATERIAL: Self = Self(((1 << 3)) as i32);
    pub const GP_DASH_USE_CYCLIC: Self = Self(((1 << 7)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMirrorGpencil_Flag(pub i32);

impl eMirrorGpencil_Flag {
    pub const GP_MIRROR_INVERT_LAYER: Self = Self(((1 << 0)) as i32);
    pub const GP_MIRROR_INVERT_PASS: Self = Self(((1 << 1)) as i32);
    pub const GP_MIRROR_CLIPPING: Self = Self(((1 << 2)) as i32);
    pub const GP_MIRROR_AXIS_X: Self = Self(((1 << 3)) as i32);
    pub const GP_MIRROR_AXIS_Y: Self = Self(((1 << 4)) as i32);
    pub const GP_MIRROR_AXIS_Z: Self = Self(((1 << 5)) as i32);
    pub const GP_MIRROR_INVERT_LAYERPASS: Self = Self(((1 << 6)) as i32);
    pub const GP_MIRROR_INVERT_MATERIAL: Self = Self(((1 << 7)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eHookGpencil_Flag(pub i32);

impl eHookGpencil_Flag {
    pub const GP_HOOK_INVERT_LAYER: Self = Self(((1 << 0)) as i32);
    pub const GP_HOOK_INVERT_PASS: Self = Self(((1 << 1)) as i32);
    pub const GP_HOOK_INVERT_VGROUP: Self = Self(((1 << 2)) as i32);
    pub const GP_HOOK_UNIFORM_SPACE: Self = Self(((1 << 3)) as i32);
    pub const GP_HOOK_INVERT_LAYERPASS: Self = Self(((1 << 4)) as i32);
    pub const GP_HOOK_INVERT_MATERIAL: Self = Self(((1 << 5)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eHookGpencil_Falloff(pub i8);

impl eHookGpencil_Falloff {
    pub const eGPHook_Falloff_None: Self = Self((0) as i8);
    pub const eGPHook_Falloff_Curve: Self = Self((1) as i8);
    pub const eGPHook_Falloff_Sharp: Self = Self((2) as i8);
    pub const eGPHook_Falloff_Smooth: Self = Self((3) as i8);
    pub const eGPHook_Falloff_Root: Self = Self((4) as i8);
    pub const eGPHook_Falloff_Linear: Self = Self((5) as i8);
    pub const eGPHook_Falloff_Const: Self = Self((6) as i8);
    pub const eGPHook_Falloff_Sphere: Self = Self((7) as i8);
    pub const eGPHook_Falloff_InvSquare: Self = Self((8) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSimplifyGpencil_Flag(pub i32);

impl eSimplifyGpencil_Flag {
    pub const GP_SIMPLIFY_INVERT_LAYER: Self = Self(((1 << 0)) as i32);
    pub const GP_SIMPLIFY_INVERT_PASS: Self = Self(((1 << 1)) as i32);
    pub const GP_SIMPLIFY_INVERT_LAYERPASS: Self = Self(((1 << 2)) as i32);
    pub const GP_SIMPLIFY_INVERT_MATERIAL: Self = Self(((1 << 3)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSimplifyGpencil_Mode(pub i16);

impl eSimplifyGpencil_Mode {
    pub const GP_SIMPLIFY_FIXED: Self = Self((0) as i16);
    pub const GP_SIMPLIFY_ADAPTIVE: Self = Self((1) as i16);
    pub const GP_SIMPLIFY_SAMPLE: Self = Self((2) as i16);
    pub const GP_SIMPLIFY_MERGE: Self = Self((3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eOffsetGpencil_Mode(pub i32);

impl eOffsetGpencil_Mode {
    pub const GP_OFFSET_RANDOM: Self = Self((0) as i32);
    pub const GP_OFFSET_LAYER: Self = Self((1) as i32);
    pub const GP_OFFSET_MATERIAL: Self = Self((2) as i32);
    pub const GP_OFFSET_STROKE: Self = Self((3) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eOffsetGpencil_Flag(pub i32);

impl eOffsetGpencil_Flag {
    pub const GP_OFFSET_INVERT_LAYER: Self = Self(((1 << 0)) as i32);
    pub const GP_OFFSET_INVERT_PASS: Self = Self(((1 << 1)) as i32);
    pub const GP_OFFSET_INVERT_VGROUP: Self = Self(((1 << 2)) as i32);
    pub const GP_OFFSET_INVERT_LAYERPASS: Self = Self(((1 << 3)) as i32);
    pub const GP_OFFSET_INVERT_MATERIAL: Self = Self(((1 << 4)) as i32);
    pub const GP_OFFSET_UNIFORM_RANDOM_SCALE: Self = Self(((1 << 5)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSmoothGpencil_Flag(pub i32);

impl eSmoothGpencil_Flag {
    pub const GP_SMOOTH_MOD_LOCATION: Self = Self(((1 << 0)) as i32);
    pub const GP_SMOOTH_MOD_STRENGTH: Self = Self(((1 << 1)) as i32);
    pub const GP_SMOOTH_MOD_THICKNESS: Self = Self(((1 << 2)) as i32);
    pub const GP_SMOOTH_INVERT_LAYER: Self = Self(((1 << 3)) as i32);
    pub const GP_SMOOTH_INVERT_PASS: Self = Self(((1 << 4)) as i32);
    pub const GP_SMOOTH_INVERT_VGROUP: Self = Self(((1 << 5)) as i32);
    pub const GP_SMOOTH_MOD_UV: Self = Self(((1 << 6)) as i32);
    pub const GP_SMOOTH_INVERT_LAYERPASS: Self = Self(((1 << 7)) as i32);
    pub const GP_SMOOTH_INVERT_MATERIAL: Self = Self(((1 << 4)) as i32);
    pub const GP_SMOOTH_CUSTOM_CURVE: Self = Self(((1 << 8)) as i32);
    pub const GP_SMOOTH_KEEP_SHAPE: Self = Self(((1 << 9)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMultiplyGpencil_Flag(pub i32);

impl eMultiplyGpencil_Flag {
    pub const GP_MULTIPLY_ENABLE_FADING: Self = Self(((1 << 2)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eTintGpencil_Type(pub i32);

impl eTintGpencil_Type {
    pub const GP_TINT_UNIFORM: Self = Self((0) as i32);
    pub const GP_TINT_GRADIENT: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eTintGpencil_Flag(pub i32);

impl eTintGpencil_Flag {
    pub const GP_TINT_INVERT_LAYER: Self = Self(((1 << 0)) as i32);
    pub const GP_TINT_INVERT_PASS: Self = Self(((1 << 1)) as i32);
    pub const GP_TINT_INVERT_VGROUP: Self = Self(((1 << 2)) as i32);
    pub const GP_TINT_INVERT_LAYERPASS: Self = Self(((1 << 4)) as i32);
    pub const GP_TINT_INVERT_MATERIAL: Self = Self(((1 << 5)) as i32);
    pub const GP_TINT_CUSTOM_CURVE: Self = Self(((1 << 6)) as i32);
    pub const GP_TINT_WEIGHT_FACTOR: Self = Self(((1 << 7)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eTextureGpencil_Flag(pub i32);

impl eTextureGpencil_Flag {
    pub const GP_TEX_INVERT_LAYER: Self = Self(((1 << 0)) as i32);
    pub const GP_TEX_INVERT_PASS: Self = Self(((1 << 1)) as i32);
    pub const GP_TEX_INVERT_VGROUP: Self = Self(((1 << 2)) as i32);
    pub const GP_TEX_INVERT_LAYERPASS: Self = Self(((1 << 3)) as i32);
    pub const GP_TEX_INVERT_MATERIAL: Self = Self(((1 << 4)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eTextureGpencil_Mode(pub i16);

impl eTextureGpencil_Mode {
    pub const STROKE: Self = Self((0) as i16);
    pub const FILL: Self = Self((1) as i16);
    pub const STROKE_AND_FILL: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eWeightGpencil_Flag(pub i32);

impl eWeightGpencil_Flag {
    pub const GP_WEIGHT_INVERT_LAYER: Self = Self(((1 << 0)) as i32);
    pub const GP_WEIGHT_INVERT_PASS: Self = Self(((1 << 1)) as i32);
    pub const GP_WEIGHT_INVERT_VGROUP: Self = Self(((1 << 2)) as i32);
    pub const GP_WEIGHT_INVERT_LAYERPASS: Self = Self(((1 << 3)) as i32);
    pub const GP_WEIGHT_INVERT_MATERIAL: Self = Self(((1 << 4)) as i32);
    pub const GP_WEIGHT_MULTIPLY_DATA: Self = Self(((1 << 5)) as i32);
    pub const GP_WEIGHT_INVERT_OUTPUT: Self = Self(((1 << 6)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eLineArtGPencilModifierFlags(pub i32);

impl eLineArtGPencilModifierFlags {
    pub const MOD_LINEART_BINARY_WEIGHTS: Self = Self(((1 << 2)) as i32);
    pub const MOD_LINEART_IS_BAKED: Self = Self(((1 << 3)) as i32);
    pub const MOD_LINEART_USE_CACHE: Self = Self(((1 << 4)) as i32);
    pub const MOD_LINEART_OFFSET_TOWARDS_CUSTOM_CAMERA: Self = Self(((1 << 5)) as i32);
    pub const MOD_LINEART_INVERT_COLLECTION: Self = Self(((1 << 6)) as i32);
    pub const MOD_LINEART_INVERT_SILHOUETTE_FILTER: Self = Self(((1 << 7)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eShrinkwrapGpencil_Flag(pub i32);

impl eShrinkwrapGpencil_Flag {
    pub const GP_SHRINKWRAP_INVERT_LAYER: Self = Self(((1 << 0)) as i32);
    pub const GP_SHRINKWRAP_INVERT_PASS: Self = Self(((1 << 1)) as i32);
    pub const GP_SHRINKWRAP_INVERT_LAYERPASS: Self = Self(((1 << 3)) as i32);
    pub const GP_SHRINKWRAP_INVERT_MATERIAL: Self = Self(((1 << 4)) as i32);
    pub const GP_SHRINKWRAP_INVERT_VGROUP: Self = Self(((1 << 6)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eEnvelopeGpencil_Flag(pub i32);

impl eEnvelopeGpencil_Flag {
    pub const GP_ENVELOPE_INVERT_LAYER: Self = Self(((1 << 0)) as i32);
    pub const GP_ENVELOPE_INVERT_PASS: Self = Self(((1 << 1)) as i32);
    pub const GP_ENVELOPE_INVERT_VGROUP: Self = Self(((1 << 2)) as i32);
    pub const GP_ENVELOPE_INVERT_LAYERPASS: Self = Self(((1 << 3)) as i32);
    pub const GP_ENVELOPE_INVERT_MATERIAL: Self = Self(((1 << 4)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eEnvelopeGpencil_Mode(pub i32);

impl eEnvelopeGpencil_Mode {
    pub const GP_ENVELOPE_DEFORM: Self = Self((0) as i32);
    pub const GP_ENVELOPE_SEGMENTS: Self = Self((1) as i32);
    pub const GP_ENVELOPE_FILLS: Self = Self((2) as i32);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GpencilModifierData {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub r#type: GpencilModifierType,
    pub mode: GpencilModifierMode,
}

impl Default for GpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NoiseGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub material: *mut core::ffi::c_void,
    pub layername: [u8; 64],
    pub vgname: [u8; 64],
    pub pass_index: i32,
    pub flag: i32,
    pub factor: f32,
    pub factor_strength: f32,
    pub factor_thickness: f32,
    pub factor_uvs: f32,
    pub noise_scale: f32,
    pub noise_offset: f32,
    pub noise_mode: i16,
    pub _pad: [u8; 2],
}

impl Default for NoiseGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SubdivGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub material: *mut core::ffi::c_void,
    pub layername: [u8; 64],
    pub pass_index: i32,
    pub flag: eSubdivGpencil_Flag,
}

impl Default for SubdivGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ThickGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub material: *mut core::ffi::c_void,
    pub layername: [u8; 64],
    pub vgname: [u8; 64],
    pub pass_index: i32,
    pub flag: eThickGpencil_Flag,
}

impl Default for ThickGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct TimeGpencilModifierSegment {
    pub name: [u8; 64],
    pub gpmd: *mut core::ffi::c_void,
    pub seg_start: i32,
    pub seg_end: i32,
    pub seg_mode: eTimeGpencil_Seg_Mode,
    pub seg_repeat: i32,
}

impl Default for TimeGpencilModifierSegment {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct TimeGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub material: *mut core::ffi::c_void,
    pub layername: [u8; 64],
    pub layer_pass: i32,
    pub flag: eTimeGpencil_Flag,
    pub offset: i32,
    pub frame_scale: f32,
    pub mode: eTimeGpencil_Mode,
    pub sfra: i32,
    pub efra: i32,
    pub _pad: [u8; 4],
}

impl Default for TimeGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ColorGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub material: *mut core::ffi::c_void,
    pub layername: [u8; 64],
    pub pass_index: i32,
    pub flag: eColorGpencil_Flag,
}

impl Default for ColorGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct OpacityGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub material: *mut core::ffi::c_void,
    pub layername: [u8; 64],
    pub vgname: [u8; 64],
    pub pass_index: i32,
    pub flag: eOpacityGpencil_Flag,
}

impl Default for OpacityGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct OutlineGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub object: *mut core::ffi::c_void,
    pub material: *mut core::ffi::c_void,
    pub layername: [u8; 64],
    pub pass_index: i32,
    pub flag: eOutlineGpencil_Flag,
    pub thickness: i32,
    pub sample_length: f32,
    pub subdiv: i32,
    pub layer_pass: i32,
    pub outline_material: *mut core::ffi::c_void,
}

impl Default for OutlineGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ArrayGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub object: *mut core::ffi::c_void,
    pub material: *mut core::ffi::c_void,
    pub count: i32,
    pub flag: i32,
    pub offset: [f32; 3],
    pub _0: f32,
    pub _0_1: f32,
}

impl Default for ArrayGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BuildGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub material: *mut core::ffi::c_void,
    pub layername: [u8; 64],
    pub pass_index: i32,
    pub layer_pass: i32,
    pub start_frame: f32,
    pub end_frame: f32,
    pub start_delay: f32,
    pub length: f32,
    pub flag: eBuildGpencil_Flag,
}

impl Default for BuildGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct LatticeGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub object: *mut core::ffi::c_void,
    pub material: *mut core::ffi::c_void,
    pub layername: [u8; 64],
    pub vgname: [u8; 64],
    pub pass_index: i32,
    pub flag: eLatticeGpencil_Flag,
}

impl Default for LatticeGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct LengthGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub material: *mut core::ffi::c_void,
    pub layername: [u8; 64],
    pub pass_index: i32,
    pub flag: i32,
    pub layer_pass: i32,
    pub start_fac: f32,
    pub end_fac: f32,
    pub rand_start_fac: f32,
    pub rand_end_fac: f32,
    pub rand_offset: f32,
    pub overshoot_fac: f32,
    pub seed: i32,
    pub step: i32,
    pub mode: i32,
    pub _pad: [u8; 4],
}

impl Default for LengthGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct DashGpencilModifierSegment {
    pub name: [u8; 64],
    pub dmd: *mut core::ffi::c_void,
    pub dash: i32,
    pub gap: i32,
    pub radius: f32,
    pub opacity: f32,
    pub mat_nr: i32,
    pub flag: i32,
}

impl Default for DashGpencilModifierSegment {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct DashGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub material: *mut core::ffi::c_void,
    pub layername: [u8; 64],
    pub pass_index: i32,
    pub flag: eDashGpencil_Flag,
}

impl Default for DashGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MirrorGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub object: *mut core::ffi::c_void,
    pub material: *mut core::ffi::c_void,
    pub layername: [u8; 64],
    pub pass_index: i32,
    pub flag: eMirrorGpencil_Flag,
    pub layer_pass: i32,
    pub _pad: [u8; 4],
}

impl Default for MirrorGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct HookGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub object: *mut core::ffi::c_void,
    pub material: *mut core::ffi::c_void,
    pub subtarget: [u8; 64],
    pub layername: [u8; 64],
    pub vgname: [u8; 64],
    pub pass_index: i32,
    pub layer_pass: i32,
    pub _pad: [u8; 4],
}

impl Default for HookGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SimplifyGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub material: *mut core::ffi::c_void,
    pub layername: [u8; 64],
    pub pass_index: i32,
    pub flag: eSimplifyGpencil_Flag,
}

impl Default for SimplifyGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct OffsetGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub material: *mut core::ffi::c_void,
    pub layername: [u8; 64],
    pub vgname: [u8; 64],
    pub pass_index: i32,
    pub flag: eOffsetGpencil_Flag,
}

impl Default for OffsetGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SmoothGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub material: *mut core::ffi::c_void,
    pub layername: [u8; 64],
    pub vgname: [u8; 64],
    pub pass_index: i32,
    pub flag: eSmoothGpencil_Flag,
    pub factor: f32,
    pub step: i32,
    pub layer_pass: i32,
    pub _pad1: [u8; 4],
}

impl Default for SmoothGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ArmatureGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub deformflag: i16,
    pub multi: i16,
    pub _pad: i32,
}

impl Default for ArmatureGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MultiplyGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub material: *mut core::ffi::c_void,
    pub layername: [u8; 64],
    pub pass_index: i32,
    pub flag: i32,
    pub layer_pass: i32,
    pub flags: eMultiplyGpencil_Flag,
}

impl Default for MultiplyGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct TintGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub object: *mut core::ffi::c_void,
    pub material: *mut core::ffi::c_void,
    pub layername: [u8; 64],
    pub vgname: [u8; 64],
    pub pass_index: i32,
    pub layer_pass: i32,
    pub flag: eTintGpencil_Flag,
}

impl Default for TintGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct TextureGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub material: *mut core::ffi::c_void,
    pub layername: [u8; 64],
    pub vgname: [u8; 64],
    pub pass_index: i32,
    pub flag: eTextureGpencil_Flag,
}

impl Default for TextureGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct WeightProxGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub target_vgname: [u8; 64],
    pub material: *mut core::ffi::c_void,
    pub layername: [u8; 64],
    pub vgname: [u8; 64],
    pub pass_index: i32,
    pub flag: eWeightGpencil_Flag,
}

impl Default for WeightProxGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct WeightAngleGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub target_vgname: [u8; 64],
    pub material: *mut core::ffi::c_void,
    pub layername: [u8; 64],
    pub vgname: [u8; 64],
    pub pass_index: i32,
    pub flag: eWeightGpencil_Flag,
}

impl Default for WeightAngleGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct LineartGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub edge_types: u16,
    pub source_type: i8,
    pub use_multiple_levels: i8,
    pub level_start: i16,
    pub level_end: i16,
    pub source_camera: *mut core::ffi::c_void,
    pub light_contour_object: *mut core::ffi::c_void,
    pub source_object: *mut core::ffi::c_void,
    pub source_collection: *mut core::ffi::c_void,
    pub target_material: *mut core::ffi::c_void,
    pub target_layer: [u8; 64],
    pub source_vertex_group: [u8; 64],
    pub vgname: [u8; 64],
    pub overscan: f32,
    pub shadow_camera_fov: f32,
    pub shadow_camera_size: f32,
    pub shadow_camera_near: f32,
    pub shadow_camera_far: f32,
    pub opacity: f32,
    pub thickness: i16,
    pub mask_switches: u8,
    pub material_mask_bits: u8,
    pub intersection_mask: u8,
    pub shadow_selection: u8,
    pub silhouette_selection: u8,
    pub _pad: [u8; 1],
}

impl Default for LineartGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ShrinkwrapGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub target: *mut core::ffi::c_void,
    pub aux_target: *mut core::ffi::c_void,
    pub material: *mut core::ffi::c_void,
    pub layername: [u8; 64],
    pub vgname: [u8; 64],
    pub pass_index: i32,
    pub flag: eShrinkwrapGpencil_Flag,
}

impl Default for ShrinkwrapGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct EnvelopeGpencilModifierData {
    pub modifier: GpencilModifierData,
    pub material: *mut core::ffi::c_void,
    pub layername: [u8; 64],
    pub vgname: [u8; 64],
    pub pass_index: i32,
    pub flag: eEnvelopeGpencil_Flag,
}

impl Default for EnvelopeGpencilModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

