//! Auto-transpiled C/C++ header module: DNA_linestyle_types

use crate::*;

pub const MAX_MTEX: i32 = 18;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct LineStyleModifier {
    pub next: *mut LineStyleModifier,
    pub name: [i8; 64],
    pub r#type: eLineStyleModifier_Type,
    pub influence: f32,
    pub flags: eLineStyleModifier_Flag,
    pub blend: eLineStyleBlend,
}

impl Default for LineStyleModifier {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleColorModifier_AlongStroke {
    pub modifier: LineStyleModifier,
    pub color_ramp: *mut ColorBand,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleAlphaModifier_AlongStroke {
    pub modifier: LineStyleModifier,
    pub curve: *mut CurveMapping,
    pub flags: eLineStyleAlphaThicknessModifier_Flag,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleThicknessModifier_AlongStroke {
    pub modifier: LineStyleModifier,
    pub curve: *mut CurveMapping,
    pub flags: eLineStyleAlphaThicknessModifier_Flag,
    pub value_min: f32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleColorModifier_DistanceFromCamera {
    pub modifier: LineStyleModifier,
    pub color_ramp: *mut ColorBand,
    pub range_min: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleAlphaModifier_DistanceFromCamera {
    pub modifier: LineStyleModifier,
    pub curve: *mut CurveMapping,
    pub flags: eLineStyleAlphaThicknessModifier_Flag,
    pub range_min: f32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleThicknessModifier_DistanceFromCamera {
    pub modifier: LineStyleModifier,
    pub curve: *mut CurveMapping,
    pub flags: eLineStyleAlphaThicknessModifier_Flag,
    pub range_min: f32,
    pub value_min: f32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleColorModifier_DistanceFromObject {
    pub modifier: LineStyleModifier,
    pub target: *mut Object,
    pub color_ramp: *mut ColorBand,
    pub range_min: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleAlphaModifier_DistanceFromObject {
    pub modifier: LineStyleModifier,
    pub target: *mut Object,
    pub curve: *mut CurveMapping,
    pub flags: eLineStyleAlphaThicknessModifier_Flag,
    pub range_min: f32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleThicknessModifier_DistanceFromObject {
    pub modifier: LineStyleModifier,
    pub target: *mut Object,
    pub curve: *mut CurveMapping,
    pub flags: eLineStyleAlphaThicknessModifier_Flag,
    pub range_min: f32,
    pub value_min: f32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleColorModifier_Curvature_3D {
    pub modifier: LineStyleModifier,
    pub min_curvature: f32,
    pub color_ramp: *mut ColorBand,
    pub range_min: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleAlphaModifier_Curvature_3D {
    pub modifier: LineStyleModifier,
    pub curve: *mut CurveMapping,
    pub flags: eLineStyleAlphaThicknessModifier_Flag,
    pub min_curvature: f32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleThicknessModifier_Curvature_3D {
    pub modifier: LineStyleModifier,
    pub curve: *mut CurveMapping,
    pub flags: eLineStyleAlphaThicknessModifier_Flag,
    pub _pad: [i8; 4],
    pub min_curvature: f32,
    pub min_thickness: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleColorModifier_Noise {
    pub modifier: LineStyleModifier,
    pub color_ramp: *mut ColorBand,
    pub period: f32,
    pub seed: i32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleAlphaModifier_Noise {
    pub modifier: LineStyleModifier,
    pub curve: *mut CurveMapping,
    pub flags: eLineStyleAlphaThicknessModifier_Flag,
    pub period: f32,
    pub seed: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleThicknessModifier_Noise {
    pub modifier: LineStyleModifier,
    pub period: f32,
    pub flags: eLineStyleThicknessNoise_Flag,
    pub seed: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleColorModifier_CreaseAngle {
    pub modifier: LineStyleModifier,
    pub color_ramp: *mut ColorBand,
    pub min_angle: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleAlphaModifier_CreaseAngle {
    pub modifier: LineStyleModifier,
    pub curve: *mut CurveMapping,
    pub flags: eLineStyleAlphaThicknessModifier_Flag,
    pub min_angle: f32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleThicknessModifier_CreaseAngle {
    pub modifier: LineStyleModifier,
    pub curve: *mut CurveMapping,
    pub flags: eLineStyleAlphaThicknessModifier_Flag,
    pub _pad: [i8; 4],
    pub min_angle: f32,
    pub min_thickness: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleColorModifier_Tangent {
    pub modifier: LineStyleModifier,
    pub color_ramp: *mut ColorBand,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleAlphaModifier_Tangent {
    pub modifier: LineStyleModifier,
    pub curve: *mut CurveMapping,
    pub flags: eLineStyleAlphaThicknessModifier_Flag,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleThicknessModifier_Tangent {
    pub modifier: LineStyleModifier,
    pub curve: *mut CurveMapping,
    pub flags: eLineStyleAlphaThicknessModifier_Flag,
    pub min_thickness: f32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleColorModifier_Material {
    pub modifier: LineStyleModifier,
    pub color_ramp: *mut ColorBand,
    pub flags: eLineStyleColorModifier_Flag,
    pub mat_attr: eLineStyleMaterialAttr,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleAlphaModifier_Material {
    pub modifier: LineStyleModifier,
    pub curve: *mut CurveMapping,
    pub flags: eLineStyleAlphaThicknessModifier_Flag,
    pub mat_attr: eLineStyleMaterialAttr,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleThicknessModifier_Material {
    pub modifier: LineStyleModifier,
    pub curve: *mut CurveMapping,
    pub flags: eLineStyleAlphaThicknessModifier_Flag,
    pub value_min: f32,
    pub mat_attr: eLineStyleMaterialAttr,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleGeometryModifier_Sampling {
    pub modifier: LineStyleModifier,
    pub sampling: f32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleGeometryModifier_BezierCurve {
    pub modifier: LineStyleModifier,
    pub error: f32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleGeometryModifier_SinusDisplacement {
    pub modifier: LineStyleModifier,
    pub wavelength: f32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleGeometryModifier_SpatialNoise {
    pub modifier: LineStyleModifier,
    pub amplitude: f32,
    pub octaves: u32,
    pub flags: eLineStyleGeomSpatialNoise_Flag,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleGeometryModifier_PerlinNoise1D {
    pub modifier: LineStyleModifier,
    pub frequency: f32,
    pub angle: f32,
    pub octaves: u32,
    pub seed: i32,
    pub _pad1: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleGeometryModifier_PerlinNoise2D {
    pub modifier: LineStyleModifier,
    pub frequency: f32,
    pub angle: f32,
    pub octaves: u32,
    pub seed: i32,
    pub _pad1: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleGeometryModifier_BackboneStretcher {
    pub modifier: LineStyleModifier,
    pub backbone_length: f32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleGeometryModifier_TipRemover {
    pub modifier: LineStyleModifier,
    pub tip_length: f32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleGeometryModifier_Polygonalization {
    pub modifier: LineStyleModifier,
    pub error: f32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleGeometryModifier_GuidingLines {
    pub modifier: LineStyleModifier,
    pub offset: f32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleGeometryModifier_Blueprint {
    pub modifier: LineStyleModifier,
    pub flags: eLineStyleGeomBlueprint_Flag,
    pub rounds: u32,
    pub backbone_length: f32,
    pub random_radius: u32,
    pub random_center: u32,
    pub random_backbone: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleGeometryModifier_2DOffset {
    pub modifier: LineStyleModifier,
    pub start: f32,
    pub x: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleGeometryModifier_2DTransform {
    pub modifier: LineStyleModifier,
    pub pivot: eLineStyleGeom2DTransform_Pivot,
    pub scale_x: f32,
    pub angle: f32,
    pub pivot_u: f32,
    pub pivot_x: f32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleGeometryModifier_Simplification {
    pub modifier: LineStyleModifier,
    pub tolerance: f32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineStyleThicknessModifier_Calligraphy {
    pub modifier: LineStyleModifier,
    pub min_thickness: f32,
    pub orientation: f32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FreestyleLineStyle {
    pub id: ID,
    pub adt: *mut AnimData,
    pub r: f32,
    pub thickness: f32,
    pub thickness_position: eLineStyle_ThicknessPosition,
    pub thickness_ratio: f32,
    pub flag: eLineStyle_Flag,
    pub caps: eLineStyle_Caps,
    pub chaining: eLineStyle_Chaining,
    pub rounds: u32,
    pub split_length: f32,
    pub min_length: f32,
    pub chain_count: u32,
    pub split_dash1: u16,
    pub split_dash2: u16,
    pub split_dash3: u16,
    pub sort_key: eLineStyle_SortKey,
    pub integration_type: eLineStyle_IntegrationType,
    pub texstep: f32,
    pub texact: i16,
    pub use_nodes: i16,
    pub _pad: [i8; 6],
    pub dash1: u16,
    pub panel: eLineStyle_Panel,
    pub mtex: [*mut MTex; 18],
    pub nodetree: *mut bNodeTree,
    pub color_modifiers: ListBaseT<LineStyleModifier>,
    pub alpha_modifiers: ListBaseT<LineStyleModifier>,
    pub thickness_modifiers: ListBaseT<LineStyleModifier>,
    pub geometry_modifiers: ListBaseT<LineStyleModifier>,
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
pub struct MTex {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Object {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bNodeTree {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLineStyleModifier_Type {
    LS_MODIFIER_ALONG_STROKE = 1,
    LS_MODIFIER_DISTANCE_FROM_CAMERA = 2,
    LS_MODIFIER_DISTANCE_FROM_OBJECT = 3,
    LS_MODIFIER_MATERIAL = 4,
    LS_MODIFIER_SAMPLING = 5,
    LS_MODIFIER_BEZIER_CURVE = 6,
    LS_MODIFIER_SINUS_DISPLACEMENT = 7,
    LS_MODIFIER_SPATIAL_NOISE = 8,
    LS_MODIFIER_PERLIN_NOISE_1D = 9,
    LS_MODIFIER_PERLIN_NOISE_2D = 10,
    LS_MODIFIER_BACKBONE_STRETCHER = 11,
    LS_MODIFIER_TIP_REMOVER = 12,
    LS_MODIFIER_CALLIGRAPHY = 13,
    LS_MODIFIER_POLYGONIZATION = 14,
    LS_MODIFIER_GUIDING_LINES = 15,
    LS_MODIFIER_BLUEPRINT = 16,
    LS_MODIFIER_2D_OFFSET = 17,
    LS_MODIFIER_2D_TRANSFORM = 18,
    LS_MODIFIER_TANGENT = 19,
    LS_MODIFIER_NOISE = 20,
    LS_MODIFIER_CREASE_ANGLE = 21,
    LS_MODIFIER_SIMPLIFICATION = 22,
    LS_MODIFIER_CURVATURE_3D = 23,
    LS_MODIFIER_NUM = 24,
}

impl Default for eLineStyleModifier_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LS_MODIFIER_ALONG_STROKE: i32 = eLineStyleModifier_Type::LS_MODIFIER_ALONG_STROKE as i32;
pub const LS_MODIFIER_DISTANCE_FROM_CAMERA: i32 = eLineStyleModifier_Type::LS_MODIFIER_DISTANCE_FROM_CAMERA as i32;
pub const LS_MODIFIER_DISTANCE_FROM_OBJECT: i32 = eLineStyleModifier_Type::LS_MODIFIER_DISTANCE_FROM_OBJECT as i32;
pub const LS_MODIFIER_MATERIAL: i32 = eLineStyleModifier_Type::LS_MODIFIER_MATERIAL as i32;
pub const LS_MODIFIER_SAMPLING: i32 = eLineStyleModifier_Type::LS_MODIFIER_SAMPLING as i32;
pub const LS_MODIFIER_BEZIER_CURVE: i32 = eLineStyleModifier_Type::LS_MODIFIER_BEZIER_CURVE as i32;
pub const LS_MODIFIER_SINUS_DISPLACEMENT: i32 = eLineStyleModifier_Type::LS_MODIFIER_SINUS_DISPLACEMENT as i32;
pub const LS_MODIFIER_SPATIAL_NOISE: i32 = eLineStyleModifier_Type::LS_MODIFIER_SPATIAL_NOISE as i32;
pub const LS_MODIFIER_PERLIN_NOISE_1D: i32 = eLineStyleModifier_Type::LS_MODIFIER_PERLIN_NOISE_1D as i32;
pub const LS_MODIFIER_PERLIN_NOISE_2D: i32 = eLineStyleModifier_Type::LS_MODIFIER_PERLIN_NOISE_2D as i32;
pub const LS_MODIFIER_BACKBONE_STRETCHER: i32 = eLineStyleModifier_Type::LS_MODIFIER_BACKBONE_STRETCHER as i32;
pub const LS_MODIFIER_TIP_REMOVER: i32 = eLineStyleModifier_Type::LS_MODIFIER_TIP_REMOVER as i32;
pub const LS_MODIFIER_CALLIGRAPHY: i32 = eLineStyleModifier_Type::LS_MODIFIER_CALLIGRAPHY as i32;
pub const LS_MODIFIER_POLYGONIZATION: i32 = eLineStyleModifier_Type::LS_MODIFIER_POLYGONIZATION as i32;
pub const LS_MODIFIER_GUIDING_LINES: i32 = eLineStyleModifier_Type::LS_MODIFIER_GUIDING_LINES as i32;
pub const LS_MODIFIER_BLUEPRINT: i32 = eLineStyleModifier_Type::LS_MODIFIER_BLUEPRINT as i32;
pub const LS_MODIFIER_2D_OFFSET: i32 = eLineStyleModifier_Type::LS_MODIFIER_2D_OFFSET as i32;
pub const LS_MODIFIER_2D_TRANSFORM: i32 = eLineStyleModifier_Type::LS_MODIFIER_2D_TRANSFORM as i32;
pub const LS_MODIFIER_TANGENT: i32 = eLineStyleModifier_Type::LS_MODIFIER_TANGENT as i32;
pub const LS_MODIFIER_NOISE: i32 = eLineStyleModifier_Type::LS_MODIFIER_NOISE as i32;
pub const LS_MODIFIER_CREASE_ANGLE: i32 = eLineStyleModifier_Type::LS_MODIFIER_CREASE_ANGLE as i32;
pub const LS_MODIFIER_SIMPLIFICATION: i32 = eLineStyleModifier_Type::LS_MODIFIER_SIMPLIFICATION as i32;
pub const LS_MODIFIER_CURVATURE_3D: i32 = eLineStyleModifier_Type::LS_MODIFIER_CURVATURE_3D as i32;
pub const LS_MODIFIER_NUM: i32 = eLineStyleModifier_Type::LS_MODIFIER_NUM as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLineStyleModifier_Flag {
    LS_MODIFIER_ENABLED = 1,
    LS_MODIFIER_EXPANDED = 2,
}

impl Default for eLineStyleModifier_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LS_MODIFIER_ENABLED: i32 = eLineStyleModifier_Flag::LS_MODIFIER_ENABLED as i32;
pub const LS_MODIFIER_EXPANDED: i32 = eLineStyleModifier_Flag::LS_MODIFIER_EXPANDED as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLineStyleColorModifier_Flag {
    LS_MODIFIER_USE_RAMP = 1,
}

impl Default for eLineStyleColorModifier_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LS_MODIFIER_USE_RAMP: i32 = eLineStyleColorModifier_Flag::LS_MODIFIER_USE_RAMP as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLineStyleAlphaThicknessModifier_Flag {
    LS_MODIFIER_USE_CURVE = 1,
    LS_MODIFIER_INVERT = 2,
}

impl Default for eLineStyleAlphaThicknessModifier_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LS_MODIFIER_USE_CURVE: i32 = eLineStyleAlphaThicknessModifier_Flag::LS_MODIFIER_USE_CURVE as i32;
pub const LS_MODIFIER_INVERT: i32 = eLineStyleAlphaThicknessModifier_Flag::LS_MODIFIER_INVERT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLineStyleThicknessNoise_Flag {
    LS_THICKNESS_ASYMMETRIC = 1,
}

impl Default for eLineStyleThicknessNoise_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LS_THICKNESS_ASYMMETRIC: i32 = eLineStyleThicknessNoise_Flag::LS_THICKNESS_ASYMMETRIC as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLineStyleBlend {
    LS_VALUE_BLEND = 0,
    LS_VALUE_ADD = 1,
    LS_VALUE_MULT = 2,
    LS_VALUE_SUB = 3,
    LS_VALUE_DIV = 4,
    LS_VALUE_DIFF = 5,
    LS_VALUE_MIN = 6,
    LS_VALUE_MAX = 7,
}

impl Default for eLineStyleBlend {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LS_VALUE_BLEND: i32 = eLineStyleBlend::LS_VALUE_BLEND as i32;
pub const LS_VALUE_ADD: i32 = eLineStyleBlend::LS_VALUE_ADD as i32;
pub const LS_VALUE_MULT: i32 = eLineStyleBlend::LS_VALUE_MULT as i32;
pub const LS_VALUE_SUB: i32 = eLineStyleBlend::LS_VALUE_SUB as i32;
pub const LS_VALUE_DIV: i32 = eLineStyleBlend::LS_VALUE_DIV as i32;
pub const LS_VALUE_DIFF: i32 = eLineStyleBlend::LS_VALUE_DIFF as i32;
pub const LS_VALUE_MIN: i32 = eLineStyleBlend::LS_VALUE_MIN as i32;
pub const LS_VALUE_MAX: i32 = eLineStyleBlend::LS_VALUE_MAX as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLineStyleMaterialAttr {
    LS_MODIFIER_MATERIAL_DIFF = 1,
    LS_MODIFIER_MATERIAL_DIFF_R = 2,
    LS_MODIFIER_MATERIAL_DIFF_G = 3,
    LS_MODIFIER_MATERIAL_DIFF_B = 4,
    LS_MODIFIER_MATERIAL_SPEC = 5,
    LS_MODIFIER_MATERIAL_SPEC_R = 6,
    LS_MODIFIER_MATERIAL_SPEC_G = 7,
    LS_MODIFIER_MATERIAL_SPEC_B = 8,
    LS_MODIFIER_MATERIAL_SPEC_HARD = 9,
    LS_MODIFIER_MATERIAL_ALPHA = 10,
    LS_MODIFIER_MATERIAL_LINE = 11,
    LS_MODIFIER_MATERIAL_LINE_R = 12,
    LS_MODIFIER_MATERIAL_LINE_G = 13,
    LS_MODIFIER_MATERIAL_LINE_B = 14,
    LS_MODIFIER_MATERIAL_LINE_A = 15,
}

impl Default for eLineStyleMaterialAttr {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LS_MODIFIER_MATERIAL_DIFF: i32 = eLineStyleMaterialAttr::LS_MODIFIER_MATERIAL_DIFF as i32;
pub const LS_MODIFIER_MATERIAL_DIFF_R: i32 = eLineStyleMaterialAttr::LS_MODIFIER_MATERIAL_DIFF_R as i32;
pub const LS_MODIFIER_MATERIAL_DIFF_G: i32 = eLineStyleMaterialAttr::LS_MODIFIER_MATERIAL_DIFF_G as i32;
pub const LS_MODIFIER_MATERIAL_DIFF_B: i32 = eLineStyleMaterialAttr::LS_MODIFIER_MATERIAL_DIFF_B as i32;
pub const LS_MODIFIER_MATERIAL_SPEC: i32 = eLineStyleMaterialAttr::LS_MODIFIER_MATERIAL_SPEC as i32;
pub const LS_MODIFIER_MATERIAL_SPEC_R: i32 = eLineStyleMaterialAttr::LS_MODIFIER_MATERIAL_SPEC_R as i32;
pub const LS_MODIFIER_MATERIAL_SPEC_G: i32 = eLineStyleMaterialAttr::LS_MODIFIER_MATERIAL_SPEC_G as i32;
pub const LS_MODIFIER_MATERIAL_SPEC_B: i32 = eLineStyleMaterialAttr::LS_MODIFIER_MATERIAL_SPEC_B as i32;
pub const LS_MODIFIER_MATERIAL_SPEC_HARD: i32 = eLineStyleMaterialAttr::LS_MODIFIER_MATERIAL_SPEC_HARD as i32;
pub const LS_MODIFIER_MATERIAL_ALPHA: i32 = eLineStyleMaterialAttr::LS_MODIFIER_MATERIAL_ALPHA as i32;
pub const LS_MODIFIER_MATERIAL_LINE: i32 = eLineStyleMaterialAttr::LS_MODIFIER_MATERIAL_LINE as i32;
pub const LS_MODIFIER_MATERIAL_LINE_R: i32 = eLineStyleMaterialAttr::LS_MODIFIER_MATERIAL_LINE_R as i32;
pub const LS_MODIFIER_MATERIAL_LINE_G: i32 = eLineStyleMaterialAttr::LS_MODIFIER_MATERIAL_LINE_G as i32;
pub const LS_MODIFIER_MATERIAL_LINE_B: i32 = eLineStyleMaterialAttr::LS_MODIFIER_MATERIAL_LINE_B as i32;
pub const LS_MODIFIER_MATERIAL_LINE_A: i32 = eLineStyleMaterialAttr::LS_MODIFIER_MATERIAL_LINE_A as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLineStyleGeomSpatialNoise_Flag {
    LS_MODIFIER_SPATIAL_NOISE_SMOOTH = 1,
    LS_MODIFIER_SPATIAL_NOISE_PURERANDOM = 2,
}

impl Default for eLineStyleGeomSpatialNoise_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LS_MODIFIER_SPATIAL_NOISE_SMOOTH: i32 = eLineStyleGeomSpatialNoise_Flag::LS_MODIFIER_SPATIAL_NOISE_SMOOTH as i32;
pub const LS_MODIFIER_SPATIAL_NOISE_PURERANDOM: i32 = eLineStyleGeomSpatialNoise_Flag::LS_MODIFIER_SPATIAL_NOISE_PURERANDOM as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLineStyleGeomBlueprint_Flag {
    LS_MODIFIER_BLUEPRINT_CIRCLES = 1,
    LS_MODIFIER_BLUEPRINT_ELLIPSES = 2,
    LS_MODIFIER_BLUEPRINT_SQUARES = 4,
}

impl Default for eLineStyleGeomBlueprint_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LS_MODIFIER_BLUEPRINT_CIRCLES: i32 = eLineStyleGeomBlueprint_Flag::LS_MODIFIER_BLUEPRINT_CIRCLES as i32;
pub const LS_MODIFIER_BLUEPRINT_ELLIPSES: i32 = eLineStyleGeomBlueprint_Flag::LS_MODIFIER_BLUEPRINT_ELLIPSES as i32;
pub const LS_MODIFIER_BLUEPRINT_SQUARES: i32 = eLineStyleGeomBlueprint_Flag::LS_MODIFIER_BLUEPRINT_SQUARES as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLineStyleGeom2DTransform_Pivot {
    LS_MODIFIER_2D_TRANSFORM_PIVOT_CENTER = 1,
    LS_MODIFIER_2D_TRANSFORM_PIVOT_START = 2,
    LS_MODIFIER_2D_TRANSFORM_PIVOT_END = 3,
    LS_MODIFIER_2D_TRANSFORM_PIVOT_PARAM = 4,
    LS_MODIFIER_2D_TRANSFORM_PIVOT_ABSOLUTE = 5,
}

impl Default for eLineStyleGeom2DTransform_Pivot {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LS_MODIFIER_2D_TRANSFORM_PIVOT_CENTER: i32 = eLineStyleGeom2DTransform_Pivot::LS_MODIFIER_2D_TRANSFORM_PIVOT_CENTER as i32;
pub const LS_MODIFIER_2D_TRANSFORM_PIVOT_START: i32 = eLineStyleGeom2DTransform_Pivot::LS_MODIFIER_2D_TRANSFORM_PIVOT_START as i32;
pub const LS_MODIFIER_2D_TRANSFORM_PIVOT_END: i32 = eLineStyleGeom2DTransform_Pivot::LS_MODIFIER_2D_TRANSFORM_PIVOT_END as i32;
pub const LS_MODIFIER_2D_TRANSFORM_PIVOT_PARAM: i32 = eLineStyleGeom2DTransform_Pivot::LS_MODIFIER_2D_TRANSFORM_PIVOT_PARAM as i32;
pub const LS_MODIFIER_2D_TRANSFORM_PIVOT_ABSOLUTE: i32 = eLineStyleGeom2DTransform_Pivot::LS_MODIFIER_2D_TRANSFORM_PIVOT_ABSOLUTE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLineStyle_Panel {
    LS_PANEL_STROKES = 1,
    LS_PANEL_COLOR = 2,
    LS_PANEL_ALPHA = 3,
    LS_PANEL_THICKNESS = 4,
    LS_PANEL_GEOMETRY = 5,
    LS_PANEL_TEXTURE = 6,
    LS_PANEL_MISC = 7,
}

impl Default for eLineStyle_Panel {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LS_PANEL_STROKES: i32 = eLineStyle_Panel::LS_PANEL_STROKES as i32;
pub const LS_PANEL_COLOR: i32 = eLineStyle_Panel::LS_PANEL_COLOR as i32;
pub const LS_PANEL_ALPHA: i32 = eLineStyle_Panel::LS_PANEL_ALPHA as i32;
pub const LS_PANEL_THICKNESS: i32 = eLineStyle_Panel::LS_PANEL_THICKNESS as i32;
pub const LS_PANEL_GEOMETRY: i32 = eLineStyle_Panel::LS_PANEL_GEOMETRY as i32;
pub const LS_PANEL_TEXTURE: i32 = eLineStyle_Panel::LS_PANEL_TEXTURE as i32;
pub const LS_PANEL_MISC: i32 = eLineStyle_Panel::LS_PANEL_MISC as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLineStyle_Flag {
    LS_DS_EXPAND = 1 << 0,
    LS_SAME_OBJECT = 1 << 1,
    LS_DASHED_LINE = 1 << 2,
    LS_MATERIAL_BOUNDARY = 1 << 3,
    LS_MIN_2D_LENGTH = 1 << 4,
    LS_MAX_2D_LENGTH = 1 << 5,
    LS_NO_CHAINING = 1 << 6,
    LS_MIN_2D_ANGLE = 1 << 7,
    LS_MAX_2D_ANGLE = 1 << 8,
    LS_SPLIT_LENGTH = 1 << 9,
    LS_SPLIT_PATTERN = 1 << 10,
    LS_NO_SORTING = 1 << 11,
    LS_REVERSE_ORDER = 1 << 12,
    LS_TEXTURE = 1 << 13,
    LS_CHAIN_COUNT = 1 << 14,
}

impl Default for eLineStyle_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LS_DS_EXPAND: i32 = eLineStyle_Flag::LS_DS_EXPAND as i32;
pub const LS_SAME_OBJECT: i32 = eLineStyle_Flag::LS_SAME_OBJECT as i32;
pub const LS_DASHED_LINE: i32 = eLineStyle_Flag::LS_DASHED_LINE as i32;
pub const LS_MATERIAL_BOUNDARY: i32 = eLineStyle_Flag::LS_MATERIAL_BOUNDARY as i32;
pub const LS_MIN_2D_LENGTH: i32 = eLineStyle_Flag::LS_MIN_2D_LENGTH as i32;
pub const LS_MAX_2D_LENGTH: i32 = eLineStyle_Flag::LS_MAX_2D_LENGTH as i32;
pub const LS_NO_CHAINING: i32 = eLineStyle_Flag::LS_NO_CHAINING as i32;
pub const LS_MIN_2D_ANGLE: i32 = eLineStyle_Flag::LS_MIN_2D_ANGLE as i32;
pub const LS_MAX_2D_ANGLE: i32 = eLineStyle_Flag::LS_MAX_2D_ANGLE as i32;
pub const LS_SPLIT_LENGTH: i32 = eLineStyle_Flag::LS_SPLIT_LENGTH as i32;
pub const LS_SPLIT_PATTERN: i32 = eLineStyle_Flag::LS_SPLIT_PATTERN as i32;
pub const LS_NO_SORTING: i32 = eLineStyle_Flag::LS_NO_SORTING as i32;
pub const LS_REVERSE_ORDER: i32 = eLineStyle_Flag::LS_REVERSE_ORDER as i32;
pub const LS_TEXTURE: i32 = eLineStyle_Flag::LS_TEXTURE as i32;
pub const LS_CHAIN_COUNT: i32 = eLineStyle_Flag::LS_CHAIN_COUNT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLineStyle_Chaining {
    LS_CHAINING_PLAIN = 1,
    LS_CHAINING_SKETCHY = 2,
}

impl Default for eLineStyle_Chaining {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LS_CHAINING_PLAIN: i32 = eLineStyle_Chaining::LS_CHAINING_PLAIN as i32;
pub const LS_CHAINING_SKETCHY: i32 = eLineStyle_Chaining::LS_CHAINING_SKETCHY as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLineStyle_Caps {
    LS_CAPS_BUTT = 1,
    LS_CAPS_ROUND = 2,
    LS_CAPS_SQUARE = 3,
}

impl Default for eLineStyle_Caps {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LS_CAPS_BUTT: i32 = eLineStyle_Caps::LS_CAPS_BUTT as i32;
pub const LS_CAPS_ROUND: i32 = eLineStyle_Caps::LS_CAPS_ROUND as i32;
pub const LS_CAPS_SQUARE: i32 = eLineStyle_Caps::LS_CAPS_SQUARE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLineStyle_ThicknessPosition {
    LS_THICKNESS_CENTER = 1,
    LS_THICKNESS_INSIDE = 2,
    LS_THICKNESS_OUTSIDE = 3,
    LS_THICKNESS_RELATIVE = 4,
}

impl Default for eLineStyle_ThicknessPosition {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LS_THICKNESS_CENTER: i32 = eLineStyle_ThicknessPosition::LS_THICKNESS_CENTER as i32;
pub const LS_THICKNESS_INSIDE: i32 = eLineStyle_ThicknessPosition::LS_THICKNESS_INSIDE as i32;
pub const LS_THICKNESS_OUTSIDE: i32 = eLineStyle_ThicknessPosition::LS_THICKNESS_OUTSIDE as i32;
pub const LS_THICKNESS_RELATIVE: i32 = eLineStyle_ThicknessPosition::LS_THICKNESS_RELATIVE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLineStyle_SortKey {
    LS_SORT_KEY_DISTANCE_FROM_CAMERA = 1,
    LS_SORT_KEY_2D_LENGTH = 2,
    LS_SORT_KEY_PROJECTED_X = 3,
    LS_SORT_KEY_PROJECTED_Y = 4,
}

impl Default for eLineStyle_SortKey {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LS_SORT_KEY_DISTANCE_FROM_CAMERA: i32 = eLineStyle_SortKey::LS_SORT_KEY_DISTANCE_FROM_CAMERA as i32;
pub const LS_SORT_KEY_2D_LENGTH: i32 = eLineStyle_SortKey::LS_SORT_KEY_2D_LENGTH as i32;
pub const LS_SORT_KEY_PROJECTED_X: i32 = eLineStyle_SortKey::LS_SORT_KEY_PROJECTED_X as i32;
pub const LS_SORT_KEY_PROJECTED_Y: i32 = eLineStyle_SortKey::LS_SORT_KEY_PROJECTED_Y as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLineStyle_IntegrationType {
    LS_INTEGRATION_MEAN = 1,
    LS_INTEGRATION_MIN = 2,
    LS_INTEGRATION_MAX = 3,
    LS_INTEGRATION_FIRST = 4,
    LS_INTEGRATION_LAST = 5,
}

impl Default for eLineStyle_IntegrationType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LS_INTEGRATION_MEAN: i32 = eLineStyle_IntegrationType::LS_INTEGRATION_MEAN as i32;
pub const LS_INTEGRATION_MIN: i32 = eLineStyle_IntegrationType::LS_INTEGRATION_MIN as i32;
pub const LS_INTEGRATION_MAX: i32 = eLineStyle_IntegrationType::LS_INTEGRATION_MAX as i32;
pub const LS_INTEGRATION_FIRST: i32 = eLineStyle_IntegrationType::LS_INTEGRATION_FIRST as i32;
pub const LS_INTEGRATION_LAST: i32 = eLineStyle_IntegrationType::LS_INTEGRATION_LAST as i32;

