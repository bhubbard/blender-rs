/* SPDX-FileCopyrightText: 2025-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! OpenColorIO configuration, display parameters, and look transform rules.

#[derive(Debug, Clone)]
pub struct DisplayParameters {
    pub from_colorspace: String,
    pub view: String,
    pub display: String,
    pub look: String,
    pub scale: f32,
    pub exponent: f32,
    pub temperature: f32,
    pub tint: f32,
    pub use_white_balance: bool,
    pub use_hdr_buffer: bool,
    pub use_hdr_display: bool,
    pub is_image_output: bool,
    pub use_display_emulation: bool,
    pub use_scope_space: bool,
    pub inverse: bool,
}

impl Default for DisplayParameters {
    fn default() -> Self {
        Self {
            from_colorspace: String::new(),
            view: String::new(),
            display: String::new(),
            look: String::new(),
            scale: 1.0,
            exponent: 1.0,
            temperature: 6500.0,
            tint: 10.0,
            use_white_balance: false,
            use_hdr_buffer: false,
            use_hdr_display: false,
            is_image_output: false,
            use_display_emulation: false,
            use_scope_space: false,
            inverse: false,
        }
    }
}

/// OpenColorIO Config wrapper.
pub struct OCIOConfig {
    pub description: String,
}

impl OCIOConfig {
    pub fn new() -> Self {
        Self {
            description: "Default OCIO Config".to_string(),
        }
    }
}

impl Default for OCIOConfig {
    fn default() -> Self {
        Self::new()
    }
}
