/* SPDX-FileCopyrightText: 2025-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! OpenColorIO color transform 3x3 and 4x4 matrix helpers.

/// 4x4 matrix for affine color conversions (white balance, chromatic adaptation).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OCIOMatrix4x4 {
    pub m: [[f32; 4]; 4],
}

impl Default for OCIOMatrix4x4 {
    fn default() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

impl OCIOMatrix4x4 {
    pub fn identity() -> Self {
        Self::default()
    }

    pub fn transform_rgb(&self, rgb: [f32; 3]) -> [f32; 3] {
        [
            self.m[0][0] * rgb[0] + self.m[0][1] * rgb[1] + self.m[0][2] * rgb[2] + self.m[0][3],
            self.m[1][0] * rgb[0] + self.m[1][1] * rgb[1] + self.m[1][2] * rgb[2] + self.m[1][3],
            self.m[2][0] * rgb[0] + self.m[2][1] * rgb[1] + self.m[2][2] * rgb[2] + self.m[2][3],
        ]
    }
}
