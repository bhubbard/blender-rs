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
pub struct CurveType(pub i8);

impl CurveType {
    pub const CURVE_TYPE_CATMULL_ROM: Self = Self((0) as i8);
    pub const CURVE_TYPE_POLY: Self = Self((1) as i8);
    pub const CURVE_TYPE_BEZIER: Self = Self((2) as i8);
    pub const CURVE_TYPE_NURBS: Self = Self((3) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct HandleType(pub i8);

impl HandleType {
    pub const BEZIER_HANDLE_FREE: Self = Self((0) as i8);
    pub const BEZIER_HANDLE_AUTO: Self = Self((1) as i8);
    pub const BEZIER_HANDLE_VECTOR: Self = Self((2) as i8);
    pub const BEZIER_HANDLE_ALIGN: Self = Self((3) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct KnotsMode(pub i8);

impl KnotsMode {
    pub const NURBS_KNOT_MODE_NORMAL: Self = Self((0) as i8);
    pub const NURBS_KNOT_MODE_ENDPOINT: Self = Self((1) as i8);
    pub const NURBS_KNOT_MODE_BEZIER: Self = Self((2) as i8);
    pub const NURBS_KNOT_MODE_ENDPOINT_BEZIER: Self = Self((3) as i8);
    pub const NURBS_KNOT_MODE_CUSTOM: Self = Self((4) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NormalMode(pub i8);

impl NormalMode {
    pub const NORMAL_MODE_MINIMUM_TWIST: Self = Self((0) as i8);
    pub const NORMAL_MODE_Z_UP: Self = Self((1) as i8);
    pub const NORMAL_MODE_FREE: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCurves_Flag(pub i32);

impl eCurves_Flag {
    pub const HA_DS_EXPAND: Self = Self(((1 << 0)) as i32);
    pub const CV_SCULPT_COLLISION_ENABLED: Self = Self(((1 << 1)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCurvesSymmetryType(pub i8);

impl eCurvesSymmetryType {
    pub const CURVES_SYMMETRY_X: Self = Self((1 << 0) as i8);
    pub const CURVES_SYMMETRY_Y: Self = Self((1 << 1) as i8);
    pub const CURVES_SYMMETRY_Z: Self = Self((1 << 2) as i8);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CurvesGeometry {
    pub curve_offsets: *mut core::ffi::c_void,
    pub attribute_storage: AttributeStorage,
    pub point_data: CustomData,
    pub curve_data_legacy: CustomData,
    pub point_num: i32,
    pub curve_num: i32,
    pub vertex_group_names: ListBaseT<bDeformGroup>,
    pub nullptr: ListBaseT<bDeformGroup>,
}

impl Default for CurvesGeometry {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Curves {
    pub adt: *mut core::ffi::c_void,
    pub geometry: CurvesGeometry,
    pub flag: eCurves_Flag,
}

impl Default for Curves {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

