//! Auto-transpiled C/C++ header module: bmesh_edgenet

use crate::bmesh_class::BMesh;

extern "C" {
    pub fn BM_mesh_edgenet(bm: *mut BMesh, use_edge_tag: bool, use_new_face_tag: bool);
}
