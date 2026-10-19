//! Auto-transpiled C/C++ header module: bmesh_intersect_edges

use crate::bmesh_class::BMesh;
use blender_mem::BLI_ghash::GHash;

extern "C" {
    pub fn BM_mesh_intersect_edges(
        bm: *mut BMesh,
        hflag: i8,
        dist: f32,
        split_faces: bool,
        r_targetmap: *mut GHash,
    ) -> bool;
}
