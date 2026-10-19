//! Auto-transpiled C/C++ header module: bmesh_mesh_duplicate

use crate::bmesh_class::{BMesh, BMVert, BMEdge, BMFace};

extern "C" {
    pub fn BM_mesh_copy_arrays(
        bm_src: *mut BMesh,
        bm_dst: *mut BMesh,
        verts_src: *mut *mut BMVert,
        verts_src_len: u32,
        edges_src: *mut *mut BMEdge,
        edges_src_len: u32,
        faces_src: *mut *mut BMFace,
        faces_src_len: u32,
    );
}
