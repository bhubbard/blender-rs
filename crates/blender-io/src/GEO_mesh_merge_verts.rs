//! Auto-transpiled C/C++ header module: GEO_mesh_merge_verts

use core::ffi::c_void;
use blender_dna::Mesh;
use blender_math::cpp_types::IndexMask;

extern "C" {
    pub fn mesh_merge_by_distance_all(
        mesh: *const Mesh,
        selection: *const IndexMask,
        merge_distance: f32,
        attribute_filter: *const c_void,
    ) -> *mut Mesh;

    pub fn mesh_merge_by_distance_connected(
        mesh: *const Mesh,
        selection: *const bool,
        selection_len: usize,
        merge_distance: f32,
        only_loose_edges: bool,
        attribute_filter: *const c_void,
    ) -> *mut Mesh;
}
