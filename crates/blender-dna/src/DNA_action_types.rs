//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[allow(non_camel_case_types)]
type int32_t = i32;
#[allow(non_camel_case_types)]
type uint32_t = u32;
#[allow(non_camel_case_types)]
type int16_t = i16;
#[allow(non_camel_case_types)]
type uint16_t = u16;
#[allow(non_camel_case_types)]
type int64_t = i64;
#[allow(non_camel_case_types)]
type uint64_t = u64;
#[allow(non_camel_case_types)]
type int8_t = i8;
#[allow(non_camel_case_types)]
type uint8_t = u8;
#[allow(non_camel_case_types)]
type uchar = u8;
#[allow(non_camel_case_types)]
type ushort = u16;
#[allow(non_camel_case_types)]
type uint = u32;
#[allow(non_camel_case_types)]
type ulong = u64;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMotionPathVert_Flag(pub i32);

impl eMotionPathVert_Flag {
    pub const MOTIONPATH_VERT_SEL: Self = Self(((1 << 0)) as i32);
    pub const MOTIONPATH_VERT_KEY: Self = Self(((1 << 1)) as i32);
    pub const MOTIONPATH_VERT_EVALUATED: Self = Self(((1 << 2)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMotionPath_Flag(pub i32);

impl eMotionPath_Flag {
    pub const MOTIONPATH_FLAG_BHEAD: Self = Self(((1 << 0)) as i32);
    pub const MOTIONPATH_FLAG_EDIT: Self = Self(((1 << 1)) as i32);
    pub const MOTIONPATH_FLAG_CUSTOM: Self = Self(((1 << 2)) as i32);
    pub const MOTIONPATH_FLAG_LINES: Self = Self(((1 << 3)) as i32);
    pub const MOTIONPATH_FLAG_BAKE_CAMERA: Self = Self(((1 << 4)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eAnimViz_RecalcFlags(pub i16);

impl eAnimViz_RecalcFlags {
    pub const ANIMVIZ_RECALC_PATHS: Self = Self(((1 << 0)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMotionPaths_Types(pub i16);

impl eMotionPaths_Types {
    pub const MOTIONPATH_TYPE_RANGE: Self = Self((0) as i16);
    pub const MOTIONPATH_TYPE_ACFRA: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMotionPath_Ranges(pub i16);

impl eMotionPath_Ranges {
    pub const MOTIONPATH_RANGE_SCENE: Self = Self((0) as i16);
    pub const MOTIONPATH_RANGE_KEYS_SELECTED: Self = Self((1) as i16);
    pub const MOTIONPATH_RANGE_KEYS_ALL: Self = Self((2) as i16);
    pub const MOTIONPATH_RANGE_MANUAL: Self = Self((3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMotionPaths_ViewFlag(pub i16);

impl eMotionPaths_ViewFlag {
    pub const MOTIONPATH_VIEW_FNUMS: Self = Self(((1 << 0)) as i16);
    pub const MOTIONPATH_VIEW_KFRAS: Self = Self(((1 << 1)) as i16);
    pub const MOTIONPATH_VIEW_KFNOS: Self = Self(((1 << 2)) as i16);
    pub const MOTIONPATH_VIEW_KFACT: Self = Self(((1 << 3)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMotionPath_BakeFlag(pub i16);

impl eMotionPath_BakeFlag {
    pub const MOTIONPATH_BAKE_HEADS: Self = Self(((1 << 1)) as i16);
    pub const MOTIONPATH_BAKE_HAS_PATHS: Self = Self(((1 << 2)) as i16);
    pub const MOTIONPATH_BAKE_CAMERA_SPACE: Self = Self(((1 << 3)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct bPoseChannelRuntimeFlag(pub u8);

impl bPoseChannelRuntimeFlag {
    pub const POSE_RUNTIME_TRANSFORM: Self = Self(((1 << 0)) as u8);
    pub const POSE_RUNTIME_HINGE_CHILD_TRANSFORM: Self = Self(((1 << 1)) as u8);
    pub const POSE_RUNTIME_TRANSFORM_CHILD: Self = Self(((1 << 2)) as u8);
    pub const POSE_RUNTIME_IN_SELECTION_AREA: Self = Self(((1 << 3)) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePchan_Flag(pub i16);

impl ePchan_Flag {
    pub const POSE_TRANSFORM_AT_CUSTOM_TX: Self = Self(((1 << 4)) as i16);
    pub const POSE_TRANSFORM_AROUND_CUSTOM_TX: Self = Self(((1 << 5)) as i16);
    pub const POSE_SELECTED: Self = Self(((1 << 6)) as i16);
    pub const POSE_SELECTED_ROOT: Self = Self(((1 << 7)) as i16);
    pub const POSE_SELECTED_TIP: Self = Self(((1 << 8)) as i16);
    pub const POSE_SELECTED_ALL: Self = Self(5 as i16);
    pub const POSE_CHAIN: Self = Self(((1 << 9)) as i16);
    pub const POSE_DONE: Self = Self(((1 << 10)) as i16);
    pub const POSE_IKTREE: Self = Self(((1 << 13)) as i16);
    pub const POSE_IKSPLINE: Self = Self(9 as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePchan_ConstFlag(pub i8);

impl ePchan_ConstFlag {
    pub const PCHAN_HAS_IK: Self = Self(((1 << 0)) as i8);
    pub const PCHAN_HAS_CONST: Self = Self(((1 << 1)) as i8);
    pub const PCHAN_HAS_NO_TARGET: Self = Self(((1 << 3)) as i8);
    pub const PCHAN_HAS_SPLINEIK: Self = Self(((1 << 5)) as i8);
    pub const PCHAN_INFLUENCED_BY_IK: Self = Self(((1 << 6)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePchan_IkFlag(pub i16);

impl ePchan_IkFlag {
    pub const BONE_IK_NO_XDOF: Self = Self(((1 << 0)) as i16);
    pub const BONE_IK_NO_YDOF: Self = Self(((1 << 1)) as i16);
    pub const BONE_IK_NO_ZDOF: Self = Self(((1 << 2)) as i16);
    pub const BONE_IK_XLIMIT: Self = Self(((1 << 3)) as i16);
    pub const BONE_IK_YLIMIT: Self = Self(((1 << 4)) as i16);
    pub const BONE_IK_ZLIMIT: Self = Self(((1 << 5)) as i16);
    pub const BONE_IK_ROTCTL: Self = Self(((1 << 6)) as i16);
    pub const BONE_IK_LINCTL: Self = Self(((1 << 7)) as i16);
    pub const BONE_IK_NO_XDOF_TEMP: Self = Self(((1 << 10)) as i16);
    pub const BONE_IK_NO_YDOF_TEMP: Self = Self(((1 << 11)) as i16);
    pub const BONE_IK_NO_ZDOF_TEMP: Self = Self(((1 << 12)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePchan_DrawFlag(pub i8);

impl ePchan_DrawFlag {
    pub const PCHAN_DRAW_NO_CUSTOM_BONE_SIZE: Self = Self(((1 << 0)) as i8);
    pub const PCHAN_DRAW_HIDDEN: Self = Self(((1 << 1)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePchan_BBoneFlag(pub i8);

impl ePchan_BBoneFlag {
    pub const PCHAN_BBONE_CUSTOM_HANDLES: Self = Self(((1 << 1)) as i8);
    pub const PCHAN_BBONE_CUSTOM_START_REL: Self = Self(((1 << 2)) as i8);
    pub const PCHAN_BBONE_CUSTOM_END_REL: Self = Self(((1 << 3)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRotationModes(pub i16);

impl eRotationModes {
    pub const ROT_MODE_QUAT: Self = Self((0) as i16);
    pub const ROT_MODE_EUL: Self = Self((1) as i16);
    pub const ROT_MODE_XYZ: Self = Self((1) as i16);
    pub const ROT_MODE_XZY: Self = Self((2) as i16);
    pub const ROT_MODE_YXZ: Self = Self((3) as i16);
    pub const ROT_MODE_YZX: Self = Self((4) as i16);
    pub const ROT_MODE_ZXY: Self = Self((5) as i16);
    pub const ROT_MODE_ZYX: Self = Self((6) as i16);
    pub const ROT_MODE_AXISANGLE: Self = Self((-1) as i16);
    pub const ROT_MODE_MIN: Self = Self(9 as i16);
    pub const ROT_MODE_MAX: Self = Self(10 as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePose_Flags(pub i16);

impl ePose_Flags {
    pub const POSE_RECALC: Self = Self(((1 << 0)) as i16);
    pub const POSE_CONSTRAINTS_TIMEDEPEND: Self = Self(((1 << 3)) as i16);
    pub const POSE_WAS_REBUILT: Self = Self(((1 << 5)) as i16);
    pub const POSE_FLAG_DEPRECATED: Self = Self(((1 << 6)) as i16);
    pub const POSE_CONSTRAINTS_NEED_UPDATE_FLAGS: Self = Self(((1 << 7)) as i16);
    pub const POSE_AUTO_IK: Self = Self(((1 << 8)) as i16);
    pub const POSE_MIRROR_EDIT: Self = Self(((1 << 9)) as i16);
    pub const POSE_MIRROR_RELATIVE: Self = Self(((1 << 10)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePose_IKSolverType(pub i32);

impl ePose_IKSolverType {
    pub const IKSOLVER_STANDARD: Self = Self((0) as i32);
    pub const IKSOLVER_ITASC: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eItasc_Flags(pub i16);

impl eItasc_Flags {
    pub const ITASC_AUTO_STEP: Self = Self(((1 << 0)) as i16);
    pub const ITASC_INITIAL_REITERATION: Self = Self(((1 << 1)) as i16);
    pub const ITASC_REITERATION: Self = Self(((1 << 2)) as i16);
    pub const ITASC_SIMULATION: Self = Self(((1 << 3)) as i16);
    pub const ITASC_TRANSLATE_ROOT_BONES: Self = Self(((1 << 4)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eItasc_Solver(pub i16);

impl eItasc_Solver {
    pub const ITASC_SOLVER_SDLS: Self = Self((0) as i16);
    pub const ITASC_SOLVER_DLS: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eActionGroup_Flag(pub u32);

impl eActionGroup_Flag {
    pub const AGRP_SELECTED: Self = Self(((1 << 0)) as u32);
    pub const AGRP_ACTIVE: Self = Self(((1 << 1)) as u32);
    pub const AGRP_PROTECTED: Self = Self(((1 << 2)) as u32);
    pub const AGRP_EXPANDED: Self = Self(((1 << 3)) as u32);
    pub const AGRP_MUTED: Self = Self(((1 << 4)) as u32);
    pub const AGRP_NOTVISIBLE: Self = Self(((1 << 5)) as u32);
    pub const AGRP_EXPANDED_G: Self = Self(((1 << 6)) as u32);
    pub const AGRP_MODIFIERS_OFF: Self = Self(((1 << 7)) as u32);
    pub const AGRP_CURVES_ALWAYS_VISIBLE: Self = Self(((1 << 17)) as u32);
    pub const AGRP_TEMP: Self = Self(((1 << 30)) as u32);
    pub const AGRP_MOVED: Self = Self(10 as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eAction_Flags(pub i32);

impl eAction_Flags {
    pub const ACT_COLLAPSED: Self = Self(((1 << 0)) as i32);
    pub const ACT_SELECTED: Self = Self(((1 << 1)) as i32);
    pub const ACT_MUTED: Self = Self(((1 << 9)) as i32);
    pub const ACT_FRAME_RANGE: Self = Self(((1 << 12)) as i32);
    pub const ACT_CYCLIC: Self = Self(((1 << 13)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eDopeSheet_FilterFlag(pub i32);

impl eDopeSheet_FilterFlag {
    pub const ADS_FILTER_ONLYSEL: Self = Self(((1 << 0)) as i32);
    pub const ADS_FILTER_ONLYDRIVERS: Self = Self(((1 << 1)) as i32);
    pub const ADS_FILTER_ONLYNLA: Self = Self(((1 << 2)) as i32);
    pub const ADS_FILTER_SELEDIT: Self = Self(((1 << 3)) as i32);
    pub const ADS_FILTER_SUMMARY: Self = Self(((1 << 4)) as i32);
    pub const ADS_FILTER_ONLY_SLOTS_OF_ACTIVE: Self = Self(((1 << 5)) as i32);
    pub const ADS_FILTER_NOSHAPEKEYS: Self = Self(((1 << 6)) as i32);
    pub const ADS_FILTER_NOMESH: Self = Self(((1 << 7)) as i32);
    pub const ADS_FILTER_NOOBJ: Self = Self(((1 << 8)) as i32);
    pub const ADS_FILTER_NOLAT: Self = Self(((1 << 9)) as i32);
    pub const ADS_FILTER_NOCAM: Self = Self(((1 << 10)) as i32);
    pub const ADS_FILTER_NOMAT: Self = Self(((1 << 11)) as i32);
    pub const ADS_FILTER_NOLAM: Self = Self(((1 << 12)) as i32);
    pub const ADS_FILTER_NOCUR: Self = Self(((1 << 13)) as i32);
    pub const ADS_FILTER_NOWOR: Self = Self(((1 << 14)) as i32);
    pub const ADS_FILTER_NOSCE: Self = Self(((1 << 15)) as i32);
    pub const ADS_FILTER_NOPART: Self = Self(((1 << 16)) as i32);
    pub const ADS_FILTER_NOMBA: Self = Self(((1 << 17)) as i32);
    pub const ADS_FILTER_NOARM: Self = Self(((1 << 18)) as i32);
    pub const ADS_FILTER_NONTREE: Self = Self(((1 << 19)) as i32);
    pub const ADS_FILTER_NOTEX: Self = Self(((1 << 20)) as i32);
    pub const ADS_FILTER_NOSPK: Self = Self(((1 << 21)) as i32);
    pub const ADS_FILTER_NOLINESTYLE: Self = Self(((1 << 22)) as i32);
    pub const ADS_FILTER_NOMODIFIERS: Self = Self(((1 << 23)) as i32);
    pub const ADS_FILTER_NOGPENCIL: Self = Self(((1 << 24)) as i32);
    pub const ADS_FILTER_NLA_NOACT: Self = Self(((1 << 25)) as i32);
    pub const ADS_FILTER_INCL_HIDDEN: Self = Self(((1 << 26)) as i32);
    pub const ADS_FILTER_ONLY_ERRORS: Self = Self(((1 << 28)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eDopeSheet_FilterFlag2(pub i32);

impl eDopeSheet_FilterFlag2 {
    pub const ADS_FILTER_NOCACHEFILES: Self = Self(((1 << 1)) as i32);
    pub const ADS_FILTER_NOMOVIECLIPS: Self = Self(((1 << 2)) as i32);
    pub const ADS_FILTER_NOHAIR: Self = Self(((1 << 3)) as i32);
    pub const ADS_FILTER_NOPOINTCLOUD: Self = Self(((1 << 4)) as i32);
    pub const ADS_FILTER_NOVOLUME: Self = Self(((1 << 5)) as i32);
    pub const ADS_FILTER_DRIVER_FALLBACK_AS_ERROR: Self = Self(((1 << 6)) as i32);
    pub const ADS_FILTER_NOLIGHTPROBE: Self = Self(((1 << 7)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eDopeSheet_Flag(pub i32);

impl eDopeSheet_Flag {
    pub const ADS_FLAG_SUMMARY_COLLAPSED: Self = Self(((1 << 0)) as i32);
    pub const ADS_FLAG_SHOW_DBFILTERS: Self = Self(((1 << 1)) as i32);
    pub const ADS_FLAG_FUZZY_NAMES: Self = Self(((1 << 2)) as i32);
    pub const ADS_FLAG_NO_DB_SORT: Self = Self(((1 << 3)) as i32);
    pub const ADS_FLAG_INVERT_FILTER: Self = Self(((1 << 4)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SpaceActionOverlays_Flag(pub i32);

impl SpaceActionOverlays_Flag {
    pub const ADS_OVERLAY_SHOW_OVERLAYS: Self = Self(((1 << 0)) as i32);
    pub const ADS_SHOW_SCENE_STRIP_FRAME_RANGE: Self = Self(((1 << 1)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSAction_Flag(pub i16);

impl eSAction_Flag {
    pub const SACTION_MOVING: Self = Self(((1 << 0)) as i16);
    pub const SACTION_SLIDERS: Self = Self(((1 << 1)) as i16);
    pub const SACTION_DRAWTIME: Self = Self(((1 << 2)) as i16);
    pub const SACTION_NOTRANSKEYCULL: Self = Self(((1 << 4)) as i16);
    pub const SACTION_POSEMARKERS_SHOW: Self = Self(((1 << 6)) as i16);
    pub const SACTION_NOREALTIMEUPDATES: Self = Self(((1 << 10)) as i16);
    pub const SACTION_MARKERS_MOVE: Self = Self(((1 << 11)) as i16);
    pub const SACTION_SHOW_INTERPOLATION: Self = Self(((1 << 12)) as i16);
    pub const SACTION_SHOW_EXTREMES: Self = Self(((1 << 13)) as i16);
    pub const SACTION_SHOW_MARKERS: Self = Self(((1 << 14)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSAction_Runtime_Flag(pub i8);

impl eSAction_Runtime_Flag {
    pub const SACTION_RUNTIME_FLAG_NEED_CHAN_SYNC: Self = Self(((1 << 0)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eAnimEdit_Context(pub i8);

impl eAnimEdit_Context {
    pub const SACTCONT_ACTION: Self = Self((0) as i8);
    pub const SACTCONT_SHAPEKEY: Self = Self((1) as i8);
    pub const SACTCONT_GPENCIL: Self = Self((2) as i8);
    pub const SACTCONT_DOPESHEET: Self = Self((3) as i8);
    pub const SACTCONT_MASK: Self = Self((4) as i8);
    pub const SACTCONT_CACHEFILE: Self = Self((5) as i8);
    pub const SACTCONT_TIMELINE: Self = Self((6) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eTimeline_Cache_Flag(pub u16);

impl eTimeline_Cache_Flag {
    pub const TIME_CACHE_DISPLAY: Self = Self(((1 << 0)) as u16);
    pub const TIME_CACHE_SOFTBODY: Self = Self(((1 << 1)) as u16);
    pub const TIME_CACHE_PARTICLES: Self = Self(((1 << 2)) as u16);
    pub const TIME_CACHE_CLOTH: Self = Self(((1 << 3)) as u16);
    pub const TIME_CACHE_SMOKE: Self = Self(((1 << 4)) as u16);
    pub const TIME_CACHE_DYNAMICPAINT: Self = Self(((1 << 5)) as u16);
    pub const TIME_CACHE_RIGIDBODY: Self = Self(((1 << 6)) as u16);
    pub const TIME_CACHE_SIMULATION_NODES: Self = Self(((1 << 7)) as u16);
    pub const TIME_CACHE_COMPOSITOR: Self = Self(((1 << 8)) as u16);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bMotionPathVert {
    pub co: [f32; 3],
}

impl Default for bMotionPathVert {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bMotionPath {
    pub points: *mut core::ffi::c_void,
    pub length: i32,
    pub start_frame: i32,
    pub end_frame: i32,
    pub color: [f32; 3],
}

impl Default for bMotionPath {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bAnimVizSettings {
    pub recalc: eAnimViz_RecalcFlags,
}

impl Default for bAnimVizSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bPoseChannelDrawData {
    pub solid_color: [f32; 4],
}

impl Default for bPoseChannelDrawData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bPoseChannel_BBoneSegmentBoundary {
    pub point: [f32; 3],
}

impl Default for bPoseChannel_BBoneSegmentBoundary {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bPoseChannel_Runtime {
    pub session_uid: SessionUID,
    pub bone_index: i64,
    pub deform_dual_quat: DualQuat,
    pub bbone_segments: i32,
    pub bbone_arc_length_reciprocal: f32,
    pub flag: bPoseChannelRuntimeFlag,
}

impl Default for bPoseChannel_Runtime {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bPoseChannel {
    pub prop: *mut core::ffi::c_void,
    pub system_properties: *mut core::ffi::c_void,
    pub constraints: ListBaseT<bConstraint>,
    pub nullptr: ListBaseT<bConstraint>,
}

impl Default for bPoseChannel {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bPose {
    pub chanbase: ListBaseT<bPoseChannel>,
    pub nullptr: ListBaseT<bPoseChannel>,
}

impl Default for bPose {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bIKParam {
    pub iksolver: ePose_IKSolverType,
}

impl Default for bIKParam {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bItasc {
    pub iksolver: ePose_IKSolverType,
}

impl Default for bItasc {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bActionGroup {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub channels: ListBaseT<FCurve>,
    pub nullptr: ListBaseT<FCurve>,
}

impl Default for bActionGroup {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bAction {
    pub layer_array: *mut core::ffi::c_void,
    pub layer_array_num: i32,
    pub layer_active_index: i32,
    pub slot_array: *mut core::ffi::c_void,
    pub slot_array_num: i32,
    pub last_slot_handle: i32,
    pub strip_keyframe_data_array: *mut core::ffi::c_void,
    pub strip_keyframe_data_array_num: i32,
    pub _pad0: [u8; 4],
}

impl Default for bAction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bDopeSheet {
    pub source: *mut core::ffi::c_void,
    pub chanbase: ListBase,
    pub nullptr: ListBase,
}

impl Default for bDopeSheet {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SpaceAction_Runtime {
    pub flag: eSAction_Runtime_Flag,
}

impl Default for SpaceAction_Runtime {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SpaceActionOverlays {
    pub flag: SpaceActionOverlays_Flag,
}

impl Default for SpaceActionOverlays {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SpaceAction {
    pub regionbase: ListBaseT<ARegion>,
    pub nullptr: ListBaseT<ARegion>,
}

impl Default for SpaceAction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ActionLayer {
    pub name: [u8; 64],
    pub influence: f32,
    pub layer_flags: u8,
    pub layer_mix_mode: i8,
    pub _pad0: [u8; 2],
}

impl Default for ActionLayer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ActionSlot {
    pub identifier: [u8; 258],
    pub idtype: i16,
    pub handle: i32,
    pub slot_flags: i8,
    pub _pad1: [u8; 7],
}

impl Default for ActionSlot {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ActionStrip {
    pub strip_type: i8,
    pub _pad0: [u8; 3],
}

impl Default for ActionStrip {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ActionStripKeyframeData {
    pub channelbag_array: *mut core::ffi::c_void,
    pub channelbag_array_num: i32,
    pub _pad: [u8; 4],
}

impl Default for ActionStripKeyframeData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ActionChannelbag {
    pub slot_handle: i32,
    pub group_array_num: i32,
    pub group_array: *mut core::ffi::c_void,
    pub _pad: [u8; 4],
}

impl Default for ActionChannelbag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

