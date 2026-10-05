//! Auto-transpiled C/C++ header module: DNA_texture_types

use core::ffi::c_void;
use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct MTex {
    pub texco: i16,
    pub blendtype: eMTex_BlendType,
    pub _pad2: [i8; 2],
    pub object: *mut Object,
    pub tex: *mut Tex,
    pub uvname: [i8; 68],
    pub projx: eTex_Projection,
    pub mapping: eMTex_Mapping,
    pub brush_map_mode: eMTex_BrushMapMode,
    pub brush_angle_mode: eMTex_BrushAngleMode,
    pub which_output: i16,
    pub ofs: [f32; 3],
    pub size: [f32; 3],
    pub r: f32,
    pub def_var: f32,
    pub colfac: f32,
    pub alphafac: f32,
    pub timefac: f32,
    pub kinkfac: f32,
    pub lifefac: f32,
    pub twistfac: f32,
}

impl Default for MTex {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Tex_Runtime {
    pub last_update: u64,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Tex {
    pub id: ID,
    pub adt: *mut AnimData,
    pub _pad3: *mut core::ffi::c_void,
    pub noisesize: f32,
    pub bright: f32,
    pub filtersize: f32,
    pub mg_H: f32,
    pub dist_amount: f32,
    pub vn_w1: f32,
    pub vn_w2: f32,
    pub vn_w3: f32,
    pub vn_w4: f32,
    pub vn_mexp: f32,
    pub vn_distm: eTex_VoronoiDistMetric,
    pub vn_coltype: eTex_VoronoiColType,
    pub noisedepth: i16,
    pub noisetype: eTex_NoiseType,
    pub noisebasis: eTex_NoiseBasis,
    pub imaflag: eTex_ImaFlag,
    pub flag: eTex_Flag,
    pub r#type: eTex_Type,
    pub stype: i16,
    pub cropxmin: f32,
    pub xrepeat: i16,
    pub extend: eTex_Extend,
    pub _pad0: i16,
    pub len: i32,
    pub frames: i32,
    pub offset: i32,
    pub sfra: i32,
    pub checkerdist: f32,
    pub iuser: ImageUser,
    pub nodetree: *mut bNodeTree,
    pub ima: *mut Image,
    pub coba: *mut ColorBand,
    pub preview: *mut PreviewImage,
    pub use_nodes: i8,
    pub _pad: [i8; 7],
    pub runtime: Tex_Runtime,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TexMapping {
    pub loc: [f32; 3],
    pub rot: [f32; 3],
    pub size: [f32; 3],
    pub flag: eTexMapping_Flag,
    pub projx: eTex_Projection,
    pub mapping: i8,
    pub r#type: eTexMapping_Type,
    pub mat: [[f32; 4]; 4],
    pub min: [f32; 3],
    pub ob: *mut Object,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ColorMapping {
    pub coba: ColorBand,
    pub bright: f32,
    pub flag: eColorMapping_Flag,
    pub blend_color: [f32; 3],
    pub blend_factor: f32,
    pub blend_type: i32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnimData {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ColorBand {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CurveMapping {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Image {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Object {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PreviewImage {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct member {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImageUser {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bNodeTree {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTexMapping_Flag {
    TEXMAP_CLIP_MIN = 1 << 0,
    TEXMAP_CLIP_MAX = 1 << 1,
    TEXMAP_UNIT_MATRIX = 1 << 2,
}

impl Default for eTexMapping_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TEXMAP_CLIP_MIN: i32 = eTexMapping_Flag::TEXMAP_CLIP_MIN as i32;
pub const TEXMAP_CLIP_MAX: i32 = eTexMapping_Flag::TEXMAP_CLIP_MAX as i32;
pub const TEXMAP_UNIT_MATRIX: i32 = eTexMapping_Flag::TEXMAP_UNIT_MATRIX as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTexMapping_Type {
    TEXMAP_TYPE_POINT = 0,
    TEXMAP_TYPE_TEXTURE = 1,
    TEXMAP_TYPE_VECTOR = 2,
    TEXMAP_TYPE_NORMAL = 3,
}

impl Default for eTexMapping_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TEXMAP_TYPE_POINT: i32 = eTexMapping_Type::TEXMAP_TYPE_POINT as i32;
pub const TEXMAP_TYPE_TEXTURE: i32 = eTexMapping_Type::TEXMAP_TYPE_TEXTURE as i32;
pub const TEXMAP_TYPE_VECTOR: i32 = eTexMapping_Type::TEXMAP_TYPE_VECTOR as i32;
pub const TEXMAP_TYPE_NORMAL: i32 = eTexMapping_Type::TEXMAP_TYPE_NORMAL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eColorMapping_Flag {
    COLORMAP_USE_RAMP = 1,
}

impl Default for eColorMapping_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const COLORMAP_USE_RAMP: i32 = eColorMapping_Flag::COLORMAP_USE_RAMP as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTex_Type {
    TEX_CLOUDS = 1,
    TEX_WOOD = 2,
    TEX_MARBLE = 3,
    TEX_MAGIC = 4,
    TEX_BLEND = 5,
    TEX_STUCCI = 6,
    TEX_NOISE = 7,
    TEX_IMAGE = 8,
    TEX_MUSGRAVE = 11,
    TEX_VORONOI = 12,
    TEX_DISTNOISE = 13,
}

impl Default for eTex_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TEX_CLOUDS: i32 = eTex_Type::TEX_CLOUDS as i32;
pub const TEX_WOOD: i32 = eTex_Type::TEX_WOOD as i32;
pub const TEX_MARBLE: i32 = eTex_Type::TEX_MARBLE as i32;
pub const TEX_MAGIC: i32 = eTex_Type::TEX_MAGIC as i32;
pub const TEX_BLEND: i32 = eTex_Type::TEX_BLEND as i32;
pub const TEX_STUCCI: i32 = eTex_Type::TEX_STUCCI as i32;
pub const TEX_NOISE: i32 = eTex_Type::TEX_NOISE as i32;
pub const TEX_IMAGE: i32 = eTex_Type::TEX_IMAGE as i32;
pub const TEX_MUSGRAVE: i32 = eTex_Type::TEX_MUSGRAVE as i32;
pub const TEX_VORONOI: i32 = eTex_Type::TEX_VORONOI as i32;
pub const TEX_DISTNOISE: i32 = eTex_Type::TEX_DISTNOISE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTex_MusgraveType {
    TEX_MFRACTAL = 0,
    TEX_RIDGEDMF = 1,
    TEX_HYBRIDMF = 2,
    TEX_FBM = 3,
    TEX_HTERRAIN = 4,
}

impl Default for eTex_MusgraveType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TEX_MFRACTAL: i32 = eTex_MusgraveType::TEX_MFRACTAL as i32;
pub const TEX_RIDGEDMF: i32 = eTex_MusgraveType::TEX_RIDGEDMF as i32;
pub const TEX_HYBRIDMF: i32 = eTex_MusgraveType::TEX_HYBRIDMF as i32;
pub const TEX_FBM: i32 = eTex_MusgraveType::TEX_FBM as i32;
pub const TEX_HTERRAIN: i32 = eTex_MusgraveType::TEX_HTERRAIN as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTex_NoiseBasis {
    TEX_BLENDER = 0,
    TEX_STDPERLIN = 1,
    TEX_NEWPERLIN = 2,
    TEX_VORONOI_F1 = 3,
    TEX_VORONOI_F2 = 4,
    TEX_VORONOI_F3 = 5,
    TEX_VORONOI_F4 = 6,
    TEX_VORONOI_F2F1 = 7,
    TEX_VORONOI_CRACKLE = 8,
    TEX_CELLNOISE = 14,
}

impl Default for eTex_NoiseBasis {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TEX_BLENDER: i32 = eTex_NoiseBasis::TEX_BLENDER as i32;
pub const TEX_STDPERLIN: i32 = eTex_NoiseBasis::TEX_STDPERLIN as i32;
pub const TEX_NEWPERLIN: i32 = eTex_NoiseBasis::TEX_NEWPERLIN as i32;
pub const TEX_VORONOI_F1: i32 = eTex_NoiseBasis::TEX_VORONOI_F1 as i32;
pub const TEX_VORONOI_F2: i32 = eTex_NoiseBasis::TEX_VORONOI_F2 as i32;
pub const TEX_VORONOI_F3: i32 = eTex_NoiseBasis::TEX_VORONOI_F3 as i32;
pub const TEX_VORONOI_F4: i32 = eTex_NoiseBasis::TEX_VORONOI_F4 as i32;
pub const TEX_VORONOI_F2F1: i32 = eTex_NoiseBasis::TEX_VORONOI_F2F1 as i32;
pub const TEX_VORONOI_CRACKLE: i32 = eTex_NoiseBasis::TEX_VORONOI_CRACKLE as i32;
pub const TEX_CELLNOISE: i32 = eTex_NoiseBasis::TEX_CELLNOISE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTex_VoronoiDistMetric {
    TEX_DISTANCE = 0,
    TEX_DISTANCE_SQUARED = 1,
    TEX_MANHATTAN = 2,
    TEX_CHEBYCHEV = 3,
    TEX_MINKOVSKY_HALF = 4,
    TEX_MINKOVSKY_FOUR = 5,
    TEX_MINKOVSKY = 6,
}

impl Default for eTex_VoronoiDistMetric {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TEX_DISTANCE: i32 = eTex_VoronoiDistMetric::TEX_DISTANCE as i32;
pub const TEX_DISTANCE_SQUARED: i32 = eTex_VoronoiDistMetric::TEX_DISTANCE_SQUARED as i32;
pub const TEX_MANHATTAN: i32 = eTex_VoronoiDistMetric::TEX_MANHATTAN as i32;
pub const TEX_CHEBYCHEV: i32 = eTex_VoronoiDistMetric::TEX_CHEBYCHEV as i32;
pub const TEX_MINKOVSKY_HALF: i32 = eTex_VoronoiDistMetric::TEX_MINKOVSKY_HALF as i32;
pub const TEX_MINKOVSKY_FOUR: i32 = eTex_VoronoiDistMetric::TEX_MINKOVSKY_FOUR as i32;
pub const TEX_MINKOVSKY: i32 = eTex_VoronoiDistMetric::TEX_MINKOVSKY as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTex_ImaFlag {
    TEX_INTERPOL = 1 << 0,
    TEX_USEALPHA = 1 << 1,
    TEX_IMAROT = 1 << 4,
    TEX_CALCALPHA = 1 << 5,
    TEX_NORMALMAP = 1 << 11,
    TEX_DERIVATIVEMAP = 1 << 14,
}

impl Default for eTex_ImaFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TEX_INTERPOL: i32 = eTex_ImaFlag::TEX_INTERPOL as i32;
pub const TEX_USEALPHA: i32 = eTex_ImaFlag::TEX_USEALPHA as i32;
pub const TEX_IMAROT: i32 = eTex_ImaFlag::TEX_IMAROT as i32;
pub const TEX_CALCALPHA: i32 = eTex_ImaFlag::TEX_CALCALPHA as i32;
pub const TEX_NORMALMAP: i32 = eTex_ImaFlag::TEX_NORMALMAP as i32;
pub const TEX_DERIVATIVEMAP: i32 = eTex_ImaFlag::TEX_DERIVATIVEMAP as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTex_Flag {
    TEX_COLORBAND = 1 << 0,
    TEX_FLIPBLEND = 1 << 1,
    TEX_NEGALPHA = 1 << 2,
    TEX_CHECKER_ODD = 1 << 3,
    TEX_CHECKER_EVEN = 1 << 4,
    TEX_PRV_ALPHA = 1 << 5,
    TEX_PRV_NOR = 1 << 6,
    TEX_REPEAT_XMIR = 1 << 7,
    TEX_REPEAT_YMIR = 1 << 8,
    TEX_DS_EXPAND = 1 << 9,
    TEX_NO_CLAMP = 1 << 10,
}

impl Default for eTex_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TEX_COLORBAND: i32 = eTex_Flag::TEX_COLORBAND as i32;
pub const TEX_FLIPBLEND: i32 = eTex_Flag::TEX_FLIPBLEND as i32;
pub const TEX_NEGALPHA: i32 = eTex_Flag::TEX_NEGALPHA as i32;
pub const TEX_CHECKER_ODD: i32 = eTex_Flag::TEX_CHECKER_ODD as i32;
pub const TEX_CHECKER_EVEN: i32 = eTex_Flag::TEX_CHECKER_EVEN as i32;
pub const TEX_PRV_ALPHA: i32 = eTex_Flag::TEX_PRV_ALPHA as i32;
pub const TEX_PRV_NOR: i32 = eTex_Flag::TEX_PRV_NOR as i32;
pub const TEX_REPEAT_XMIR: i32 = eTex_Flag::TEX_REPEAT_XMIR as i32;
pub const TEX_REPEAT_YMIR: i32 = eTex_Flag::TEX_REPEAT_YMIR as i32;
pub const TEX_DS_EXPAND: i32 = eTex_Flag::TEX_DS_EXPAND as i32;
pub const TEX_NO_CLAMP: i32 = eTex_Flag::TEX_NO_CLAMP as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTex_Extend {
    TEX_EXTEND = 1,
    TEX_CLIP = 2,
    TEX_REPEAT = 3,
    TEX_CLIPCUBE = 4,
    TEX_CHECKER = 5,
}

impl Default for eTex_Extend {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TEX_EXTEND: i32 = eTex_Extend::TEX_EXTEND as i32;
pub const TEX_CLIP: i32 = eTex_Extend::TEX_CLIP as i32;
pub const TEX_REPEAT: i32 = eTex_Extend::TEX_REPEAT as i32;
pub const TEX_CLIPCUBE: i32 = eTex_Extend::TEX_CLIPCUBE as i32;
pub const TEX_CHECKER: i32 = eTex_Extend::TEX_CHECKER as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTex_NoiseType {
    TEX_NOISESOFT = 0,
    TEX_NOISEPERL = 1,
}

impl Default for eTex_NoiseType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TEX_NOISESOFT: i32 = eTex_NoiseType::TEX_NOISESOFT as i32;
pub const TEX_NOISEPERL: i32 = eTex_NoiseType::TEX_NOISEPERL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTex_WoodWaveform {
    TEX_SIN = 0,
    TEX_SAW = 1,
    TEX_TRI = 2,
}

impl Default for eTex_WoodWaveform {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TEX_SIN: i32 = eTex_WoodWaveform::TEX_SIN as i32;
pub const TEX_SAW: i32 = eTex_WoodWaveform::TEX_SAW as i32;
pub const TEX_TRI: i32 = eTex_WoodWaveform::TEX_TRI as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTex_WoodType {
    TEX_BAND = 0,
    TEX_RING = 1,
    TEX_BANDNOISE = 2,
    TEX_RINGNOISE = 3,
}

impl Default for eTex_WoodType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TEX_BAND: i32 = eTex_WoodType::TEX_BAND as i32;
pub const TEX_RING: i32 = eTex_WoodType::TEX_RING as i32;
pub const TEX_BANDNOISE: i32 = eTex_WoodType::TEX_BANDNOISE as i32;
pub const TEX_RINGNOISE: i32 = eTex_WoodType::TEX_RINGNOISE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTex_CloudType {
    TEX_DEFAULT = 0,
    TEX_COLOR = 1,
}

impl Default for eTex_CloudType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TEX_DEFAULT: i32 = eTex_CloudType::TEX_DEFAULT as i32;
pub const TEX_COLOR: i32 = eTex_CloudType::TEX_COLOR as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTex_MarbleType {
    TEX_SOFT = 0,
    TEX_SHARP = 1,
    TEX_SHARPER = 2,
}

impl Default for eTex_MarbleType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TEX_SOFT: i32 = eTex_MarbleType::TEX_SOFT as i32;
pub const TEX_SHARP: i32 = eTex_MarbleType::TEX_SHARP as i32;
pub const TEX_SHARPER: i32 = eTex_MarbleType::TEX_SHARPER as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTex_BlendType {
    TEX_LIN = 0,
    TEX_QUAD = 1,
    TEX_EASE = 2,
    TEX_DIAG = 3,
    TEX_SPHERE = 4,
    TEX_HALO = 5,
    TEX_RAD = 6,
}

impl Default for eTex_BlendType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TEX_LIN: i32 = eTex_BlendType::TEX_LIN as i32;
pub const TEX_QUAD: i32 = eTex_BlendType::TEX_QUAD as i32;
pub const TEX_EASE: i32 = eTex_BlendType::TEX_EASE as i32;
pub const TEX_DIAG: i32 = eTex_BlendType::TEX_DIAG as i32;
pub const TEX_SPHERE: i32 = eTex_BlendType::TEX_SPHERE as i32;
pub const TEX_HALO: i32 = eTex_BlendType::TEX_HALO as i32;
pub const TEX_RAD: i32 = eTex_BlendType::TEX_RAD as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTex_StucciType {
    TEX_PLASTIC = 0,
    TEX_WALLIN = 1,
    TEX_WALLOUT = 2,
}

impl Default for eTex_StucciType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TEX_PLASTIC: i32 = eTex_StucciType::TEX_PLASTIC as i32;
pub const TEX_WALLIN: i32 = eTex_StucciType::TEX_WALLIN as i32;
pub const TEX_WALLOUT: i32 = eTex_StucciType::TEX_WALLOUT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTex_VoronoiColType {
    TEX_INTENSITY = 0,
    TEX_COL1 = 1,
    TEX_COL2 = 2,
    TEX_COL3 = 3,
}

impl Default for eTex_VoronoiColType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TEX_INTENSITY: i32 = eTex_VoronoiColType::TEX_INTENSITY as i32;
pub const TEX_COL1: i32 = eTex_VoronoiColType::TEX_COL1 as i32;
pub const TEX_COL2: i32 = eTex_VoronoiColType::TEX_COL2 as i32;
pub const TEX_COL3: i32 = eTex_VoronoiColType::TEX_COL3 as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTex_ReturnValue {
    TEX_INT = 0,
    TEX_RGB = 1,
}

impl Default for eTex_ReturnValue {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TEX_INT: i32 = eTex_ReturnValue::TEX_INT as i32;
pub const TEX_RGB: i32 = eTex_ReturnValue::TEX_RGB as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTex_PreviewType {
    TEX_PR_TEXTURE = 0,
    TEX_PR_OTHER = 1,
    TEX_PR_BOTH = 2,
}

impl Default for eTex_PreviewType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TEX_PR_TEXTURE: i32 = eTex_PreviewType::TEX_PR_TEXTURE as i32;
pub const TEX_PR_OTHER: i32 = eTex_PreviewType::TEX_PR_OTHER as i32;
pub const TEX_PR_BOTH: i32 = eTex_PreviewType::TEX_PR_BOTH as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTex_Projection {
    PROJ_N = 0,
    PROJ_X = 1,
    PROJ_Y = 2,
    PROJ_Z = 3,
}

impl Default for eTex_Projection {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const PROJ_N: i32 = eTex_Projection::PROJ_N as i32;
pub const PROJ_X: i32 = eTex_Projection::PROJ_X as i32;
pub const PROJ_Y: i32 = eTex_Projection::PROJ_Y as i32;
pub const PROJ_Z: i32 = eTex_Projection::PROJ_Z as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMTex_Mapping {
    MTEX_FLAT = 0,
    MTEX_CUBE = 1,
    MTEX_TUBE = 2,
    MTEX_SPHERE = 3,
}

impl Default for eMTex_Mapping {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MTEX_FLAT: i32 = eMTex_Mapping::MTEX_FLAT as i32;
pub const MTEX_CUBE: i32 = eMTex_Mapping::MTEX_CUBE as i32;
pub const MTEX_TUBE: i32 = eMTex_Mapping::MTEX_TUBE as i32;
pub const MTEX_SPHERE: i32 = eMTex_Mapping::MTEX_SPHERE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMTex_BlendType {
    MTEX_BLEND = 0,
    MTEX_MUL = 1,
    MTEX_ADD = 2,
    MTEX_SUB = 3,
    MTEX_DIV = 4,
    MTEX_DARK = 5,
    MTEX_DIFF = 6,
    MTEX_LIGHT = 7,
    MTEX_SCREEN = 8,
    MTEX_OVERLAY = 9,
    MTEX_BLEND_HUE = 10,
    MTEX_BLEND_SAT = 11,
    MTEX_BLEND_VAL = 12,
    MTEX_BLEND_COLOR = 13,
    MTEX_SOFT_LIGHT = 15,
    MTEX_LIN_LIGHT = 16,
}

impl Default for eMTex_BlendType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MTEX_BLEND: i32 = eMTex_BlendType::MTEX_BLEND as i32;
pub const MTEX_MUL: i32 = eMTex_BlendType::MTEX_MUL as i32;
pub const MTEX_ADD: i32 = eMTex_BlendType::MTEX_ADD as i32;
pub const MTEX_SUB: i32 = eMTex_BlendType::MTEX_SUB as i32;
pub const MTEX_DIV: i32 = eMTex_BlendType::MTEX_DIV as i32;
pub const MTEX_DARK: i32 = eMTex_BlendType::MTEX_DARK as i32;
pub const MTEX_DIFF: i32 = eMTex_BlendType::MTEX_DIFF as i32;
pub const MTEX_LIGHT: i32 = eMTex_BlendType::MTEX_LIGHT as i32;
pub const MTEX_SCREEN: i32 = eMTex_BlendType::MTEX_SCREEN as i32;
pub const MTEX_OVERLAY: i32 = eMTex_BlendType::MTEX_OVERLAY as i32;
pub const MTEX_BLEND_HUE: i32 = eMTex_BlendType::MTEX_BLEND_HUE as i32;
pub const MTEX_BLEND_SAT: i32 = eMTex_BlendType::MTEX_BLEND_SAT as i32;
pub const MTEX_BLEND_VAL: i32 = eMTex_BlendType::MTEX_BLEND_VAL as i32;
pub const MTEX_BLEND_COLOR: i32 = eMTex_BlendType::MTEX_BLEND_COLOR as i32;
pub const MTEX_SOFT_LIGHT: i32 = eMTex_BlendType::MTEX_SOFT_LIGHT as i32;
pub const MTEX_LIN_LIGHT: i32 = eMTex_BlendType::MTEX_LIN_LIGHT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMTex_BrushMapMode {
    MTEX_MAP_MODE_VIEW = 0,
    MTEX_MAP_MODE_TILED = 1,
    MTEX_MAP_MODE_3D = 2,
    MTEX_MAP_MODE_AREA = 3,
    MTEX_MAP_MODE_RANDOM = 4,
    MTEX_MAP_MODE_STENCIL = 5,
}

impl Default for eMTex_BrushMapMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MTEX_MAP_MODE_VIEW: i32 = eMTex_BrushMapMode::MTEX_MAP_MODE_VIEW as i32;
pub const MTEX_MAP_MODE_TILED: i32 = eMTex_BrushMapMode::MTEX_MAP_MODE_TILED as i32;
pub const MTEX_MAP_MODE_3D: i32 = eMTex_BrushMapMode::MTEX_MAP_MODE_3D as i32;
pub const MTEX_MAP_MODE_AREA: i32 = eMTex_BrushMapMode::MTEX_MAP_MODE_AREA as i32;
pub const MTEX_MAP_MODE_RANDOM: i32 = eMTex_BrushMapMode::MTEX_MAP_MODE_RANDOM as i32;
pub const MTEX_MAP_MODE_STENCIL: i32 = eMTex_BrushMapMode::MTEX_MAP_MODE_STENCIL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMTex_BrushAngleMode {
    MTEX_ANGLE_RANDOM = 1,
    MTEX_ANGLE_RAKE = 2,
}

impl Default for eMTex_BrushAngleMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MTEX_ANGLE_RANDOM: i32 = eMTex_BrushAngleMode::MTEX_ANGLE_RANDOM as i32;
pub const MTEX_ANGLE_RAKE: i32 = eMTex_BrushAngleMode::MTEX_ANGLE_RAKE as i32;

