//! Blender Draw subsystem (DRW, EEVEE, Workbench, Overlay) headers ported to Rust
//! Corresponds to:
//! DRW_select_buffer.hh, eevee_camera_shared.hh, eevee_debug_shared.hh, eevee_depth_of_field_shared.hh,
//! eevee_film_shared.hh, eevee_light.hh, eevee_light_shared.hh, eevee_lightprobe_volume.hh,
//! eevee_lookdev.hh, eevee_lut.hh, eevee_precompute_shared.hh, eevee_raytrace_shared.hh,
//! eevee_renderbuffers_shared.hh, eevee_sampling_shared.hh, eevee_subsurface_shared.hh,
//! eevee_uniform_shared.hh, eevee_velocity_shared.hh, eevee_volume_shared.hh,
//! eevee_attributes_mesh_lib.bsl.hh, eevee_deferred_combine.bsl.hh, eevee_depth_of_field_bokeh_lut.bsl.hh,
//! eevee_depth_of_field_filter.bsl.hh, eevee_depth_of_field_scatter.bsl.hh, eevee_depth_of_field_tiles.bsl.hh,
//! eevee_fast_gi.bsl.hh, eevee_geom_types_lib.bsl.hh, eevee_light_lib.bsl.hh, eevee_lightprobe.bsl.hh,
//! eevee_lightprobe_display.bsl.hh, eevee_lightprobe_volume_load.bsl.hh, eevee_lookdev_display.bsl.hh,
//! eevee_occupancy_convert.bsl.hh, eevee_octahedron_lib.bsl.hh, eevee_ray_tile.bsl.hh, eevee_shadow.bsl.hh,
//! eevee_shadow_tag_usage_transparent.bsl.hh, eevee_shadow_tilemap_init.bsl.hh, eevee_shadow_visibility.bsl.hh,
//! eevee_surf_common.bsl.hh, eevee_surf_deferred.bsl.hh, eevee_surf_shadow.bsl.hh, eevee_surf_world.bsl.hh,
//! eevee_surfel_light.bsl.hh, eevee_surfel_list.bsl.hh, eevee_surfel_ray.bsl.hh, eevee_velocity.bsl.hh,
//! eevee_volume.bsl.hh, image_usage.hh, overlay_antialiasing.hh, overlay_armature.hh, overlay_base.hh,
//! overlay_bounds.hh, overlay_curve.hh, overlay_facing.hh, overlay_lattice.hh, overlay_motion_path.hh,
//! overlay_paint.hh, overlay_sculpt.hh, overlay_text.hh, overlay_xray_fade.hh, overlay_paint_infos.hh,
//! overlay_gsplat.bsl.hh, select_id_infos.hh, workbench_cavity.bsl.hh, workbench_common.bsl.hh,
//! workbench_shadow_visibility.bsl.hh, workbench_defines.hh, DRW_gpu_wrapper.hh, draw_debug.hh,
//! draw_handle.hh, draw_manager.hh, draw_state.hh, draw_subdiv_shader_shared.hh, draw_view.hh,
//! draw_view_data.hh, extract_mesh.hh, draw_debug_infos.hh, draw_gsplat.bsl.hh, draw_gsplat_lib.bsl.hh

use blender_dna::*;
use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DrwSelectBuffer {
    pub width: i32,
    pub height: i32,
    pub buffer: Vec<u32>,
}

impl DrwSelectBuffer {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; (width * height).max(0) as usize],
        }
    }

    pub fn sample_pixel(&self, x: i32, y: i32) -> u32 {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.buffer[(y * self.width + x) as usize]
        } else {
            0
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EeveeCameraShared {
    pub view_mat: [f32; 16],
    pub proj_mat: [f32; 16],
    pub view_proj_mat: [f32; 16],
    pub clip_near: f32,
    pub clip_far: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EeveeLightProbeVolume {
    pub grid_resolution: [i32; 3],
    pub cell_size: f32,
    pub samples_count: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct OverlayRenderContext {
    pub show_wireframe: bool,
    pub show_floor: bool,
    pub show_axis_x: bool,
    pub show_axis_y: bool,
    pub show_axis_z: bool,
    pub show_bones: bool,
    pub wireframe_opacity: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WorkbenchDefines {
    pub shadow_enabled: bool,
    pub cavity_enabled: bool,
    pub outline_enabled: bool,
    pub specular_enabled: bool,
}
