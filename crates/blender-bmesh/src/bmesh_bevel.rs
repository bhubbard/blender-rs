//! Auto-transpiled C/C++ header module: bmesh_bevel

use crate::bmesh_class::BMesh;
use blender_dna::DNA_meshdata_types::MDeformVert;
use blender_dna::CurveProfile;

extern "C" {
    pub fn BM_mesh_bevel(
        bm: *mut BMesh,
        offset: f32,
        offset_type: i32,
        profile_type: i32,
        segments: i32,
        profile: f32,
        affect_type: bool,
        use_weights: bool,
        limit_offset: bool,
        dvert: *const MDeformVert,
        vertex_group: i32,
        mat: i32,
        loop_slide: bool,
        mark_seam: bool,
        mark_sharp: bool,
        harden_normals: bool,
        face_strength_mode: i32,
        miter_outer: i32,
        miter_inner: i32,
        spread: f32,
        custom_profile: *const CurveProfile,
        vmesh_method: i32,
        bweight_offset_vert: i32,
        bweight_offset_edge: i32,
    );
}
