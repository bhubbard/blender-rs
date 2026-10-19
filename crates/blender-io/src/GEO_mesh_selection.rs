//! Auto-transpiled C/C++ header module: GEO_mesh_selection
//! Source: upstream/source/blender/geometry/GEO_mesh_selection.hh

use blender_math::cpp_types::IndexMask;

extern "C" {
    pub fn vert_selection_from_edge(
        edges: *const [i32; 2],
        edges_len: usize,
        edge_mask: *const IndexMask,
        verts_num: i32,
        memory: *mut core::ffi::c_void,
    ) -> IndexMask;

    pub fn vert_selection_from_face(
        faces_offsets: *const i32,
        faces_len: usize,
        face_mask: *const IndexMask,
        corner_verts: *const i32,
        corner_verts_len: usize,
        verts_num: i32,
        memory: *mut core::ffi::c_void,
    ) -> IndexMask;

    pub fn edge_selection_from_face(
        faces_offsets: *const i32,
        faces_len: usize,
        face_mask: *const IndexMask,
        corner_edges: *const i32,
        corner_edges_len: usize,
        edges_num: i32,
        memory: *mut core::ffi::c_void,
    ) -> IndexMask;

    pub fn edge_selection_from_vert(
        edges: *const [i32; 2],
        edges_len: usize,
        vert_selection: *const bool,
        vert_selection_len: usize,
        memory: *mut core::ffi::c_void,
    ) -> IndexMask;

    pub fn face_selection_from_vert(
        faces_offsets: *const i32,
        faces_len: usize,
        corner_verts: *const i32,
        corner_verts_len: usize,
        vert_selection: *const bool,
        vert_selection_len: usize,
        memory: *mut core::ffi::c_void,
    ) -> IndexMask;

    pub fn face_selection_from_edge(
        faces_offsets: *const i32,
        faces_len: usize,
        corner_edges: *const i32,
        corner_edges_len: usize,
        edge_mask: *const bool,
        edge_mask_len: usize,
        memory: *mut core::ffi::c_void,
    ) -> IndexMask;
}
