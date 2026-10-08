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
pub struct eEffectorWeight_Flag(pub i16);

impl eEffectorWeight_Flag {
    pub const EFF_WEIGHT_DO_HAIR: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePField_Flag(pub i32);

impl ePField_Flag {
    pub const PFIELD_USEMAX: Self = Self((1 << 0) as i32);
    pub const PFIELD_GUIDE_PATH_ADD: Self = Self((1 << 2) as i32);
    pub const PFIELD_PLANAR: Self = Self((1 << 3) as i32);
    pub const PDEFLE_KILL_PART: Self = Self((1 << 4) as i32);
    pub const PFIELD_POSZ: Self = Self((1 << 5) as i32);
    pub const PFIELD_TEX_OBJECT: Self = Self((1 << 6) as i32);
    pub const PFIELD_GLOBAL_CO: Self = Self((1 << 6) as i32);
    pub const PFIELD_TEX_2D: Self = Self((1 << 7) as i32);
    pub const PFIELD_MULTIPLE_SPRINGS: Self = Self((1 << 7) as i32);
    pub const PFIELD_USEMIN: Self = Self((1 << 8) as i32);
    pub const PFIELD_USEMAXR: Self = Self((1 << 9) as i32);
    pub const PFIELD_USEMINR: Self = Self((1 << 10) as i32);
    pub const PFIELD_TEX_ROOTCO: Self = Self((1 << 11) as i32);
    pub const PFIELD_SURFACE: Self = Self((1 << 12) as i32);
    pub const PFIELD_VISIBILITY: Self = Self((1 << 13) as i32);
    pub const PFIELD_DO_LOCATION: Self = Self((1 << 14) as i32);
    pub const PFIELD_DO_ROTATION: Self = Self((1 << 15) as i32);
    pub const PFIELD_GUIDE_PATH_WEIGHT: Self = Self((1 << 16) as i32);
    pub const PFIELD_SMOKE_DENSITY: Self = Self((1 << 17) as i32);
    pub const PFIELD_GRAVITATION: Self = Self((1 << 18) as i32);
    pub const PFIELD_CLOTH_USE_CULLING: Self = Self((1 << 19) as i32);
    pub const PFIELD_CLOTH_USE_NORMAL: Self = Self((1 << 20) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePField_Falloff(pub i16);

impl ePField_Falloff {
    pub const PFIELD_FALL_SPHERE: Self = Self((0) as i16);
    pub const PFIELD_FALL_TUBE: Self = Self((1) as i16);
    pub const PFIELD_FALL_CONE: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePField_Shape(pub i16);

impl ePField_Shape {
    pub const PFIELD_SHAPE_POINT: Self = Self((0) as i16);
    pub const PFIELD_SHAPE_PLANE: Self = Self((1) as i16);
    pub const PFIELD_SHAPE_SURFACE: Self = Self((2) as i16);
    pub const PFIELD_SHAPE_POINTS: Self = Self((3) as i16);
    pub const PFIELD_SHAPE_LINE: Self = Self((4) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePField_TexMode(pub i16);

impl ePField_TexMode {
    pub const PFIELD_TEX_RGB: Self = Self((0) as i16);
    pub const PFIELD_TEX_GRAD: Self = Self((1) as i16);
    pub const PFIELD_TEX_CURL: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePField_ZDir(pub i16);

impl ePField_ZDir {
    pub const PFIELD_Z_BOTH: Self = Self((0) as i16);
    pub const PFIELD_Z_POS: Self = Self((1) as i16);
    pub const PFIELD_Z_NEG: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSoftBody_Flag(pub i32);

impl eSoftBody_Flag {
    pub const OB_SB_ENABLE: Self = Self((1 << 0) as i32);
    pub const OB_SB_GOAL: Self = Self((1 << 1) as i32);
    pub const OB_SB_EDGES: Self = Self((1 << 2) as i32);
    pub const OB_SB_QUADS: Self = Self((1 << 3) as i32);
    pub const OB_SB_POSTDEF: Self = Self((1 << 4) as i32);
    pub const OB_SB_SELF: Self = Self((1 << 9) as i32);
    pub const OB_SB_FACECOLL: Self = Self((1 << 10) as i32);
    pub const OB_SB_EDGECOLL: Self = Self((1 << 11) as i32);
    pub const OB_SB_AERO_ANGLE: Self = Self((1 << 14) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSoftBody_SolverFlag(pub i8);

impl eSoftBody_SolverFlag {
    pub const SBSO_MONITOR: Self = Self((1 << 0) as i8);
    pub const SBSO_OLDERR: Self = Self((1 << 1) as i8);
    pub const SBSO_ESTIMATEIPO: Self = Self((1 << 2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSoftBody_Mode(pub i16);

impl eSoftBody_Mode {
    pub const SBC_MODE_MANUAL: Self = Self((0) as i16);
    pub const SBC_MODE_AVG: Self = Self((1) as i16);
    pub const SBC_MODE_MIN: Self = Self((2) as i16);
    pub const SBC_MODE_MAX: Self = Self((3) as i16);
    pub const SBC_MODE_AVGMINMAX: Self = Self((4) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePFieldType(pub i16);

impl ePFieldType {
    pub const PFIELD_NULL: Self = Self((0) as i16);
    pub const PFIELD_FORCE: Self = Self((1) as i16);
    pub const PFIELD_VORTEX: Self = Self((2) as i16);
    pub const PFIELD_MAGNET: Self = Self((3) as i16);
    pub const PFIELD_WIND: Self = Self((4) as i16);
    pub const PFIELD_GUIDE: Self = Self((5) as i16);
    pub const PFIELD_TEXTURE: Self = Self((6) as i16);
    pub const PFIELD_HARMONIC: Self = Self((7) as i16);
    pub const PFIELD_CHARGE: Self = Self((8) as i16);
    pub const PFIELD_LENNARDJ: Self = Self((9) as i16);
    pub const PFIELD_BOID: Self = Self((10) as i16);
    pub const PFIELD_TURBULENCE: Self = Self((11) as i16);
    pub const PFIELD_DRAG: Self = Self((12) as i16);
    pub const PFIELD_FLUIDFLOW: Self = Self((13) as i16);
    pub const NUM_PFIELD_TYPES: Self = Self(14 as i16);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct PartDeflect {
    pub flag: ePField_Flag,
}

impl Default for PartDeflect {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct EffectorWeights {
    pub group: *mut core::ffi::c_void,
    pub weight: [f32; 14],
}

impl Default for EffectorWeights {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SBVertex {
    pub vec: [f32; 4],
}

impl Default for SBVertex {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SoftBody_Shared {
    pub pointcache: *mut core::ffi::c_void,
    pub ptcaches: ListBaseT<PointCache>,
    pub nullptr: ListBaseT<PointCache>,
}

impl Default for SoftBody_Shared {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SoftBody {
    pub bpoint: *mut core::ffi::c_void,
    pub bspring: *mut core::ffi::c_void,
    pub _pad: i8,
}

impl Default for SoftBody {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

