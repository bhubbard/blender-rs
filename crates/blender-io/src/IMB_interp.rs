/* SPDX-FileCopyrightText: 2024-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Image buffer pixel sampling and interpolation routines (nearest, bilinear, bicubic).

/// Sample nearest byte pixel with clamp-to-border semantics.
pub fn interpolate_nearest_border_byte(data: &[u8], width: usize, height: usize, u: f32, v: f32) -> [u8; 4] {
    if data.is_empty() || width == 0 || height == 0 {
        return [0, 0, 0, 0];
    }
    let x = ((u * width as f32).floor() as isize).clamp(0, width as isize - 1) as usize;
    let y = ((v * height as f32).floor() as isize).clamp(0, height as isize - 1) as usize;
    let idx = (y * width + x) * 4;
    if idx + 4 <= data.len() {
        [data[idx], data[idx + 1], data[idx + 2], data[idx + 3]]
    } else {
        [0, 0, 0, 0]
    }
}

/// Sample nearest float pixel with clamp-to-border semantics.
pub fn interpolate_nearest_border_fl(data: &[f32], width: usize, height: usize, u: f32, v: f32) -> [f32; 4] {
    if data.is_empty() || width == 0 || height == 0 {
        return [0.0, 0.0, 0.0, 0.0];
    }
    let x = ((u * width as f32).floor() as isize).clamp(0, width as isize - 1) as usize;
    let y = ((v * height as f32).floor() as isize).clamp(0, height as isize - 1) as usize;
    let idx = (y * width + x) * 4;
    if idx + 4 <= data.len() {
        [data[idx], data[idx + 1], data[idx + 2], data[idx + 3]]
    } else {
        [0.0, 0.0, 0.0, 0.0]
    }
}

/// Bilinear interpolation of byte pixel data.
pub fn interpolate_bilinear_byte(data: &[u8], width: usize, height: usize, u: f32, v: f32) -> [u8; 4] {
    if data.is_empty() || width == 0 || height == 0 {
        return [0, 0, 0, 0];
    }
    let fx = u * width as f32 - 0.5;
    let fy = v * height as f32 - 0.5;
    let x0 = (fx.floor() as isize).clamp(0, width as isize - 1) as usize;
    let y0 = (fy.floor() as isize).clamp(0, height as isize - 1) as usize;
    let x1 = (x0 + 1).min(width - 1);
    let y1 = (y0 + 1).min(height - 1);

    let wx = fx - fx.floor();
    let wy = fy - fy.floor();

    let mut out = [0u8; 4];
    for c in 0..4 {
        let p00 = data[(y0 * width + x0) * 4 + c] as f32;
        let p10 = data[(y0 * width + x1) * 4 + c] as f32;
        let p01 = data[(y1 * width + x0) * 4 + c] as f32;
        let p11 = data[(y1 * width + x1) * 4 + c] as f32;

        let top = p00 * (1.0 - wx) + p10 * wx;
        let bottom = p01 * (1.0 - wx) + p11 * wx;
        out[c] = (top * (1.0 - wy) + bottom * wy).round().clamp(0.0, 255.0) as u8;
    }
    out
}

/// Bilinear interpolation of float pixel data.
pub fn interpolate_bilinear_fl(data: &[f32], width: usize, height: usize, u: f32, v: f32) -> [f32; 4] {
    if data.is_empty() || width == 0 || height == 0 {
        return [0.0, 0.0, 0.0, 0.0];
    }
    let fx = u * width as f32 - 0.5;
    let fy = v * height as f32 - 0.5;
    let x0 = (fx.floor() as isize).clamp(0, width as isize - 1) as usize;
    let y0 = (fy.floor() as isize).clamp(0, height as isize - 1) as usize;
    let x1 = (x0 + 1).min(width - 1);
    let y1 = (y0 + 1).min(height - 1);

    let wx = fx - fx.floor();
    let wy = fy - fy.floor();

    let mut out = [0.0f32; 4];
    for c in 0..4 {
        let p00 = data[(y0 * width + x0) * 4 + c];
        let p10 = data[(y0 * width + x1) * 4 + c];
        let p01 = data[(y1 * width + x0) * 4 + c];
        let p11 = data[(y1 * width + x1) * 4 + c];

        let top = p00 * (1.0 - wx) + p10 * wx;
        let bottom = p01 * (1.0 - wx) + p11 * wx;
        out[c] = top * (1.0 - wy) + bottom * wy;
    }
    out
}
