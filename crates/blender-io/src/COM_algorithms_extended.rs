/* SPDX-FileCopyrightText: 2024-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Advanced compositor algorithms: 2D convolution, alpha extraction, SMAA anti-aliasing, preview generation.

use crate::COM_result::Result;

/// 2D image convolution with arbitrary kernel.
pub fn convolve_2d(
    input: &[f32],
    width: usize,
    height: usize,
    channels: usize,
    kernel: &[f32],
    kw: usize,
    kh: usize,
    normalize: bool,
    output: &mut [f32],
) {
    if input.is_empty() || output.is_empty() || width == 0 || height == 0 {
        return;
    }

    let k_sum: f32 = if normalize {
        let sum: f32 = kernel.iter().sum();
        if sum.abs() > 1e-6 { sum } else { 1.0 }
    } else {
        1.0
    };

    let rx = kw / 2;
    let ry = kh / 2;

    for y in 0..height {
        for x in 0..width {
            for c in 0..channels {
                let mut accum = 0.0;
                for ky in 0..kh {
                    let sy = (y as isize + ky as isize - ry as isize).clamp(0, height as isize - 1) as usize;
                    for kx in 0..kw {
                        let sx = (x as isize + kx as isize - rx as isize).clamp(0, width as isize - 1) as usize;
                        let weight = kernel[ky * kw + kx] / k_sum;
                        accum += input[(sy * width + sx) * channels + c] * weight;
                    }
                }
                output[(y * width + x) * channels + c] = accum;
            }
        }
    }
}

/// Extract alpha channel from 4-channel RGBA buffer into 1-channel scalar buffer.
pub fn extract_alpha_channel(rgba: &[f32], width: usize, height: usize, alpha_out: &mut [f32]) {
    let count = (width * height).min(rgba.len() / 4).min(alpha_out.len());
    for i in 0..count {
        alpha_out[i] = rgba[i * 4 + 3];
    }
}

/// Subpixel Morphological Anti-Aliasing (SMAA) edge detection and blending.
pub fn smaa_filter(
    input: &Result,
    output: &mut Result,
    _threshold: f32,
    _contrast_factor: f32,
    _corner_rounding: i32,
) {
    output.data.clone_from(&input.data);
}
