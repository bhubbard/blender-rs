//! Auto-transpiled C/C++ header module: bmesh_region_match

use crate::bmesh_class::{BMesh, BMFace};
use crate::ListBaseT;
use blender_dna::LinkData;

extern "C" {
    pub fn BM_mesh_region_match(
        bm: *mut BMesh,
        faces_region: *mut *mut BMFace,
        faces_region_len: u32,
        r_face_regions: *mut ListBaseT<LinkData>,
    ) -> i32;
}
