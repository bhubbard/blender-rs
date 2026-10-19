//! Auto-transpiled C/C++ header module: bmesh_triangulate

use crate::bmesh_class::BMesh;
use crate::bmesh_operator_api::{BMOperator, BMOpSlot};

extern "C" {
    pub fn BM_mesh_triangulate(
        bm: *mut BMesh,
        quad_method: i32,
        ngon_method: i32,
        min_vertices: i32,
        tag_only: bool,
        op: *mut BMOperator,
        slot_facemap_out: *mut BMOpSlot,
        slot_facemap_double_out: *mut BMOpSlot,
    );
}
