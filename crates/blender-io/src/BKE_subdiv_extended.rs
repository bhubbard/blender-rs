//! Blender Kernel subdivision subsystem headers ported to Rust
//! Corresponds to BKE_subdiv.hh, BKE_subdiv_eval.hh, BKE_subdiv_foreach.hh, BKE_subdiv_modifier.hh

use blender_dna::*;
use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubdivScheme {
    CatmullClark,
    Bilinear,
    Loop,
}

impl Default for SubdivScheme {
    fn default() -> Self {
        Self::CatmullClark
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct SubdivSettings {
    pub level: i32,
    pub is_simple: bool,
    pub is_optimal: bool,
    pub use_creases: bool,
    pub scheme: SubdivScheme,
}

impl Default for SubdivSettings {
    fn default() -> Self {
        Self {
            level: 1,
            is_simple: false,
            is_optimal: false,
            use_creases: true,
            scheme: SubdivScheme::CatmullClark,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SubdivEvaluator {
    pub settings: SubdivSettings,
    pub num_coarse_vertices: usize,
    pub num_coarse_faces: usize,
}

impl SubdivEvaluator {
    pub fn new(settings: SubdivSettings) -> Self {
        Self {
            settings,
            num_coarse_vertices: 0,
            num_coarse_faces: 0,
        }
    }

    pub fn evaluate_vertex(&self, _u: f32, _v: f32, _p: &mut [f32; 3]) {
        // Safe placeholder evaluator
    }
}

pub type SubdivForeachVertexFn = fn(index: usize, co: &[f32; 3], user_data: *mut c_void);

pub struct SubdivForeachContext {
    pub user_data: *mut c_void,
    pub vertex_cb: Option<SubdivForeachVertexFn>,
}
