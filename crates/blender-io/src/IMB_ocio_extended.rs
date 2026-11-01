//! Blender ImBuf OpenColorIO extended internal architecture headers ported to Rust
//! Corresponds to:
//! logmemfile.h, openimageio_api.h, gpu_shader_binder_internal.hh, error_handling.hh,
//! libocio_colorspace.hh, libocio_config.hh, libocio_cpu_processor.hh, libocio_display.hh,
//! libocio_display_processor.hh, libocio_gpu_shader_binder.hh, libocio_look.hh,
//! libocio_processor.hh, libocio_view.hh, ocio_shader_shared.hh, opencolorio.hh,
//! source_processor.hh, view_specific_look.hh

use blender_dna::*;
use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LogMemFile {
    pub buffer: Vec<u8>,
    pub position: usize,
}

impl LogMemFile {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct OpenImageIOApiContext {
    pub has_oiio_support: bool,
    pub max_threads: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct OcioProcessorHandle {
    pub is_valid: bool,
    pub colorspace_from: String,
    pub colorspace_to: String,
}

impl OcioProcessorHandle {
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            is_valid: true,
            colorspace_from: from.into(),
            colorspace_to: to.into(),
        }
    }

    pub fn apply_transform_rgb(&self, rgb: &mut [f32; 3]) {
        // Safe transform placeholder
        let _ = rgb;
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ViewSpecificLook {
    pub view_name: String,
    pub look_name: String,
}
