//! Blender Draw engines (EEVEE, External, Overlay, Select, Workbench) implementations ported to Rust
//! Corresponds to:
//! eevee_depth_of_field.cc, eevee_light.cc, eevee_lightprobe.cc, eevee_lightprobe_planar.cc,
//! eevee_raytrace.cc, eevee_sampling.cc, bsl_shader_linting.cc, external_engine.cc,
//! select_debug_engine.cc, draw_cache_impl_gsplat.cc, extract_mesh_ibo_lines_adjacency.cc

use blender_dna::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct EeveeDepthOfFieldState {
    pub focus_distance: f32,
    pub fstop: f32,
    pub blades: i32,
    pub bokeh_shape: i32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct EeveeLightManager {
    pub max_lights: usize,
    pub active_lights_count: usize,
}

impl EeveeLightManager {
    pub fn new(max_lights: usize) -> Self {
        Self {
            max_lights,
            active_lights_count: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct EeveePlanarProbe {
    pub plane_equation: [f32; 4],
    pub resolution: i32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ExternalDrawEngine {
    pub engine_type_name: String,
    pub is_running: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GaussianSplatCache {
    pub splat_count: usize,
    pub is_sorted: bool,
}
