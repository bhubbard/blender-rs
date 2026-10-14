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
pub struct ShrinkwrapBoundaryVertData {
    pub direction: [f32; 3],
    pub normal_plane: [f32; 3],
}

impl Default for ShrinkwrapBoundaryVertData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ShrinkwrapTreeData {
    pub mesh: *mut core::ffi::c_void,
    pub bvh: *mut core::ffi::c_void,
    pub treeData: bke::BVHTreeFromMesh,
    pub vert_tree: *mut core::ffi::c_void,
    pub faces: OffsetIndices<int>,
    pub edges: Span<int2>,
    pub corner_edges: Span<int>,
    pub face_normals: Span<float3>,
    pub vert_normals: Span<float3>,
    pub corner_normals: Span<float3>,
    pub sharp_faces: VArraySpan<bool>,
    pub boundary: *mut core::ffi::c_void,
}

impl Default for ShrinkwrapTreeData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ShrinkwrapParams {
    pub target: *mut core::ffi::c_void,
    pub aux_target: *mut core::ffi::c_void,
    pub invert_vertex_weights: bool,
    pub keep_distance: f32,
    pub shrink_type: i16,
    pub shrink_options: i8,
    pub shrink_mode: i8,
    pub projection_limit: f32,
    pub projection_axis: i8,
    pub subsurf_levels: i8,
}

impl Default for ShrinkwrapParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

