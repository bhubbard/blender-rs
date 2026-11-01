/* SPDX-FileCopyrightText: 2023-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Compositor GPU shader pipeline stages (cryptomatte, lens distortion, directional blur, morphological filter, OCIO).

#[derive(Debug, Clone)]
pub struct CryptomatteShaderStage {
    pub manifest_hash: u64,
    pub num_layers: usize,
}

#[derive(Debug, Clone)]
pub struct LensDistortionShaderStage {
    pub distort: f32,
    pub dispersion: f32,
    pub k1: f32,
    pub k2: f32,
}

#[derive(Debug, Clone)]
pub struct DirectionalBlurShaderStage {
    pub angle: f32,
    pub distance: f32,
    pub num_samples: usize,
}

#[derive(Debug, Clone)]
pub struct MorphologicalFilterStage {
    pub radius: usize,
    pub is_dilation: bool,
}
