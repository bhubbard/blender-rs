//! Auto-transpiled C/C++ header module: bmesh_edgesplit

use crate::bmesh_class::BMesh;

extern "C" {
    pub fn BM_mesh_edgesplit(bm: *mut BMesh, use_verts: bool, tag_only: bool, copy_select: bool);
}
