//! Auto-transpiled C/C++ header module: GEO_grid_samplers
//! Source: upstream/source/blender/geometry/GEO_grid_samplers.hh

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridSamplerType {
    Point,
    Box,
    Quadratic,
    Cubic,
}

#[derive(Debug, Clone, Default)]
pub struct GridSampler {
    pub sampler_type: Option<GridSamplerType>,
}

impl GridSampler {
    pub fn new(sampler_type: GridSamplerType) -> Self {
        Self {
            sampler_type: Some(sampler_type),
        }
    }
}
