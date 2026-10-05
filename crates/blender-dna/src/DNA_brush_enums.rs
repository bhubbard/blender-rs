//! Auto-transpiled C/C++ header module: DNA_brush_enums

use crate::*;

pub const MAX_BRUSH_PIXEL_RADIUS: i32 = 500;
pub const MAX_BRUSH_PIXEL_DIAMETER: i32 = 1000;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGPDbrush_Flag {
    GP_BRUSH_USE_PRESSURE = (1 << 0),
    GP_BRUSH_USE_STRENGTH_PRESSURE = (1 << 1),
    GP_BRUSH_USE_JITTER_PRESSURE = (1 << 2),
    GP_BRUSH_FILL_FIT_DISABLE = (1 << 3),
    GP_BRUSH_FILL_SHOW_EXTENDLINES = (1 << 4),
    GP_BRUSH_FILL_HIDE = (1 << 6),
    GP_BRUSH_FILL_SHOW_HELPLINES = (1 << 7),
    GP_BRUSH_STABILIZE_MOUSE = (1 << 8),
    GP_BRUSH_STABILIZE_MOUSE_TEMP = (1 << 9),
    GP_BRUSH_UNUSED_1 = (1 << 10),
    GP_BRUSH_GROUP_SETTINGS = (1 << 11),
    GP_BRUSH_GROUP_RANDOM = (1 << 12),
    GP_BRUSH_MATERIAL_PINNED = (1 << 13),
    GP_BRUSH_DISSABLE_LASSO = (1 << 14),
    GP_BRUSH_OCCLUDE_ERASER = (1 << 15),
    GP_BRUSH_TRIM_STROKE = (1 << 16),
    GP_BRUSH_OUTLINE_STROKE = (1 << 17),
    GP_BRUSH_FILL_STROKE_COLLIDE = (1 << 18),
    GP_BRUSH_ERASER_KEEP_CAPS = (1 << 19),
    GP_BRUSH_ACTIVE_LAYER_ONLY = (1 << 20),
    GP_BRUSH_FILL_AUTO_REMOVE_FILL_GUIDES = (1 << 21),
    GP_BRUSH_FILL_INTERNAL_GAPS = (1 << 22),
    GP_BRUSH_USE_CYCLIC_STROKE = (1 << 23),
}

impl Default for eGPDbrush_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GP_BRUSH_USE_PRESSURE: i32 = eGPDbrush_Flag::GP_BRUSH_USE_PRESSURE as i32;
pub const GP_BRUSH_USE_STRENGTH_PRESSURE: i32 = eGPDbrush_Flag::GP_BRUSH_USE_STRENGTH_PRESSURE as i32;
pub const GP_BRUSH_USE_JITTER_PRESSURE: i32 = eGPDbrush_Flag::GP_BRUSH_USE_JITTER_PRESSURE as i32;
pub const GP_BRUSH_FILL_FIT_DISABLE: i32 = eGPDbrush_Flag::GP_BRUSH_FILL_FIT_DISABLE as i32;
pub const GP_BRUSH_FILL_SHOW_EXTENDLINES: i32 = eGPDbrush_Flag::GP_BRUSH_FILL_SHOW_EXTENDLINES as i32;
pub const GP_BRUSH_FILL_HIDE: i32 = eGPDbrush_Flag::GP_BRUSH_FILL_HIDE as i32;
pub const GP_BRUSH_FILL_SHOW_HELPLINES: i32 = eGPDbrush_Flag::GP_BRUSH_FILL_SHOW_HELPLINES as i32;
pub const GP_BRUSH_STABILIZE_MOUSE: i32 = eGPDbrush_Flag::GP_BRUSH_STABILIZE_MOUSE as i32;
pub const GP_BRUSH_STABILIZE_MOUSE_TEMP: i32 = eGPDbrush_Flag::GP_BRUSH_STABILIZE_MOUSE_TEMP as i32;
pub const GP_BRUSH_UNUSED_1: i32 = eGPDbrush_Flag::GP_BRUSH_UNUSED_1 as i32;
pub const GP_BRUSH_GROUP_SETTINGS: i32 = eGPDbrush_Flag::GP_BRUSH_GROUP_SETTINGS as i32;
pub const GP_BRUSH_GROUP_RANDOM: i32 = eGPDbrush_Flag::GP_BRUSH_GROUP_RANDOM as i32;
pub const GP_BRUSH_MATERIAL_PINNED: i32 = eGPDbrush_Flag::GP_BRUSH_MATERIAL_PINNED as i32;
pub const GP_BRUSH_DISSABLE_LASSO: i32 = eGPDbrush_Flag::GP_BRUSH_DISSABLE_LASSO as i32;
pub const GP_BRUSH_OCCLUDE_ERASER: i32 = eGPDbrush_Flag::GP_BRUSH_OCCLUDE_ERASER as i32;
pub const GP_BRUSH_TRIM_STROKE: i32 = eGPDbrush_Flag::GP_BRUSH_TRIM_STROKE as i32;
pub const GP_BRUSH_OUTLINE_STROKE: i32 = eGPDbrush_Flag::GP_BRUSH_OUTLINE_STROKE as i32;
pub const GP_BRUSH_FILL_STROKE_COLLIDE: i32 = eGPDbrush_Flag::GP_BRUSH_FILL_STROKE_COLLIDE as i32;
pub const GP_BRUSH_ERASER_KEEP_CAPS: i32 = eGPDbrush_Flag::GP_BRUSH_ERASER_KEEP_CAPS as i32;
pub const GP_BRUSH_ACTIVE_LAYER_ONLY: i32 = eGPDbrush_Flag::GP_BRUSH_ACTIVE_LAYER_ONLY as i32;
pub const GP_BRUSH_FILL_AUTO_REMOVE_FILL_GUIDES: i32 = eGPDbrush_Flag::GP_BRUSH_FILL_AUTO_REMOVE_FILL_GUIDES as i32;
pub const GP_BRUSH_FILL_INTERNAL_GAPS: i32 = eGPDbrush_Flag::GP_BRUSH_FILL_INTERNAL_GAPS as i32;
pub const GP_BRUSH_USE_CYCLIC_STROKE: i32 = eGPDbrush_Flag::GP_BRUSH_USE_CYCLIC_STROKE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGPDbrush_Flag2 {
    GP_BRUSH_UNUSED_2 = (1 << 0),
    GP_BRUSH_UNUSED_3 = (1 << 1),
    GP_BRUSH_UNUSED_4 = (1 << 2),
    GP_BRUSH_USE_PRESS_AT_STROKE = (1 << 3),
    GP_BRUSH_USE_STRENGTH_AT_STROKE = (1 << 4),
    GP_BRUSH_USE_UV_AT_STROKE = (1 << 5),
    GP_BRUSH_UNUSED_5 = (1 << 6),
    GP_BRUSH_UNUSED_6 = (1 << 7),
    GP_BRUSH_UNUSED_7 = (1 << 8),
    GP_BRUSH_USE_PRESSURE_RAND_PRESS = (1 << 9),
    GP_BRUSH_USE_STRENGTH_RAND_PRESS = (1 << 10),
    GP_BRUSH_USE_UV_RAND_PRESS = (1 << 11),
    GP_BRUSH_USE_STROKE = (1 << 12),
    GP_BRUSH_USE_FILL = (1 << 13),
}

impl Default for eGPDbrush_Flag2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GP_BRUSH_UNUSED_2: i32 = eGPDbrush_Flag2::GP_BRUSH_UNUSED_2 as i32;
pub const GP_BRUSH_UNUSED_3: i32 = eGPDbrush_Flag2::GP_BRUSH_UNUSED_3 as i32;
pub const GP_BRUSH_UNUSED_4: i32 = eGPDbrush_Flag2::GP_BRUSH_UNUSED_4 as i32;
pub const GP_BRUSH_USE_PRESS_AT_STROKE: i32 = eGPDbrush_Flag2::GP_BRUSH_USE_PRESS_AT_STROKE as i32;
pub const GP_BRUSH_USE_STRENGTH_AT_STROKE: i32 = eGPDbrush_Flag2::GP_BRUSH_USE_STRENGTH_AT_STROKE as i32;
pub const GP_BRUSH_USE_UV_AT_STROKE: i32 = eGPDbrush_Flag2::GP_BRUSH_USE_UV_AT_STROKE as i32;
pub const GP_BRUSH_UNUSED_5: i32 = eGPDbrush_Flag2::GP_BRUSH_UNUSED_5 as i32;
pub const GP_BRUSH_UNUSED_6: i32 = eGPDbrush_Flag2::GP_BRUSH_UNUSED_6 as i32;
pub const GP_BRUSH_UNUSED_7: i32 = eGPDbrush_Flag2::GP_BRUSH_UNUSED_7 as i32;
pub const GP_BRUSH_USE_PRESSURE_RAND_PRESS: i32 = eGPDbrush_Flag2::GP_BRUSH_USE_PRESSURE_RAND_PRESS as i32;
pub const GP_BRUSH_USE_STRENGTH_RAND_PRESS: i32 = eGPDbrush_Flag2::GP_BRUSH_USE_STRENGTH_RAND_PRESS as i32;
pub const GP_BRUSH_USE_UV_RAND_PRESS: i32 = eGPDbrush_Flag2::GP_BRUSH_USE_UV_RAND_PRESS as i32;
pub const GP_BRUSH_USE_STROKE: i32 = eGPDbrush_Flag2::GP_BRUSH_USE_STROKE as i32;
pub const GP_BRUSH_USE_FILL: i32 = eGPDbrush_Flag2::GP_BRUSH_USE_FILL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGP_FillDrawModes {
    GP_FILL_DMODE_BOTH = 0,
    GP_FILL_DMODE_STROKE = 1,
    GP_FILL_DMODE_CONTROL = 2,
}

impl Default for eGP_FillDrawModes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GP_FILL_DMODE_BOTH: i32 = eGP_FillDrawModes::GP_FILL_DMODE_BOTH as i32;
pub const GP_FILL_DMODE_STROKE: i32 = eGP_FillDrawModes::GP_FILL_DMODE_STROKE as i32;
pub const GP_FILL_DMODE_CONTROL: i32 = eGP_FillDrawModes::GP_FILL_DMODE_CONTROL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGP_FillExtendModes {
    GP_FILL_EMODE_EXTEND = 0,
    GP_FILL_EMODE_RADIUS = 1,
}

impl Default for eGP_FillExtendModes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GP_FILL_EMODE_EXTEND: i32 = eGP_FillExtendModes::GP_FILL_EMODE_EXTEND as i32;
pub const GP_FILL_EMODE_RADIUS: i32 = eGP_FillExtendModes::GP_FILL_EMODE_RADIUS as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGP_FillLayerModes {
    GP_FILL_GPLMODE_VISIBLE = 0,
    GP_FILL_GPLMODE_ACTIVE = 1,
    GP_FILL_GPLMODE_ALL_ABOVE = 2,
    GP_FILL_GPLMODE_ALL_BELOW = 3,
    GP_FILL_GPLMODE_ABOVE = 4,
    GP_FILL_GPLMODE_BELOW = 5,
}

impl Default for eGP_FillLayerModes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GP_FILL_GPLMODE_VISIBLE: i32 = eGP_FillLayerModes::GP_FILL_GPLMODE_VISIBLE as i32;
pub const GP_FILL_GPLMODE_ACTIVE: i32 = eGP_FillLayerModes::GP_FILL_GPLMODE_ACTIVE as i32;
pub const GP_FILL_GPLMODE_ALL_ABOVE: i32 = eGP_FillLayerModes::GP_FILL_GPLMODE_ALL_ABOVE as i32;
pub const GP_FILL_GPLMODE_ALL_BELOW: i32 = eGP_FillLayerModes::GP_FILL_GPLMODE_ALL_BELOW as i32;
pub const GP_FILL_GPLMODE_ABOVE: i32 = eGP_FillLayerModes::GP_FILL_GPLMODE_ABOVE as i32;
pub const GP_FILL_GPLMODE_BELOW: i32 = eGP_FillLayerModes::GP_FILL_GPLMODE_BELOW as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGP_FillSolver {
    GP_FILL_SOLVER_DELAUNAY = 0,
    GP_FILL_SOLVER_PIXEL = 1,
}

impl Default for eGP_FillSolver {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GP_FILL_SOLVER_DELAUNAY: i32 = eGP_FillSolver::GP_FILL_SOLVER_DELAUNAY as i32;
pub const GP_FILL_SOLVER_PIXEL: i32 = eGP_FillSolver::GP_FILL_SOLVER_PIXEL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGP_BrushEraserMode {
    GP_BRUSH_ERASER_SOFT = 0,
    GP_BRUSH_ERASER_HARD = 1,
    GP_BRUSH_ERASER_STROKE = 2,
}

impl Default for eGP_BrushEraserMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GP_BRUSH_ERASER_SOFT: i32 = eGP_BrushEraserMode::GP_BRUSH_ERASER_SOFT as i32;
pub const GP_BRUSH_ERASER_HARD: i32 = eGP_BrushEraserMode::GP_BRUSH_ERASER_HARD as i32;
pub const GP_BRUSH_ERASER_STROKE: i32 = eGP_BrushEraserMode::GP_BRUSH_ERASER_STROKE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGP_BrushMode {
    GP_BRUSH_MODE_ACTIVE = 0,
    GP_BRUSH_MODE_MATERIAL = 1,
    GP_BRUSH_MODE_VERTEXCOLOR = 2,
}

impl Default for eGP_BrushMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GP_BRUSH_MODE_ACTIVE: i32 = eGP_BrushMode::GP_BRUSH_MODE_ACTIVE as i32;
pub const GP_BRUSH_MODE_MATERIAL: i32 = eGP_BrushMode::GP_BRUSH_MODE_MATERIAL as i32;
pub const GP_BRUSH_MODE_VERTEXCOLOR: i32 = eGP_BrushMode::GP_BRUSH_MODE_VERTEXCOLOR as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushCurvePreset {
    BRUSH_CURVE_CUSTOM = 0,
    BRUSH_CURVE_SMOOTH = 1,
    BRUSH_CURVE_SPHERE = 2,
    BRUSH_CURVE_ROOT = 3,
    BRUSH_CURVE_SHARP = 4,
    BRUSH_CURVE_LIN = 5,
    BRUSH_CURVE_POW4 = 6,
    BRUSH_CURVE_INVSQUARE = 7,
    BRUSH_CURVE_CONSTANT = 8,
    BRUSH_CURVE_SMOOTHER = 9,
}

impl Default for eBrushCurvePreset {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_CURVE_CUSTOM: i32 = eBrushCurvePreset::BRUSH_CURVE_CUSTOM as i32;
pub const BRUSH_CURVE_SMOOTH: i32 = eBrushCurvePreset::BRUSH_CURVE_SMOOTH as i32;
pub const BRUSH_CURVE_SPHERE: i32 = eBrushCurvePreset::BRUSH_CURVE_SPHERE as i32;
pub const BRUSH_CURVE_ROOT: i32 = eBrushCurvePreset::BRUSH_CURVE_ROOT as i32;
pub const BRUSH_CURVE_SHARP: i32 = eBrushCurvePreset::BRUSH_CURVE_SHARP as i32;
pub const BRUSH_CURVE_LIN: i32 = eBrushCurvePreset::BRUSH_CURVE_LIN as i32;
pub const BRUSH_CURVE_POW4: i32 = eBrushCurvePreset::BRUSH_CURVE_POW4 as i32;
pub const BRUSH_CURVE_INVSQUARE: i32 = eBrushCurvePreset::BRUSH_CURVE_INVSQUARE as i32;
pub const BRUSH_CURVE_CONSTANT: i32 = eBrushCurvePreset::BRUSH_CURVE_CONSTANT as i32;
pub const BRUSH_CURVE_SMOOTHER: i32 = eBrushCurvePreset::BRUSH_CURVE_SMOOTHER as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushDeformTarget {
    BRUSH_DEFORM_TARGET_GEOMETRY = 0,
    BRUSH_DEFORM_TARGET_CLOTH_SIM = 1,
}

impl Default for eBrushDeformTarget {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_DEFORM_TARGET_GEOMETRY: i32 = eBrushDeformTarget::BRUSH_DEFORM_TARGET_GEOMETRY as i32;
pub const BRUSH_DEFORM_TARGET_CLOTH_SIM: i32 = eBrushDeformTarget::BRUSH_DEFORM_TARGET_CLOTH_SIM as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushElasticDeformType {
    BRUSH_ELASTIC_DEFORM_GRAB = 0,
    BRUSH_ELASTIC_DEFORM_GRAB_BISCALE = 1,
    BRUSH_ELASTIC_DEFORM_GRAB_TRISCALE = 2,
    BRUSH_ELASTIC_DEFORM_SCALE = 3,
    BRUSH_ELASTIC_DEFORM_TWIST = 4,
}

impl Default for eBrushElasticDeformType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_ELASTIC_DEFORM_GRAB: i32 = eBrushElasticDeformType::BRUSH_ELASTIC_DEFORM_GRAB as i32;
pub const BRUSH_ELASTIC_DEFORM_GRAB_BISCALE: i32 = eBrushElasticDeformType::BRUSH_ELASTIC_DEFORM_GRAB_BISCALE as i32;
pub const BRUSH_ELASTIC_DEFORM_GRAB_TRISCALE: i32 = eBrushElasticDeformType::BRUSH_ELASTIC_DEFORM_GRAB_TRISCALE as i32;
pub const BRUSH_ELASTIC_DEFORM_SCALE: i32 = eBrushElasticDeformType::BRUSH_ELASTIC_DEFORM_SCALE as i32;
pub const BRUSH_ELASTIC_DEFORM_TWIST: i32 = eBrushElasticDeformType::BRUSH_ELASTIC_DEFORM_TWIST as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushClothDeformType {
    BRUSH_CLOTH_DEFORM_DRAG = 0,
    BRUSH_CLOTH_DEFORM_PUSH = 1,
    BRUSH_CLOTH_DEFORM_GRAB = 2,
    BRUSH_CLOTH_DEFORM_PINCH_POINT = 3,
    BRUSH_CLOTH_DEFORM_PINCH_PERPENDICULAR = 4,
    BRUSH_CLOTH_DEFORM_INFLATE = 5,
    BRUSH_CLOTH_DEFORM_EXPAND = 6,
    BRUSH_CLOTH_DEFORM_SNAKE_HOOK = 7,
}

impl Default for eBrushClothDeformType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_CLOTH_DEFORM_DRAG: i32 = eBrushClothDeformType::BRUSH_CLOTH_DEFORM_DRAG as i32;
pub const BRUSH_CLOTH_DEFORM_PUSH: i32 = eBrushClothDeformType::BRUSH_CLOTH_DEFORM_PUSH as i32;
pub const BRUSH_CLOTH_DEFORM_GRAB: i32 = eBrushClothDeformType::BRUSH_CLOTH_DEFORM_GRAB as i32;
pub const BRUSH_CLOTH_DEFORM_PINCH_POINT: i32 = eBrushClothDeformType::BRUSH_CLOTH_DEFORM_PINCH_POINT as i32;
pub const BRUSH_CLOTH_DEFORM_PINCH_PERPENDICULAR: i32 = eBrushClothDeformType::BRUSH_CLOTH_DEFORM_PINCH_PERPENDICULAR as i32;
pub const BRUSH_CLOTH_DEFORM_INFLATE: i32 = eBrushClothDeformType::BRUSH_CLOTH_DEFORM_INFLATE as i32;
pub const BRUSH_CLOTH_DEFORM_EXPAND: i32 = eBrushClothDeformType::BRUSH_CLOTH_DEFORM_EXPAND as i32;
pub const BRUSH_CLOTH_DEFORM_SNAKE_HOOK: i32 = eBrushClothDeformType::BRUSH_CLOTH_DEFORM_SNAKE_HOOK as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushSmoothDeformType {
    BRUSH_SMOOTH_DEFORM_LAPLACIAN = 0,
    BRUSH_SMOOTH_DEFORM_SURFACE = 1,
}

impl Default for eBrushSmoothDeformType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_SMOOTH_DEFORM_LAPLACIAN: i32 = eBrushSmoothDeformType::BRUSH_SMOOTH_DEFORM_LAPLACIAN as i32;
pub const BRUSH_SMOOTH_DEFORM_SURFACE: i32 = eBrushSmoothDeformType::BRUSH_SMOOTH_DEFORM_SURFACE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushClothForceFalloffType {
    BRUSH_CLOTH_FORCE_FALLOFF_RADIAL = 0,
    BRUSH_CLOTH_FORCE_FALLOFF_PLANE = 1,
}

impl Default for eBrushClothForceFalloffType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_CLOTH_FORCE_FALLOFF_RADIAL: i32 = eBrushClothForceFalloffType::BRUSH_CLOTH_FORCE_FALLOFF_RADIAL as i32;
pub const BRUSH_CLOTH_FORCE_FALLOFF_PLANE: i32 = eBrushClothForceFalloffType::BRUSH_CLOTH_FORCE_FALLOFF_PLANE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushClothSimulationAreaType {
    BRUSH_CLOTH_SIMULATION_AREA_LOCAL = 0,
    BRUSH_CLOTH_SIMULATION_AREA_GLOBAL = 1,
    BRUSH_CLOTH_SIMULATION_AREA_DYNAMIC = 2,
}

impl Default for eBrushClothSimulationAreaType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_CLOTH_SIMULATION_AREA_LOCAL: i32 = eBrushClothSimulationAreaType::BRUSH_CLOTH_SIMULATION_AREA_LOCAL as i32;
pub const BRUSH_CLOTH_SIMULATION_AREA_GLOBAL: i32 = eBrushClothSimulationAreaType::BRUSH_CLOTH_SIMULATION_AREA_GLOBAL as i32;
pub const BRUSH_CLOTH_SIMULATION_AREA_DYNAMIC: i32 = eBrushClothSimulationAreaType::BRUSH_CLOTH_SIMULATION_AREA_DYNAMIC as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushPoseDeformType {
    BRUSH_POSE_DEFORM_ROTATE_TWIST = 0,
    BRUSH_POSE_DEFORM_SCALE_TRANSLATE = 1,
    BRUSH_POSE_DEFORM_SQUASH_STRETCH = 2,
}

impl Default for eBrushPoseDeformType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_POSE_DEFORM_ROTATE_TWIST: i32 = eBrushPoseDeformType::BRUSH_POSE_DEFORM_ROTATE_TWIST as i32;
pub const BRUSH_POSE_DEFORM_SCALE_TRANSLATE: i32 = eBrushPoseDeformType::BRUSH_POSE_DEFORM_SCALE_TRANSLATE as i32;
pub const BRUSH_POSE_DEFORM_SQUASH_STRETCH: i32 = eBrushPoseDeformType::BRUSH_POSE_DEFORM_SQUASH_STRETCH as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushPoseOriginType {
    BRUSH_POSE_ORIGIN_TOPOLOGY = 0,
    BRUSH_POSE_ORIGIN_FACE_SETS = 1,
    BRUSH_POSE_ORIGIN_FACE_SETS_FK = 2,
}

impl Default for eBrushPoseOriginType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_POSE_ORIGIN_TOPOLOGY: i32 = eBrushPoseOriginType::BRUSH_POSE_ORIGIN_TOPOLOGY as i32;
pub const BRUSH_POSE_ORIGIN_FACE_SETS: i32 = eBrushPoseOriginType::BRUSH_POSE_ORIGIN_FACE_SETS as i32;
pub const BRUSH_POSE_ORIGIN_FACE_SETS_FK: i32 = eBrushPoseOriginType::BRUSH_POSE_ORIGIN_FACE_SETS_FK as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushSmearDeformType {
    BRUSH_SMEAR_DEFORM_DRAG = 0,
    BRUSH_SMEAR_DEFORM_PINCH = 1,
    BRUSH_SMEAR_DEFORM_EXPAND = 2,
}

impl Default for eBrushSmearDeformType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_SMEAR_DEFORM_DRAG: i32 = eBrushSmearDeformType::BRUSH_SMEAR_DEFORM_DRAG as i32;
pub const BRUSH_SMEAR_DEFORM_PINCH: i32 = eBrushSmearDeformType::BRUSH_SMEAR_DEFORM_PINCH as i32;
pub const BRUSH_SMEAR_DEFORM_EXPAND: i32 = eBrushSmearDeformType::BRUSH_SMEAR_DEFORM_EXPAND as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushSlideDeformType {
    BRUSH_SLIDE_DEFORM_DRAG = 0,
    BRUSH_SLIDE_DEFORM_PINCH = 1,
    BRUSH_SLIDE_DEFORM_EXPAND = 2,
}

impl Default for eBrushSlideDeformType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_SLIDE_DEFORM_DRAG: i32 = eBrushSlideDeformType::BRUSH_SLIDE_DEFORM_DRAG as i32;
pub const BRUSH_SLIDE_DEFORM_PINCH: i32 = eBrushSlideDeformType::BRUSH_SLIDE_DEFORM_PINCH as i32;
pub const BRUSH_SLIDE_DEFORM_EXPAND: i32 = eBrushSlideDeformType::BRUSH_SLIDE_DEFORM_EXPAND as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushBoundaryDeformType {
    BRUSH_BOUNDARY_DEFORM_BEND = 0,
    BRUSH_BOUNDARY_DEFORM_EXPAND = 1,
    BRUSH_BOUNDARY_DEFORM_INFLATE = 2,
    BRUSH_BOUNDARY_DEFORM_GRAB = 3,
    BRUSH_BOUNDARY_DEFORM_TWIST = 4,
    BRUSH_BOUNDARY_DEFORM_SMOOTH = 5,
}

impl Default for eBrushBoundaryDeformType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_BOUNDARY_DEFORM_BEND: i32 = eBrushBoundaryDeformType::BRUSH_BOUNDARY_DEFORM_BEND as i32;
pub const BRUSH_BOUNDARY_DEFORM_EXPAND: i32 = eBrushBoundaryDeformType::BRUSH_BOUNDARY_DEFORM_EXPAND as i32;
pub const BRUSH_BOUNDARY_DEFORM_INFLATE: i32 = eBrushBoundaryDeformType::BRUSH_BOUNDARY_DEFORM_INFLATE as i32;
pub const BRUSH_BOUNDARY_DEFORM_GRAB: i32 = eBrushBoundaryDeformType::BRUSH_BOUNDARY_DEFORM_GRAB as i32;
pub const BRUSH_BOUNDARY_DEFORM_TWIST: i32 = eBrushBoundaryDeformType::BRUSH_BOUNDARY_DEFORM_TWIST as i32;
pub const BRUSH_BOUNDARY_DEFORM_SMOOTH: i32 = eBrushBoundaryDeformType::BRUSH_BOUNDARY_DEFORM_SMOOTH as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushBoundaryFalloffType {
    BRUSH_BOUNDARY_FALLOFF_CONSTANT = 0,
    BRUSH_BOUNDARY_FALLOFF_RADIUS = 1,
    BRUSH_BOUNDARY_FALLOFF_LOOP = 2,
    BRUSH_BOUNDARY_FALLOFF_LOOP_INVERT = 3,
}

impl Default for eBrushBoundaryFalloffType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_BOUNDARY_FALLOFF_CONSTANT: i32 = eBrushBoundaryFalloffType::BRUSH_BOUNDARY_FALLOFF_CONSTANT as i32;
pub const BRUSH_BOUNDARY_FALLOFF_RADIUS: i32 = eBrushBoundaryFalloffType::BRUSH_BOUNDARY_FALLOFF_RADIUS as i32;
pub const BRUSH_BOUNDARY_FALLOFF_LOOP: i32 = eBrushBoundaryFalloffType::BRUSH_BOUNDARY_FALLOFF_LOOP as i32;
pub const BRUSH_BOUNDARY_FALLOFF_LOOP_INVERT: i32 = eBrushBoundaryFalloffType::BRUSH_BOUNDARY_FALLOFF_LOOP_INVERT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushSnakeHookDeformType {
    BRUSH_SNAKE_HOOK_DEFORM_FALLOFF = 0,
    BRUSH_SNAKE_HOOK_DEFORM_ELASTIC = 1,
}

impl Default for eBrushSnakeHookDeformType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_SNAKE_HOOK_DEFORM_FALLOFF: i32 = eBrushSnakeHookDeformType::BRUSH_SNAKE_HOOK_DEFORM_FALLOFF as i32;
pub const BRUSH_SNAKE_HOOK_DEFORM_ELASTIC: i32 = eBrushSnakeHookDeformType::BRUSH_SNAKE_HOOK_DEFORM_ELASTIC as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushPlaneInversionMode {
    BRUSH_PLANE_INVERT_DISPLACEMENT = 0,
    BRUSH_PLANE_SWAP_HEIGHT_AND_DEPTH = 1,
}

impl Default for eBrushPlaneInversionMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_PLANE_INVERT_DISPLACEMENT: i32 = eBrushPlaneInversionMode::BRUSH_PLANE_INVERT_DISPLACEMENT as i32;
pub const BRUSH_PLANE_SWAP_HEIGHT_AND_DEPTH: i32 = eBrushPlaneInversionMode::BRUSH_PLANE_SWAP_HEIGHT_AND_DEPTH as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushProjectRayDirection {
    BRUSH_PROJECT_RAY_DIRECTION_VIEW_NORMAL = 0,
    BRUSH_PROJECT_RAY_DIRECTION_PLANE_NORMAL = 1,
}

impl Default for eBrushProjectRayDirection {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_PROJECT_RAY_DIRECTION_VIEW_NORMAL: i32 = eBrushProjectRayDirection::BRUSH_PROJECT_RAY_DIRECTION_VIEW_NORMAL as i32;
pub const BRUSH_PROJECT_RAY_DIRECTION_PLANE_NORMAL: i32 = eBrushProjectRayDirection::BRUSH_PROJECT_RAY_DIRECTION_PLANE_NORMAL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGp_Vertex_Mode {
    GPPAINT_MODE_STROKE = 0,
    GPPAINT_MODE_FILL = 1,
    GPPAINT_MODE_BOTH = 2,
}

impl Default for eGp_Vertex_Mode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GPPAINT_MODE_STROKE: i32 = eGp_Vertex_Mode::GPPAINT_MODE_STROKE as i32;
pub const GPPAINT_MODE_FILL: i32 = eGp_Vertex_Mode::GPPAINT_MODE_FILL as i32;
pub const GPPAINT_MODE_BOTH: i32 = eGp_Vertex_Mode::GPPAINT_MODE_BOTH as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGP_Sculpt_Flag {
    GP_SCULPT_FLAG_INVERT = (1 << 0),
    GP_SCULPT_FLAG_TMP_INVERT = (1 << 3),
}

impl Default for eGP_Sculpt_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GP_SCULPT_FLAG_INVERT: i32 = eGP_Sculpt_Flag::GP_SCULPT_FLAG_INVERT as i32;
pub const GP_SCULPT_FLAG_TMP_INVERT: i32 = eGP_Sculpt_Flag::GP_SCULPT_FLAG_TMP_INVERT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGP_Sculpt_Mode_Flag {
    GP_SCULPT_FLAGMODE_APPLY_POSITION = (1 << 0),
    GP_SCULPT_FLAGMODE_APPLY_STRENGTH = (1 << 1),
    GP_SCULPT_FLAGMODE_APPLY_THICKNESS = (1 << 2),
    GP_SCULPT_FLAGMODE_APPLY_UV = (1 << 3),
}

impl Default for eGP_Sculpt_Mode_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GP_SCULPT_FLAGMODE_APPLY_POSITION: i32 = eGP_Sculpt_Mode_Flag::GP_SCULPT_FLAGMODE_APPLY_POSITION as i32;
pub const GP_SCULPT_FLAGMODE_APPLY_STRENGTH: i32 = eGP_Sculpt_Mode_Flag::GP_SCULPT_FLAGMODE_APPLY_STRENGTH as i32;
pub const GP_SCULPT_FLAGMODE_APPLY_THICKNESS: i32 = eGP_Sculpt_Mode_Flag::GP_SCULPT_FLAGMODE_APPLY_THICKNESS as i32;
pub const GP_SCULPT_FLAGMODE_APPLY_UV: i32 = eGP_Sculpt_Mode_Flag::GP_SCULPT_FLAGMODE_APPLY_UV as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eAutomasking_flag {
    BRUSH_AUTOMASKING_TOPOLOGY = (1 << 0),
    BRUSH_AUTOMASKING_FACE_SETS = (1 << 1),
    BRUSH_AUTOMASKING_BOUNDARY_EDGES = (1 << 2),
    BRUSH_AUTOMASKING_BOUNDARY_FACE_SETS = (1 << 3),
    BRUSH_AUTOMASKING_CAVITY_NORMAL = (1 << 4),
    BRUSH_AUTOMASKING_CAVITY_INVERTED = (1 << 5),
    BRUSH_AUTOMASKING_CAVITY_ALL = (1 << 4) | (1 << 5),
    BRUSH_AUTOMASKING_CAVITY_USE_CURVE = (1 << 6),
    BRUSH_AUTOMASKING_BRUSH_NORMAL = (1 << 8),
    BRUSH_AUTOMASKING_VIEW_NORMAL = (1 << 9),
    BRUSH_AUTOMASKING_VIEW_OCCLUSION = (1 << 10),
}

impl Default for eAutomasking_flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_AUTOMASKING_TOPOLOGY: i32 = eAutomasking_flag::BRUSH_AUTOMASKING_TOPOLOGY as i32;
pub const BRUSH_AUTOMASKING_FACE_SETS: i32 = eAutomasking_flag::BRUSH_AUTOMASKING_FACE_SETS as i32;
pub const BRUSH_AUTOMASKING_BOUNDARY_EDGES: i32 = eAutomasking_flag::BRUSH_AUTOMASKING_BOUNDARY_EDGES as i32;
pub const BRUSH_AUTOMASKING_BOUNDARY_FACE_SETS: i32 = eAutomasking_flag::BRUSH_AUTOMASKING_BOUNDARY_FACE_SETS as i32;
pub const BRUSH_AUTOMASKING_CAVITY_NORMAL: i32 = eAutomasking_flag::BRUSH_AUTOMASKING_CAVITY_NORMAL as i32;
pub const BRUSH_AUTOMASKING_CAVITY_INVERTED: i32 = eAutomasking_flag::BRUSH_AUTOMASKING_CAVITY_INVERTED as i32;
pub const BRUSH_AUTOMASKING_CAVITY_ALL: i32 = eAutomasking_flag::BRUSH_AUTOMASKING_CAVITY_ALL as i32;
pub const BRUSH_AUTOMASKING_CAVITY_USE_CURVE: i32 = eAutomasking_flag::BRUSH_AUTOMASKING_CAVITY_USE_CURVE as i32;
pub const BRUSH_AUTOMASKING_BRUSH_NORMAL: i32 = eAutomasking_flag::BRUSH_AUTOMASKING_BRUSH_NORMAL as i32;
pub const BRUSH_AUTOMASKING_VIEW_NORMAL: i32 = eAutomasking_flag::BRUSH_AUTOMASKING_VIEW_NORMAL as i32;
pub const BRUSH_AUTOMASKING_VIEW_OCCLUSION: i32 = eAutomasking_flag::BRUSH_AUTOMASKING_VIEW_OCCLUSION as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ePaintBrush_flag {
    BRUSH_PAINT_UNUSED_1 = (1 << 0),
    BRUSH_PAINT_UNUSED_2 = (1 << 1),
    BRUSH_PAINT_FLOW_PRESSURE = (1 << 2),
    BRUSH_PAINT_FLOW_PRESSURE_INVERT = (1 << 3),
    BRUSH_PAINT_WET_MIX_PRESSURE = (1 << 4),
    BRUSH_PAINT_WET_MIX_PRESSURE_INVERT = (1 << 5),
    BRUSH_PAINT_WET_PERSISTENCE_PRESSURE = (1 << 6),
    BRUSH_PAINT_WET_PERSISTENCE_PRESSURE_INVERT = (1 << 7),
    BRUSH_PAINT_DENSITY_PRESSURE = (1 << 8),
    BRUSH_PAINT_DENSITY_PRESSURE_INVERT = (1 << 9),
}

impl Default for ePaintBrush_flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_PAINT_UNUSED_1: i32 = ePaintBrush_flag::BRUSH_PAINT_UNUSED_1 as i32;
pub const BRUSH_PAINT_UNUSED_2: i32 = ePaintBrush_flag::BRUSH_PAINT_UNUSED_2 as i32;
pub const BRUSH_PAINT_FLOW_PRESSURE: i32 = ePaintBrush_flag::BRUSH_PAINT_FLOW_PRESSURE as i32;
pub const BRUSH_PAINT_FLOW_PRESSURE_INVERT: i32 = ePaintBrush_flag::BRUSH_PAINT_FLOW_PRESSURE_INVERT as i32;
pub const BRUSH_PAINT_WET_MIX_PRESSURE: i32 = ePaintBrush_flag::BRUSH_PAINT_WET_MIX_PRESSURE as i32;
pub const BRUSH_PAINT_WET_MIX_PRESSURE_INVERT: i32 = ePaintBrush_flag::BRUSH_PAINT_WET_MIX_PRESSURE_INVERT as i32;
pub const BRUSH_PAINT_WET_PERSISTENCE_PRESSURE: i32 = ePaintBrush_flag::BRUSH_PAINT_WET_PERSISTENCE_PRESSURE as i32;
pub const BRUSH_PAINT_WET_PERSISTENCE_PRESSURE_INVERT: i32 = ePaintBrush_flag::BRUSH_PAINT_WET_PERSISTENCE_PRESSURE_INVERT as i32;
pub const BRUSH_PAINT_DENSITY_PRESSURE: i32 = ePaintBrush_flag::BRUSH_PAINT_DENSITY_PRESSURE as i32;
pub const BRUSH_PAINT_DENSITY_PRESSURE_INVERT: i32 = ePaintBrush_flag::BRUSH_PAINT_DENSITY_PRESSURE_INVERT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushGradientSourceStroke {
    BRUSH_GRADIENT_PRESSURE = 0,
    BRUSH_GRADIENT_SPACING_REPEAT = 1,
    BRUSH_GRADIENT_SPACING_CLAMP = 2,
}

impl Default for eBrushGradientSourceStroke {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_GRADIENT_PRESSURE: i32 = eBrushGradientSourceStroke::BRUSH_GRADIENT_PRESSURE as i32;
pub const BRUSH_GRADIENT_SPACING_REPEAT: i32 = eBrushGradientSourceStroke::BRUSH_GRADIENT_SPACING_REPEAT as i32;
pub const BRUSH_GRADIENT_SPACING_CLAMP: i32 = eBrushGradientSourceStroke::BRUSH_GRADIENT_SPACING_CLAMP as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushGradientSourceFill {
    BRUSH_GRADIENT_LINEAR = 0,
    BRUSH_GRADIENT_RADIAL = 1,
}

impl Default for eBrushGradientSourceFill {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_GRADIENT_LINEAR: i32 = eBrushGradientSourceFill::BRUSH_GRADIENT_LINEAR as i32;
pub const BRUSH_GRADIENT_RADIAL: i32 = eBrushGradientSourceFill::BRUSH_GRADIENT_RADIAL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushStrokeType {
    BRUSH_STROKE_DOTS = 0,
    BRUSH_STROKE_AIRBRUSH = 1,
    BRUSH_STROKE_ANCHORED = 2,
    BRUSH_STROKE_SPACE = 3,
    BRUSH_STROKE_DRAG_DOT = 4,
    BRUSH_STROKE_LINE = 5,
    BRUSH_STROKE_CURVE = 6,
}

impl Default for eBrushStrokeType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_STROKE_DOTS: i32 = eBrushStrokeType::BRUSH_STROKE_DOTS as i32;
pub const BRUSH_STROKE_AIRBRUSH: i32 = eBrushStrokeType::BRUSH_STROKE_AIRBRUSH as i32;
pub const BRUSH_STROKE_ANCHORED: i32 = eBrushStrokeType::BRUSH_STROKE_ANCHORED as i32;
pub const BRUSH_STROKE_SPACE: i32 = eBrushStrokeType::BRUSH_STROKE_SPACE as i32;
pub const BRUSH_STROKE_DRAG_DOT: i32 = eBrushStrokeType::BRUSH_STROKE_DRAG_DOT as i32;
pub const BRUSH_STROKE_LINE: i32 = eBrushStrokeType::BRUSH_STROKE_LINE as i32;
pub const BRUSH_STROKE_CURVE: i32 = eBrushStrokeType::BRUSH_STROKE_CURVE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushFlags {
    BRUSH_HARDNESS_PRESSURE = (1 << 0),
    BRUSH_INVERT_TO_SCRAPE_FILL = (1 << 1),
    BRUSH_ALPHA_PRESSURE = (1 << 2),
    BRUSH_SIZE_PRESSURE = (1 << 3),
    BRUSH_JITTER_PRESSURE = (1 << 4),
    BRUSH_SPACING_PRESSURE = (1 << 5),
    BRUSH_ORIGINAL_PLANE = (1 << 6),
    BRUSH_GRAB_ACTIVE_VERTEX = (1 << 7),
    BRUSH_UNUSED_2 = (1 << 8),
    BRUSH_DIR_IN = (1 << 9),
    BRUSH_UNUSED_3 = (1 << 10),
    BRUSH_SMOOTH_STROKE = (1 << 11),
    BRUSH_PERSISTENT = (1 << 12),
    BRUSH_ACCUMULATE = (1 << 13),
    BRUSH_LOCK_ALPHA = (1 << 14),
    BRUSH_ORIGINAL_NORMAL = (1 << 15),
    BRUSH_OFFSET_PRESSURE = (1 << 16),
    BRUSH_SCENE_SPACING = (1 << 17),
    BRUSH_SPACE_ATTEN = (1 << 18),
    BRUSH_ADAPTIVE_SPACE = (1 << 19),
    BRUSH_LOCK_SIZE = (1 << 20),
    BRUSH_USE_GRADIENT = (1 << 21),
    BRUSH_EDGE_TO_EDGE = (1 << 22),
    BRUSH_UNUSED_4 = (1 << 23),
    BRUSH_SMOOTH_PRESSURE = (1 << 24),
    BRUSH_UNUSED_7 = (1 << 25),
    BRUSH_PLANE_TRIM = (1 << 26),
    BRUSH_FRONTFACE = (1 << 27),
    BRUSH_UNUSED_8 = (1 << 28),
    BRUSH_UNUSED_5 = (1 << 29),
    BRUSH_ABSOLUTE_JITTER = (1 << 30),
    BRUSH_UNUSED_6 = (i32::MIN),
}

impl Default for eBrushFlags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_HARDNESS_PRESSURE: i32 = eBrushFlags::BRUSH_HARDNESS_PRESSURE as i32;
pub const BRUSH_INVERT_TO_SCRAPE_FILL: i32 = eBrushFlags::BRUSH_INVERT_TO_SCRAPE_FILL as i32;
pub const BRUSH_ALPHA_PRESSURE: i32 = eBrushFlags::BRUSH_ALPHA_PRESSURE as i32;
pub const BRUSH_SIZE_PRESSURE: i32 = eBrushFlags::BRUSH_SIZE_PRESSURE as i32;
pub const BRUSH_JITTER_PRESSURE: i32 = eBrushFlags::BRUSH_JITTER_PRESSURE as i32;
pub const BRUSH_SPACING_PRESSURE: i32 = eBrushFlags::BRUSH_SPACING_PRESSURE as i32;
pub const BRUSH_ORIGINAL_PLANE: i32 = eBrushFlags::BRUSH_ORIGINAL_PLANE as i32;
pub const BRUSH_GRAB_ACTIVE_VERTEX: i32 = eBrushFlags::BRUSH_GRAB_ACTIVE_VERTEX as i32;
pub const BRUSH_UNUSED_2: i32 = eBrushFlags::BRUSH_UNUSED_2 as i32;
pub const BRUSH_DIR_IN: i32 = eBrushFlags::BRUSH_DIR_IN as i32;
pub const BRUSH_UNUSED_3: i32 = eBrushFlags::BRUSH_UNUSED_3 as i32;
pub const BRUSH_SMOOTH_STROKE: i32 = eBrushFlags::BRUSH_SMOOTH_STROKE as i32;
pub const BRUSH_PERSISTENT: i32 = eBrushFlags::BRUSH_PERSISTENT as i32;
pub const BRUSH_ACCUMULATE: i32 = eBrushFlags::BRUSH_ACCUMULATE as i32;
pub const BRUSH_LOCK_ALPHA: i32 = eBrushFlags::BRUSH_LOCK_ALPHA as i32;
pub const BRUSH_ORIGINAL_NORMAL: i32 = eBrushFlags::BRUSH_ORIGINAL_NORMAL as i32;
pub const BRUSH_OFFSET_PRESSURE: i32 = eBrushFlags::BRUSH_OFFSET_PRESSURE as i32;
pub const BRUSH_SCENE_SPACING: i32 = eBrushFlags::BRUSH_SCENE_SPACING as i32;
pub const BRUSH_SPACE_ATTEN: i32 = eBrushFlags::BRUSH_SPACE_ATTEN as i32;
pub const BRUSH_ADAPTIVE_SPACE: i32 = eBrushFlags::BRUSH_ADAPTIVE_SPACE as i32;
pub const BRUSH_LOCK_SIZE: i32 = eBrushFlags::BRUSH_LOCK_SIZE as i32;
pub const BRUSH_USE_GRADIENT: i32 = eBrushFlags::BRUSH_USE_GRADIENT as i32;
pub const BRUSH_EDGE_TO_EDGE: i32 = eBrushFlags::BRUSH_EDGE_TO_EDGE as i32;
pub const BRUSH_UNUSED_4: i32 = eBrushFlags::BRUSH_UNUSED_4 as i32;
pub const BRUSH_SMOOTH_PRESSURE: i32 = eBrushFlags::BRUSH_SMOOTH_PRESSURE as i32;
pub const BRUSH_UNUSED_7: i32 = eBrushFlags::BRUSH_UNUSED_7 as i32;
pub const BRUSH_PLANE_TRIM: i32 = eBrushFlags::BRUSH_PLANE_TRIM as i32;
pub const BRUSH_FRONTFACE: i32 = eBrushFlags::BRUSH_FRONTFACE as i32;
pub const BRUSH_UNUSED_8: i32 = eBrushFlags::BRUSH_UNUSED_8 as i32;
pub const BRUSH_UNUSED_5: i32 = eBrushFlags::BRUSH_UNUSED_5 as i32;
pub const BRUSH_ABSOLUTE_JITTER: i32 = eBrushFlags::BRUSH_ABSOLUTE_JITTER as i32;
pub const BRUSH_UNUSED_6: i32 = eBrushFlags::BRUSH_UNUSED_6 as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushSamplingFlags {
    BRUSH_PAINT_ANTIALIASING = (1 << 0),
}

impl Default for eBrushSamplingFlags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_PAINT_ANTIALIASING: i32 = eBrushSamplingFlags::BRUSH_PAINT_ANTIALIASING as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushFlags2 {
    BRUSH_MULTIPLANE_SCRAPE_DYNAMIC = (1 << 0),
    BRUSH_MULTIPLANE_SCRAPE_PLANES_PREVIEW = (1 << 1),
    BRUSH_POSE_IK_ANCHORED = (1 << 2),
    BRUSH_USE_CONNECTED_ONLY = (1 << 3),
    BRUSH_CLOTH_PIN_SIMULATION_BOUNDARY = (1 << 4),
    BRUSH_POSE_USE_LOCK_ROTATION = (1 << 5),
    BRUSH_CLOTH_USE_COLLISION = (1 << 6),
    BRUSH_AREA_RADIUS_PRESSURE = (1 << 7),
    BRUSH_GRAB_SILHOUETTE = (1 << 8),
    BRUSH_USE_COLOR_AS_DISPLACEMENT = (1 << 9),
    BRUSH_JITTER_COLOR = (1 << 10),
    BRUSH_PROJECT_USE_BIDIRECTIONAL = (1 << 11),
}

impl Default for eBrushFlags2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_MULTIPLANE_SCRAPE_DYNAMIC: i32 = eBrushFlags2::BRUSH_MULTIPLANE_SCRAPE_DYNAMIC as i32;
pub const BRUSH_MULTIPLANE_SCRAPE_PLANES_PREVIEW: i32 = eBrushFlags2::BRUSH_MULTIPLANE_SCRAPE_PLANES_PREVIEW as i32;
pub const BRUSH_POSE_IK_ANCHORED: i32 = eBrushFlags2::BRUSH_POSE_IK_ANCHORED as i32;
pub const BRUSH_USE_CONNECTED_ONLY: i32 = eBrushFlags2::BRUSH_USE_CONNECTED_ONLY as i32;
pub const BRUSH_CLOTH_PIN_SIMULATION_BOUNDARY: i32 = eBrushFlags2::BRUSH_CLOTH_PIN_SIMULATION_BOUNDARY as i32;
pub const BRUSH_POSE_USE_LOCK_ROTATION: i32 = eBrushFlags2::BRUSH_POSE_USE_LOCK_ROTATION as i32;
pub const BRUSH_CLOTH_USE_COLLISION: i32 = eBrushFlags2::BRUSH_CLOTH_USE_COLLISION as i32;
pub const BRUSH_AREA_RADIUS_PRESSURE: i32 = eBrushFlags2::BRUSH_AREA_RADIUS_PRESSURE as i32;
pub const BRUSH_GRAB_SILHOUETTE: i32 = eBrushFlags2::BRUSH_GRAB_SILHOUETTE as i32;
pub const BRUSH_USE_COLOR_AS_DISPLACEMENT: i32 = eBrushFlags2::BRUSH_USE_COLOR_AS_DISPLACEMENT as i32;
pub const BRUSH_JITTER_COLOR: i32 = eBrushFlags2::BRUSH_JITTER_COLOR as i32;
pub const BRUSH_PROJECT_USE_BIDIRECTIONAL: i32 = eBrushFlags2::BRUSH_PROJECT_USE_BIDIRECTIONAL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushUnifiedPaintFlags {
    BRUSH_USE_UNIFIED_PAINT_SIZE = (1 << 0),
    BRUSH_USE_UNIFIED_PAINT_ALPHA = (1 << 1),
    BRUSH_USE_UNIFIED_PAINT_WEIGHT = (1 << 2),
    BRUSH_USE_UNIFIED_PAINT_COLOR = (1 << 3),
    BRUSH_USE_UNIFIED_PAINT_INPUT_SAMPLES = (1 << 4),
}

impl Default for eBrushUnifiedPaintFlags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_USE_UNIFIED_PAINT_SIZE: i32 = eBrushUnifiedPaintFlags::BRUSH_USE_UNIFIED_PAINT_SIZE as i32;
pub const BRUSH_USE_UNIFIED_PAINT_ALPHA: i32 = eBrushUnifiedPaintFlags::BRUSH_USE_UNIFIED_PAINT_ALPHA as i32;
pub const BRUSH_USE_UNIFIED_PAINT_WEIGHT: i32 = eBrushUnifiedPaintFlags::BRUSH_USE_UNIFIED_PAINT_WEIGHT as i32;
pub const BRUSH_USE_UNIFIED_PAINT_COLOR: i32 = eBrushUnifiedPaintFlags::BRUSH_USE_UNIFIED_PAINT_COLOR as i32;
pub const BRUSH_USE_UNIFIED_PAINT_INPUT_SAMPLES: i32 = eBrushUnifiedPaintFlags::BRUSH_USE_UNIFIED_PAINT_INPUT_SAMPLES as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrushMaskPressureFlags {
    BRUSH_MASK_PRESSURE_RAMP = (1 << 1),
    BRUSH_MASK_PRESSURE_CUTOFF = (1 << 2),
}

impl Default for BrushMaskPressureFlags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_MASK_PRESSURE_RAMP: i32 = BrushMaskPressureFlags::BRUSH_MASK_PRESSURE_RAMP as i32;
pub const BRUSH_MASK_PRESSURE_CUTOFF: i32 = BrushMaskPressureFlags::BRUSH_MASK_PRESSURE_CUTOFF as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eOverlayFlags {
    BRUSH_OVERLAY_CURSOR = (1),
    BRUSH_OVERLAY_PRIMARY = (1 << 1),
    BRUSH_OVERLAY_SECONDARY = (1 << 2),
    BRUSH_OVERLAY_CURSOR_OVERRIDE_ON_STROKE = (1 << 3),
    BRUSH_OVERLAY_PRIMARY_OVERRIDE_ON_STROKE = (1 << 4),
    BRUSH_OVERLAY_SECONDARY_OVERRIDE_ON_STROKE = (1 << 5),
}

impl Default for eOverlayFlags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_OVERLAY_CURSOR: i32 = eOverlayFlags::BRUSH_OVERLAY_CURSOR as i32;
pub const BRUSH_OVERLAY_PRIMARY: i32 = eOverlayFlags::BRUSH_OVERLAY_PRIMARY as i32;
pub const BRUSH_OVERLAY_SECONDARY: i32 = eOverlayFlags::BRUSH_OVERLAY_SECONDARY as i32;
pub const BRUSH_OVERLAY_CURSOR_OVERRIDE_ON_STROKE: i32 = eOverlayFlags::BRUSH_OVERLAY_CURSOR_OVERRIDE_ON_STROKE as i32;
pub const BRUSH_OVERLAY_PRIMARY_OVERRIDE_ON_STROKE: i32 = eOverlayFlags::BRUSH_OVERLAY_PRIMARY_OVERRIDE_ON_STROKE as i32;
pub const BRUSH_OVERLAY_SECONDARY_OVERRIDE_ON_STROKE: i32 = eOverlayFlags::BRUSH_OVERLAY_SECONDARY_OVERRIDE_ON_STROKE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushSculptType {
    SCULPT_BRUSH_TYPE_DRAW = 1,
    SCULPT_BRUSH_TYPE_SMOOTH = 2,
    SCULPT_BRUSH_TYPE_PINCH = 3,
    SCULPT_BRUSH_TYPE_INFLATE = 4,
    SCULPT_BRUSH_TYPE_GRAB = 5,
    SCULPT_BRUSH_TYPE_LAYER = 6,
    SCULPT_BRUSH_TYPE_FLATTEN = 7,
    SCULPT_BRUSH_TYPE_CLAY = 8,
    SCULPT_BRUSH_TYPE_FILL = 9,
    SCULPT_BRUSH_TYPE_SCRAPE = 10,
    SCULPT_BRUSH_TYPE_NUDGE = 11,
    SCULPT_BRUSH_TYPE_THUMB = 12,
    SCULPT_BRUSH_TYPE_SNAKE_HOOK = 13,
    SCULPT_BRUSH_TYPE_ROTATE = 14,
    SCULPT_BRUSH_TYPE_SIMPLIFY = 15,
    SCULPT_BRUSH_TYPE_CREASE = 16,
    SCULPT_BRUSH_TYPE_BLOB = 17,
    SCULPT_BRUSH_TYPE_CLAY_STRIPS = 18,
    SCULPT_BRUSH_TYPE_MASK = 19,
    SCULPT_BRUSH_TYPE_DRAW_SHARP = 20,
    SCULPT_BRUSH_TYPE_ELASTIC_DEFORM = 21,
    SCULPT_BRUSH_TYPE_POSE = 22,
    SCULPT_BRUSH_TYPE_MULTIPLANE_SCRAPE = 23,
    SCULPT_BRUSH_TYPE_SLIDE_RELAX = 24,
    SCULPT_BRUSH_TYPE_CLAY_THUMB = 25,
    SCULPT_BRUSH_TYPE_CLOTH = 26,
    SCULPT_BRUSH_TYPE_DRAW_FACE_SETS = 27,
    SCULPT_BRUSH_TYPE_PAINT = 28,
    SCULPT_BRUSH_TYPE_SMEAR = 29,
    SCULPT_BRUSH_TYPE_BOUNDARY = 30,
    SCULPT_BRUSH_TYPE_DISPLACEMENT_ERASER = 31,
    SCULPT_BRUSH_TYPE_DISPLACEMENT_SMEAR = 32,
    SCULPT_BRUSH_TYPE_PLANE = 33,
    SCULPT_BRUSH_TYPE_BLUR = 34,
    SCULPT_BRUSH_TYPE_SCENE_PROJECT = 35,
}

impl Default for eBrushSculptType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const SCULPT_BRUSH_TYPE_DRAW: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_DRAW as i32;
pub const SCULPT_BRUSH_TYPE_SMOOTH: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_SMOOTH as i32;
pub const SCULPT_BRUSH_TYPE_PINCH: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_PINCH as i32;
pub const SCULPT_BRUSH_TYPE_INFLATE: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_INFLATE as i32;
pub const SCULPT_BRUSH_TYPE_GRAB: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_GRAB as i32;
pub const SCULPT_BRUSH_TYPE_LAYER: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_LAYER as i32;
pub const SCULPT_BRUSH_TYPE_FLATTEN: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_FLATTEN as i32;
pub const SCULPT_BRUSH_TYPE_CLAY: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_CLAY as i32;
pub const SCULPT_BRUSH_TYPE_FILL: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_FILL as i32;
pub const SCULPT_BRUSH_TYPE_SCRAPE: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_SCRAPE as i32;
pub const SCULPT_BRUSH_TYPE_NUDGE: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_NUDGE as i32;
pub const SCULPT_BRUSH_TYPE_THUMB: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_THUMB as i32;
pub const SCULPT_BRUSH_TYPE_SNAKE_HOOK: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_SNAKE_HOOK as i32;
pub const SCULPT_BRUSH_TYPE_ROTATE: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_ROTATE as i32;
pub const SCULPT_BRUSH_TYPE_SIMPLIFY: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_SIMPLIFY as i32;
pub const SCULPT_BRUSH_TYPE_CREASE: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_CREASE as i32;
pub const SCULPT_BRUSH_TYPE_BLOB: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_BLOB as i32;
pub const SCULPT_BRUSH_TYPE_CLAY_STRIPS: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_CLAY_STRIPS as i32;
pub const SCULPT_BRUSH_TYPE_MASK: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_MASK as i32;
pub const SCULPT_BRUSH_TYPE_DRAW_SHARP: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_DRAW_SHARP as i32;
pub const SCULPT_BRUSH_TYPE_ELASTIC_DEFORM: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_ELASTIC_DEFORM as i32;
pub const SCULPT_BRUSH_TYPE_POSE: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_POSE as i32;
pub const SCULPT_BRUSH_TYPE_MULTIPLANE_SCRAPE: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_MULTIPLANE_SCRAPE as i32;
pub const SCULPT_BRUSH_TYPE_SLIDE_RELAX: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_SLIDE_RELAX as i32;
pub const SCULPT_BRUSH_TYPE_CLAY_THUMB: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_CLAY_THUMB as i32;
pub const SCULPT_BRUSH_TYPE_CLOTH: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_CLOTH as i32;
pub const SCULPT_BRUSH_TYPE_DRAW_FACE_SETS: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_DRAW_FACE_SETS as i32;
pub const SCULPT_BRUSH_TYPE_PAINT: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_PAINT as i32;
pub const SCULPT_BRUSH_TYPE_SMEAR: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_SMEAR as i32;
pub const SCULPT_BRUSH_TYPE_BOUNDARY: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_BOUNDARY as i32;
pub const SCULPT_BRUSH_TYPE_DISPLACEMENT_ERASER: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_DISPLACEMENT_ERASER as i32;
pub const SCULPT_BRUSH_TYPE_DISPLACEMENT_SMEAR: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_DISPLACEMENT_SMEAR as i32;
pub const SCULPT_BRUSH_TYPE_PLANE: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_PLANE as i32;
pub const SCULPT_BRUSH_TYPE_BLUR: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_BLUR as i32;
pub const SCULPT_BRUSH_TYPE_SCENE_PROJECT: i32 = eBrushSculptType::SCULPT_BRUSH_TYPE_SCENE_PROJECT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushCurvesSculptType {
    CURVES_SCULPT_BRUSH_TYPE_COMB = 0,
    CURVES_SCULPT_BRUSH_TYPE_DELETE = 1,
    CURVES_SCULPT_BRUSH_TYPE_SNAKE_HOOK = 2,
    CURVES_SCULPT_BRUSH_TYPE_ADD = 3,
    CURVES_SCULPT_BRUSH_TYPE_GROW_SHRINK = 4,
    CURVES_SCULPT_BRUSH_TYPE_SELECTION_PAINT = 5,
    CURVES_SCULPT_BRUSH_TYPE_PINCH = 6,
    CURVES_SCULPT_BRUSH_TYPE_SMOOTH = 7,
    CURVES_SCULPT_BRUSH_TYPE_PUFF = 8,
    CURVES_SCULPT_BRUSH_TYPE_DENSITY = 9,
    CURVES_SCULPT_BRUSH_TYPE_SLIDE = 10,
    CURVES_SCULPT_BRUSH_TYPE_CUT = 11,
}

impl Default for eBrushCurvesSculptType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CURVES_SCULPT_BRUSH_TYPE_COMB: i32 = eBrushCurvesSculptType::CURVES_SCULPT_BRUSH_TYPE_COMB as i32;
pub const CURVES_SCULPT_BRUSH_TYPE_DELETE: i32 = eBrushCurvesSculptType::CURVES_SCULPT_BRUSH_TYPE_DELETE as i32;
pub const CURVES_SCULPT_BRUSH_TYPE_SNAKE_HOOK: i32 = eBrushCurvesSculptType::CURVES_SCULPT_BRUSH_TYPE_SNAKE_HOOK as i32;
pub const CURVES_SCULPT_BRUSH_TYPE_ADD: i32 = eBrushCurvesSculptType::CURVES_SCULPT_BRUSH_TYPE_ADD as i32;
pub const CURVES_SCULPT_BRUSH_TYPE_GROW_SHRINK: i32 = eBrushCurvesSculptType::CURVES_SCULPT_BRUSH_TYPE_GROW_SHRINK as i32;
pub const CURVES_SCULPT_BRUSH_TYPE_SELECTION_PAINT: i32 = eBrushCurvesSculptType::CURVES_SCULPT_BRUSH_TYPE_SELECTION_PAINT as i32;
pub const CURVES_SCULPT_BRUSH_TYPE_PINCH: i32 = eBrushCurvesSculptType::CURVES_SCULPT_BRUSH_TYPE_PINCH as i32;
pub const CURVES_SCULPT_BRUSH_TYPE_SMOOTH: i32 = eBrushCurvesSculptType::CURVES_SCULPT_BRUSH_TYPE_SMOOTH as i32;
pub const CURVES_SCULPT_BRUSH_TYPE_PUFF: i32 = eBrushCurvesSculptType::CURVES_SCULPT_BRUSH_TYPE_PUFF as i32;
pub const CURVES_SCULPT_BRUSH_TYPE_DENSITY: i32 = eBrushCurvesSculptType::CURVES_SCULPT_BRUSH_TYPE_DENSITY as i32;
pub const CURVES_SCULPT_BRUSH_TYPE_SLIDE: i32 = eBrushCurvesSculptType::CURVES_SCULPT_BRUSH_TYPE_SLIDE as i32;
pub const CURVES_SCULPT_BRUSH_TYPE_CUT: i32 = eBrushCurvesSculptType::CURVES_SCULPT_BRUSH_TYPE_CUT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushImagePaintType {
    IMAGE_PAINT_BRUSH_TYPE_DRAW = 0,
    IMAGE_PAINT_BRUSH_TYPE_SOFTEN = 1,
    IMAGE_PAINT_BRUSH_TYPE_SMEAR = 2,
    IMAGE_PAINT_BRUSH_TYPE_CLONE = 3,
    IMAGE_PAINT_BRUSH_TYPE_FILL = 4,
    IMAGE_PAINT_BRUSH_TYPE_MASK = 5,
}

impl Default for eBrushImagePaintType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const IMAGE_PAINT_BRUSH_TYPE_DRAW: i32 = eBrushImagePaintType::IMAGE_PAINT_BRUSH_TYPE_DRAW as i32;
pub const IMAGE_PAINT_BRUSH_TYPE_SOFTEN: i32 = eBrushImagePaintType::IMAGE_PAINT_BRUSH_TYPE_SOFTEN as i32;
pub const IMAGE_PAINT_BRUSH_TYPE_SMEAR: i32 = eBrushImagePaintType::IMAGE_PAINT_BRUSH_TYPE_SMEAR as i32;
pub const IMAGE_PAINT_BRUSH_TYPE_CLONE: i32 = eBrushImagePaintType::IMAGE_PAINT_BRUSH_TYPE_CLONE as i32;
pub const IMAGE_PAINT_BRUSH_TYPE_FILL: i32 = eBrushImagePaintType::IMAGE_PAINT_BRUSH_TYPE_FILL as i32;
pub const IMAGE_PAINT_BRUSH_TYPE_MASK: i32 = eBrushImagePaintType::IMAGE_PAINT_BRUSH_TYPE_MASK as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushVertexPaintType {
    VPAINT_BRUSH_TYPE_DRAW = 0,
    VPAINT_BRUSH_TYPE_BLUR = 1,
    VPAINT_BRUSH_TYPE_AVERAGE = 2,
    VPAINT_BRUSH_TYPE_SMEAR = 3,
}

impl Default for eBrushVertexPaintType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const VPAINT_BRUSH_TYPE_DRAW: i32 = eBrushVertexPaintType::VPAINT_BRUSH_TYPE_DRAW as i32;
pub const VPAINT_BRUSH_TYPE_BLUR: i32 = eBrushVertexPaintType::VPAINT_BRUSH_TYPE_BLUR as i32;
pub const VPAINT_BRUSH_TYPE_AVERAGE: i32 = eBrushVertexPaintType::VPAINT_BRUSH_TYPE_AVERAGE as i32;
pub const VPAINT_BRUSH_TYPE_SMEAR: i32 = eBrushVertexPaintType::VPAINT_BRUSH_TYPE_SMEAR as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushWeightPaintType {
    WPAINT_BRUSH_TYPE_DRAW = 0,
    WPAINT_BRUSH_TYPE_BLUR = 1,
    WPAINT_BRUSH_TYPE_AVERAGE = 2,
    WPAINT_BRUSH_TYPE_SMEAR = 3,
}

impl Default for eBrushWeightPaintType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const WPAINT_BRUSH_TYPE_DRAW: i32 = eBrushWeightPaintType::WPAINT_BRUSH_TYPE_DRAW as i32;
pub const WPAINT_BRUSH_TYPE_BLUR: i32 = eBrushWeightPaintType::WPAINT_BRUSH_TYPE_BLUR as i32;
pub const WPAINT_BRUSH_TYPE_AVERAGE: i32 = eBrushWeightPaintType::WPAINT_BRUSH_TYPE_AVERAGE as i32;
pub const WPAINT_BRUSH_TYPE_SMEAR: i32 = eBrushWeightPaintType::WPAINT_BRUSH_TYPE_SMEAR as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushGPaintType {
    GPAINT_BRUSH_TYPE_DRAW = 0,
    GPAINT_BRUSH_TYPE_FILL = 1,
    GPAINT_BRUSH_TYPE_ERASE = 2,
    GPAINT_BRUSH_TYPE_TINT = 3,
}

impl Default for eBrushGPaintType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GPAINT_BRUSH_TYPE_DRAW: i32 = eBrushGPaintType::GPAINT_BRUSH_TYPE_DRAW as i32;
pub const GPAINT_BRUSH_TYPE_FILL: i32 = eBrushGPaintType::GPAINT_BRUSH_TYPE_FILL as i32;
pub const GPAINT_BRUSH_TYPE_ERASE: i32 = eBrushGPaintType::GPAINT_BRUSH_TYPE_ERASE as i32;
pub const GPAINT_BRUSH_TYPE_TINT: i32 = eBrushGPaintType::GPAINT_BRUSH_TYPE_TINT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushGPVertexType {
    GPVERTEX_BRUSH_TYPE_DRAW = 0,
    GPVERTEX_BRUSH_TYPE_BLUR = 1,
    GPVERTEX_BRUSH_TYPE_AVERAGE = 2,
    GPVERTEX_BRUSH_TYPE_TINT = 3,
    GPVERTEX_BRUSH_TYPE_SMEAR = 4,
    GPVERTEX_BRUSH_TYPE_REPLACE = 5,
}

impl Default for eBrushGPVertexType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GPVERTEX_BRUSH_TYPE_DRAW: i32 = eBrushGPVertexType::GPVERTEX_BRUSH_TYPE_DRAW as i32;
pub const GPVERTEX_BRUSH_TYPE_BLUR: i32 = eBrushGPVertexType::GPVERTEX_BRUSH_TYPE_BLUR as i32;
pub const GPVERTEX_BRUSH_TYPE_AVERAGE: i32 = eBrushGPVertexType::GPVERTEX_BRUSH_TYPE_AVERAGE as i32;
pub const GPVERTEX_BRUSH_TYPE_TINT: i32 = eBrushGPVertexType::GPVERTEX_BRUSH_TYPE_TINT as i32;
pub const GPVERTEX_BRUSH_TYPE_SMEAR: i32 = eBrushGPVertexType::GPVERTEX_BRUSH_TYPE_SMEAR as i32;
pub const GPVERTEX_BRUSH_TYPE_REPLACE: i32 = eBrushGPVertexType::GPVERTEX_BRUSH_TYPE_REPLACE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushGPSculptType {
    GPSCULPT_BRUSH_TYPE_SMOOTH = 0,
    GPSCULPT_BRUSH_TYPE_THICKNESS = 1,
    GPSCULPT_BRUSH_TYPE_STRENGTH = 2,
    GPSCULPT_BRUSH_TYPE_GRAB = 3,
    GPSCULPT_BRUSH_TYPE_PUSH = 4,
    GPSCULPT_BRUSH_TYPE_TWIST = 5,
    GPSCULPT_BRUSH_TYPE_PINCH = 6,
    GPSCULPT_BRUSH_TYPE_RANDOMIZE = 7,
    GPSCULPT_BRUSH_TYPE_CLONE = 8,
}

impl Default for eBrushGPSculptType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GPSCULPT_BRUSH_TYPE_SMOOTH: i32 = eBrushGPSculptType::GPSCULPT_BRUSH_TYPE_SMOOTH as i32;
pub const GPSCULPT_BRUSH_TYPE_THICKNESS: i32 = eBrushGPSculptType::GPSCULPT_BRUSH_TYPE_THICKNESS as i32;
pub const GPSCULPT_BRUSH_TYPE_STRENGTH: i32 = eBrushGPSculptType::GPSCULPT_BRUSH_TYPE_STRENGTH as i32;
pub const GPSCULPT_BRUSH_TYPE_GRAB: i32 = eBrushGPSculptType::GPSCULPT_BRUSH_TYPE_GRAB as i32;
pub const GPSCULPT_BRUSH_TYPE_PUSH: i32 = eBrushGPSculptType::GPSCULPT_BRUSH_TYPE_PUSH as i32;
pub const GPSCULPT_BRUSH_TYPE_TWIST: i32 = eBrushGPSculptType::GPSCULPT_BRUSH_TYPE_TWIST as i32;
pub const GPSCULPT_BRUSH_TYPE_PINCH: i32 = eBrushGPSculptType::GPSCULPT_BRUSH_TYPE_PINCH as i32;
pub const GPSCULPT_BRUSH_TYPE_RANDOMIZE: i32 = eBrushGPSculptType::GPSCULPT_BRUSH_TYPE_RANDOMIZE as i32;
pub const GPSCULPT_BRUSH_TYPE_CLONE: i32 = eBrushGPSculptType::GPSCULPT_BRUSH_TYPE_CLONE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushGPWeightType {
    GPWEIGHT_BRUSH_TYPE_DRAW = 0,
    GPWEIGHT_BRUSH_TYPE_BLUR = 1,
    GPWEIGHT_BRUSH_TYPE_AVERAGE = 2,
    GPWEIGHT_BRUSH_TYPE_SMEAR = 3,
}

impl Default for eBrushGPWeightType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GPWEIGHT_BRUSH_TYPE_DRAW: i32 = eBrushGPWeightType::GPWEIGHT_BRUSH_TYPE_DRAW as i32;
pub const GPWEIGHT_BRUSH_TYPE_BLUR: i32 = eBrushGPWeightType::GPWEIGHT_BRUSH_TYPE_BLUR as i32;
pub const GPWEIGHT_BRUSH_TYPE_AVERAGE: i32 = eBrushGPWeightType::GPWEIGHT_BRUSH_TYPE_AVERAGE as i32;
pub const GPWEIGHT_BRUSH_TYPE_SMEAR: i32 = eBrushGPWeightType::GPWEIGHT_BRUSH_TYPE_SMEAR as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushSculpt_DispDir {
    SCULPT_DISP_DIR_AREA = 0,
    SCULPT_DISP_DIR_VIEW = 1,
    SCULPT_DISP_DIR_X = 2,
    SCULPT_DISP_DIR_Y = 3,
    SCULPT_DISP_DIR_Z = 4,
}

impl Default for eBrushSculpt_DispDir {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const SCULPT_DISP_DIR_AREA: i32 = eBrushSculpt_DispDir::SCULPT_DISP_DIR_AREA as i32;
pub const SCULPT_DISP_DIR_VIEW: i32 = eBrushSculpt_DispDir::SCULPT_DISP_DIR_VIEW as i32;
pub const SCULPT_DISP_DIR_X: i32 = eBrushSculpt_DispDir::SCULPT_DISP_DIR_X as i32;
pub const SCULPT_DISP_DIR_Y: i32 = eBrushSculpt_DispDir::SCULPT_DISP_DIR_Y as i32;
pub const SCULPT_DISP_DIR_Z: i32 = eBrushSculpt_DispDir::SCULPT_DISP_DIR_Z as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrushMaskTool {
    BRUSH_MASK_DRAW = 0,
    BRUSH_MASK_SMOOTH = 1,
}

impl Default for BrushMaskTool {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_MASK_DRAW: i32 = BrushMaskTool::BRUSH_MASK_DRAW as i32;
pub const BRUSH_MASK_SMOOTH: i32 = BrushMaskTool::BRUSH_MASK_SMOOTH as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBlurKernelType {
    KERNEL_GAUSSIAN = 0,
    KERNEL_BOX = 1,
}

impl Default for eBlurKernelType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const KERNEL_GAUSSIAN: i32 = eBlurKernelType::KERNEL_GAUSSIAN as i32;
pub const KERNEL_BOX: i32 = eBlurKernelType::KERNEL_BOX as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushFalloffShape {
    PAINT_FALLOFF_SHAPE_SPHERE = 0,
    PAINT_FALLOFF_SHAPE_TUBE = 1,
}

impl Default for eBrushFalloffShape {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const PAINT_FALLOFF_SHAPE_SPHERE: i32 = eBrushFalloffShape::PAINT_FALLOFF_SHAPE_SPHERE as i32;
pub const PAINT_FALLOFF_SHAPE_TUBE: i32 = eBrushFalloffShape::PAINT_FALLOFF_SHAPE_TUBE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushCurvesSculptFlag {
    BRUSH_CURVES_SCULPT_FLAG_SCALE_UNIFORM = (1 << 0),
    BRUSH_CURVES_SCULPT_FLAG_GROW_SHRINK_INVERT = (1 << 1),
    BRUSH_CURVES_SCULPT_FLAG_INTERPOLATE_LENGTH = (1 << 2),
    BRUSH_CURVES_SCULPT_FLAG_INTERPOLATE_SHAPE = (1 << 3),
    BRUSH_CURVES_SCULPT_FLAG_INTERPOLATE_POINT_COUNT = (1 << 4),
    BRUSH_CURVES_SCULPT_FLAG_INTERPOLATE_RADIUS = (1 << 5),
}

impl Default for eBrushCurvesSculptFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_CURVES_SCULPT_FLAG_SCALE_UNIFORM: i32 = eBrushCurvesSculptFlag::BRUSH_CURVES_SCULPT_FLAG_SCALE_UNIFORM as i32;
pub const BRUSH_CURVES_SCULPT_FLAG_GROW_SHRINK_INVERT: i32 = eBrushCurvesSculptFlag::BRUSH_CURVES_SCULPT_FLAG_GROW_SHRINK_INVERT as i32;
pub const BRUSH_CURVES_SCULPT_FLAG_INTERPOLATE_LENGTH: i32 = eBrushCurvesSculptFlag::BRUSH_CURVES_SCULPT_FLAG_INTERPOLATE_LENGTH as i32;
pub const BRUSH_CURVES_SCULPT_FLAG_INTERPOLATE_SHAPE: i32 = eBrushCurvesSculptFlag::BRUSH_CURVES_SCULPT_FLAG_INTERPOLATE_SHAPE as i32;
pub const BRUSH_CURVES_SCULPT_FLAG_INTERPOLATE_POINT_COUNT: i32 = eBrushCurvesSculptFlag::BRUSH_CURVES_SCULPT_FLAG_INTERPOLATE_POINT_COUNT as i32;
pub const BRUSH_CURVES_SCULPT_FLAG_INTERPOLATE_RADIUS: i32 = eBrushCurvesSculptFlag::BRUSH_CURVES_SCULPT_FLAG_INTERPOLATE_RADIUS as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushCurvesSculptDensityMode {
    BRUSH_CURVES_SCULPT_DENSITY_MODE_AUTO = 0,
    BRUSH_CURVES_SCULPT_DENSITY_MODE_ADD = 1,
    BRUSH_CURVES_SCULPT_DENSITY_MODE_REMOVE = 2,
}

impl Default for eBrushCurvesSculptDensityMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_CURVES_SCULPT_DENSITY_MODE_AUTO: i32 = eBrushCurvesSculptDensityMode::BRUSH_CURVES_SCULPT_DENSITY_MODE_AUTO as i32;
pub const BRUSH_CURVES_SCULPT_DENSITY_MODE_ADD: i32 = eBrushCurvesSculptDensityMode::BRUSH_CURVES_SCULPT_DENSITY_MODE_ADD as i32;
pub const BRUSH_CURVES_SCULPT_DENSITY_MODE_REMOVE: i32 = eBrushCurvesSculptDensityMode::BRUSH_CURVES_SCULPT_DENSITY_MODE_REMOVE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBrushColorJitterSettings_Flag {
    BRUSH_COLOR_JITTER_USE_HUE_AT_STROKE = (1 << 0),
    BRUSH_COLOR_JITTER_USE_SAT_AT_STROKE = (1 << 1),
    BRUSH_COLOR_JITTER_USE_VAL_AT_STROKE = (1 << 2),
    BRUSH_COLOR_JITTER_USE_HUE_RAND_PRESS = (1 << 3),
    BRUSH_COLOR_JITTER_USE_SAT_RAND_PRESS = (1 << 4),
    BRUSH_COLOR_JITTER_USE_VAL_RAND_PRESS = (1 << 5),
}

impl Default for eBrushColorJitterSettings_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BRUSH_COLOR_JITTER_USE_HUE_AT_STROKE: i32 = eBrushColorJitterSettings_Flag::BRUSH_COLOR_JITTER_USE_HUE_AT_STROKE as i32;
pub const BRUSH_COLOR_JITTER_USE_SAT_AT_STROKE: i32 = eBrushColorJitterSettings_Flag::BRUSH_COLOR_JITTER_USE_SAT_AT_STROKE as i32;
pub const BRUSH_COLOR_JITTER_USE_VAL_AT_STROKE: i32 = eBrushColorJitterSettings_Flag::BRUSH_COLOR_JITTER_USE_VAL_AT_STROKE as i32;
pub const BRUSH_COLOR_JITTER_USE_HUE_RAND_PRESS: i32 = eBrushColorJitterSettings_Flag::BRUSH_COLOR_JITTER_USE_HUE_RAND_PRESS as i32;
pub const BRUSH_COLOR_JITTER_USE_SAT_RAND_PRESS: i32 = eBrushColorJitterSettings_Flag::BRUSH_COLOR_JITTER_USE_SAT_RAND_PRESS as i32;
pub const BRUSH_COLOR_JITTER_USE_VAL_RAND_PRESS: i32 = eBrushColorJitterSettings_Flag::BRUSH_COLOR_JITTER_USE_VAL_RAND_PRESS as i32;

