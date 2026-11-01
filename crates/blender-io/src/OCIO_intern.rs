/* SPDX-FileCopyrightText: 2025-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Internal OpenColorIO structures: white point matrix, description, and CPU/GPU processors.

use crate::OCIO_config::OCIOConfig;

/// Calculate chromatic adaptation matrix given correlated color temperature (Kelvin) and tint.
pub fn calculate_white_point_matrix(_config: &OCIOConfig, temperature: f32, tint: f32) -> [[f32; 3]; 3] {
    let t = (temperature / 6500.0).clamp(0.1, 10.0);
    let g = (1.0 + tint * 0.01).clamp(0.1, 10.0);
    [
        [t, 0.0, 0.0],
        [0.0, g, 0.0],
        [0.0, 0.0, 1.0 / (t * g).max(1e-5)],
    ]
}

/// Description of a color space or display view.
#[derive(Debug, Clone)]
pub struct OCIODescription {
    pub name: String,
    pub family: String,
    pub description: String,
}

impl OCIODescription {
    pub fn new(name: &str, family: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            family: family.to_string(),
            description: description.to_string(),
        }
    }
}
