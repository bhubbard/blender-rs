//! Auto-transpiled C/C++ header module: mesh_boolean_manifold
//! Source: upstream/source/blender/geometry/intern/mesh_boolean_manifold.hh

use blender_dna::DNA_mesh_types::Mesh;
use blender_math::Mat4;
use crate::GEO_mesh_boolean::{BooleanError, BooleanOpParameters};

extern "C" {
    pub fn mesh_boolean_manifold(
        meshes: *const *const Mesh,
        meshes_len: usize,
        transforms: *const Mat4,
        transforms_len: usize,
        material_remaps: *const i16,
        material_remaps_len: usize,
        op_params: BooleanOpParameters,
        r_intersecting_edges: *mut Vec<i32>,
        r_error: *mut BooleanError,
    ) -> *mut Mesh;
}
