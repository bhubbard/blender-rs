//! Auto-transpiled C/C++ header module: vk_device

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VKExtensions {
    pub fragment_shader_barycentric: bool,
    pub wide_lines: bool,
    pub dynamic_rendering_local_read: bool,
    pub dynamic_rendering_unused_attachments: bool,
    pub external_memory: bool,
    pub maintenance4: bool,
    pub memory_priority: bool,
    pub pageable_device_local_memory: bool,
    pub graphics_pipeline_library: bool,
    pub line_rasterization: bool,
    pub extended_dynamic_state: bool,
    pub vertex_input_dynamic_state: bool,
    pub provoking_vertex: bool,
    pub host_image_copy: bool,
    pub shader_viewport_index_layer: bool,
    pub spirv_1_4: bool,
    pub multi_draw_indirect: bool,
    pub shader_clip_distance: bool,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VKWorkarounds {
    pub not_aligned_pixel_formats: bool,
}
