//! Auto-transpiled C/C++ header module: DNA_anim_enums

use crate::*;

pub const MAX_DRIVER_TARGETS: i32 = 8;
pub const SELECT: i32 = 1;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFModifier_Types {
    FMODIFIER_TYPE_NULL = 0,
    FMODIFIER_TYPE_GENERATOR = 1,
    FMODIFIER_TYPE_FN_GENERATOR = 2,
    FMODIFIER_TYPE_ENVELOPE = 3,
    FMODIFIER_TYPE_CYCLES = 4,
    FMODIFIER_TYPE_NOISE = 5,
    FMODIFIER_TYPE_FILTER = 6,
    FMODIFIER_TYPE_PYTHON = 7,
    FMODIFIER_TYPE_LIMITS = 8,
    FMODIFIER_TYPE_STEPPED = 9,
    FMODIFIER_TYPE_SMOOTH = 10,
    FMODIFIER_NUM_TYPES,
}

impl Default for eFModifier_Types {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FMODIFIER_TYPE_NULL: i32 = eFModifier_Types::FMODIFIER_TYPE_NULL as i32;
pub const FMODIFIER_TYPE_GENERATOR: i32 = eFModifier_Types::FMODIFIER_TYPE_GENERATOR as i32;
pub const FMODIFIER_TYPE_FN_GENERATOR: i32 = eFModifier_Types::FMODIFIER_TYPE_FN_GENERATOR as i32;
pub const FMODIFIER_TYPE_ENVELOPE: i32 = eFModifier_Types::FMODIFIER_TYPE_ENVELOPE as i32;
pub const FMODIFIER_TYPE_CYCLES: i32 = eFModifier_Types::FMODIFIER_TYPE_CYCLES as i32;
pub const FMODIFIER_TYPE_NOISE: i32 = eFModifier_Types::FMODIFIER_TYPE_NOISE as i32;
pub const FMODIFIER_TYPE_FILTER: i32 = eFModifier_Types::FMODIFIER_TYPE_FILTER as i32;
pub const FMODIFIER_TYPE_PYTHON: i32 = eFModifier_Types::FMODIFIER_TYPE_PYTHON as i32;
pub const FMODIFIER_TYPE_LIMITS: i32 = eFModifier_Types::FMODIFIER_TYPE_LIMITS as i32;
pub const FMODIFIER_TYPE_STEPPED: i32 = eFModifier_Types::FMODIFIER_TYPE_STEPPED as i32;
pub const FMODIFIER_TYPE_SMOOTH: i32 = eFModifier_Types::FMODIFIER_TYPE_SMOOTH as i32;
pub const FMODIFIER_NUM_TYPES: i32 = eFModifier_Types::FMODIFIER_NUM_TYPES as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFModifier_Flags {
    FMODIFIER_FLAG_DISABLED = (1 << 0),
    FMODIFIER_FLAG_EXPANDED = (1 << 1),
    FMODIFIER_FLAG_ACTIVE = (1 << 2),
    FMODIFIER_FLAG_MUTED = (1 << 3),
    FMODIFIER_FLAG_RANGERESTRICT = (1 << 4),
    FMODIFIER_FLAG_USEINFLUENCE = (1 << 5),
}

impl Default for eFModifier_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FMODIFIER_FLAG_DISABLED: i32 = eFModifier_Flags::FMODIFIER_FLAG_DISABLED as i32;
pub const FMODIFIER_FLAG_EXPANDED: i32 = eFModifier_Flags::FMODIFIER_FLAG_EXPANDED as i32;
pub const FMODIFIER_FLAG_ACTIVE: i32 = eFModifier_Flags::FMODIFIER_FLAG_ACTIVE as i32;
pub const FMODIFIER_FLAG_MUTED: i32 = eFModifier_Flags::FMODIFIER_FLAG_MUTED as i32;
pub const FMODIFIER_FLAG_RANGERESTRICT: i32 = eFModifier_Flags::FMODIFIER_FLAG_RANGERESTRICT as i32;
pub const FMODIFIER_FLAG_USEINFLUENCE: i32 = eFModifier_Flags::FMODIFIER_FLAG_USEINFLUENCE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFMod_Generator_Modes {
    FCM_GENERATOR_POLYNOMIAL = 0,
    FCM_GENERATOR_POLYNOMIAL_FACTORISED = 1,
}

impl Default for eFMod_Generator_Modes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FCM_GENERATOR_POLYNOMIAL: i32 = eFMod_Generator_Modes::FCM_GENERATOR_POLYNOMIAL as i32;
pub const FCM_GENERATOR_POLYNOMIAL_FACTORISED: i32 = eFMod_Generator_Modes::FCM_GENERATOR_POLYNOMIAL_FACTORISED as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFMod_Generator_Flags {
    FCM_GENERATOR_ADDITIVE = (1 << 0),
}

impl Default for eFMod_Generator_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FCM_GENERATOR_ADDITIVE: i32 = eFMod_Generator_Flags::FCM_GENERATOR_ADDITIVE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFMod_Generator_Functions {
    FCM_GENERATOR_FN_SIN = 0,
    FCM_GENERATOR_FN_COS = 1,
    FCM_GENERATOR_FN_TAN = 2,
    FCM_GENERATOR_FN_SQRT = 3,
    FCM_GENERATOR_FN_LN = 4,
    FCM_GENERATOR_FN_SINC = 5,
}

impl Default for eFMod_Generator_Functions {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FCM_GENERATOR_FN_SIN: i32 = eFMod_Generator_Functions::FCM_GENERATOR_FN_SIN as i32;
pub const FCM_GENERATOR_FN_COS: i32 = eFMod_Generator_Functions::FCM_GENERATOR_FN_COS as i32;
pub const FCM_GENERATOR_FN_TAN: i32 = eFMod_Generator_Functions::FCM_GENERATOR_FN_TAN as i32;
pub const FCM_GENERATOR_FN_SQRT: i32 = eFMod_Generator_Functions::FCM_GENERATOR_FN_SQRT as i32;
pub const FCM_GENERATOR_FN_LN: i32 = eFMod_Generator_Functions::FCM_GENERATOR_FN_LN as i32;
pub const FCM_GENERATOR_FN_SINC: i32 = eFMod_Generator_Functions::FCM_GENERATOR_FN_SINC as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFMod_Cycling_Modes {
    FCM_EXTRAPOLATE_NONE = 0,
    FCM_EXTRAPOLATE_CYCLIC,
    FCM_EXTRAPOLATE_CYCLIC_OFFSET,
    FCM_EXTRAPOLATE_MIRROR,
}

impl Default for eFMod_Cycling_Modes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FCM_EXTRAPOLATE_NONE: i32 = eFMod_Cycling_Modes::FCM_EXTRAPOLATE_NONE as i32;
pub const FCM_EXTRAPOLATE_CYCLIC: i32 = eFMod_Cycling_Modes::FCM_EXTRAPOLATE_CYCLIC as i32;
pub const FCM_EXTRAPOLATE_CYCLIC_OFFSET: i32 = eFMod_Cycling_Modes::FCM_EXTRAPOLATE_CYCLIC_OFFSET as i32;
pub const FCM_EXTRAPOLATE_MIRROR: i32 = eFMod_Cycling_Modes::FCM_EXTRAPOLATE_MIRROR as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFMod_Limit_Flags {
    FCM_LIMIT_XMIN = (1 << 0),
    FCM_LIMIT_XMAX = (1 << 1),
    FCM_LIMIT_YMIN = (1 << 2),
    FCM_LIMIT_YMAX = (1 << 3),
}

impl Default for eFMod_Limit_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FCM_LIMIT_XMIN: i32 = eFMod_Limit_Flags::FCM_LIMIT_XMIN as i32;
pub const FCM_LIMIT_XMAX: i32 = eFMod_Limit_Flags::FCM_LIMIT_XMAX as i32;
pub const FCM_LIMIT_YMIN: i32 = eFMod_Limit_Flags::FCM_LIMIT_YMIN as i32;
pub const FCM_LIMIT_YMAX: i32 = eFMod_Limit_Flags::FCM_LIMIT_YMAX as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFMod_Noise_Modifications {
    FCM_NOISE_MODIF_REPLACE = 0,
    FCM_NOISE_MODIF_ADD,
    FCM_NOISE_MODIF_SUBTRACT,
    FCM_NOISE_MODIF_MULTIPLY,
}

impl Default for eFMod_Noise_Modifications {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FCM_NOISE_MODIF_REPLACE: i32 = eFMod_Noise_Modifications::FCM_NOISE_MODIF_REPLACE as i32;
pub const FCM_NOISE_MODIF_ADD: i32 = eFMod_Noise_Modifications::FCM_NOISE_MODIF_ADD as i32;
pub const FCM_NOISE_MODIF_SUBTRACT: i32 = eFMod_Noise_Modifications::FCM_NOISE_MODIF_SUBTRACT as i32;
pub const FCM_NOISE_MODIF_MULTIPLY: i32 = eFMod_Noise_Modifications::FCM_NOISE_MODIF_MULTIPLY as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFMod_Stepped_Flags {
    FCM_STEPPED_NO_BEFORE = (1 << 0),
    FCM_STEPPED_NO_AFTER = (1 << 1),
}

impl Default for eFMod_Stepped_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FCM_STEPPED_NO_BEFORE: i32 = eFMod_Stepped_Flags::FCM_STEPPED_NO_BEFORE as i32;
pub const FCM_STEPPED_NO_AFTER: i32 = eFMod_Stepped_Flags::FCM_STEPPED_NO_AFTER as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDriverTarget_Options {
    DTAR_OPTION_USE_FALLBACK = (1 << 0),
}

impl Default for eDriverTarget_Options {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const DTAR_OPTION_USE_FALLBACK: i32 = eDriverTarget_Options::DTAR_OPTION_USE_FALLBACK as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDriverTarget_Flag {
    DTAR_FLAG_STRUCT_REF = (1 << 0),
    DTAR_FLAG_ID_OB_ONLY = (1 << 1),
    DTAR_FLAG_LOCALSPACE = (1 << 2),
    DTAR_FLAG_LOCAL_CONSTS = (1 << 3),
    DTAR_FLAG_INVALID = (1 << 4),
    DTAR_FLAG_FALLBACK_USED = (1 << 5),
}

impl Default for eDriverTarget_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const DTAR_FLAG_STRUCT_REF: i32 = eDriverTarget_Flag::DTAR_FLAG_STRUCT_REF as i32;
pub const DTAR_FLAG_ID_OB_ONLY: i32 = eDriverTarget_Flag::DTAR_FLAG_ID_OB_ONLY as i32;
pub const DTAR_FLAG_LOCALSPACE: i32 = eDriverTarget_Flag::DTAR_FLAG_LOCALSPACE as i32;
pub const DTAR_FLAG_LOCAL_CONSTS: i32 = eDriverTarget_Flag::DTAR_FLAG_LOCAL_CONSTS as i32;
pub const DTAR_FLAG_INVALID: i32 = eDriverTarget_Flag::DTAR_FLAG_INVALID as i32;
pub const DTAR_FLAG_FALLBACK_USED: i32 = eDriverTarget_Flag::DTAR_FLAG_FALLBACK_USED as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDriverTarget_TransformChannels {
    DTAR_TRANSCHAN_LOCX = 0,
    DTAR_TRANSCHAN_LOCY,
    DTAR_TRANSCHAN_LOCZ,
    DTAR_TRANSCHAN_ROTX,
    DTAR_TRANSCHAN_ROTY,
    DTAR_TRANSCHAN_ROTZ,
    DTAR_TRANSCHAN_SCALEX,
    DTAR_TRANSCHAN_SCALEY,
    DTAR_TRANSCHAN_SCALEZ,
    DTAR_TRANSCHAN_SCALE_AVG,
    DTAR_TRANSCHAN_ROTW,
    MAX_DTAR_TRANSCHAN_TYPES,
}

impl Default for eDriverTarget_TransformChannels {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const DTAR_TRANSCHAN_LOCX: i32 = eDriverTarget_TransformChannels::DTAR_TRANSCHAN_LOCX as i32;
pub const DTAR_TRANSCHAN_LOCY: i32 = eDriverTarget_TransformChannels::DTAR_TRANSCHAN_LOCY as i32;
pub const DTAR_TRANSCHAN_LOCZ: i32 = eDriverTarget_TransformChannels::DTAR_TRANSCHAN_LOCZ as i32;
pub const DTAR_TRANSCHAN_ROTX: i32 = eDriverTarget_TransformChannels::DTAR_TRANSCHAN_ROTX as i32;
pub const DTAR_TRANSCHAN_ROTY: i32 = eDriverTarget_TransformChannels::DTAR_TRANSCHAN_ROTY as i32;
pub const DTAR_TRANSCHAN_ROTZ: i32 = eDriverTarget_TransformChannels::DTAR_TRANSCHAN_ROTZ as i32;
pub const DTAR_TRANSCHAN_SCALEX: i32 = eDriverTarget_TransformChannels::DTAR_TRANSCHAN_SCALEX as i32;
pub const DTAR_TRANSCHAN_SCALEY: i32 = eDriverTarget_TransformChannels::DTAR_TRANSCHAN_SCALEY as i32;
pub const DTAR_TRANSCHAN_SCALEZ: i32 = eDriverTarget_TransformChannels::DTAR_TRANSCHAN_SCALEZ as i32;
pub const DTAR_TRANSCHAN_SCALE_AVG: i32 = eDriverTarget_TransformChannels::DTAR_TRANSCHAN_SCALE_AVG as i32;
pub const DTAR_TRANSCHAN_ROTW: i32 = eDriverTarget_TransformChannels::DTAR_TRANSCHAN_ROTW as i32;
pub const MAX_DTAR_TRANSCHAN_TYPES: i32 = eDriverTarget_TransformChannels::MAX_DTAR_TRANSCHAN_TYPES as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDriverTarget_RotationMode {
    DTAR_ROTMODE_AUTO = 0,
    DTAR_ROTMODE_EULER_XYZ = 1,
    DTAR_ROTMODE_EULER_XZY,
    DTAR_ROTMODE_EULER_YXZ,
    DTAR_ROTMODE_EULER_YZX,
    DTAR_ROTMODE_EULER_ZXY,
    DTAR_ROTMODE_EULER_ZYX,
    DTAR_ROTMODE_QUATERNION,
    DTAR_ROTMODE_SWING_TWIST_X,
    DTAR_ROTMODE_SWING_TWIST_Y,
    DTAR_ROTMODE_SWING_TWIST_Z,
}

impl Default for eDriverTarget_RotationMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const DTAR_ROTMODE_AUTO: i32 = eDriverTarget_RotationMode::DTAR_ROTMODE_AUTO as i32;
pub const DTAR_ROTMODE_EULER_XYZ: i32 = eDriverTarget_RotationMode::DTAR_ROTMODE_EULER_XYZ as i32;
pub const DTAR_ROTMODE_EULER_XZY: i32 = eDriverTarget_RotationMode::DTAR_ROTMODE_EULER_XZY as i32;
pub const DTAR_ROTMODE_EULER_YXZ: i32 = eDriverTarget_RotationMode::DTAR_ROTMODE_EULER_YXZ as i32;
pub const DTAR_ROTMODE_EULER_YZX: i32 = eDriverTarget_RotationMode::DTAR_ROTMODE_EULER_YZX as i32;
pub const DTAR_ROTMODE_EULER_ZXY: i32 = eDriverTarget_RotationMode::DTAR_ROTMODE_EULER_ZXY as i32;
pub const DTAR_ROTMODE_EULER_ZYX: i32 = eDriverTarget_RotationMode::DTAR_ROTMODE_EULER_ZYX as i32;
pub const DTAR_ROTMODE_QUATERNION: i32 = eDriverTarget_RotationMode::DTAR_ROTMODE_QUATERNION as i32;
pub const DTAR_ROTMODE_SWING_TWIST_X: i32 = eDriverTarget_RotationMode::DTAR_ROTMODE_SWING_TWIST_X as i32;
pub const DTAR_ROTMODE_SWING_TWIST_Y: i32 = eDriverTarget_RotationMode::DTAR_ROTMODE_SWING_TWIST_Y as i32;
pub const DTAR_ROTMODE_SWING_TWIST_Z: i32 = eDriverTarget_RotationMode::DTAR_ROTMODE_SWING_TWIST_Z as i32;
pub const DTAR_ROTMODE_EULER_MIN: i32 = DTAR_ROTMODE_EULER_XYZ;
pub const DTAR_ROTMODE_EULER_MAX: i32 = DTAR_ROTMODE_EULER_ZYX;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDriverTarget_ContextProperty {
    DTAR_CONTEXT_PROPERTY_ACTIVE_SCENE = 0,
    DTAR_CONTEXT_PROPERTY_ACTIVE_VIEW_LAYER = 1,
}

impl Default for eDriverTarget_ContextProperty {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const DTAR_CONTEXT_PROPERTY_ACTIVE_SCENE: i32 = eDriverTarget_ContextProperty::DTAR_CONTEXT_PROPERTY_ACTIVE_SCENE as i32;
pub const DTAR_CONTEXT_PROPERTY_ACTIVE_VIEW_LAYER: i32 = eDriverTarget_ContextProperty::DTAR_CONTEXT_PROPERTY_ACTIVE_VIEW_LAYER as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDriverVar_Types {
    DVAR_TYPE_SINGLE_PROP = 0,
    DVAR_TYPE_ROT_DIFF,
    DVAR_TYPE_LOC_DIFF,
    DVAR_TYPE_TRANSFORM_CHAN,
    DVAR_TYPE_CONTEXT_PROP,
    MAX_DVAR_TYPES,
}

impl Default for eDriverVar_Types {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const DVAR_TYPE_SINGLE_PROP: i32 = eDriverVar_Types::DVAR_TYPE_SINGLE_PROP as i32;
pub const DVAR_TYPE_ROT_DIFF: i32 = eDriverVar_Types::DVAR_TYPE_ROT_DIFF as i32;
pub const DVAR_TYPE_LOC_DIFF: i32 = eDriverVar_Types::DVAR_TYPE_LOC_DIFF as i32;
pub const DVAR_TYPE_TRANSFORM_CHAN: i32 = eDriverVar_Types::DVAR_TYPE_TRANSFORM_CHAN as i32;
pub const DVAR_TYPE_CONTEXT_PROP: i32 = eDriverVar_Types::DVAR_TYPE_CONTEXT_PROP as i32;
pub const MAX_DVAR_TYPES: i32 = eDriverVar_Types::MAX_DVAR_TYPES as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDriverVar_Flags {
    DVAR_FLAG_ERROR = (1 << 0),
    DVAR_FLAG_INVALID_NAME = (1 << 1),
    DVAR_FLAG_INVALID_START_NUM = (1 << 2),
    DVAR_FLAG_INVALID_START_CHAR = (1 << 3),
    DVAR_FLAG_INVALID_HAS_SPACE = (1 << 4),
    DVAR_FLAG_INVALID_HAS_DOT = (1 << 5),
    DVAR_FLAG_INVALID_HAS_SPECIAL = (1 << 6),
    DVAR_FLAG_INVALID_PY_KEYWORD = (1 << 7),
    DVAR_FLAG_INVALID_EMPTY = (1 << 8),
}

impl Default for eDriverVar_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const DVAR_FLAG_ERROR: i32 = eDriverVar_Flags::DVAR_FLAG_ERROR as i32;
pub const DVAR_FLAG_INVALID_NAME: i32 = eDriverVar_Flags::DVAR_FLAG_INVALID_NAME as i32;
pub const DVAR_FLAG_INVALID_START_NUM: i32 = eDriverVar_Flags::DVAR_FLAG_INVALID_START_NUM as i32;
pub const DVAR_FLAG_INVALID_START_CHAR: i32 = eDriverVar_Flags::DVAR_FLAG_INVALID_START_CHAR as i32;
pub const DVAR_FLAG_INVALID_HAS_SPACE: i32 = eDriverVar_Flags::DVAR_FLAG_INVALID_HAS_SPACE as i32;
pub const DVAR_FLAG_INVALID_HAS_DOT: i32 = eDriverVar_Flags::DVAR_FLAG_INVALID_HAS_DOT as i32;
pub const DVAR_FLAG_INVALID_HAS_SPECIAL: i32 = eDriverVar_Flags::DVAR_FLAG_INVALID_HAS_SPECIAL as i32;
pub const DVAR_FLAG_INVALID_PY_KEYWORD: i32 = eDriverVar_Flags::DVAR_FLAG_INVALID_PY_KEYWORD as i32;
pub const DVAR_FLAG_INVALID_EMPTY: i32 = eDriverVar_Flags::DVAR_FLAG_INVALID_EMPTY as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDriver_Types {
    DRIVER_TYPE_AVERAGE = 0,
    DRIVER_TYPE_PYTHON,
    DRIVER_TYPE_SUM,
    DRIVER_TYPE_MIN,
    DRIVER_TYPE_MAX,
}

impl Default for eDriver_Types {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const DRIVER_TYPE_AVERAGE: i32 = eDriver_Types::DRIVER_TYPE_AVERAGE as i32;
pub const DRIVER_TYPE_PYTHON: i32 = eDriver_Types::DRIVER_TYPE_PYTHON as i32;
pub const DRIVER_TYPE_SUM: i32 = eDriver_Types::DRIVER_TYPE_SUM as i32;
pub const DRIVER_TYPE_MIN: i32 = eDriver_Types::DRIVER_TYPE_MIN as i32;
pub const DRIVER_TYPE_MAX: i32 = eDriver_Types::DRIVER_TYPE_MAX as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDriver_Flags {
    DRIVER_FLAG_INVALID = (1 << 0),
    DRIVER_FLAG_DEPRECATED = (1 << 1),
    DRIVER_FLAG_RECOMPILE = (1 << 3),
    DRIVER_FLAG_RENAMEVAR = (1 << 4),
    DRIVER_FLAG_PYTHON_BLOCKED = (1 << 5),
    DRIVER_FLAG_USE_SELF = (1 << 6),
}

impl Default for eDriver_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const DRIVER_FLAG_INVALID: i32 = eDriver_Flags::DRIVER_FLAG_INVALID as i32;
pub const DRIVER_FLAG_DEPRECATED: i32 = eDriver_Flags::DRIVER_FLAG_DEPRECATED as i32;
pub const DRIVER_FLAG_RECOMPILE: i32 = eDriver_Flags::DRIVER_FLAG_RECOMPILE as i32;
pub const DRIVER_FLAG_RENAMEVAR: i32 = eDriver_Flags::DRIVER_FLAG_RENAMEVAR as i32;
pub const DRIVER_FLAG_PYTHON_BLOCKED: i32 = eDriver_Flags::DRIVER_FLAG_PYTHON_BLOCKED as i32;
pub const DRIVER_FLAG_USE_SELF: i32 = eDriver_Flags::DRIVER_FLAG_USE_SELF as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFCurve_Flags {
    FCURVE_VISIBLE = (1 << 0),
    FCURVE_SELECTED = (1 << 1),
    FCURVE_ACTIVE = (1 << 2),
    FCURVE_PROTECTED = (1 << 3),
    FCURVE_MUTED = (1 << 4),
    FCURVE_AUTO_HANDLES = (1 << 5),
    FCURVE_MOD_OFF = (1 << 6),
    FCURVE_DISABLED = (1 << 10),
    FCURVE_INT_VALUES = (1 << 11),
    FCURVE_DISCRETE_VALUES = (1 << 12),
    FCURVE_TAGGED = 1 << 15,
}

impl Default for eFCurve_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FCURVE_VISIBLE: i32 = eFCurve_Flags::FCURVE_VISIBLE as i32;
pub const FCURVE_SELECTED: i32 = eFCurve_Flags::FCURVE_SELECTED as i32;
pub const FCURVE_ACTIVE: i32 = eFCurve_Flags::FCURVE_ACTIVE as i32;
pub const FCURVE_PROTECTED: i32 = eFCurve_Flags::FCURVE_PROTECTED as i32;
pub const FCURVE_MUTED: i32 = eFCurve_Flags::FCURVE_MUTED as i32;
pub const FCURVE_AUTO_HANDLES: i32 = eFCurve_Flags::FCURVE_AUTO_HANDLES as i32;
pub const FCURVE_MOD_OFF: i32 = eFCurve_Flags::FCURVE_MOD_OFF as i32;
pub const FCURVE_DISABLED: i32 = eFCurve_Flags::FCURVE_DISABLED as i32;
pub const FCURVE_INT_VALUES: i32 = eFCurve_Flags::FCURVE_INT_VALUES as i32;
pub const FCURVE_DISCRETE_VALUES: i32 = eFCurve_Flags::FCURVE_DISCRETE_VALUES as i32;
pub const FCURVE_TAGGED: i32 = eFCurve_Flags::FCURVE_TAGGED as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFCurve_Extend {
    FCURVE_EXTRAPOLATE_CONSTANT = 0,
    FCURVE_EXTRAPOLATE_LINEAR,
}

impl Default for eFCurve_Extend {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FCURVE_EXTRAPOLATE_CONSTANT: i32 = eFCurve_Extend::FCURVE_EXTRAPOLATE_CONSTANT as i32;
pub const FCURVE_EXTRAPOLATE_LINEAR: i32 = eFCurve_Extend::FCURVE_EXTRAPOLATE_LINEAR as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFCurve_Coloring {
    FCURVE_COLOR_AUTO_RAINBOW = 0,
    FCURVE_COLOR_AUTO_RGB = 1,
    FCURVE_COLOR_AUTO_YRGB = 3,
    FCURVE_COLOR_CUSTOM = 2,
}

impl Default for eFCurve_Coloring {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FCURVE_COLOR_AUTO_RAINBOW: i32 = eFCurve_Coloring::FCURVE_COLOR_AUTO_RAINBOW as i32;
pub const FCURVE_COLOR_AUTO_RGB: i32 = eFCurve_Coloring::FCURVE_COLOR_AUTO_RGB as i32;
pub const FCURVE_COLOR_AUTO_YRGB: i32 = eFCurve_Coloring::FCURVE_COLOR_AUTO_YRGB as i32;
pub const FCURVE_COLOR_CUSTOM: i32 = eFCurve_Coloring::FCURVE_COLOR_CUSTOM as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFCurve_Smoothing {
    FCURVE_SMOOTH_NONE = 0,
    FCURVE_SMOOTH_CONT_ACCEL = 1,
}

impl Default for eFCurve_Smoothing {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FCURVE_SMOOTH_NONE: i32 = eFCurve_Smoothing::FCURVE_SMOOTH_NONE as i32;
pub const FCURVE_SMOOTH_CONT_ACCEL: i32 = eFCurve_Smoothing::FCURVE_SMOOTH_CONT_ACCEL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eNlaStrip_Blend_Mode {
    NLASTRIP_MODE_REPLACE = 0,
    NLASTRIP_MODE_ADD,
    NLASTRIP_MODE_SUBTRACT,
    NLASTRIP_MODE_MULTIPLY,
    NLASTRIP_MODE_COMBINE,
}

impl Default for eNlaStrip_Blend_Mode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const NLASTRIP_MODE_REPLACE: i32 = eNlaStrip_Blend_Mode::NLASTRIP_MODE_REPLACE as i32;
pub const NLASTRIP_MODE_ADD: i32 = eNlaStrip_Blend_Mode::NLASTRIP_MODE_ADD as i32;
pub const NLASTRIP_MODE_SUBTRACT: i32 = eNlaStrip_Blend_Mode::NLASTRIP_MODE_SUBTRACT as i32;
pub const NLASTRIP_MODE_MULTIPLY: i32 = eNlaStrip_Blend_Mode::NLASTRIP_MODE_MULTIPLY as i32;
pub const NLASTRIP_MODE_COMBINE: i32 = eNlaStrip_Blend_Mode::NLASTRIP_MODE_COMBINE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eNlaStrip_Extrapolate_Mode {
    NLASTRIP_EXTEND_HOLD = 0,
    NLASTRIP_EXTEND_HOLD_FORWARD = 1,
    NLASTRIP_EXTEND_NOTHING = 2,
}

impl Default for eNlaStrip_Extrapolate_Mode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const NLASTRIP_EXTEND_HOLD: i32 = eNlaStrip_Extrapolate_Mode::NLASTRIP_EXTEND_HOLD as i32;
pub const NLASTRIP_EXTEND_HOLD_FORWARD: i32 = eNlaStrip_Extrapolate_Mode::NLASTRIP_EXTEND_HOLD_FORWARD as i32;
pub const NLASTRIP_EXTEND_NOTHING: i32 = eNlaStrip_Extrapolate_Mode::NLASTRIP_EXTEND_NOTHING as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eNlaStrip_Flag {
    NLASTRIP_FLAG_ACTIVE = (1 << 0),
    NLASTRIP_FLAG_SELECT = (1 << 1),
    NLASTRIP_FLAG_TWEAKUSER = (1 << 4),
    NLASTRIP_FLAG_USR_INFLUENCE = (1 << 5),
    NLASTRIP_FLAG_USR_TIME = (1 << 6),
    NLASTRIP_FLAG_USR_TIME_CYCLIC = (1 << 7),
    NLASTRIP_FLAG_SYNC_LENGTH = (1 << 9),
    NLASTRIP_FLAG_AUTO_BLENDS = (1 << 10),
    NLASTRIP_FLAG_REVERSE = (1 << 11),
    NLASTRIP_FLAG_MUTED = (1 << 12),
    NLASTRIP_FLAG_INVALID_LOCATION = (1 << 28),
    NLASTRIP_FLAG_NO_TIME_MAP = (1 << 29),
    NLASTRIP_FLAG_TEMP_META = (1 << 30),
    NLASTRIP_FLAG_EDIT_TOUCHED = (i32::MIN),
}

impl Default for eNlaStrip_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const NLASTRIP_FLAG_ACTIVE: i32 = eNlaStrip_Flag::NLASTRIP_FLAG_ACTIVE as i32;
pub const NLASTRIP_FLAG_SELECT: i32 = eNlaStrip_Flag::NLASTRIP_FLAG_SELECT as i32;
pub const NLASTRIP_FLAG_TWEAKUSER: i32 = eNlaStrip_Flag::NLASTRIP_FLAG_TWEAKUSER as i32;
pub const NLASTRIP_FLAG_USR_INFLUENCE: i32 = eNlaStrip_Flag::NLASTRIP_FLAG_USR_INFLUENCE as i32;
pub const NLASTRIP_FLAG_USR_TIME: i32 = eNlaStrip_Flag::NLASTRIP_FLAG_USR_TIME as i32;
pub const NLASTRIP_FLAG_USR_TIME_CYCLIC: i32 = eNlaStrip_Flag::NLASTRIP_FLAG_USR_TIME_CYCLIC as i32;
pub const NLASTRIP_FLAG_SYNC_LENGTH: i32 = eNlaStrip_Flag::NLASTRIP_FLAG_SYNC_LENGTH as i32;
pub const NLASTRIP_FLAG_AUTO_BLENDS: i32 = eNlaStrip_Flag::NLASTRIP_FLAG_AUTO_BLENDS as i32;
pub const NLASTRIP_FLAG_REVERSE: i32 = eNlaStrip_Flag::NLASTRIP_FLAG_REVERSE as i32;
pub const NLASTRIP_FLAG_MUTED: i32 = eNlaStrip_Flag::NLASTRIP_FLAG_MUTED as i32;
pub const NLASTRIP_FLAG_INVALID_LOCATION: i32 = eNlaStrip_Flag::NLASTRIP_FLAG_INVALID_LOCATION as i32;
pub const NLASTRIP_FLAG_NO_TIME_MAP: i32 = eNlaStrip_Flag::NLASTRIP_FLAG_NO_TIME_MAP as i32;
pub const NLASTRIP_FLAG_TEMP_META: i32 = eNlaStrip_Flag::NLASTRIP_FLAG_TEMP_META as i32;
pub const NLASTRIP_FLAG_EDIT_TOUCHED: i32 = eNlaStrip_Flag::NLASTRIP_FLAG_EDIT_TOUCHED as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eNlaStrip_Type {
    NLASTRIP_TYPE_CLIP = 0,
    NLASTRIP_TYPE_TRANSITION,
    NLASTRIP_TYPE_META,
    NLASTRIP_TYPE_SOUND,
}

impl Default for eNlaStrip_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const NLASTRIP_TYPE_CLIP: i32 = eNlaStrip_Type::NLASTRIP_TYPE_CLIP as i32;
pub const NLASTRIP_TYPE_TRANSITION: i32 = eNlaStrip_Type::NLASTRIP_TYPE_TRANSITION as i32;
pub const NLASTRIP_TYPE_META: i32 = eNlaStrip_Type::NLASTRIP_TYPE_META as i32;
pub const NLASTRIP_TYPE_SOUND: i32 = eNlaStrip_Type::NLASTRIP_TYPE_SOUND as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eNlaTrack_Flag {
    NLATRACK_ACTIVE = (1 << 0),
    NLATRACK_SELECTED = (1 << 1),
    NLATRACK_MUTED = (1 << 2),
    NLATRACK_SOLO = (1 << 3),
    NLATRACK_PROTECTED = (1 << 4),
    NLATRACK_DISABLED = (1 << 10),
    NLATRACK_TEMPORARILY_ADDED = (1 << 11),
    NLATRACK_OVERRIDELIBRARY_LOCAL = 1 << 16,
}

impl Default for eNlaTrack_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const NLATRACK_ACTIVE: i32 = eNlaTrack_Flag::NLATRACK_ACTIVE as i32;
pub const NLATRACK_SELECTED: i32 = eNlaTrack_Flag::NLATRACK_SELECTED as i32;
pub const NLATRACK_MUTED: i32 = eNlaTrack_Flag::NLATRACK_MUTED as i32;
pub const NLATRACK_SOLO: i32 = eNlaTrack_Flag::NLATRACK_SOLO as i32;
pub const NLATRACK_PROTECTED: i32 = eNlaTrack_Flag::NLATRACK_PROTECTED as i32;
pub const NLATRACK_DISABLED: i32 = eNlaTrack_Flag::NLATRACK_DISABLED as i32;
pub const NLATRACK_TEMPORARILY_ADDED: i32 = eNlaTrack_Flag::NLATRACK_TEMPORARILY_ADDED as i32;
pub const NLATRACK_OVERRIDELIBRARY_LOCAL: i32 = eNlaTrack_Flag::NLATRACK_OVERRIDELIBRARY_LOCAL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eKS_Settings {
    KEYINGSET_ABSOLUTE = (1 << 1),
}

impl Default for eKS_Settings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const KEYINGSET_ABSOLUTE: i32 = eKS_Settings::KEYINGSET_ABSOLUTE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eInsertKeyFlags {
    INSERTKEY_NOFLAGS = 0,
    INSERTKEY_NEEDED = (1 << 0),
    INSERTKEY_MATRIX = (1 << 1),
    INSERTKEY_FAST = (1 << 2),
    INSERTKEY_REPLACE = (1 << 4),
    INSERTKEY_NO_USERPREF = (1 << 6),
    INSERTKEY_OVERWRITE_FULL = (1 << 7),
    INSERTKEY_CYCLE_AWARE = (1 << 9),
    INSERTKEY_AVAILABLE = (1 << 10),
}

impl Default for eInsertKeyFlags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const INSERTKEY_NOFLAGS: i32 = eInsertKeyFlags::INSERTKEY_NOFLAGS as i32;
pub const INSERTKEY_NEEDED: i32 = eInsertKeyFlags::INSERTKEY_NEEDED as i32;
pub const INSERTKEY_MATRIX: i32 = eInsertKeyFlags::INSERTKEY_MATRIX as i32;
pub const INSERTKEY_FAST: i32 = eInsertKeyFlags::INSERTKEY_FAST as i32;
pub const INSERTKEY_REPLACE: i32 = eInsertKeyFlags::INSERTKEY_REPLACE as i32;
pub const INSERTKEY_NO_USERPREF: i32 = eInsertKeyFlags::INSERTKEY_NO_USERPREF as i32;
pub const INSERTKEY_OVERWRITE_FULL: i32 = eInsertKeyFlags::INSERTKEY_OVERWRITE_FULL as i32;
pub const INSERTKEY_CYCLE_AWARE: i32 = eInsertKeyFlags::INSERTKEY_CYCLE_AWARE as i32;
pub const INSERTKEY_AVAILABLE: i32 = eInsertKeyFlags::INSERTKEY_AVAILABLE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eKSP_Settings {
    KSP_FLAG_WHOLE_ARRAY = (1 << 0),
}

impl Default for eKSP_Settings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const KSP_FLAG_WHOLE_ARRAY: i32 = eKSP_Settings::KSP_FLAG_WHOLE_ARRAY as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eKSP_Grouping {
    KSP_GROUP_NAMED = 0,
    KSP_GROUP_NONE,
    KSP_GROUP_KSNAME,
}

impl Default for eKSP_Grouping {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const KSP_GROUP_NAMED: i32 = eKSP_Grouping::KSP_GROUP_NAMED as i32;
pub const KSP_GROUP_NONE: i32 = eKSP_Grouping::KSP_GROUP_NONE as i32;
pub const KSP_GROUP_KSNAME: i32 = eKSP_Grouping::KSP_GROUP_KSNAME as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eAnimData_Flag {
    ADT_NLA_SOLO_TRACK = (1 << 0),
    ADT_NLA_EVAL_OFF = (1 << 1),
    ADT_NLA_EDIT_ON = (1 << 2),
    ADT_NLA_EDIT_NOMAP = (1 << 3),
    ADT_NLA_SKEYS_COLLAPSED = (1 << 4),
    ADT_NLA_EVAL_UPPER_TRACKS = (1 << 5),
    ADT_DRIVERS_COLLAPSED = (1 << 10),
    ADT_UI_SELECTED = (1 << 14),
    ADT_UI_ACTIVE = (1 << 15),
    ADT_CURVES_NOT_VISIBLE = (1 << 16),
    ADT_CURVES_ALWAYS_VISIBLE = (1 << 17),
    ADT_UI_EXPANDED = (1 << 18),
}

impl Default for eAnimData_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const ADT_NLA_SOLO_TRACK: i32 = eAnimData_Flag::ADT_NLA_SOLO_TRACK as i32;
pub const ADT_NLA_EVAL_OFF: i32 = eAnimData_Flag::ADT_NLA_EVAL_OFF as i32;
pub const ADT_NLA_EDIT_ON: i32 = eAnimData_Flag::ADT_NLA_EDIT_ON as i32;
pub const ADT_NLA_EDIT_NOMAP: i32 = eAnimData_Flag::ADT_NLA_EDIT_NOMAP as i32;
pub const ADT_NLA_SKEYS_COLLAPSED: i32 = eAnimData_Flag::ADT_NLA_SKEYS_COLLAPSED as i32;
pub const ADT_NLA_EVAL_UPPER_TRACKS: i32 = eAnimData_Flag::ADT_NLA_EVAL_UPPER_TRACKS as i32;
pub const ADT_DRIVERS_COLLAPSED: i32 = eAnimData_Flag::ADT_DRIVERS_COLLAPSED as i32;
pub const ADT_UI_SELECTED: i32 = eAnimData_Flag::ADT_UI_SELECTED as i32;
pub const ADT_UI_ACTIVE: i32 = eAnimData_Flag::ADT_UI_ACTIVE as i32;
pub const ADT_CURVES_NOT_VISIBLE: i32 = eAnimData_Flag::ADT_CURVES_NOT_VISIBLE as i32;
pub const ADT_CURVES_ALWAYS_VISIBLE: i32 = eAnimData_Flag::ADT_CURVES_ALWAYS_VISIBLE as i32;
pub const ADT_UI_EXPANDED: i32 = eAnimData_Flag::ADT_UI_EXPANDED as i32;
