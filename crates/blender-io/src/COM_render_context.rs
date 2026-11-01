/* SPDX-FileCopyrightText: 2023-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Compositor render context holding scene, view layer, and rendering configuration.

use blender_dna::{Scene, ViewLayer};

pub struct CompositorRenderContext {
    pub scene: *const Scene,
    pub view_layer: *const ViewLayer,
    pub is_rendering: bool,
}

impl CompositorRenderContext {
    pub fn new(scene: *const Scene, view_layer: *const ViewLayer) -> Self {
        Self {
            scene,
            view_layer,
            is_rendering: false,
        }
    }
}
