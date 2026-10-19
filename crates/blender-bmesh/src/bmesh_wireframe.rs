//! Auto-transpiled C/C++ header module: bmesh_wireframe

use crate::bmesh_class::BMesh;

extern "C" {
    pub fn BM_mesh_wireframe(
        bm: *mut BMesh,
        offset: f32,
        offset_fac: f32,
        offset_fac_vg: f32,
        use_replace: bool,
        use_boundary: bool,
        use_even_offset: bool,
        use_relative_offset: bool,
        use_crease: bool,
        crease_weight: f32,
        defgrp_index: i32,
        defgrp_invert: bool,
        mat_offset: i16,
        mat_max: i32,
        use_tag: bool,
    );
}
