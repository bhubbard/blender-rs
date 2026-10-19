//! Auto-transpiled C/C++ header module: bmesh_bisect_plane

use crate::bmesh_class::BMesh;

extern "C" {
    pub fn BM_mesh_bisect_plane(
        bm: *mut BMesh,
        plane: *const [f32; 4],
        use_snap_center: bool,
        use_tag: bool,
        oflag_center: i16,
        oflag_new: i16,
        eps: f32,
    );
}
