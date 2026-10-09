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
#[allow(non_camel_case_types)]
type int = i32;
#[allow(non_camel_case_types)]
type UString = String;
#[allow(non_camel_case_types)]
type PropertyFlag = u32;
#[allow(non_camel_case_types)]
type PropertyOverrideFlag = u32;
#[allow(non_camel_case_types)]
type ParameterFlag = u32;

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct AddCurvesOnMeshInputs {
    pub uvs: Span<float2>,
    pub interpolate_length: bool,
    pub interpolate_radius: bool,
    pub interpolate_shape: bool,
    pub interpolate_point_count: bool,
    pub interpolate_resolution: bool,
    pub fallback_curve_length: f32,
    pub fallback_curve_radius: f32,
    pub fallback_point_count: i32,
    pub surface: *mut core::ffi::c_void,
    pub surface_corner_tris: Span<int3>,
    pub reverse_uv_sampler: *mut core::ffi::c_void,
    pub corner_normals_su: Span<float3>,
    pub transforms: *mut core::ffi::c_void,
    pub old_roots_kdtree: *mut core::ffi::c_void,
    pub r_uv_error: bool,
}

impl Default for AddCurvesOnMeshInputs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct AddCurvesOnMeshOutputs {
    pub uv_error: bool,
    pub new_curves_range: IndexRange,
    pub new_points_range: IndexRange,
}

impl Default for AddCurvesOnMeshOutputs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

