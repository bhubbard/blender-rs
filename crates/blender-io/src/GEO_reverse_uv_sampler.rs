//! Auto-transpiled C/C++ header module: GEO_reverse_uv_sampler
//! Source: upstream/source/blender/geometry/GEO_reverse_uv_sampler.hh

use blender_math::{Vec2, Vec3};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultType {
    None,
    Ok,
    Multiple,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct ReverseUVSamplerResult {
    pub result_type: ResultType,
    pub tri_index: i32,
    pub bary_weights: Vec3,
}

impl Default for ReverseUVSamplerResult {
    fn default() -> Self {
        Self {
            result_type: ResultType::None,
            tri_index: -1,
            bary_weights: Vec3::ZERO,
        }
    }
}

pub struct ReverseUVSampler {
    pub uv_map: Vec<Vec2>,
    pub corner_tris: Vec<[i32; 3]>,
}

impl ReverseUVSampler {
    pub fn new(uv_map: Vec<Vec2>, corner_tris: Vec<[i32; 3]>) -> Self {
        Self { uv_map, corner_tris }
    }

    pub fn sample(&self, _query_uv: Vec2) -> ReverseUVSamplerResult {
        ReverseUVSamplerResult::default()
    }
}
