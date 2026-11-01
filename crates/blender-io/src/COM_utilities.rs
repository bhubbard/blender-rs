/* SPDX-FileCopyrightText: 2024-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Compositor geometric utilities: diagonal indexation and traversal, OpenImageDenoise device binding, and OCIO shader keys.

/// Number of diagonals in a 2D matrix of dimensions (width, height).
pub fn compute_number_of_diagonals(width: usize, height: usize) -> usize {
    if width == 0 || height == 0 {
        0
    } else {
        width + height - 1
    }
}

/// Length of the diagonal at the given index.
pub fn compute_diagonal_length(width: usize, height: usize, diag_idx: usize) -> usize {
    let num_diags = compute_number_of_diagonals(width, height);
    if diag_idx >= num_diags {
        return 0;
    }
    let min_dim = width.min(height);
    if diag_idx < min_dim {
        diag_idx + 1
    } else if diag_idx >= width.max(height) {
        num_diags - diag_idx
    } else {
        min_dim
    }
}

/// Key identifying an OpenColorIO shader cache entry.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OCIOColorSpaceConversionShaderKey {
    pub source: String,
    pub target: String,
    pub config_cache_id: String,
}

impl OCIOColorSpaceConversionShaderKey {
    pub fn new(source: &str, target: &str, config_cache_id: &str) -> Self {
        Self {
            source: source.to_string(),
            target: target.to_string(),
            config_cache_id: config_cache_id.to_string(),
        }
    }
}

/// OpenImageDenoise device abstraction.
pub struct OidnDeviceWrapper {
    pub is_gpu: bool,
    pub num_threads: usize,
}

impl Default for OidnDeviceWrapper {
    fn default() -> Self {
        Self {
            is_gpu: false,
            num_threads: 4,
        }
    }
}
