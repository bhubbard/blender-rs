//! Auto-transpiled C/C++ header module: DNA_modifier_enums

use crate::*;

pub const DT_TYPE_MAX: i32 = 30;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eArrayGpencil_Flag {
    GP_ARRAY_INVERT_LAYER = (1 << 2),
    GP_ARRAY_INVERT_PASS = (1 << 3),
    GP_ARRAY_INVERT_LAYERPASS = (1 << 5),
    GP_ARRAY_INVERT_MATERIAL = (1 << 6),
    GP_ARRAY_USE_OFFSET = (1 << 7),
    GP_ARRAY_USE_RELATIVE = (1 << 8),
    GP_ARRAY_USE_OB_OFFSET = (1 << 9),
    GP_ARRAY_UNIFORM_RANDOM_SCALE = (1 << 10),
}

impl Default for eArrayGpencil_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GP_ARRAY_INVERT_LAYER: i32 = eArrayGpencil_Flag::GP_ARRAY_INVERT_LAYER as i32;
pub const GP_ARRAY_INVERT_PASS: i32 = eArrayGpencil_Flag::GP_ARRAY_INVERT_PASS as i32;
pub const GP_ARRAY_INVERT_LAYERPASS: i32 = eArrayGpencil_Flag::GP_ARRAY_INVERT_LAYERPASS as i32;
pub const GP_ARRAY_INVERT_MATERIAL: i32 = eArrayGpencil_Flag::GP_ARRAY_INVERT_MATERIAL as i32;
pub const GP_ARRAY_USE_OFFSET: i32 = eArrayGpencil_Flag::GP_ARRAY_USE_OFFSET as i32;
pub const GP_ARRAY_USE_RELATIVE: i32 = eArrayGpencil_Flag::GP_ARRAY_USE_RELATIVE as i32;
pub const GP_ARRAY_USE_OB_OFFSET: i32 = eArrayGpencil_Flag::GP_ARRAY_USE_OB_OFFSET as i32;
pub const GP_ARRAY_UNIFORM_RANDOM_SCALE: i32 = eArrayGpencil_Flag::GP_ARRAY_UNIFORM_RANDOM_SCALE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTextureGpencil_Fit {
    GP_TEX_FIT_STROKE = 0,
    GP_TEX_CONSTANT_LENGTH = 1,
}

impl Default for eTextureGpencil_Fit {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GP_TEX_FIT_STROKE: i32 = eTextureGpencil_Fit::GP_TEX_FIT_STROKE as i32;
pub const GP_TEX_CONSTANT_LENGTH: i32 = eTextureGpencil_Fit::GP_TEX_CONSTANT_LENGTH as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eNoiseGpencil_Flag {
    GP_NOISE_USE_RANDOM = (1 << 0),
    GP_NOISE_MOD_LOCATION = (1 << 1),
    GP_NOISE_MOD_STRENGTH = (1 << 2),
    GP_NOISE_MOD_THICKNESS = (1 << 3),
    GP_NOISE_FULL_STROKE = (1 << 4),
    GP_NOISE_CUSTOM_CURVE = (1 << 5),
    GP_NOISE_INVERT_LAYER = (1 << 6),
    GP_NOISE_INVERT_PASS = (1 << 7),
    GP_NOISE_INVERT_VGROUP = (1 << 8),
    GP_NOISE_MOD_UV = (1 << 9),
    GP_NOISE_INVERT_LAYERPASS = (1 << 10),
    GP_NOISE_INVERT_MATERIAL = (1 << 11),
}

impl Default for eNoiseGpencil_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GP_NOISE_USE_RANDOM: i32 = eNoiseGpencil_Flag::GP_NOISE_USE_RANDOM as i32;
pub const GP_NOISE_MOD_LOCATION: i32 = eNoiseGpencil_Flag::GP_NOISE_MOD_LOCATION as i32;
pub const GP_NOISE_MOD_STRENGTH: i32 = eNoiseGpencil_Flag::GP_NOISE_MOD_STRENGTH as i32;
pub const GP_NOISE_MOD_THICKNESS: i32 = eNoiseGpencil_Flag::GP_NOISE_MOD_THICKNESS as i32;
pub const GP_NOISE_FULL_STROKE: i32 = eNoiseGpencil_Flag::GP_NOISE_FULL_STROKE as i32;
pub const GP_NOISE_CUSTOM_CURVE: i32 = eNoiseGpencil_Flag::GP_NOISE_CUSTOM_CURVE as i32;
pub const GP_NOISE_INVERT_LAYER: i32 = eNoiseGpencil_Flag::GP_NOISE_INVERT_LAYER as i32;
pub const GP_NOISE_INVERT_PASS: i32 = eNoiseGpencil_Flag::GP_NOISE_INVERT_PASS as i32;
pub const GP_NOISE_INVERT_VGROUP: i32 = eNoiseGpencil_Flag::GP_NOISE_INVERT_VGROUP as i32;
pub const GP_NOISE_MOD_UV: i32 = eNoiseGpencil_Flag::GP_NOISE_MOD_UV as i32;
pub const GP_NOISE_INVERT_LAYERPASS: i32 = eNoiseGpencil_Flag::GP_NOISE_INVERT_LAYERPASS as i32;
pub const GP_NOISE_INVERT_MATERIAL: i32 = eNoiseGpencil_Flag::GP_NOISE_INVERT_MATERIAL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eNoiseRandomGpencil_Mode {
    GP_NOISE_RANDOM_STEP = 0,
    GP_NOISE_RANDOM_KEYFRAME = 1,
}

impl Default for eNoiseRandomGpencil_Mode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GP_NOISE_RANDOM_STEP: i32 = eNoiseRandomGpencil_Mode::GP_NOISE_RANDOM_STEP as i32;
pub const GP_NOISE_RANDOM_KEYFRAME: i32 = eNoiseRandomGpencil_Mode::GP_NOISE_RANDOM_KEYFRAME as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLengthGpencil_Flag {
    GP_LENGTH_INVERT_LAYER = (1 << 0),
    GP_LENGTH_INVERT_PASS = (1 << 1),
    GP_LENGTH_INVERT_LAYERPASS = (1 << 2),
    GP_LENGTH_INVERT_MATERIAL = (1 << 3),
    GP_LENGTH_USE_CURVATURE = (1 << 4),
    GP_LENGTH_INVERT_CURVATURE = (1 << 5),
    GP_LENGTH_USE_RANDOM = (1 << 6),
}

impl Default for eLengthGpencil_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GP_LENGTH_INVERT_LAYER: i32 = eLengthGpencil_Flag::GP_LENGTH_INVERT_LAYER as i32;
pub const GP_LENGTH_INVERT_PASS: i32 = eLengthGpencil_Flag::GP_LENGTH_INVERT_PASS as i32;
pub const GP_LENGTH_INVERT_LAYERPASS: i32 = eLengthGpencil_Flag::GP_LENGTH_INVERT_LAYERPASS as i32;
pub const GP_LENGTH_INVERT_MATERIAL: i32 = eLengthGpencil_Flag::GP_LENGTH_INVERT_MATERIAL as i32;
pub const GP_LENGTH_USE_CURVATURE: i32 = eLengthGpencil_Flag::GP_LENGTH_USE_CURVATURE as i32;
pub const GP_LENGTH_INVERT_CURVATURE: i32 = eLengthGpencil_Flag::GP_LENGTH_INVERT_CURVATURE as i32;
pub const GP_LENGTH_USE_RANDOM: i32 = eLengthGpencil_Flag::GP_LENGTH_USE_RANDOM as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLengthGpencil_Type {
    GP_LENGTH_RELATIVE = 0,
    GP_LENGTH_ABSOLUTE = 1,
}

impl Default for eLengthGpencil_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GP_LENGTH_RELATIVE: i32 = eLengthGpencil_Type::GP_LENGTH_RELATIVE as i32;
pub const GP_LENGTH_ABSOLUTE: i32 = eLengthGpencil_Type::GP_LENGTH_ABSOLUTE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eShrinkwrap_Type {
    MOD_SHRINKWRAP_NEAREST_SURFACE = 0,
    MOD_SHRINKWRAP_PROJECT = 1,
    MOD_SHRINKWRAP_NEAREST_VERTEX = 2,
    MOD_SHRINKWRAP_TARGET_PROJECT = 3,
}

impl Default for eShrinkwrap_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MOD_SHRINKWRAP_NEAREST_SURFACE: i32 = eShrinkwrap_Type::MOD_SHRINKWRAP_NEAREST_SURFACE as i32;
pub const MOD_SHRINKWRAP_PROJECT: i32 = eShrinkwrap_Type::MOD_SHRINKWRAP_PROJECT as i32;
pub const MOD_SHRINKWRAP_NEAREST_VERTEX: i32 = eShrinkwrap_Type::MOD_SHRINKWRAP_NEAREST_VERTEX as i32;
pub const MOD_SHRINKWRAP_TARGET_PROJECT: i32 = eShrinkwrap_Type::MOD_SHRINKWRAP_TARGET_PROJECT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eShrinkwrap_Mode {
    MOD_SHRINKWRAP_ON_SURFACE = 0,
    MOD_SHRINKWRAP_INSIDE = 1,
    MOD_SHRINKWRAP_OUTSIDE = 2,
    MOD_SHRINKWRAP_OUTSIDE_SURFACE = 3,
    MOD_SHRINKWRAP_ABOVE_SURFACE = 4,
}

impl Default for eShrinkwrap_Mode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MOD_SHRINKWRAP_ON_SURFACE: i32 = eShrinkwrap_Mode::MOD_SHRINKWRAP_ON_SURFACE as i32;
pub const MOD_SHRINKWRAP_INSIDE: i32 = eShrinkwrap_Mode::MOD_SHRINKWRAP_INSIDE as i32;
pub const MOD_SHRINKWRAP_OUTSIDE: i32 = eShrinkwrap_Mode::MOD_SHRINKWRAP_OUTSIDE as i32;
pub const MOD_SHRINKWRAP_OUTSIDE_SURFACE: i32 = eShrinkwrap_Mode::MOD_SHRINKWRAP_OUTSIDE_SURFACE as i32;
pub const MOD_SHRINKWRAP_ABOVE_SURFACE: i32 = eShrinkwrap_Mode::MOD_SHRINKWRAP_ABOVE_SURFACE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eShrinkwrap_Opts {
    MOD_SHRINKWRAP_PROJECT_ALLOW_POS_DIR = (1 << 0),
    MOD_SHRINKWRAP_PROJECT_ALLOW_NEG_DIR = (1 << 1),
    MOD_SHRINKWRAP_CULL_TARGET_FRONTFACE = (1 << 3),
    MOD_SHRINKWRAP_CULL_TARGET_BACKFACE = (1 << 4),
    MOD_SHRINKWRAP_KEEP_ABOVE_SURFACE = (1 << 5),
    MOD_SHRINKWRAP_INVERT_VGROUP = (1 << 6),
    MOD_SHRINKWRAP_INVERT_CULL_TARGET = (1 << 7),
}

impl Default for eShrinkwrap_Opts {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MOD_SHRINKWRAP_PROJECT_ALLOW_POS_DIR: i32 = eShrinkwrap_Opts::MOD_SHRINKWRAP_PROJECT_ALLOW_POS_DIR as i32;
pub const MOD_SHRINKWRAP_PROJECT_ALLOW_NEG_DIR: i32 = eShrinkwrap_Opts::MOD_SHRINKWRAP_PROJECT_ALLOW_NEG_DIR as i32;
pub const MOD_SHRINKWRAP_CULL_TARGET_FRONTFACE: i32 = eShrinkwrap_Opts::MOD_SHRINKWRAP_CULL_TARGET_FRONTFACE as i32;
pub const MOD_SHRINKWRAP_CULL_TARGET_BACKFACE: i32 = eShrinkwrap_Opts::MOD_SHRINKWRAP_CULL_TARGET_BACKFACE as i32;
pub const MOD_SHRINKWRAP_KEEP_ABOVE_SURFACE: i32 = eShrinkwrap_Opts::MOD_SHRINKWRAP_KEEP_ABOVE_SURFACE as i32;
pub const MOD_SHRINKWRAP_INVERT_VGROUP: i32 = eShrinkwrap_Opts::MOD_SHRINKWRAP_INVERT_VGROUP as i32;
pub const MOD_SHRINKWRAP_INVERT_CULL_TARGET: i32 = eShrinkwrap_Opts::MOD_SHRINKWRAP_INVERT_CULL_TARGET as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eShrinkwrap_ProjAxis {
    MOD_SHRINKWRAP_PROJECT_OVER_NORMAL = 0,
    MOD_SHRINKWRAP_PROJECT_OVER_X_AXIS = (1 << 0),
    MOD_SHRINKWRAP_PROJECT_OVER_Y_AXIS = (1 << 1),
    MOD_SHRINKWRAP_PROJECT_OVER_Z_AXIS = (1 << 2),
}

impl Default for eShrinkwrap_ProjAxis {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MOD_SHRINKWRAP_PROJECT_OVER_NORMAL: i32 = eShrinkwrap_ProjAxis::MOD_SHRINKWRAP_PROJECT_OVER_NORMAL as i32;
pub const MOD_SHRINKWRAP_PROJECT_OVER_X_AXIS: i32 = eShrinkwrap_ProjAxis::MOD_SHRINKWRAP_PROJECT_OVER_X_AXIS as i32;
pub const MOD_SHRINKWRAP_PROJECT_OVER_Y_AXIS: i32 = eShrinkwrap_ProjAxis::MOD_SHRINKWRAP_PROJECT_OVER_Y_AXIS as i32;
pub const MOD_SHRINKWRAP_PROJECT_OVER_Z_AXIS: i32 = eShrinkwrap_ProjAxis::MOD_SHRINKWRAP_PROJECT_OVER_Z_AXIS as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDTType {
    DT_TYPE_MDEFORMVERT = 1 << 0,
    DT_TYPE_SKIN = 1 << 2,
    DT_TYPE_BWEIGHT_VERT = 1 << 3,
    DT_TYPE_SHARP_EDGE = 1 << 8,
    DT_TYPE_SEAM = 1 << 9,
    DT_TYPE_CREASE = 1 << 10,
    DT_TYPE_BWEIGHT_EDGE = 1 << 11,
    DT_TYPE_FREESTYLE_EDGE = 1 << 12,
    DT_TYPE_MPROPCOL_VERT = 1 << 16,
    DT_TYPE_LNOR = 1 << 17,
    DT_TYPE_UV = 1 << 24,
    DT_TYPE_SHARP_FACE = 1 << 25,
    DT_TYPE_FREESTYLE_FACE = 1 << 26,
    DT_TYPE_MLOOPCOL_VERT = 1 << 27,
    DT_TYPE_MPROPCOL_LOOP = 1 << 28,
    DT_TYPE_MLOOPCOL_LOOP = 1 << 29,
    DT_TYPE_VCOL_ALL = (1 << 16) | (1 << 27) | (1 << 28) | (1 << 29),
    DT_TYPE_VERT_ALL = DT_TYPE_MDEFORMVERT | DT_TYPE_SKIN | DT_TYPE_BWEIGHT_VERT | DT_TYPE_MPROPCOL_VERT | DT_TYPE_MLOOPCOL_VERT,
    DT_TYPE_EDGE_ALL = DT_TYPE_SHARP_EDGE | DT_TYPE_SEAM | DT_TYPE_CREASE | DT_TYPE_BWEIGHT_EDGE | DT_TYPE_FREESTYLE_EDGE,
    DT_TYPE_LOOP_ALL = DT_TYPE_LNOR | DT_TYPE_UV | DT_TYPE_MPROPCOL_LOOP | DT_TYPE_MLOOPCOL_LOOP,
    DT_TYPE_POLY_ALL = DT_TYPE_UV | DT_TYPE_SHARP_FACE | DT_TYPE_FREESTYLE_FACE,
}

impl Default for eDTType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const DT_TYPE_MDEFORMVERT: i32 = eDTType::DT_TYPE_MDEFORMVERT as i32;
pub const DT_TYPE_SKIN: i32 = eDTType::DT_TYPE_SKIN as i32;
pub const DT_TYPE_BWEIGHT_VERT: i32 = eDTType::DT_TYPE_BWEIGHT_VERT as i32;
pub const DT_TYPE_SHARP_EDGE: i32 = eDTType::DT_TYPE_SHARP_EDGE as i32;
pub const DT_TYPE_SEAM: i32 = eDTType::DT_TYPE_SEAM as i32;
pub const DT_TYPE_CREASE: i32 = eDTType::DT_TYPE_CREASE as i32;
pub const DT_TYPE_BWEIGHT_EDGE: i32 = eDTType::DT_TYPE_BWEIGHT_EDGE as i32;
pub const DT_TYPE_FREESTYLE_EDGE: i32 = eDTType::DT_TYPE_FREESTYLE_EDGE as i32;
pub const DT_TYPE_MPROPCOL_VERT: i32 = eDTType::DT_TYPE_MPROPCOL_VERT as i32;
pub const DT_TYPE_LNOR: i32 = eDTType::DT_TYPE_LNOR as i32;
pub const DT_TYPE_UV: i32 = eDTType::DT_TYPE_UV as i32;
pub const DT_TYPE_SHARP_FACE: i32 = eDTType::DT_TYPE_SHARP_FACE as i32;
pub const DT_TYPE_FREESTYLE_FACE: i32 = eDTType::DT_TYPE_FREESTYLE_FACE as i32;
pub const DT_TYPE_MLOOPCOL_VERT: i32 = eDTType::DT_TYPE_MLOOPCOL_VERT as i32;
pub const DT_TYPE_MPROPCOL_LOOP: i32 = eDTType::DT_TYPE_MPROPCOL_LOOP as i32;
pub const DT_TYPE_MLOOPCOL_LOOP: i32 = eDTType::DT_TYPE_MLOOPCOL_LOOP as i32;
pub const DT_TYPE_VCOL_ALL: i32 = eDTType::DT_TYPE_VCOL_ALL as i32;
pub const DT_TYPE_VERT_ALL: i32 = eDTType::DT_TYPE_VERT_ALL as i32;
pub const DT_TYPE_EDGE_ALL: i32 = eDTType::DT_TYPE_EDGE_ALL as i32;
pub const DT_TYPE_LOOP_ALL: i32 = eDTType::DT_TYPE_LOOP_ALL as i32;
pub const DT_TYPE_POLY_ALL: i32 = eDTType::DT_TYPE_POLY_ALL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDTMultilayerIndex {
    DT_MULTILAYER_INDEX_INVALID = -1,
    DT_MULTILAYER_INDEX_MDEFORMVERT = 0,
    DT_MULTILAYER_INDEX_VCOL_LOOP = 2,
    DT_MULTILAYER_INDEX_UV = 3,
    DT_MULTILAYER_INDEX_VCOL_VERT = 4,
    DT_MULTILAYER_INDEX_MAX = 5,
}

impl Default for eDTMultilayerIndex {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const DT_MULTILAYER_INDEX_INVALID: i32 = eDTMultilayerIndex::DT_MULTILAYER_INDEX_INVALID as i32;
pub const DT_MULTILAYER_INDEX_MDEFORMVERT: i32 = eDTMultilayerIndex::DT_MULTILAYER_INDEX_MDEFORMVERT as i32;
pub const DT_MULTILAYER_INDEX_VCOL_LOOP: i32 = eDTMultilayerIndex::DT_MULTILAYER_INDEX_VCOL_LOOP as i32;
pub const DT_MULTILAYER_INDEX_UV: i32 = eDTMultilayerIndex::DT_MULTILAYER_INDEX_UV as i32;
pub const DT_MULTILAYER_INDEX_VCOL_VERT: i32 = eDTMultilayerIndex::DT_MULTILAYER_INDEX_VCOL_VERT as i32;
pub const DT_MULTILAYER_INDEX_MAX: i32 = eDTMultilayerIndex::DT_MULTILAYER_INDEX_MAX as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDTLayersSrc {
    DT_LAYERS_ACTIVE_SRC = -1,
    DT_LAYERS_ALL_SRC = -2,
    DT_LAYERS_VGROUP_SRC = 1 << 8,
    DT_LAYERS_VGROUP_SRC_BONE_SELECT = -(DT_LAYERS_VGROUP_SRC | 1),
    DT_LAYERS_VGROUP_SRC_BONE_DEFORM = -(DT_LAYERS_VGROUP_SRC | 2),
}

impl Default for eDTLayersSrc {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const DT_LAYERS_ACTIVE_SRC: i32 = eDTLayersSrc::DT_LAYERS_ACTIVE_SRC as i32;
pub const DT_LAYERS_ALL_SRC: i32 = eDTLayersSrc::DT_LAYERS_ALL_SRC as i32;
pub const DT_LAYERS_VGROUP_SRC: i32 = eDTLayersSrc::DT_LAYERS_VGROUP_SRC as i32;
pub const DT_LAYERS_VGROUP_SRC_BONE_SELECT: i32 = eDTLayersSrc::DT_LAYERS_VGROUP_SRC_BONE_SELECT as i32;
pub const DT_LAYERS_VGROUP_SRC_BONE_DEFORM: i32 = eDTLayersSrc::DT_LAYERS_VGROUP_SRC_BONE_DEFORM as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDTLayersDst {
    DT_LAYERS_ACTIVE_DST = -1,
    DT_LAYERS_NAME_DST = -2,
    DT_LAYERS_INDEX_DST = -3,
    DT_LAYERS_CREATE_DST = -4,
}

impl Default for eDTLayersDst {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const DT_LAYERS_ACTIVE_DST: i32 = eDTLayersDst::DT_LAYERS_ACTIVE_DST as i32;
pub const DT_LAYERS_NAME_DST: i32 = eDTLayersDst::DT_LAYERS_NAME_DST as i32;
pub const DT_LAYERS_INDEX_DST: i32 = eDTLayersDst::DT_LAYERS_INDEX_DST as i32;
pub const DT_LAYERS_CREATE_DST: i32 = eDTLayersDst::DT_LAYERS_CREATE_DST as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMRemapMode {
    MREMAP_USE_VERT = 1 << 4,
    MREMAP_USE_EDGE = 1 << 5,
    MREMAP_USE_LOOP = 1 << 6,
    MREMAP_USE_POLY = 1 << 7,
    MREMAP_USE_NEAREST = 1 << 8,
    MREMAP_USE_NORPROJ = 1 << 9,
    MREMAP_USE_INTERP = 1 << 10,
    MREMAP_USE_NORMAL = 1 << 11,
    MREMAP_MODE_VERT = 1 << 24,
    MREMAP_MODE_VERT_NEAREST = MREMAP_MODE_VERT | MREMAP_USE_VERT | MREMAP_USE_NEAREST,
    MREMAP_MODE_VERT_EDGE_NEAREST = MREMAP_MODE_VERT | MREMAP_USE_EDGE | MREMAP_USE_NEAREST,
    MREMAP_MODE_VERT_EDGEINTERP_NEAREST = MREMAP_MODE_VERT | MREMAP_USE_EDGE | MREMAP_USE_NEAREST | MREMAP_USE_INTERP,
    MREMAP_MODE_VERT_FACE_NEAREST = MREMAP_MODE_VERT | MREMAP_USE_POLY | MREMAP_USE_NEAREST,
    MREMAP_MODE_VERT_POLYINTERP_NEAREST = MREMAP_MODE_VERT | MREMAP_USE_POLY | MREMAP_USE_NEAREST | MREMAP_USE_INTERP,
    MREMAP_MODE_VERT_POLYINTERP_VNORPROJ = MREMAP_MODE_VERT | MREMAP_USE_POLY | MREMAP_USE_NORPROJ | MREMAP_USE_INTERP,
    MREMAP_MODE_EDGE = 1 << 25,
    MREMAP_MODE_EDGE_VERT_NEAREST = MREMAP_MODE_EDGE | MREMAP_USE_VERT | MREMAP_USE_NEAREST,
    MREMAP_MODE_EDGE_NEAREST = MREMAP_MODE_EDGE | MREMAP_USE_EDGE | MREMAP_USE_NEAREST,
    MREMAP_MODE_EDGE_POLY_NEAREST = MREMAP_MODE_EDGE | MREMAP_USE_POLY | MREMAP_USE_NEAREST,
    MREMAP_MODE_EDGE_EDGEINTERP_VNORPROJ = MREMAP_MODE_EDGE | MREMAP_USE_VERT | MREMAP_USE_NORPROJ | MREMAP_USE_INTERP,
    MREMAP_MODE_LOOP = 1 << 26,
    MREMAP_MODE_LOOP_NEAREST_LOOPNOR = MREMAP_MODE_LOOP | MREMAP_USE_LOOP | MREMAP_USE_VERT | MREMAP_USE_NEAREST | MREMAP_USE_NORMAL,
    MREMAP_MODE_LOOP_NEAREST_POLYNOR = MREMAP_MODE_LOOP | MREMAP_USE_POLY | MREMAP_USE_VERT | MREMAP_USE_NEAREST | MREMAP_USE_NORMAL,
    MREMAP_MODE_LOOP_POLY_NEAREST = MREMAP_MODE_LOOP | MREMAP_USE_POLY | MREMAP_USE_NEAREST,
    MREMAP_MODE_LOOP_POLYINTERP_NEAREST = MREMAP_MODE_LOOP | MREMAP_USE_POLY | MREMAP_USE_NEAREST | MREMAP_USE_INTERP,
    MREMAP_MODE_LOOP_POLYINTERP_LNORPROJ = MREMAP_MODE_LOOP | MREMAP_USE_POLY | MREMAP_USE_NORPROJ | MREMAP_USE_INTERP,
    MREMAP_MODE_POLY = 1 << 27,
    MREMAP_MODE_POLY_NEAREST = MREMAP_MODE_POLY | MREMAP_USE_POLY | MREMAP_USE_NEAREST,
    MREMAP_MODE_POLY_NOR = MREMAP_MODE_POLY | MREMAP_USE_POLY | MREMAP_USE_NORMAL,
    MREMAP_MODE_POLY_POLYINTERP_PNORPROJ = MREMAP_MODE_POLY | MREMAP_USE_POLY | MREMAP_USE_NORPROJ | MREMAP_USE_INTERP,
    MREMAP_MODE_TOPOLOGY = MREMAP_MODE_VERT | MREMAP_MODE_EDGE | MREMAP_MODE_LOOP | MREMAP_MODE_POLY,
}

impl Default for eMRemapMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MREMAP_USE_VERT: i32 = eMRemapMode::MREMAP_USE_VERT as i32;
pub const MREMAP_USE_EDGE: i32 = eMRemapMode::MREMAP_USE_EDGE as i32;
pub const MREMAP_USE_LOOP: i32 = eMRemapMode::MREMAP_USE_LOOP as i32;
pub const MREMAP_USE_POLY: i32 = eMRemapMode::MREMAP_USE_POLY as i32;
pub const MREMAP_USE_NEAREST: i32 = eMRemapMode::MREMAP_USE_NEAREST as i32;
pub const MREMAP_USE_NORPROJ: i32 = eMRemapMode::MREMAP_USE_NORPROJ as i32;
pub const MREMAP_USE_INTERP: i32 = eMRemapMode::MREMAP_USE_INTERP as i32;
pub const MREMAP_USE_NORMAL: i32 = eMRemapMode::MREMAP_USE_NORMAL as i32;
pub const MREMAP_MODE_VERT: i32 = eMRemapMode::MREMAP_MODE_VERT as i32;
pub const MREMAP_MODE_VERT_NEAREST: i32 = eMRemapMode::MREMAP_MODE_VERT_NEAREST as i32;
pub const MREMAP_MODE_VERT_EDGE_NEAREST: i32 = eMRemapMode::MREMAP_MODE_VERT_EDGE_NEAREST as i32;
pub const MREMAP_MODE_VERT_EDGEINTERP_NEAREST: i32 = eMRemapMode::MREMAP_MODE_VERT_EDGEINTERP_NEAREST as i32;
pub const MREMAP_MODE_VERT_FACE_NEAREST: i32 = eMRemapMode::MREMAP_MODE_VERT_FACE_NEAREST as i32;
pub const MREMAP_MODE_VERT_POLYINTERP_NEAREST: i32 = eMRemapMode::MREMAP_MODE_VERT_POLYINTERP_NEAREST as i32;
pub const MREMAP_MODE_VERT_POLYINTERP_VNORPROJ: i32 = eMRemapMode::MREMAP_MODE_VERT_POLYINTERP_VNORPROJ as i32;
pub const MREMAP_MODE_EDGE: i32 = eMRemapMode::MREMAP_MODE_EDGE as i32;
pub const MREMAP_MODE_EDGE_VERT_NEAREST: i32 = eMRemapMode::MREMAP_MODE_EDGE_VERT_NEAREST as i32;
pub const MREMAP_MODE_EDGE_NEAREST: i32 = eMRemapMode::MREMAP_MODE_EDGE_NEAREST as i32;
pub const MREMAP_MODE_EDGE_POLY_NEAREST: i32 = eMRemapMode::MREMAP_MODE_EDGE_POLY_NEAREST as i32;
pub const MREMAP_MODE_EDGE_EDGEINTERP_VNORPROJ: i32 = eMRemapMode::MREMAP_MODE_EDGE_EDGEINTERP_VNORPROJ as i32;
pub const MREMAP_MODE_LOOP: i32 = eMRemapMode::MREMAP_MODE_LOOP as i32;
pub const MREMAP_MODE_LOOP_NEAREST_LOOPNOR: i32 = eMRemapMode::MREMAP_MODE_LOOP_NEAREST_LOOPNOR as i32;
pub const MREMAP_MODE_LOOP_NEAREST_POLYNOR: i32 = eMRemapMode::MREMAP_MODE_LOOP_NEAREST_POLYNOR as i32;
pub const MREMAP_MODE_LOOP_POLY_NEAREST: i32 = eMRemapMode::MREMAP_MODE_LOOP_POLY_NEAREST as i32;
pub const MREMAP_MODE_LOOP_POLYINTERP_NEAREST: i32 = eMRemapMode::MREMAP_MODE_LOOP_POLYINTERP_NEAREST as i32;
pub const MREMAP_MODE_LOOP_POLYINTERP_LNORPROJ: i32 = eMRemapMode::MREMAP_MODE_LOOP_POLYINTERP_LNORPROJ as i32;
pub const MREMAP_MODE_POLY: i32 = eMRemapMode::MREMAP_MODE_POLY as i32;
pub const MREMAP_MODE_POLY_NEAREST: i32 = eMRemapMode::MREMAP_MODE_POLY_NEAREST as i32;
pub const MREMAP_MODE_POLY_NOR: i32 = eMRemapMode::MREMAP_MODE_POLY_NOR as i32;
pub const MREMAP_MODE_POLY_POLYINTERP_PNORPROJ: i32 = eMRemapMode::MREMAP_MODE_POLY_POLYINTERP_PNORPROJ as i32;
pub const MREMAP_MODE_TOPOLOGY: i32 = eMRemapMode::MREMAP_MODE_TOPOLOGY as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCDTMixMode {
    CDT_MIX_NOMIX = -1,
    CDT_MIX_TRANSFER = 0,
    CDT_MIX_REPLACE_ABOVE_THRESHOLD = 1,
    CDT_MIX_REPLACE_BELOW_THRESHOLD = 2,
    CDT_MIX_MIX = 16,
    CDT_MIX_ADD = 17,
    CDT_MIX_SUB = 18,
    CDT_MIX_MUL = 19,
}

impl Default for eCDTMixMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CDT_MIX_NOMIX: i32 = eCDTMixMode::CDT_MIX_NOMIX as i32;
pub const CDT_MIX_TRANSFER: i32 = eCDTMixMode::CDT_MIX_TRANSFER as i32;
pub const CDT_MIX_REPLACE_ABOVE_THRESHOLD: i32 = eCDTMixMode::CDT_MIX_REPLACE_ABOVE_THRESHOLD as i32;
pub const CDT_MIX_REPLACE_BELOW_THRESHOLD: i32 = eCDTMixMode::CDT_MIX_REPLACE_BELOW_THRESHOLD as i32;
pub const CDT_MIX_MIX: i32 = eCDTMixMode::CDT_MIX_MIX as i32;
pub const CDT_MIX_ADD: i32 = eCDTMixMode::CDT_MIX_ADD as i32;
pub const CDT_MIX_SUB: i32 = eCDTMixMode::CDT_MIX_SUB as i32;
pub const CDT_MIX_MUL: i32 = eCDTMixMode::CDT_MIX_MUL as i32;

