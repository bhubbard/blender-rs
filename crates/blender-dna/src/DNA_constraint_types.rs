//! Auto-transpiled C/C++ header module: DNA_constraint_types

use core::ffi::c_void;
use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bConstraint {
    pub next: *mut bConstraint,
    pub data: *mut core::ffi::c_void,
    pub r#type: eBConstraint_Types,
    pub flag: eBConstraint_Flags,
    pub ownspace: eBConstraint_SpaceTypes,
    pub tarspace: eBConstraint_SpaceTypes,
    pub ui_expand_flag: i16,
    pub space_object: *mut Object,
    pub space_subtarget: [i8; 64],
    pub name: [i8; 64],
    pub enforce: f32,
    pub headtail: f32,
    pub lin_error: f32,
    pub rot_error: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bConstraintTarget {
    pub next: *mut bConstraintTarget,
    pub tar: *mut Object,
    pub subtarget: [i8; 64],
    pub matrix: [[f32; 4]; 4],
    pub space: eBConstraint_SpaceTypes,
    pub _pad1: i8,
    pub flag: eConstraintTargetFlag,
    pub r#type: eConstraintObType,
    pub rotOrder: i16,
    pub weight: f32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bKinematicConstraint {
    pub tar: *mut Object,
    pub iterations: i16,
    pub flag: eKinematic_Flags,
    pub rootbone: i16,
    pub max_rootbone: i16,
    pub subtarget: [i8; 64],
    pub poletar: *mut Object,
    pub polesubtarget: [i8; 64],
    pub poleangle: f32,
    pub weight: f32,
    pub orientweight: f32,
    pub grabtarget: [f32; 3],
    pub r#type: eConstraint_IK_Type,
    pub mode: eDistLimit_Modes,
    pub dist: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bSplineIKConstraint {
    pub tar: *mut Object,
    pub points: *mut f32,
    pub numpoints: i16,
    pub chainlen: i16,
    pub flag: eSplineIK_Flags,
    pub xzScaleMode: eSplineIK_XZScaleModes,
    pub yScaleMode: eSplineIK_YScaleModes,
    pub _pad: [i16; 3],
    pub bulge: f32,
    pub bulge_min: f32,
    pub bulge_max: f32,
    pub bulge_smooth: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bArmatureConstraint {
    pub flag: eArmature_Flags,
    pub _pad: [i8; 4],
    pub targets: ListBaseT<bConstraintTarget>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bTrackToConstraint {
    pub tar: *mut Object,
    pub reserved1: eTrackToAxis_Modes,
    pub reserved2: eUpAxis_Modes,
    pub flags: eTrackTo_Flags,
    pub _pad: [i8; 4],
    pub subtarget: [i8; 64],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bRotateLikeConstraint {
    pub tar: *mut Object,
    pub flag: eCopyRotation_Flags,
    pub euler_order: eConstraint_EulerOrder,
    pub mix_mode: eCopyRotation_MixMode,
    pub _pad: [i8; 2],
    pub subtarget: [i8; 64],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bLocateLikeConstraint {
    pub tar: *mut Object,
    pub flag: eCopyLocation_Flags,
    pub reserved1: i32,
    pub subtarget: [i8; 64],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bSizeLikeConstraint {
    pub tar: *mut Object,
    pub flag: eCopyScale_Flags,
    pub power: f32,
    pub subtarget: [i8; 64],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bSameVolumeConstraint {
    pub free_axis: eSameVolume_Axis,
    pub mode: eSameVolume_Mode,
    pub _pad: [i8; 2],
    pub volume: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bTransLikeConstraint {
    pub tar: *mut Object,
    pub flag: eCopyTransforms_Flags,
    pub mix_mode: eCopyTransforms_MixMode,
    pub _pad: [i8; 3],
    pub subtarget: [i8; 64],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bMinMaxConstraint {
    pub tar: *mut Object,
    pub minmaxflag: i32,
    pub offset: f32,
    pub flag: eFloor_Flags,
    pub subtarget: [i8; 64],
    pub _pad: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bActionConstraint {
    pub tar: *mut Object,
    pub r#type: i16,
    pub local: i16,
    pub start: i32,
    pub end: i32,
    pub min: f32,
    pub max: f32,
    pub flag: eActionConstraint_Flags,
    pub mix_mode: eActionConstraint_MixMode,
    pub _pad: [i8; 3],
    pub eval_time: f32,
    pub act: *mut bAction,
    pub action_slot_handle: i32,
    pub last_slot_identifier: [i8; 258],
    pub _pad1: [i8; 2],
    pub subtarget: [i8; 64],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bLockTrackConstraint {
    pub tar: *mut Object,
    pub trackflag: eTrackToAxis_Modes,
    pub lockflag: eLockAxis_Modes,
    pub subtarget: [i8; 64],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bDampTrackConstraint {
    pub tar: *mut Object,
    pub trackflag: eTrackToAxis_Modes,
    pub _pad: [i8; 4],
    pub subtarget: [i8; 64],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bFollowPathConstraint {
    pub tar: *mut Object,
    pub offset: f32,
    pub offset_fac: f32,
    pub followflag: eFollowPath_Flags,
    pub trackflag: i16,
    pub upflag: i16,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bStretchToConstraint {
    pub tar: *mut Object,
    pub flag: eStretchTo_Flags,
    pub volmode: eStretchTo_VolMode,
    pub plane: eStretchTo_PlaneMode,
    pub orglength: f32,
    pub bulge: f32,
    pub bulge_min: f32,
    pub bulge_max: f32,
    pub bulge_smooth: f32,
    pub subtarget: [i8; 64],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bRigidBodyJointConstraint {
    pub tar: *mut Object,
    pub child: *mut Object,
    pub r#type: i32,
    pub pivX: f32,
    pub pivY: f32,
    pub pivZ: f32,
    pub axX: f32,
    pub axY: f32,
    pub axZ: f32,
    pub minLimit: [f32; 6],
    pub maxLimit: [f32; 6],
    pub extraFz: f32,
    pub flag: i16,
    pub _pad: [i8; 6],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bClampToConstraint {
    pub tar: *mut Object,
    pub flag: eClampTo_Modes,
    pub flag2: eClampTo_Flags,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bChildOfConstraint {
    pub tar: *mut Object,
    pub _pad: [i8; 4],
    pub invmat: [[f32; 4]; 4],
    pub subtarget: [i8; 64],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bTransformConstraint {
    pub tar: *mut Object,
    pub subtarget: [i8; 64],
    pub from: eTransform_ToFrom,
    pub map: [i8; 3],
    pub expo: i8,
    pub from_rotation_mode: i8,
    pub to_euler_order: eConstraint_EulerOrder,
    pub mix_mode_loc: eTransform_MixModeLoc,
    pub mix_mode_rot: eTransform_MixModeRot,
    pub mix_mode_scale: eTransform_MixModeScale,
    pub _pad: [i8; 3],
    pub from_min: [f32; 3],
    pub from_max: [f32; 3],
    pub to_min: [f32; 3],
    pub to_max: [f32; 3],
    pub from_min_rot: [f32; 3],
    pub from_max_rot: [f32; 3],
    pub to_min_rot: [f32; 3],
    pub to_max_rot: [f32; 3],
    pub from_min_scale: [f32; 3],
    pub from_max_scale: [f32; 3],
    pub to_min_scale: [f32; 3],
    pub to_max_scale: [f32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bPivotConstraint {
    pub tar: *mut Object,
    pub subtarget: [i8; 64],
    pub offset: [f32; 3],
    pub rotAxis: ePivotConstraint_Axis,
    pub flag: ePivotConstraint_Flag,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bLocLimitConstraint {
    pub xmin: f32,
    pub ymin: f32,
    pub zmin: f32,
    pub flag: eTransformLimits_Flags,
    pub flag2: eTransformLimits_Flags2,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bRotLimitConstraint {
    pub xmin: f32,
    pub ymin: f32,
    pub zmin: f32,
    pub flag: eRotLimit_Flags,
    pub flag2: eTransformLimits_Flags2,
    pub euler_order: eConstraint_EulerOrder,
    pub _pad: [i8; 3],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bSizeLimitConstraint {
    pub xmin: f32,
    pub ymin: f32,
    pub zmin: f32,
    pub flag: eTransformLimits_Flags,
    pub flag2: eTransformLimits_Flags2,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bDistLimitConstraint {
    pub tar: *mut Object,
    pub subtarget: [i8; 64],
    pub dist: f32,
    pub soft: f32,
    pub flag: eDistLimit_Flag,
    pub mode: eDistLimit_Modes,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bShrinkwrapConstraint {
    pub target: *mut Object,
    pub dist: f32,
    pub shrinkType: i16,
    pub projAxis: i8,
    pub projAxisSpace: i8,
    pub projLimit: f32,
    pub shrinkMode: i8,
    pub flag: eShrinkwrap_Flags,
    pub trackAxis: i8,
    pub _pad: i8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bFollowTrackConstraint {
    pub clip: *mut MovieClip,
    pub track: [i8; 64],
    pub flag: eFollowTrack_Flags,
    pub frame_method: eFollowTrack_FrameMethod,
    pub object: [i8; 64],
    pub camera: *mut Object,
    pub depth_ob: *mut Object,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bCameraSolverConstraint {
    pub clip: *mut MovieClip,
    pub flag: eCameraSolver_Flags,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bObjectSolverConstraint {
    pub clip: *mut MovieClip,
    pub flag: eObjectSolver_Flags,
    pub _pad: [i8; 4],
    pub object: [i8; 64],
    pub invmat: [[f32; 4]; 4],
    pub camera: *mut Object,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bTransformCacheConstraint {
    pub cache_file: *mut CacheFile,
    pub object_path: [i8; 1024],
    pub reader: *mut CacheReader,
    pub reader_object_path: [i8; 1024],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bGeometryAttributeConstraint {
    pub target: *mut Object,
    pub attribute_name: *mut i8,
    pub sample_index: i32,
    pub apply_target_transform: u8,
    pub mix_mode: Attribute_MixMode,
    pub domain: Attribute_Domain,
    pub data_type: Attribute_Data_Type,
    pub flags: eGeometryAttributeConstraint_Flags,
    pub _pad0: [i8; 7],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Text {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Object {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bAction {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MovieClip {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CacheFile {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CacheReader {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eConstraintTargetFlag {
    CONSTRAINT_TAR_TEMP = (1 << 0),
    CONSTRAINT_TAR_CUSTOM_SPACE = (1 << 1),
}

impl Default for eConstraintTargetFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CONSTRAINT_TAR_TEMP: i32 = eConstraintTargetFlag::CONSTRAINT_TAR_TEMP as i32;
pub const CONSTRAINT_TAR_CUSTOM_SPACE: i32 = eConstraintTargetFlag::CONSTRAINT_TAR_CUSTOM_SPACE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eConstraintObType {
    CONSTRAINT_OBTYPE_OBJECT = 1,
    CONSTRAINT_OBTYPE_BONE = 2,
    CONSTRAINT_OBTYPE_VERT = 3,
}

impl Default for eConstraintObType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CONSTRAINT_OBTYPE_OBJECT: i32 = eConstraintObType::CONSTRAINT_OBTYPE_OBJECT as i32;
pub const CONSTRAINT_OBTYPE_BONE: i32 = eConstraintObType::CONSTRAINT_OBTYPE_BONE as i32;
pub const CONSTRAINT_OBTYPE_VERT: i32 = eConstraintObType::CONSTRAINT_OBTYPE_VERT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eConstraint_IK_Type {
    CONSTRAINT_IK_COPYPOSE = 0,
    CONSTRAINT_IK_DISTANCE = 1,
}

impl Default for eConstraint_IK_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CONSTRAINT_IK_COPYPOSE: i32 = eConstraint_IK_Type::CONSTRAINT_IK_COPYPOSE as i32;
pub const CONSTRAINT_IK_DISTANCE: i32 = eConstraint_IK_Type::CONSTRAINT_IK_DISTANCE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGeometryAttributeConstraint_Flags {
    APPLY_TARGET_TRANSFORM = (1 << 0),
    MIX_LOC = (1 << 1),
    MIX_ROT = (1 << 2),
    MIX_SCALE = (1 << 3),
}

impl Default for eGeometryAttributeConstraint_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const APPLY_TARGET_TRANSFORM: i32 = eGeometryAttributeConstraint_Flags::APPLY_TARGET_TRANSFORM as i32;
pub const MIX_LOC: i32 = eGeometryAttributeConstraint_Flags::MIX_LOC as i32;
pub const MIX_ROT: i32 = eGeometryAttributeConstraint_Flags::MIX_ROT as i32;
pub const MIX_SCALE: i32 = eGeometryAttributeConstraint_Flags::MIX_SCALE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attribute_Domain {
    CON_ATTRIBUTE_DOMAIN_POINT = 0,
    CON_ATTRIBUTE_DOMAIN_EDGE = 1,
    CON_ATTRIBUTE_DOMAIN_FACE = 2,
    CON_ATTRIBUTE_DOMAIN_FACE_CORNER = 3,
    CON_ATTRIBUTE_DOMAIN_CURVE = 4,
    CON_ATTRIBUTE_DOMAIN_INSTANCE = 5,
}

impl Default for Attribute_Domain {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CON_ATTRIBUTE_DOMAIN_POINT: i32 = Attribute_Domain::CON_ATTRIBUTE_DOMAIN_POINT as i32;
pub const CON_ATTRIBUTE_DOMAIN_EDGE: i32 = Attribute_Domain::CON_ATTRIBUTE_DOMAIN_EDGE as i32;
pub const CON_ATTRIBUTE_DOMAIN_FACE: i32 = Attribute_Domain::CON_ATTRIBUTE_DOMAIN_FACE as i32;
pub const CON_ATTRIBUTE_DOMAIN_FACE_CORNER: i32 = Attribute_Domain::CON_ATTRIBUTE_DOMAIN_FACE_CORNER as i32;
pub const CON_ATTRIBUTE_DOMAIN_CURVE: i32 = Attribute_Domain::CON_ATTRIBUTE_DOMAIN_CURVE as i32;
pub const CON_ATTRIBUTE_DOMAIN_INSTANCE: i32 = Attribute_Domain::CON_ATTRIBUTE_DOMAIN_INSTANCE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attribute_Data_Type {
    CON_ATTRIBUTE_VECTOR = 0,
    CON_ATTRIBUTE_QUATERNION = 1,
    CON_ATTRIBUTE_4X4MATRIX = 2,
}

impl Default for Attribute_Data_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CON_ATTRIBUTE_VECTOR: i32 = Attribute_Data_Type::CON_ATTRIBUTE_VECTOR as i32;
pub const CON_ATTRIBUTE_QUATERNION: i32 = Attribute_Data_Type::CON_ATTRIBUTE_QUATERNION as i32;
pub const CON_ATTRIBUTE_4X4MATRIX: i32 = Attribute_Data_Type::CON_ATTRIBUTE_4X4MATRIX as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attribute_MixMode {
    CON_ATTRIBUTE_MIX_REPLACE = 0,
    CON_ATTRIBUTE_MIX_BEFORE_SPLIT = 1,
    CON_ATTRIBUTE_MIX_AFTER_SPLIT = 2,
    CON_ATTRIBUTE_MIX_BEFORE_FULL = 3,
    CON_ATTRIBUTE_MIX_AFTER_FULL = 4,
}

impl Default for Attribute_MixMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CON_ATTRIBUTE_MIX_REPLACE: i32 = Attribute_MixMode::CON_ATTRIBUTE_MIX_REPLACE as i32;
pub const CON_ATTRIBUTE_MIX_BEFORE_SPLIT: i32 = Attribute_MixMode::CON_ATTRIBUTE_MIX_BEFORE_SPLIT as i32;
pub const CON_ATTRIBUTE_MIX_AFTER_SPLIT: i32 = Attribute_MixMode::CON_ATTRIBUTE_MIX_AFTER_SPLIT as i32;
pub const CON_ATTRIBUTE_MIX_BEFORE_FULL: i32 = Attribute_MixMode::CON_ATTRIBUTE_MIX_BEFORE_FULL as i32;
pub const CON_ATTRIBUTE_MIX_AFTER_FULL: i32 = Attribute_MixMode::CON_ATTRIBUTE_MIX_AFTER_FULL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBConstraint_Types {
    CONSTRAINT_TYPE_NULL = 0,
    CONSTRAINT_TYPE_CHILDOF = 1,
    CONSTRAINT_TYPE_TRACKTO = 2,
    CONSTRAINT_TYPE_KINEMATIC = 3,
    CONSTRAINT_TYPE_FOLLOWPATH = 4,
    CONSTRAINT_TYPE_ROTLIMIT = 5,
    CONSTRAINT_TYPE_LOCLIMIT = 6,
    CONSTRAINT_TYPE_SIZELIMIT = 7,
    CONSTRAINT_TYPE_ROTLIKE = 8,
    CONSTRAINT_TYPE_LOCLIKE = 9,
    CONSTRAINT_TYPE_SIZELIKE = 10,
    CONSTRAINT_TYPE_ACTION = 12,
    CONSTRAINT_TYPE_LOCKTRACK = 13,
    CONSTRAINT_TYPE_DISTLIMIT = 14,
    CONSTRAINT_TYPE_STRETCHTO = 15,
    CONSTRAINT_TYPE_MINMAX = 16,
    CONSTRAINT_TYPE_CLAMPTO = 18,
    CONSTRAINT_TYPE_TRANSFORM = 19,
    CONSTRAINT_TYPE_SHRINKWRAP = 20,
    CONSTRAINT_TYPE_DAMPTRACK = 21,
    CONSTRAINT_TYPE_SPLINEIK = 22,
    CONSTRAINT_TYPE_TRANSLIKE = 23,
    CONSTRAINT_TYPE_SAMEVOL = 24,
    CONSTRAINT_TYPE_PIVOT = 25,
    CONSTRAINT_TYPE_FOLLOWTRACK = 26,
    CONSTRAINT_TYPE_CAMERASOLVER = 27,
    CONSTRAINT_TYPE_OBJECTSOLVER = 28,
    CONSTRAINT_TYPE_TRANSFORM_CACHE = 29,
    CONSTRAINT_TYPE_ARMATURE = 30,
    CONSTRAINT_TYPE_GEOMETRY_ATTRIBUTE = 31,
    NUM_CONSTRAINT_TYPES,
}

impl Default for eBConstraint_Types {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CONSTRAINT_TYPE_NULL: i32 = eBConstraint_Types::CONSTRAINT_TYPE_NULL as i32;
pub const CONSTRAINT_TYPE_CHILDOF: i32 = eBConstraint_Types::CONSTRAINT_TYPE_CHILDOF as i32;
pub const CONSTRAINT_TYPE_TRACKTO: i32 = eBConstraint_Types::CONSTRAINT_TYPE_TRACKTO as i32;
pub const CONSTRAINT_TYPE_KINEMATIC: i32 = eBConstraint_Types::CONSTRAINT_TYPE_KINEMATIC as i32;
pub const CONSTRAINT_TYPE_FOLLOWPATH: i32 = eBConstraint_Types::CONSTRAINT_TYPE_FOLLOWPATH as i32;
pub const CONSTRAINT_TYPE_ROTLIMIT: i32 = eBConstraint_Types::CONSTRAINT_TYPE_ROTLIMIT as i32;
pub const CONSTRAINT_TYPE_LOCLIMIT: i32 = eBConstraint_Types::CONSTRAINT_TYPE_LOCLIMIT as i32;
pub const CONSTRAINT_TYPE_SIZELIMIT: i32 = eBConstraint_Types::CONSTRAINT_TYPE_SIZELIMIT as i32;
pub const CONSTRAINT_TYPE_ROTLIKE: i32 = eBConstraint_Types::CONSTRAINT_TYPE_ROTLIKE as i32;
pub const CONSTRAINT_TYPE_LOCLIKE: i32 = eBConstraint_Types::CONSTRAINT_TYPE_LOCLIKE as i32;
pub const CONSTRAINT_TYPE_SIZELIKE: i32 = eBConstraint_Types::CONSTRAINT_TYPE_SIZELIKE as i32;
pub const CONSTRAINT_TYPE_ACTION: i32 = eBConstraint_Types::CONSTRAINT_TYPE_ACTION as i32;
pub const CONSTRAINT_TYPE_LOCKTRACK: i32 = eBConstraint_Types::CONSTRAINT_TYPE_LOCKTRACK as i32;
pub const CONSTRAINT_TYPE_DISTLIMIT: i32 = eBConstraint_Types::CONSTRAINT_TYPE_DISTLIMIT as i32;
pub const CONSTRAINT_TYPE_STRETCHTO: i32 = eBConstraint_Types::CONSTRAINT_TYPE_STRETCHTO as i32;
pub const CONSTRAINT_TYPE_MINMAX: i32 = eBConstraint_Types::CONSTRAINT_TYPE_MINMAX as i32;
pub const CONSTRAINT_TYPE_CLAMPTO: i32 = eBConstraint_Types::CONSTRAINT_TYPE_CLAMPTO as i32;
pub const CONSTRAINT_TYPE_TRANSFORM: i32 = eBConstraint_Types::CONSTRAINT_TYPE_TRANSFORM as i32;
pub const CONSTRAINT_TYPE_SHRINKWRAP: i32 = eBConstraint_Types::CONSTRAINT_TYPE_SHRINKWRAP as i32;
pub const CONSTRAINT_TYPE_DAMPTRACK: i32 = eBConstraint_Types::CONSTRAINT_TYPE_DAMPTRACK as i32;
pub const CONSTRAINT_TYPE_SPLINEIK: i32 = eBConstraint_Types::CONSTRAINT_TYPE_SPLINEIK as i32;
pub const CONSTRAINT_TYPE_TRANSLIKE: i32 = eBConstraint_Types::CONSTRAINT_TYPE_TRANSLIKE as i32;
pub const CONSTRAINT_TYPE_SAMEVOL: i32 = eBConstraint_Types::CONSTRAINT_TYPE_SAMEVOL as i32;
pub const CONSTRAINT_TYPE_PIVOT: i32 = eBConstraint_Types::CONSTRAINT_TYPE_PIVOT as i32;
pub const CONSTRAINT_TYPE_FOLLOWTRACK: i32 = eBConstraint_Types::CONSTRAINT_TYPE_FOLLOWTRACK as i32;
pub const CONSTRAINT_TYPE_CAMERASOLVER: i32 = eBConstraint_Types::CONSTRAINT_TYPE_CAMERASOLVER as i32;
pub const CONSTRAINT_TYPE_OBJECTSOLVER: i32 = eBConstraint_Types::CONSTRAINT_TYPE_OBJECTSOLVER as i32;
pub const CONSTRAINT_TYPE_TRANSFORM_CACHE: i32 = eBConstraint_Types::CONSTRAINT_TYPE_TRANSFORM_CACHE as i32;
pub const CONSTRAINT_TYPE_ARMATURE: i32 = eBConstraint_Types::CONSTRAINT_TYPE_ARMATURE as i32;
pub const CONSTRAINT_TYPE_GEOMETRY_ATTRIBUTE: i32 = eBConstraint_Types::CONSTRAINT_TYPE_GEOMETRY_ATTRIBUTE as i32;
pub const NUM_CONSTRAINT_TYPES: i32 = eBConstraint_Types::NUM_CONSTRAINT_TYPES as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBConstraint_Flags {
    CONSTRAINT_EXPAND_DEPRECATED = (1 << 0),
    CONSTRAINT_DISABLE = (1 << 2),
    CONSTRAINT_ACTIVE = (1 << 4),
    CONSTRAINT_SPACEONCE = (1 << 6),
    CONSTRAINT_OWN_IPO = (1 << 7),
    CONSTRAINT_OFF = (1 << 9),
    CONSTRAINT_BBONE_SHAPE = (1 << 10),
    CONSTRAINT_OVERRIDE_LIBRARY_LOCAL = (1 << 11),
    CONSTRAINT_BBONE_SHAPE_FULL = (1 << 12),
}

impl Default for eBConstraint_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CONSTRAINT_EXPAND_DEPRECATED: i32 = eBConstraint_Flags::CONSTRAINT_EXPAND_DEPRECATED as i32;
pub const CONSTRAINT_DISABLE: i32 = eBConstraint_Flags::CONSTRAINT_DISABLE as i32;
pub const CONSTRAINT_ACTIVE: i32 = eBConstraint_Flags::CONSTRAINT_ACTIVE as i32;
pub const CONSTRAINT_SPACEONCE: i32 = eBConstraint_Flags::CONSTRAINT_SPACEONCE as i32;
pub const CONSTRAINT_OWN_IPO: i32 = eBConstraint_Flags::CONSTRAINT_OWN_IPO as i32;
pub const CONSTRAINT_OFF: i32 = eBConstraint_Flags::CONSTRAINT_OFF as i32;
pub const CONSTRAINT_BBONE_SHAPE: i32 = eBConstraint_Flags::CONSTRAINT_BBONE_SHAPE as i32;
pub const CONSTRAINT_OVERRIDE_LIBRARY_LOCAL: i32 = eBConstraint_Flags::CONSTRAINT_OVERRIDE_LIBRARY_LOCAL as i32;
pub const CONSTRAINT_BBONE_SHAPE_FULL: i32 = eBConstraint_Flags::CONSTRAINT_BBONE_SHAPE_FULL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBConstraint_SpaceTypes {
    CONSTRAINT_SPACE_WORLD = 0,
    CONSTRAINT_SPACE_CUSTOM = 5,
    CONSTRAINT_SPACE_LOCAL = 1,
    CONSTRAINT_SPACE_POSE = 2,
    CONSTRAINT_SPACE_PARLOCAL = 3,
    CONSTRAINT_SPACE_OWNLOCAL = 6,
    CONSTRAINT_SPACE_INVALID = 4,
}

impl Default for eBConstraint_SpaceTypes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CONSTRAINT_SPACE_WORLD: i32 = eBConstraint_SpaceTypes::CONSTRAINT_SPACE_WORLD as i32;
pub const CONSTRAINT_SPACE_CUSTOM: i32 = eBConstraint_SpaceTypes::CONSTRAINT_SPACE_CUSTOM as i32;
pub const CONSTRAINT_SPACE_LOCAL: i32 = eBConstraint_SpaceTypes::CONSTRAINT_SPACE_LOCAL as i32;
pub const CONSTRAINT_SPACE_POSE: i32 = eBConstraint_SpaceTypes::CONSTRAINT_SPACE_POSE as i32;
pub const CONSTRAINT_SPACE_PARLOCAL: i32 = eBConstraint_SpaceTypes::CONSTRAINT_SPACE_PARLOCAL as i32;
pub const CONSTRAINT_SPACE_OWNLOCAL: i32 = eBConstraint_SpaceTypes::CONSTRAINT_SPACE_OWNLOCAL as i32;
pub const CONSTRAINT_SPACE_INVALID: i32 = eBConstraint_SpaceTypes::CONSTRAINT_SPACE_INVALID as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eConstraint_EulerOrder {
    CONSTRAINT_EULER_AUTO = 0,
    CONSTRAINT_EULER_XYZ = 1,
    CONSTRAINT_EULER_XZY = 2,
    CONSTRAINT_EULER_YXZ = 3,
    CONSTRAINT_EULER_YZX = 4,
    CONSTRAINT_EULER_ZXY = 5,
    CONSTRAINT_EULER_ZYX = 6,
}

impl Default for eConstraint_EulerOrder {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CONSTRAINT_EULER_AUTO: i32 = eConstraint_EulerOrder::CONSTRAINT_EULER_AUTO as i32;
pub const CONSTRAINT_EULER_XYZ: i32 = eConstraint_EulerOrder::CONSTRAINT_EULER_XYZ as i32;
pub const CONSTRAINT_EULER_XZY: i32 = eConstraint_EulerOrder::CONSTRAINT_EULER_XZY as i32;
pub const CONSTRAINT_EULER_YXZ: i32 = eConstraint_EulerOrder::CONSTRAINT_EULER_YXZ as i32;
pub const CONSTRAINT_EULER_YZX: i32 = eConstraint_EulerOrder::CONSTRAINT_EULER_YZX as i32;
pub const CONSTRAINT_EULER_ZXY: i32 = eConstraint_EulerOrder::CONSTRAINT_EULER_ZXY as i32;
pub const CONSTRAINT_EULER_ZYX: i32 = eConstraint_EulerOrder::CONSTRAINT_EULER_ZYX as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCopyRotation_Flags {
    ROTLIKE_X = (1 << 0),
    ROTLIKE_Y = (1 << 1),
    ROTLIKE_Z = (1 << 2),
    ROTLIKE_X_INVERT = (1 << 4),
    ROTLIKE_Y_INVERT = (1 << 5),
    ROTLIKE_Z_INVERT = (1 << 6),
    ROTLIKE_OFFSET = (1 << 7),
}

impl Default for eCopyRotation_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const ROTLIKE_X: i32 = eCopyRotation_Flags::ROTLIKE_X as i32;
pub const ROTLIKE_Y: i32 = eCopyRotation_Flags::ROTLIKE_Y as i32;
pub const ROTLIKE_Z: i32 = eCopyRotation_Flags::ROTLIKE_Z as i32;
pub const ROTLIKE_X_INVERT: i32 = eCopyRotation_Flags::ROTLIKE_X_INVERT as i32;
pub const ROTLIKE_Y_INVERT: i32 = eCopyRotation_Flags::ROTLIKE_Y_INVERT as i32;
pub const ROTLIKE_Z_INVERT: i32 = eCopyRotation_Flags::ROTLIKE_Z_INVERT as i32;
pub const ROTLIKE_OFFSET: i32 = eCopyRotation_Flags::ROTLIKE_OFFSET as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCopyRotation_MixMode {
    ROTLIKE_MIX_REPLACE = 0,
    ROTLIKE_MIX_OFFSET = 1,
    ROTLIKE_MIX_ADD = 2,
    ROTLIKE_MIX_BEFORE = 3,
    ROTLIKE_MIX_AFTER = 4,
}

impl Default for eCopyRotation_MixMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const ROTLIKE_MIX_REPLACE: i32 = eCopyRotation_MixMode::ROTLIKE_MIX_REPLACE as i32;
pub const ROTLIKE_MIX_OFFSET: i32 = eCopyRotation_MixMode::ROTLIKE_MIX_OFFSET as i32;
pub const ROTLIKE_MIX_ADD: i32 = eCopyRotation_MixMode::ROTLIKE_MIX_ADD as i32;
pub const ROTLIKE_MIX_BEFORE: i32 = eCopyRotation_MixMode::ROTLIKE_MIX_BEFORE as i32;
pub const ROTLIKE_MIX_AFTER: i32 = eCopyRotation_MixMode::ROTLIKE_MIX_AFTER as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCopyLocation_Flags {
    LOCLIKE_X = (1 << 0),
    LOCLIKE_Y = (1 << 1),
    LOCLIKE_Z = (1 << 2),
    LOCLIKE_TIP = (1 << 3),
    LOCLIKE_X_INVERT = (1 << 4),
    LOCLIKE_Y_INVERT = (1 << 5),
    LOCLIKE_Z_INVERT = (1 << 6),
    LOCLIKE_OFFSET = (1 << 7),
}

impl Default for eCopyLocation_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LOCLIKE_X: i32 = eCopyLocation_Flags::LOCLIKE_X as i32;
pub const LOCLIKE_Y: i32 = eCopyLocation_Flags::LOCLIKE_Y as i32;
pub const LOCLIKE_Z: i32 = eCopyLocation_Flags::LOCLIKE_Z as i32;
pub const LOCLIKE_TIP: i32 = eCopyLocation_Flags::LOCLIKE_TIP as i32;
pub const LOCLIKE_X_INVERT: i32 = eCopyLocation_Flags::LOCLIKE_X_INVERT as i32;
pub const LOCLIKE_Y_INVERT: i32 = eCopyLocation_Flags::LOCLIKE_Y_INVERT as i32;
pub const LOCLIKE_Z_INVERT: i32 = eCopyLocation_Flags::LOCLIKE_Z_INVERT as i32;
pub const LOCLIKE_OFFSET: i32 = eCopyLocation_Flags::LOCLIKE_OFFSET as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCopyScale_Flags {
    SIZELIKE_X = (1 << 0),
    SIZELIKE_Y = (1 << 1),
    SIZELIKE_Z = (1 << 2),
    SIZELIKE_OFFSET = (1 << 3),
    SIZELIKE_MULTIPLY = (1 << 4),
    SIZELIKE_UNIFORM = (1 << 5),
}

impl Default for eCopyScale_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const SIZELIKE_X: i32 = eCopyScale_Flags::SIZELIKE_X as i32;
pub const SIZELIKE_Y: i32 = eCopyScale_Flags::SIZELIKE_Y as i32;
pub const SIZELIKE_Z: i32 = eCopyScale_Flags::SIZELIKE_Z as i32;
pub const SIZELIKE_OFFSET: i32 = eCopyScale_Flags::SIZELIKE_OFFSET as i32;
pub const SIZELIKE_MULTIPLY: i32 = eCopyScale_Flags::SIZELIKE_MULTIPLY as i32;
pub const SIZELIKE_UNIFORM: i32 = eCopyScale_Flags::SIZELIKE_UNIFORM as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCopyTransforms_Flags {
    TRANSLIKE_REMOVE_TARGET_SHEAR = (1 << 0),
}

impl Default for eCopyTransforms_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TRANSLIKE_REMOVE_TARGET_SHEAR: i32 = eCopyTransforms_Flags::TRANSLIKE_REMOVE_TARGET_SHEAR as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCopyTransforms_MixMode {
    TRANSLIKE_MIX_REPLACE = 0,
    TRANSLIKE_MIX_BEFORE = 1,
    TRANSLIKE_MIX_AFTER = 2,
    TRANSLIKE_MIX_BEFORE_SPLIT = 3,
    TRANSLIKE_MIX_AFTER_SPLIT = 4,
    TRANSLIKE_MIX_BEFORE_FULL = 5,
    TRANSLIKE_MIX_AFTER_FULL = 6,
}

impl Default for eCopyTransforms_MixMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TRANSLIKE_MIX_REPLACE: i32 = eCopyTransforms_MixMode::TRANSLIKE_MIX_REPLACE as i32;
pub const TRANSLIKE_MIX_BEFORE: i32 = eCopyTransforms_MixMode::TRANSLIKE_MIX_BEFORE as i32;
pub const TRANSLIKE_MIX_AFTER: i32 = eCopyTransforms_MixMode::TRANSLIKE_MIX_AFTER as i32;
pub const TRANSLIKE_MIX_BEFORE_SPLIT: i32 = eCopyTransforms_MixMode::TRANSLIKE_MIX_BEFORE_SPLIT as i32;
pub const TRANSLIKE_MIX_AFTER_SPLIT: i32 = eCopyTransforms_MixMode::TRANSLIKE_MIX_AFTER_SPLIT as i32;
pub const TRANSLIKE_MIX_BEFORE_FULL: i32 = eCopyTransforms_MixMode::TRANSLIKE_MIX_BEFORE_FULL as i32;
pub const TRANSLIKE_MIX_AFTER_FULL: i32 = eCopyTransforms_MixMode::TRANSLIKE_MIX_AFTER_FULL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTransform_ToFrom {
    TRANS_LOCATION = 0,
    TRANS_ROTATION = 1,
    TRANS_SCALE = 2,
}

impl Default for eTransform_ToFrom {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TRANS_LOCATION: i32 = eTransform_ToFrom::TRANS_LOCATION as i32;
pub const TRANS_ROTATION: i32 = eTransform_ToFrom::TRANS_ROTATION as i32;
pub const TRANS_SCALE: i32 = eTransform_ToFrom::TRANS_SCALE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTransform_MixModeLoc {
    TRANS_MIXLOC_ADD = 0,
    TRANS_MIXLOC_REPLACE = 1,
}

impl Default for eTransform_MixModeLoc {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TRANS_MIXLOC_ADD: i32 = eTransform_MixModeLoc::TRANS_MIXLOC_ADD as i32;
pub const TRANS_MIXLOC_REPLACE: i32 = eTransform_MixModeLoc::TRANS_MIXLOC_REPLACE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTransform_MixModeRot {
    TRANS_MIXROT_ADD = 0,
    TRANS_MIXROT_REPLACE = 1,
    TRANS_MIXROT_BEFORE = 2,
    TRANS_MIXROT_AFTER = 3,
}

impl Default for eTransform_MixModeRot {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TRANS_MIXROT_ADD: i32 = eTransform_MixModeRot::TRANS_MIXROT_ADD as i32;
pub const TRANS_MIXROT_REPLACE: i32 = eTransform_MixModeRot::TRANS_MIXROT_REPLACE as i32;
pub const TRANS_MIXROT_BEFORE: i32 = eTransform_MixModeRot::TRANS_MIXROT_BEFORE as i32;
pub const TRANS_MIXROT_AFTER: i32 = eTransform_MixModeRot::TRANS_MIXROT_AFTER as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTransform_MixModeScale {
    TRANS_MIXSCALE_REPLACE = 0,
    TRANS_MIXSCALE_MULTIPLY = 1,
}

impl Default for eTransform_MixModeScale {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TRANS_MIXSCALE_REPLACE: i32 = eTransform_MixModeScale::TRANS_MIXSCALE_REPLACE as i32;
pub const TRANS_MIXSCALE_MULTIPLY: i32 = eTransform_MixModeScale::TRANS_MIXSCALE_MULTIPLY as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eSameVolume_Axis {
    SAMEVOL_X = 0,
    SAMEVOL_Y = 1,
    SAMEVOL_Z = 2,
}

impl Default for eSameVolume_Axis {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const SAMEVOL_X: i32 = eSameVolume_Axis::SAMEVOL_X as i32;
pub const SAMEVOL_Y: i32 = eSameVolume_Axis::SAMEVOL_Y as i32;
pub const SAMEVOL_Z: i32 = eSameVolume_Axis::SAMEVOL_Z as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eSameVolume_Mode {
    SAMEVOL_STRICT = 0,
    SAMEVOL_UNIFORM = 1,
    SAMEVOL_SINGLE_AXIS = 2,
}

impl Default for eSameVolume_Mode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const SAMEVOL_STRICT: i32 = eSameVolume_Mode::SAMEVOL_STRICT as i32;
pub const SAMEVOL_UNIFORM: i32 = eSameVolume_Mode::SAMEVOL_UNIFORM as i32;
pub const SAMEVOL_SINGLE_AXIS: i32 = eSameVolume_Mode::SAMEVOL_SINGLE_AXIS as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eActionConstraint_Flags {
    ACTCON_BONE_USE_OBJECT_ACTION = (1 << 0),
    ACTCON_USE_EVAL_TIME = (1 << 1),
}

impl Default for eActionConstraint_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const ACTCON_BONE_USE_OBJECT_ACTION: i32 = eActionConstraint_Flags::ACTCON_BONE_USE_OBJECT_ACTION as i32;
pub const ACTCON_USE_EVAL_TIME: i32 = eActionConstraint_Flags::ACTCON_USE_EVAL_TIME as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eActionConstraint_MixMode {
    ACTCON_MIX_REPLACE = 6,
    ACTCON_MIX_AFTER_FULL = 0,
    ACTCON_MIX_BEFORE_FULL = 3,
    ACTCON_MIX_AFTER = 1,
    ACTCON_MIX_BEFORE = 2,
    ACTCON_MIX_AFTER_SPLIT = 4,
    ACTCON_MIX_BEFORE_SPLIT = 5,
}

impl Default for eActionConstraint_MixMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const ACTCON_MIX_REPLACE: i32 = eActionConstraint_MixMode::ACTCON_MIX_REPLACE as i32;
pub const ACTCON_MIX_AFTER_FULL: i32 = eActionConstraint_MixMode::ACTCON_MIX_AFTER_FULL as i32;
pub const ACTCON_MIX_BEFORE_FULL: i32 = eActionConstraint_MixMode::ACTCON_MIX_BEFORE_FULL as i32;
pub const ACTCON_MIX_AFTER: i32 = eActionConstraint_MixMode::ACTCON_MIX_AFTER as i32;
pub const ACTCON_MIX_BEFORE: i32 = eActionConstraint_MixMode::ACTCON_MIX_BEFORE as i32;
pub const ACTCON_MIX_AFTER_SPLIT: i32 = eActionConstraint_MixMode::ACTCON_MIX_AFTER_SPLIT as i32;
pub const ACTCON_MIX_BEFORE_SPLIT: i32 = eActionConstraint_MixMode::ACTCON_MIX_BEFORE_SPLIT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLockAxis_Modes {
    LOCK_X = 0,
    LOCK_Y = 1,
    LOCK_Z = 2,
}

impl Default for eLockAxis_Modes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LOCK_X: i32 = eLockAxis_Modes::LOCK_X as i32;
pub const LOCK_Y: i32 = eLockAxis_Modes::LOCK_Y as i32;
pub const LOCK_Z: i32 = eLockAxis_Modes::LOCK_Z as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUpAxis_Modes {
    UP_X = 0,
    UP_Y = 1,
    UP_Z = 2,
}

impl Default for eUpAxis_Modes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const UP_X: i32 = eUpAxis_Modes::UP_X as i32;
pub const UP_Y: i32 = eUpAxis_Modes::UP_Y as i32;
pub const UP_Z: i32 = eUpAxis_Modes::UP_Z as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTrackToAxis_Modes {
    TRACK_X = 0,
    TRACK_Y = 1,
    TRACK_Z = 2,
    TRACK_nX = 3,
    TRACK_nY = 4,
    TRACK_nZ = 5,
}

impl Default for eTrackToAxis_Modes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TRACK_X: i32 = eTrackToAxis_Modes::TRACK_X as i32;
pub const TRACK_Y: i32 = eTrackToAxis_Modes::TRACK_Y as i32;
pub const TRACK_Z: i32 = eTrackToAxis_Modes::TRACK_Z as i32;
pub const TRACK_nX: i32 = eTrackToAxis_Modes::TRACK_nX as i32;
pub const TRACK_nY: i32 = eTrackToAxis_Modes::TRACK_nY as i32;
pub const TRACK_nZ: i32 = eTrackToAxis_Modes::TRACK_nZ as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eShrinkwrap_Flags {
    CON_SHRINKWRAP_PROJECT_OPPOSITE = (1 << 0),
    CON_SHRINKWRAP_PROJECT_INVERT_CULL = (1 << 1),
    CON_SHRINKWRAP_TRACK_NORMAL = (1 << 2),
    CON_SHRINKWRAP_PROJECT_CULL_FRONTFACE = (1 << 3),
    CON_SHRINKWRAP_PROJECT_CULL_BACKFACE = (1 << 4),
}

impl Default for eShrinkwrap_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CON_SHRINKWRAP_PROJECT_OPPOSITE: i32 = eShrinkwrap_Flags::CON_SHRINKWRAP_PROJECT_OPPOSITE as i32;
pub const CON_SHRINKWRAP_PROJECT_INVERT_CULL: i32 = eShrinkwrap_Flags::CON_SHRINKWRAP_PROJECT_INVERT_CULL as i32;
pub const CON_SHRINKWRAP_TRACK_NORMAL: i32 = eShrinkwrap_Flags::CON_SHRINKWRAP_TRACK_NORMAL as i32;
pub const CON_SHRINKWRAP_PROJECT_CULL_FRONTFACE: i32 = eShrinkwrap_Flags::CON_SHRINKWRAP_PROJECT_CULL_FRONTFACE as i32;
pub const CON_SHRINKWRAP_PROJECT_CULL_BACKFACE: i32 = eShrinkwrap_Flags::CON_SHRINKWRAP_PROJECT_CULL_BACKFACE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFollowPath_Flags {
    FOLLOWPATH_FOLLOW = (1 << 0),
    FOLLOWPATH_STATIC = (1 << 1),
    FOLLOWPATH_RADIUS = (1 << 2),
}

impl Default for eFollowPath_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FOLLOWPATH_FOLLOW: i32 = eFollowPath_Flags::FOLLOWPATH_FOLLOW as i32;
pub const FOLLOWPATH_STATIC: i32 = eFollowPath_Flags::FOLLOWPATH_STATIC as i32;
pub const FOLLOWPATH_RADIUS: i32 = eFollowPath_Flags::FOLLOWPATH_RADIUS as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTrackTo_Flags {
    TARGET_Z_UP = (1 << 0),
}

impl Default for eTrackTo_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TARGET_Z_UP: i32 = eTrackTo_Flags::TARGET_Z_UP as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eStretchTo_VolMode {
    VOLUME_XZ = 0,
    VOLUME_X = 1,
    VOLUME_Z = 2,
    NO_VOLUME = 3,
}

impl Default for eStretchTo_VolMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const VOLUME_XZ: i32 = eStretchTo_VolMode::VOLUME_XZ as i32;
pub const VOLUME_X: i32 = eStretchTo_VolMode::VOLUME_X as i32;
pub const VOLUME_Z: i32 = eStretchTo_VolMode::VOLUME_Z as i32;
pub const NO_VOLUME: i32 = eStretchTo_VolMode::NO_VOLUME as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eStretchTo_PlaneMode {
    PLANE_X = 0,
    SWING_Y = 1,
    PLANE_Z = 2,
}

impl Default for eStretchTo_PlaneMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const PLANE_X: i32 = eStretchTo_PlaneMode::PLANE_X as i32;
pub const SWING_Y: i32 = eStretchTo_PlaneMode::SWING_Y as i32;
pub const PLANE_Z: i32 = eStretchTo_PlaneMode::PLANE_Z as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eClampTo_Modes {
    CLAMPTO_AUTO = 0,
    CLAMPTO_X = 1,
    CLAMPTO_Y = 2,
    CLAMPTO_Z = 3,
}

impl Default for eClampTo_Modes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CLAMPTO_AUTO: i32 = eClampTo_Modes::CLAMPTO_AUTO as i32;
pub const CLAMPTO_X: i32 = eClampTo_Modes::CLAMPTO_X as i32;
pub const CLAMPTO_Y: i32 = eClampTo_Modes::CLAMPTO_Y as i32;
pub const CLAMPTO_Z: i32 = eClampTo_Modes::CLAMPTO_Z as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eClampTo_Flags {
    CLAMPTO_CYCLIC = (1 << 0),
}

impl Default for eClampTo_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CLAMPTO_CYCLIC: i32 = eClampTo_Flags::CLAMPTO_CYCLIC as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eKinematic_Flags {
    CONSTRAINT_IK_TIP = (1 << 0),
    CONSTRAINT_IK_ROT = (1 << 1),
    CONSTRAINT_IK_AUTO = (1 << 2),
    CONSTRAINT_IK_TEMP = (1 << 3),
    CONSTRAINT_IK_STRETCH = (1 << 4),
    CONSTRAINT_IK_POS = (1 << 5),
    CONSTRAINT_IK_SETANGLE = (1 << 6),
    CONSTRAINT_IK_GETANGLE = (1 << 7),
    CONSTRAINT_IK_NO_POS_X = (1 << 8),
    CONSTRAINT_IK_NO_POS_Y = (1 << 9),
    CONSTRAINT_IK_NO_POS_Z = (1 << 10),
    CONSTRAINT_IK_NO_ROT_X = (1 << 11),
    CONSTRAINT_IK_NO_ROT_Y = (1 << 12),
    CONSTRAINT_IK_NO_ROT_Z = (1 << 13),
    CONSTRAINT_IK_TARGETAXIS = (1 << 14),
}

impl Default for eKinematic_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CONSTRAINT_IK_TIP: i32 = eKinematic_Flags::CONSTRAINT_IK_TIP as i32;
pub const CONSTRAINT_IK_ROT: i32 = eKinematic_Flags::CONSTRAINT_IK_ROT as i32;
pub const CONSTRAINT_IK_AUTO: i32 = eKinematic_Flags::CONSTRAINT_IK_AUTO as i32;
pub const CONSTRAINT_IK_TEMP: i32 = eKinematic_Flags::CONSTRAINT_IK_TEMP as i32;
pub const CONSTRAINT_IK_STRETCH: i32 = eKinematic_Flags::CONSTRAINT_IK_STRETCH as i32;
pub const CONSTRAINT_IK_POS: i32 = eKinematic_Flags::CONSTRAINT_IK_POS as i32;
pub const CONSTRAINT_IK_SETANGLE: i32 = eKinematic_Flags::CONSTRAINT_IK_SETANGLE as i32;
pub const CONSTRAINT_IK_GETANGLE: i32 = eKinematic_Flags::CONSTRAINT_IK_GETANGLE as i32;
pub const CONSTRAINT_IK_NO_POS_X: i32 = eKinematic_Flags::CONSTRAINT_IK_NO_POS_X as i32;
pub const CONSTRAINT_IK_NO_POS_Y: i32 = eKinematic_Flags::CONSTRAINT_IK_NO_POS_Y as i32;
pub const CONSTRAINT_IK_NO_POS_Z: i32 = eKinematic_Flags::CONSTRAINT_IK_NO_POS_Z as i32;
pub const CONSTRAINT_IK_NO_ROT_X: i32 = eKinematic_Flags::CONSTRAINT_IK_NO_ROT_X as i32;
pub const CONSTRAINT_IK_NO_ROT_Y: i32 = eKinematic_Flags::CONSTRAINT_IK_NO_ROT_Y as i32;
pub const CONSTRAINT_IK_NO_ROT_Z: i32 = eKinematic_Flags::CONSTRAINT_IK_NO_ROT_Z as i32;
pub const CONSTRAINT_IK_TARGETAXIS: i32 = eKinematic_Flags::CONSTRAINT_IK_TARGETAXIS as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eSplineIK_Flags {
    CONSTRAINT_SPLINEIK_BOUND = (1 << 0),
    CONSTRAINT_SPLINEIK_NO_ROOT = (1 << 1),
    CONSTRAINT_SPLINEIK_SCALE_LIMITED = (1 << 2),
    CONSTRAINT_SPLINEIK_EVENSPLITS = (1 << 3),
    CONSTRAINT_SPLINEIK_NO_CURVERAD = (1 << 4),
    CONSTRAINT_SPLINEIK_USE_BULGE_MIN = (1 << 5),
    CONSTRAINT_SPLINEIK_USE_BULGE_MAX = (1 << 6),
    CONSTRAINT_SPLINEIK_USE_ORIGINAL_SCALE = (1 << 7),
}

impl Default for eSplineIK_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CONSTRAINT_SPLINEIK_BOUND: i32 = eSplineIK_Flags::CONSTRAINT_SPLINEIK_BOUND as i32;
pub const CONSTRAINT_SPLINEIK_NO_ROOT: i32 = eSplineIK_Flags::CONSTRAINT_SPLINEIK_NO_ROOT as i32;
pub const CONSTRAINT_SPLINEIK_SCALE_LIMITED: i32 = eSplineIK_Flags::CONSTRAINT_SPLINEIK_SCALE_LIMITED as i32;
pub const CONSTRAINT_SPLINEIK_EVENSPLITS: i32 = eSplineIK_Flags::CONSTRAINT_SPLINEIK_EVENSPLITS as i32;
pub const CONSTRAINT_SPLINEIK_NO_CURVERAD: i32 = eSplineIK_Flags::CONSTRAINT_SPLINEIK_NO_CURVERAD as i32;
pub const CONSTRAINT_SPLINEIK_USE_BULGE_MIN: i32 = eSplineIK_Flags::CONSTRAINT_SPLINEIK_USE_BULGE_MIN as i32;
pub const CONSTRAINT_SPLINEIK_USE_BULGE_MAX: i32 = eSplineIK_Flags::CONSTRAINT_SPLINEIK_USE_BULGE_MAX as i32;
pub const CONSTRAINT_SPLINEIK_USE_ORIGINAL_SCALE: i32 = eSplineIK_Flags::CONSTRAINT_SPLINEIK_USE_ORIGINAL_SCALE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eSplineIK_XZScaleModes {
    CONSTRAINT_SPLINEIK_XZS_NONE = 0,
    CONSTRAINT_SPLINEIK_XZS_ORIGINAL = 1,
    CONSTRAINT_SPLINEIK_XZS_INVERSE = 2,
    CONSTRAINT_SPLINEIK_XZS_VOLUMETRIC = 3,
}

impl Default for eSplineIK_XZScaleModes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CONSTRAINT_SPLINEIK_XZS_NONE: i32 = eSplineIK_XZScaleModes::CONSTRAINT_SPLINEIK_XZS_NONE as i32;
pub const CONSTRAINT_SPLINEIK_XZS_ORIGINAL: i32 = eSplineIK_XZScaleModes::CONSTRAINT_SPLINEIK_XZS_ORIGINAL as i32;
pub const CONSTRAINT_SPLINEIK_XZS_INVERSE: i32 = eSplineIK_XZScaleModes::CONSTRAINT_SPLINEIK_XZS_INVERSE as i32;
pub const CONSTRAINT_SPLINEIK_XZS_VOLUMETRIC: i32 = eSplineIK_XZScaleModes::CONSTRAINT_SPLINEIK_XZS_VOLUMETRIC as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eSplineIK_YScaleModes {
    CONSTRAINT_SPLINEIK_YS_NONE = 0,
    CONSTRAINT_SPLINEIK_YS_FIT_CURVE = 1,
    CONSTRAINT_SPLINEIK_YS_ORIGINAL = 2,
}

impl Default for eSplineIK_YScaleModes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CONSTRAINT_SPLINEIK_YS_NONE: i32 = eSplineIK_YScaleModes::CONSTRAINT_SPLINEIK_YS_NONE as i32;
pub const CONSTRAINT_SPLINEIK_YS_FIT_CURVE: i32 = eSplineIK_YScaleModes::CONSTRAINT_SPLINEIK_YS_FIT_CURVE as i32;
pub const CONSTRAINT_SPLINEIK_YS_ORIGINAL: i32 = eSplineIK_YScaleModes::CONSTRAINT_SPLINEIK_YS_ORIGINAL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eArmature_Flags {
    CONSTRAINT_ARMATURE_QUATERNION = (1 << 0),
    CONSTRAINT_ARMATURE_ENVELOPE = (1 << 1),
    CONSTRAINT_ARMATURE_CUR_LOCATION = (1 << 2),
}

impl Default for eArmature_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CONSTRAINT_ARMATURE_QUATERNION: i32 = eArmature_Flags::CONSTRAINT_ARMATURE_QUATERNION as i32;
pub const CONSTRAINT_ARMATURE_ENVELOPE: i32 = eArmature_Flags::CONSTRAINT_ARMATURE_ENVELOPE as i32;
pub const CONSTRAINT_ARMATURE_CUR_LOCATION: i32 = eArmature_Flags::CONSTRAINT_ARMATURE_CUR_LOCATION as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFloor_Flags {
    MINMAX_USEROT = (1 << 2),
}

impl Default for eFloor_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MINMAX_USEROT: i32 = eFloor_Flags::MINMAX_USEROT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTransformLimits_Flags2 {
    LIMIT_TRANSFORM = (1 << 1),
}

impl Default for eTransformLimits_Flags2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LIMIT_TRANSFORM: i32 = eTransformLimits_Flags2::LIMIT_TRANSFORM as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTransformLimits_Flags {
    LIMIT_XMIN = (1 << 0),
    LIMIT_XMAX = (1 << 1),
    LIMIT_YMIN = (1 << 2),
    LIMIT_YMAX = (1 << 3),
    LIMIT_ZMIN = (1 << 4),
    LIMIT_ZMAX = (1 << 5),
}

impl Default for eTransformLimits_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LIMIT_XMIN: i32 = eTransformLimits_Flags::LIMIT_XMIN as i32;
pub const LIMIT_XMAX: i32 = eTransformLimits_Flags::LIMIT_XMAX as i32;
pub const LIMIT_YMIN: i32 = eTransformLimits_Flags::LIMIT_YMIN as i32;
pub const LIMIT_YMAX: i32 = eTransformLimits_Flags::LIMIT_YMAX as i32;
pub const LIMIT_ZMIN: i32 = eTransformLimits_Flags::LIMIT_ZMIN as i32;
pub const LIMIT_ZMAX: i32 = eTransformLimits_Flags::LIMIT_ZMAX as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eRotLimit_Flags {
    LIMIT_XROT = (1 << 0),
    LIMIT_YROT = (1 << 1),
    LIMIT_ZROT = (1 << 2),
    LIMIT_ROT_LEGACY_BEHAVIOR = (1 << 3),
}

impl Default for eRotLimit_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LIMIT_XROT: i32 = eRotLimit_Flags::LIMIT_XROT as i32;
pub const LIMIT_YROT: i32 = eRotLimit_Flags::LIMIT_YROT as i32;
pub const LIMIT_ZROT: i32 = eRotLimit_Flags::LIMIT_ZROT as i32;
pub const LIMIT_ROT_LEGACY_BEHAVIOR: i32 = eRotLimit_Flags::LIMIT_ROT_LEGACY_BEHAVIOR as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDistLimit_Flag {
    LIMITDIST_USESOFT = (1 << 0),
    LIMITDIST_TRANSFORM = (1 << 1),
}

impl Default for eDistLimit_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LIMITDIST_USESOFT: i32 = eDistLimit_Flag::LIMITDIST_USESOFT as i32;
pub const LIMITDIST_TRANSFORM: i32 = eDistLimit_Flag::LIMITDIST_TRANSFORM as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDistLimit_Modes {
    LIMITDIST_INSIDE = 0,
    LIMITDIST_OUTSIDE = 1,
    LIMITDIST_ONSURFACE = 2,
}

impl Default for eDistLimit_Modes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LIMITDIST_INSIDE: i32 = eDistLimit_Modes::LIMITDIST_INSIDE as i32;
pub const LIMITDIST_OUTSIDE: i32 = eDistLimit_Modes::LIMITDIST_OUTSIDE as i32;
pub const LIMITDIST_ONSURFACE: i32 = eDistLimit_Modes::LIMITDIST_ONSURFACE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eChildOf_Flags {
    CHILDOF_LOCX = (1 << 0),
    CHILDOF_LOCY = (1 << 1),
    CHILDOF_LOCZ = (1 << 2),
    CHILDOF_ROTX = (1 << 3),
    CHILDOF_ROTY = (1 << 4),
    CHILDOF_ROTZ = (1 << 5),
    CHILDOF_SIZEX = (1 << 6),
    CHILDOF_SIZEY = (1 << 7),
    CHILDOF_SIZEZ = (1 << 8),
    CHILDOF_ALL = 511,
    CHILDOF_SET_INVERSE = (1 << 9),
}

impl Default for eChildOf_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CHILDOF_LOCX: i32 = eChildOf_Flags::CHILDOF_LOCX as i32;
pub const CHILDOF_LOCY: i32 = eChildOf_Flags::CHILDOF_LOCY as i32;
pub const CHILDOF_LOCZ: i32 = eChildOf_Flags::CHILDOF_LOCZ as i32;
pub const CHILDOF_ROTX: i32 = eChildOf_Flags::CHILDOF_ROTX as i32;
pub const CHILDOF_ROTY: i32 = eChildOf_Flags::CHILDOF_ROTY as i32;
pub const CHILDOF_ROTZ: i32 = eChildOf_Flags::CHILDOF_ROTZ as i32;
pub const CHILDOF_SIZEX: i32 = eChildOf_Flags::CHILDOF_SIZEX as i32;
pub const CHILDOF_SIZEY: i32 = eChildOf_Flags::CHILDOF_SIZEY as i32;
pub const CHILDOF_SIZEZ: i32 = eChildOf_Flags::CHILDOF_SIZEZ as i32;
pub const CHILDOF_ALL: i32 = eChildOf_Flags::CHILDOF_ALL as i32;
pub const CHILDOF_SET_INVERSE: i32 = eChildOf_Flags::CHILDOF_SET_INVERSE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ePivotConstraint_Axis {
    PIVOTCON_AXIS_NONE = -1,
    PIVOTCON_AXIS_X_NEG = 0,
    PIVOTCON_AXIS_Y_NEG = 1,
    PIVOTCON_AXIS_Z_NEG = 2,
    PIVOTCON_AXIS_X = 3,
    PIVOTCON_AXIS_Y = 4,
    PIVOTCON_AXIS_Z = 5,
}

impl Default for ePivotConstraint_Axis {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const PIVOTCON_AXIS_NONE: i32 = ePivotConstraint_Axis::PIVOTCON_AXIS_NONE as i32;
pub const PIVOTCON_AXIS_X_NEG: i32 = ePivotConstraint_Axis::PIVOTCON_AXIS_X_NEG as i32;
pub const PIVOTCON_AXIS_Y_NEG: i32 = ePivotConstraint_Axis::PIVOTCON_AXIS_Y_NEG as i32;
pub const PIVOTCON_AXIS_Z_NEG: i32 = ePivotConstraint_Axis::PIVOTCON_AXIS_Z_NEG as i32;
pub const PIVOTCON_AXIS_X: i32 = ePivotConstraint_Axis::PIVOTCON_AXIS_X as i32;
pub const PIVOTCON_AXIS_Y: i32 = ePivotConstraint_Axis::PIVOTCON_AXIS_Y as i32;
pub const PIVOTCON_AXIS_Z: i32 = ePivotConstraint_Axis::PIVOTCON_AXIS_Z as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ePivotConstraint_Flag {
    PIVOTCON_FLAG_OFFSET_ABS = (1 << 0),
    PIVOTCON_FLAG_ROTACT_NEG = (1 << 1),
}

impl Default for ePivotConstraint_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const PIVOTCON_FLAG_OFFSET_ABS: i32 = ePivotConstraint_Flag::PIVOTCON_FLAG_OFFSET_ABS as i32;
pub const PIVOTCON_FLAG_ROTACT_NEG: i32 = ePivotConstraint_Flag::PIVOTCON_FLAG_ROTACT_NEG as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFollowTrack_Flags {
    FOLLOWTRACK_ACTIVECLIP = (1 << 0),
    FOLLOWTRACK_USE_3D_POSITION = (1 << 1),
    FOLLOWTRACK_USE_UNDISTORTION = (1 << 2),
}

impl Default for eFollowTrack_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FOLLOWTRACK_ACTIVECLIP: i32 = eFollowTrack_Flags::FOLLOWTRACK_ACTIVECLIP as i32;
pub const FOLLOWTRACK_USE_3D_POSITION: i32 = eFollowTrack_Flags::FOLLOWTRACK_USE_3D_POSITION as i32;
pub const FOLLOWTRACK_USE_UNDISTORTION: i32 = eFollowTrack_Flags::FOLLOWTRACK_USE_UNDISTORTION as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFollowTrack_FrameMethod {
    FOLLOWTRACK_FRAME_STRETCH = 0,
    FOLLOWTRACK_FRAME_FIT = 1,
    FOLLOWTRACK_FRAME_CROP = 2,
}

impl Default for eFollowTrack_FrameMethod {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FOLLOWTRACK_FRAME_STRETCH: i32 = eFollowTrack_FrameMethod::FOLLOWTRACK_FRAME_STRETCH as i32;
pub const FOLLOWTRACK_FRAME_FIT: i32 = eFollowTrack_FrameMethod::FOLLOWTRACK_FRAME_FIT as i32;
pub const FOLLOWTRACK_FRAME_CROP: i32 = eFollowTrack_FrameMethod::FOLLOWTRACK_FRAME_CROP as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCameraSolver_Flags {
    CAMERASOLVER_ACTIVECLIP = (1 << 0),
}

impl Default for eCameraSolver_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CAMERASOLVER_ACTIVECLIP: i32 = eCameraSolver_Flags::CAMERASOLVER_ACTIVECLIP as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eObjectSolver_Flags {
    OBJECTSOLVER_ACTIVECLIP = (1 << 0),
    OBJECTSOLVER_SET_INVERSE = (1 << 1),
}

impl Default for eObjectSolver_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const OBJECTSOLVER_ACTIVECLIP: i32 = eObjectSolver_Flags::OBJECTSOLVER_ACTIVECLIP as i32;
pub const OBJECTSOLVER_SET_INVERSE: i32 = eObjectSolver_Flags::OBJECTSOLVER_SET_INVERSE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eStretchTo_Flags {
    STRETCHTOCON_USE_BULGE_MIN = (1 << 0),
    STRETCHTOCON_USE_BULGE_MAX = (1 << 1),
}

impl Default for eStretchTo_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const STRETCHTOCON_USE_BULGE_MIN: i32 = eStretchTo_Flags::STRETCHTOCON_USE_BULGE_MIN as i32;
pub const STRETCHTOCON_USE_BULGE_MAX: i32 = eStretchTo_Flags::STRETCHTOCON_USE_BULGE_MAX as i32;

