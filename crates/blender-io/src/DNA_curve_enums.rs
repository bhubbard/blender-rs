//! Auto-transpiled C/C++ header module: DNA_curve_enums

use crate::*;

pub const MAXTEXTBOX: i32 = 256;
pub const KEY_CU_EASE: i32 = 3;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCurveTexspaceFlag {
    CU_TEXSPACE_FLAG_AUTO = 1 << 0,
    CU_TEXSPACE_FLAG_AUTO_EVALUATED = 1 << 1,
}

impl Default for eCurveTexspaceFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CU_TEXSPACE_FLAG_AUTO: i32 = eCurveTexspaceFlag::CU_TEXSPACE_FLAG_AUTO as i32;
pub const CU_TEXSPACE_FLAG_AUTO_EVALUATED: i32 = eCurveTexspaceFlag::CU_TEXSPACE_FLAG_AUTO_EVALUATED as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCurveFlag {
    CU_3D = 1 << 0,
    CU_FRONT = 1 << 1,
    CU_BACK = 1 << 2,
    CU_PATH = 1 << 3,
    CU_FOLLOW = 1 << 4,
    CU_PATH_CLAMP = 1 << 5,
    CU_DEFORM_BOUNDS_OFF = 1 << 6,
    CU_STRETCH = 1 << 7,
    CU_FAST = 1 << 9,
    CU_DS_EXPAND = 1 << 11,
    CU_PATH_RADIUS = 1 << 12,
    CU_FILL_CAPS = 1 << 14,
    CU_MAP_TAPER = 1 << 15,
}

impl Default for eCurveFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CU_3D: i32 = eCurveFlag::CU_3D as i32;
pub const CU_FRONT: i32 = eCurveFlag::CU_FRONT as i32;
pub const CU_BACK: i32 = eCurveFlag::CU_BACK as i32;
pub const CU_PATH: i32 = eCurveFlag::CU_PATH as i32;
pub const CU_FOLLOW: i32 = eCurveFlag::CU_FOLLOW as i32;
pub const CU_PATH_CLAMP: i32 = eCurveFlag::CU_PATH_CLAMP as i32;
pub const CU_DEFORM_BOUNDS_OFF: i32 = eCurveFlag::CU_DEFORM_BOUNDS_OFF as i32;
pub const CU_STRETCH: i32 = eCurveFlag::CU_STRETCH as i32;
pub const CU_FAST: i32 = eCurveFlag::CU_FAST as i32;
pub const CU_DS_EXPAND: i32 = eCurveFlag::CU_DS_EXPAND as i32;
pub const CU_PATH_RADIUS: i32 = eCurveFlag::CU_PATH_RADIUS as i32;
pub const CU_FILL_CAPS: i32 = eCurveFlag::CU_FILL_CAPS as i32;
pub const CU_MAP_TAPER: i32 = eCurveFlag::CU_MAP_TAPER as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCurveTwistMode {
    CU_TWIST_Z_UP = 0,
    CU_TWIST_MINIMUM = 3,
    CU_TWIST_TANGENT = 4,
}

impl Default for eCurveTwistMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CU_TWIST_Z_UP: i32 = eCurveTwistMode::CU_TWIST_Z_UP as i32;
pub const CU_TWIST_MINIMUM: i32 = eCurveTwistMode::CU_TWIST_MINIMUM as i32;
pub const CU_TWIST_TANGENT: i32 = eCurveTwistMode::CU_TWIST_TANGENT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCurveBevfacMapping {
    CU_BEVFAC_MAP_RESOLU = 0,
    CU_BEVFAC_MAP_SEGMENT = 1,
    CU_BEVFAC_MAP_SPLINE = 2,
}

impl Default for eCurveBevfacMapping {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CU_BEVFAC_MAP_RESOLU: i32 = eCurveBevfacMapping::CU_BEVFAC_MAP_RESOLU as i32;
pub const CU_BEVFAC_MAP_SEGMENT: i32 = eCurveBevfacMapping::CU_BEVFAC_MAP_SEGMENT as i32;
pub const CU_BEVFAC_MAP_SPLINE: i32 = eCurveBevfacMapping::CU_BEVFAC_MAP_SPLINE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCurveSpaceMode {
    CU_ALIGN_X_LEFT = 0,
    CU_ALIGN_X_MIDDLE = 1,
    CU_ALIGN_X_RIGHT = 2,
    CU_ALIGN_X_JUSTIFY = 3,
    CU_ALIGN_X_FLUSH = 4,
}

impl Default for eCurveSpaceMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CU_ALIGN_X_LEFT: i32 = eCurveSpaceMode::CU_ALIGN_X_LEFT as i32;
pub const CU_ALIGN_X_MIDDLE: i32 = eCurveSpaceMode::CU_ALIGN_X_MIDDLE as i32;
pub const CU_ALIGN_X_RIGHT: i32 = eCurveSpaceMode::CU_ALIGN_X_RIGHT as i32;
pub const CU_ALIGN_X_JUSTIFY: i32 = eCurveSpaceMode::CU_ALIGN_X_JUSTIFY as i32;
pub const CU_ALIGN_X_FLUSH: i32 = eCurveSpaceMode::CU_ALIGN_X_FLUSH as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCurveAlignY {
    CU_ALIGN_Y_TOP_BASELINE = 0,
    CU_ALIGN_Y_TOP = 1,
    CU_ALIGN_Y_CENTER = 2,
    CU_ALIGN_Y_BOTTOM_BASELINE = 3,
    CU_ALIGN_Y_BOTTOM = 4,
}

impl Default for eCurveAlignY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CU_ALIGN_Y_TOP_BASELINE: i32 = eCurveAlignY::CU_ALIGN_Y_TOP_BASELINE as i32;
pub const CU_ALIGN_Y_TOP: i32 = eCurveAlignY::CU_ALIGN_Y_TOP as i32;
pub const CU_ALIGN_Y_CENTER: i32 = eCurveAlignY::CU_ALIGN_Y_CENTER as i32;
pub const CU_ALIGN_Y_BOTTOM_BASELINE: i32 = eCurveAlignY::CU_ALIGN_Y_BOTTOM_BASELINE as i32;
pub const CU_ALIGN_Y_BOTTOM: i32 = eCurveAlignY::CU_ALIGN_Y_BOTTOM as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCurveBevelMode {
    CU_BEV_MODE_ROUND = 0,
    CU_BEV_MODE_OBJECT = 1,
    CU_BEV_MODE_CURVE_PROFILE = 2,
}

impl Default for eCurveBevelMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CU_BEV_MODE_ROUND: i32 = eCurveBevelMode::CU_BEV_MODE_ROUND as i32;
pub const CU_BEV_MODE_OBJECT: i32 = eCurveBevelMode::CU_BEV_MODE_OBJECT as i32;
pub const CU_BEV_MODE_CURVE_PROFILE: i32 = eCurveBevelMode::CU_BEV_MODE_CURVE_PROFILE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCurveTaperRadiusMode {
    CU_TAPER_RADIUS_OVERRIDE = 0,
    CU_TAPER_RADIUS_MULTIPLY = 1,
    CU_TAPER_RADIUS_ADD = 2,
}

impl Default for eCurveTaperRadiusMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CU_TAPER_RADIUS_OVERRIDE: i32 = eCurveTaperRadiusMode::CU_TAPER_RADIUS_OVERRIDE as i32;
pub const CU_TAPER_RADIUS_MULTIPLY: i32 = eCurveTaperRadiusMode::CU_TAPER_RADIUS_MULTIPLY as i32;
pub const CU_TAPER_RADIUS_ADD: i32 = eCurveTaperRadiusMode::CU_TAPER_RADIUS_ADD as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurveFillSolverType {
    CU_FILL_SOLVER_SWEEP_LINE = 0,
    CU_FILL_SOLVER_CDT = 1,
}

impl Default for CurveFillSolverType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CU_FILL_SOLVER_SWEEP_LINE: i32 = CurveFillSolverType::CU_FILL_SOLVER_SWEEP_LINE as i32;
pub const CU_FILL_SOLVER_CDT: i32 = CurveFillSolverType::CU_FILL_SOLVER_CDT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurveFillRuleType {
    CU_FILL_RULE_EVEN_ODD = 0,
    CU_FILL_RULE_NONZERO = 1,
}

impl Default for CurveFillRuleType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CU_FILL_RULE_EVEN_ODD: i32 = CurveFillRuleType::CU_FILL_RULE_EVEN_ODD as i32;
pub const CU_FILL_RULE_NONZERO: i32 = CurveFillRuleType::CU_FILL_RULE_NONZERO as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCurveOverflow {
    CU_OVERFLOW_NONE = 0,
    CU_OVERFLOW_SCALE = 1,
    CU_OVERFLOW_TRUNCATE = 2,
}

impl Default for eCurveOverflow {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CU_OVERFLOW_NONE: i32 = eCurveOverflow::CU_OVERFLOW_NONE as i32;
pub const CU_OVERFLOW_SCALE: i32 = eCurveOverflow::CU_OVERFLOW_SCALE as i32;
pub const CU_OVERFLOW_TRUNCATE: i32 = eCurveOverflow::CU_OVERFLOW_TRUNCATE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eNurbFlag {
    CU_SMOOTH = 1 << 0,
}

impl Default for eNurbFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CU_SMOOTH: i32 = eNurbFlag::CU_SMOOTH as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eNurbType {
    CU_POLY = 0,
    CU_BEZIER = 1,
    CU_NURBS = 4,
    CU_TYPE = (CU_POLY | CU_BEZIER | CU_NURBS),
    CU_PRIMITIVE = 0xF00,
    CU_PRIM_CURVE = 0x100,
    CU_PRIM_CIRCLE = 0x200,
    CU_PRIM_PATCH = 0x300,
    CU_PRIM_TUBE = 0x400,
    CU_PRIM_SPHERE = 0x500,
    CU_PRIM_DONUT = 0x600,
    CU_PRIM_PATH = 0x700,
}

impl Default for eNurbType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CU_POLY: i32 = eNurbType::CU_POLY as i32;
pub const CU_BEZIER: i32 = eNurbType::CU_BEZIER as i32;
pub const CU_NURBS: i32 = eNurbType::CU_NURBS as i32;
pub const CU_TYPE: i32 = eNurbType::CU_TYPE as i32;
pub const CU_PRIMITIVE: i32 = eNurbType::CU_PRIMITIVE as i32;
pub const CU_PRIM_CURVE: i32 = eNurbType::CU_PRIM_CURVE as i32;
pub const CU_PRIM_CIRCLE: i32 = eNurbType::CU_PRIM_CIRCLE as i32;
pub const CU_PRIM_PATCH: i32 = eNurbType::CU_PRIM_PATCH as i32;
pub const CU_PRIM_TUBE: i32 = eNurbType::CU_PRIM_TUBE as i32;
pub const CU_PRIM_SPHERE: i32 = eNurbType::CU_PRIM_SPHERE as i32;
pub const CU_PRIM_DONUT: i32 = eNurbType::CU_PRIM_DONUT as i32;
pub const CU_PRIM_PATH: i32 = eNurbType::CU_PRIM_PATH as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eNurbKnotFlag {
    CU_NURB_CYCLIC = 1 << 0,
    CU_NURB_ENDPOINT = 1 << 1,
    CU_NURB_BEZIER = 1 << 2,
    CU_NURB_CUSTOM = 1 << 3,
}

impl Default for eNurbKnotFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CU_NURB_CYCLIC: i32 = eNurbKnotFlag::CU_NURB_CYCLIC as i32;
pub const CU_NURB_ENDPOINT: i32 = eNurbKnotFlag::CU_NURB_ENDPOINT as i32;
pub const CU_NURB_BEZIER: i32 = eNurbKnotFlag::CU_NURB_BEZIER as i32;
pub const CU_NURB_CUSTOM: i32 = eNurbKnotFlag::CU_NURB_CUSTOM as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBezTriple_Flag {
    BEZT_FLAG_SELECT = (1 << 0),
    BEZT_FLAG_TEMP_TAG = (1 << 1),
    BEZT_FLAG_IGNORE_TAG = (1 << 2),
}

impl Default for eBezTriple_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BEZT_FLAG_SELECT: i32 = eBezTriple_Flag::BEZT_FLAG_SELECT as i32;
pub const BEZT_FLAG_TEMP_TAG: i32 = eBezTriple_Flag::BEZT_FLAG_TEMP_TAG as i32;
pub const BEZT_FLAG_IGNORE_TAG: i32 = eBezTriple_Flag::BEZT_FLAG_IGNORE_TAG as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBezTriple_Handle {
    HD_FREE = 0,
    HD_AUTO = 1,
    HD_VECT = 2,
    HD_ALIGN = 3,
    HD_AUTO_ANIM = 4,
    HD_ALIGN_DOUBLESIDE = 5,
}

impl Default for eBezTriple_Handle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const HD_FREE: i32 = eBezTriple_Handle::HD_FREE as i32;
pub const HD_AUTO: i32 = eBezTriple_Handle::HD_AUTO as i32;
pub const HD_VECT: i32 = eBezTriple_Handle::HD_VECT as i32;
pub const HD_ALIGN: i32 = eBezTriple_Handle::HD_ALIGN as i32;
pub const HD_AUTO_ANIM: i32 = eBezTriple_Handle::HD_AUTO_ANIM as i32;
pub const HD_ALIGN_DOUBLESIDE: i32 = eBezTriple_Handle::HD_ALIGN_DOUBLESIDE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBezTriple_Auto_Type {
    HD_AUTOTYPE_NORMAL = 0,
    HD_AUTOTYPE_LOCKED_FINAL = 1,
}

impl Default for eBezTriple_Auto_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const HD_AUTOTYPE_NORMAL: i32 = eBezTriple_Auto_Type::HD_AUTOTYPE_NORMAL as i32;
pub const HD_AUTOTYPE_LOCKED_FINAL: i32 = eBezTriple_Auto_Type::HD_AUTOTYPE_LOCKED_FINAL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBezTriple_Interpolation {
    BEZT_IPO_CONST = 0,
    BEZT_IPO_LIN = 1,
    BEZT_IPO_BEZ = 2,
    BEZT_IPO_BACK = 3,
    BEZT_IPO_BOUNCE = 4,
    BEZT_IPO_CIRC = 5,
    BEZT_IPO_CUBIC = 6,
    BEZT_IPO_ELASTIC = 7,
    BEZT_IPO_EXPO = 8,
    BEZT_IPO_QUAD = 9,
    BEZT_IPO_QUART = 10,
    BEZT_IPO_QUINT = 11,
    BEZT_IPO_SINE = 12,
}

impl Default for eBezTriple_Interpolation {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BEZT_IPO_CONST: i32 = eBezTriple_Interpolation::BEZT_IPO_CONST as i32;
pub const BEZT_IPO_LIN: i32 = eBezTriple_Interpolation::BEZT_IPO_LIN as i32;
pub const BEZT_IPO_BEZ: i32 = eBezTriple_Interpolation::BEZT_IPO_BEZ as i32;
pub const BEZT_IPO_BACK: i32 = eBezTriple_Interpolation::BEZT_IPO_BACK as i32;
pub const BEZT_IPO_BOUNCE: i32 = eBezTriple_Interpolation::BEZT_IPO_BOUNCE as i32;
pub const BEZT_IPO_CIRC: i32 = eBezTriple_Interpolation::BEZT_IPO_CIRC as i32;
pub const BEZT_IPO_CUBIC: i32 = eBezTriple_Interpolation::BEZT_IPO_CUBIC as i32;
pub const BEZT_IPO_ELASTIC: i32 = eBezTriple_Interpolation::BEZT_IPO_ELASTIC as i32;
pub const BEZT_IPO_EXPO: i32 = eBezTriple_Interpolation::BEZT_IPO_EXPO as i32;
pub const BEZT_IPO_QUAD: i32 = eBezTriple_Interpolation::BEZT_IPO_QUAD as i32;
pub const BEZT_IPO_QUART: i32 = eBezTriple_Interpolation::BEZT_IPO_QUART as i32;
pub const BEZT_IPO_QUINT: i32 = eBezTriple_Interpolation::BEZT_IPO_QUINT as i32;
pub const BEZT_IPO_SINE: i32 = eBezTriple_Interpolation::BEZT_IPO_SINE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBezTriple_Easing {
    BEZT_IPO_EASE_AUTO = 0,
    BEZT_IPO_EASE_IN = 1,
    BEZT_IPO_EASE_OUT = 2,
    BEZT_IPO_EASE_IN_OUT = 3,
}

impl Default for eBezTriple_Easing {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BEZT_IPO_EASE_AUTO: i32 = eBezTriple_Easing::BEZT_IPO_EASE_AUTO as i32;
pub const BEZT_IPO_EASE_IN: i32 = eBezTriple_Easing::BEZT_IPO_EASE_IN as i32;
pub const BEZT_IPO_EASE_OUT: i32 = eBezTriple_Easing::BEZT_IPO_EASE_OUT as i32;
pub const BEZT_IPO_EASE_IN_OUT: i32 = eBezTriple_Easing::BEZT_IPO_EASE_IN_OUT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBezTriple_KeyframeType {
    BEZT_KEYTYPE_KEYFRAME = 0,
    BEZT_KEYTYPE_EXTREME = 1,
    BEZT_KEYTYPE_BREAKDOWN = 2,
    BEZT_KEYTYPE_JITTER = 3,
    BEZT_KEYTYPE_MOVEHOLD = 4,
    BEZT_KEYTYPE_GENERATED = 5,
}

impl Default for eBezTriple_KeyframeType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const BEZT_KEYTYPE_KEYFRAME: i32 = eBezTriple_KeyframeType::BEZT_KEYTYPE_KEYFRAME as i32;
pub const BEZT_KEYTYPE_EXTREME: i32 = eBezTriple_KeyframeType::BEZT_KEYTYPE_EXTREME as i32;
pub const BEZT_KEYTYPE_BREAKDOWN: i32 = eBezTriple_KeyframeType::BEZT_KEYTYPE_BREAKDOWN as i32;
pub const BEZT_KEYTYPE_JITTER: i32 = eBezTriple_KeyframeType::BEZT_KEYTYPE_JITTER as i32;
pub const BEZT_KEYTYPE_MOVEHOLD: i32 = eBezTriple_KeyframeType::BEZT_KEYTYPE_MOVEHOLD as i32;
pub const BEZT_KEYTYPE_GENERATED: i32 = eBezTriple_KeyframeType::BEZT_KEYTYPE_GENERATED as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCharInfoFlag {
    CU_CHINFO_BOLD = 1 << 0,
    CU_CHINFO_ITALIC = 1 << 1,
    CU_CHINFO_UNDERLINE = 1 << 2,
    CU_CHINFO_UNUSED_3 = 1 << 3,
    CU_CHINFO_SMALLCAPS = 1 << 4,
    CU_CHINFO_UNUSED_5 = 1 << 5,
    CU_CHINFO_UNUSED_6 = 1 << 6,
}

impl Default for eCharInfoFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CU_CHINFO_BOLD: i32 = eCharInfoFlag::CU_CHINFO_BOLD as i32;
pub const CU_CHINFO_ITALIC: i32 = eCharInfoFlag::CU_CHINFO_ITALIC as i32;
pub const CU_CHINFO_UNDERLINE: i32 = eCharInfoFlag::CU_CHINFO_UNDERLINE as i32;
pub const CU_CHINFO_UNUSED_3: i32 = eCharInfoFlag::CU_CHINFO_UNUSED_3 as i32;
pub const CU_CHINFO_SMALLCAPS: i32 = eCharInfoFlag::CU_CHINFO_SMALLCAPS as i32;
pub const CU_CHINFO_UNUSED_5: i32 = eCharInfoFlag::CU_CHINFO_UNUSED_5 as i32;
pub const CU_CHINFO_UNUSED_6: i32 = eCharInfoFlag::CU_CHINFO_UNUSED_6 as i32;
