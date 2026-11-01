/* SPDX-FileCopyrightText: 2025-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! GPU shader binding for OpenColorIO hardware color transform display pipelines.

pub struct OCIOGpuShaderBinder {
    pub shader_code: String,
    pub lut3d_size: usize,
}

impl OCIOGpuShaderBinder {
    pub fn new(lut3d_size: usize) -> Self {
        Self {
            shader_code: String::new(),
            lut3d_size,
        }
    }

    pub fn generate_shader_code(&mut self) -> &str {
        self.shader_code = "// OCIO GPU Color Transformation Shader\n".to_string();
        &self.shader_code
    }
}
