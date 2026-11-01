/* SPDX-FileCopyrightText: 2024-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Inline color management routines: luminance calculation, XYZ and Rec.709 conversions.

pub const LUMA_COEFFICIENTS: [f32; 3] = [0.2126, 0.7152, 0.0722];

pub fn get_luminance(rgb: &[f32; 3]) -> f32 {
    LUMA_COEFFICIENTS[0] * rgb[0] + LUMA_COEFFICIENTS[1] * rgb[1] + LUMA_COEFFICIENTS[2] * rgb[2]
}

pub fn get_luminance_byte(rgb: &[u8; 3]) -> u8 {
    let rgbf = [
        rgb[0] as f32 / 255.0,
        rgb[1] as f32 / 255.0,
        rgb[2] as f32 / 255.0,
    ];
    let val = get_luminance(&rgbf);
    (val * 255.0).round().clamp(0.0, 255.0) as u8
}

pub fn xyz_to_scene_linear(xyz: &[f32; 3]) -> [f32; 3] {
    // Standard Bradford / sRGB matrix
    [
        3.2404542 * xyz[0] - 1.5371385 * xyz[1] - 0.4985314 * xyz[2],
        -0.9692660 * xyz[0] + 1.8760108 * xyz[1] + 0.0415560 * xyz[2],
        0.0556434 * xyz[0] - 0.2040259 * xyz[1] + 1.0572252 * xyz[2],
    ]
}

pub fn scene_linear_to_xyz(linear: &[f32; 3]) -> [f32; 3] {
    [
        0.4124564 * linear[0] + 0.3575761 * linear[1] + 0.1804375 * linear[2],
        0.2126729 * linear[0] + 0.7151522 * linear[1] + 0.0721750 * linear[2],
        0.0193339 * linear[0] + 0.1191920 * linear[1] + 0.9503041 * linear[2],
    ]
}
