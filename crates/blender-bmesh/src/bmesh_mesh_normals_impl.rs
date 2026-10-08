//! Auto-transpiled C/C++ header module: bmesh_mesh_normals_impl

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMVertsCalcNormalsWithCoordsData {
    pub fnos: Span<float3>,
    pub vcos: Span<float3>,
    pub vnos: MutableSpan<float3>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMLoopsCalcNormalsWithCoordsData {
    pub vcos: Span<float3>,
    pub fnos: Span<float3>,
    pub bm: *mut BMesh,
    pub cd_loop_clnors_offset: i32,
    pub do_rebuild: bool,
    pub split_angle_cos: f32,
    pub r_lnos: MutableSpan<float3>,
    pub r_lnors_spacearr: *mut MLoopNorSpaceArray,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMLoopsCalcNormalsWithCoords_TLS {
    pub lnors_spacearr: *mut MLoopNorSpaceArray,
    pub lnors_spacearr_buf: MLoopNorSpaceArray,
}
