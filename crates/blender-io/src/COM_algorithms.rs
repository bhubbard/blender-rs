/* SPDX-FileCopyrightText: 2023-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Compositor image processing algorithms (convolution, SMAA, symmetric blur, jump flooding, reduction).

/// 2D separable symmetric Gaussian blur.
pub fn symmetric_separable_blur_1d(
    src: &[f32],
    dst: &mut [f32],
    len: usize,
    stride: usize,
    weights: &[f32],
) {
    let radius = weights.len() / 2;
    for i in 0..len {
        let mut accum = 0.0;
        for (w_idx, &w) in weights.iter().enumerate() {
            let offset = w_idx as isize - radius as isize;
            let sample_idx = (i as isize + offset).clamp(0, len as isize - 1) as usize;
            accum += src[sample_idx * stride] * w;
        }
        dst[i * stride] = accum;
    }
}

/// Jump flooding distance transform step.
pub fn jump_flooding_step(
    grid: &[[f32; 2]],
    out: &mut [[f32; 2]],
    width: usize,
    height: usize,
    step: usize,
) {
    for y in 0..height {
        for x in 0..width {
            let mut best_dist = f32::MAX;
            let mut best_pt = grid[y * width + x];

            for dy in [-1isize, 0, 1] {
                for dx in [-1isize, 0, 1] {
                    let nx = x as isize + dx * step as isize;
                    let ny = y as isize + dy * step as isize;
                    if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                        let candidate = grid[ny as usize * width + nx as usize];
                        if candidate[0] >= 0.0 {
                            let dist = (candidate[0] - x as f32).powi(2) + (candidate[1] - y as f32).powi(2);
                            if dist < best_dist {
                                best_dist = dist;
                                best_pt = candidate;
                            }
                        }
                    }
                }
            }
            out[y * width + x] = best_pt;
        }
    }
}
