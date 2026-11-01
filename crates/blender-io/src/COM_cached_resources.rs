/* SPDX-FileCopyrightText: 2023-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Compositor cached resources: distortion grid, OpenColorIO shader cache, bokeh kernels.

use std::collections::HashMap;

/// 2D lens/movie distortion grid mesh.
#[derive(Debug, Clone)]
pub struct DistortionGrid {
    pub width: usize,
    pub height: usize,
    pub uvs: Vec<[f32; 2]>,
}

impl DistortionGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut uvs = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                uvs.push([
                    x as f32 / (width - 1).max(1) as f32,
                    y as f32 / (height - 1).max(1) as f32,
                ]);
            }
        }
        Self { width, height, uvs }
    }
}

/// Generic cache manager for expensive compositor procedural resources.
#[derive(Default)]
pub struct CompositorResourceCache {
    pub entries: HashMap<String, Vec<u8>>,
}

impl CompositorResourceCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<&[u8]> {
        self.entries.get(key).map(|v| v.as_slice())
    }

    pub fn insert(&mut self, key: String, data: Vec<u8>) {
        self.entries.insert(key, data);
    }
}
