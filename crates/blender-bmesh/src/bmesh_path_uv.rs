//! Auto-transpiled C/C++ header module: bmesh_path_uv

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMCalcPathUVParams {
    pub use_topology_distance: u32,
    pub use_step_face: u32,
    pub cd_loop_uv_offset: i32,
    pub aspect_y: f32,
}

