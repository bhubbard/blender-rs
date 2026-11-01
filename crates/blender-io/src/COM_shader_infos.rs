/* SPDX-FileCopyrightText: 2023-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Compositor GPU shader pipeline descriptors (cryptomatte, lens distortion, blur, OCIO).

/// GPU shader pipeline stage descriptor.
#[derive(Debug, Clone)]
pub struct CompositorShaderInfo {
    pub name: String,
    pub source: String,
}

impl CompositorShaderInfo {
    pub fn new(name: &str, source: &str) -> Self {
        Self {
            name: name.to_string(),
            source: source.to_string(),
        }
    }
}
