/* SPDX-FileCopyrightText: 2011-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Hydra USD rendering architecture and delegate interfaces.

use std::collections::HashMap;
use std::os::raw::c_void;
use blender_dna::{Object, Scene};
use crate::DEG_depsgraph::Depsgraph;
use crate::RE_engine::RenderEngine;

/// Camera parameters projected for Universal Scene Description (USD) Hydra delegates.
#[derive(Debug, Clone)]
pub struct GfCamera {
    pub focal_length: f32,
    pub horizontal_aperture: f32,
    pub vertical_aperture: f32,
    pub horizontal_aperture_offset: f32,
    pub vertical_aperture_offset: f32,
    pub clipping_range: [f32; 2],
    pub transform: [f32; 16],
}

impl Default for GfCamera {
    fn default() -> Self {
        Self {
            focal_length: 50.0,
            horizontal_aperture: 36.0,
            vertical_aperture: 24.0,
            horizontal_aperture_offset: 0.0,
            vertical_aperture_offset: 0.0,
            clipping_range: [0.1, 1000.0],
            transform: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
}

/// Abstract Hydra rendering engine connecting Blender to USD Hydra render delegates.
pub struct HydraEngine {
    pub delegate_name: String,
    pub render_engine: *mut RenderEngine,
    pub depsgraph: *mut Depsgraph,
    pub scene: *mut Scene,
    pub render_settings: HashMap<String, String>,
}

impl HydraEngine {
    pub fn new(delegate_name: &str) -> Self {
        Self {
            delegate_name: delegate_name.to_string(),
            render_engine: std::ptr::null_mut(),
            depsgraph: std::ptr::null_mut(),
            scene: std::ptr::null_mut(),
            render_settings: HashMap::new(),
        }
    }

    pub fn set_render_setting(&mut self, key: &str, value: &str) {
        self.render_settings.insert(key.to_string(), value.to_string());
    }

    pub fn render(&mut self) {}

    pub fn notify_status(&self, _progress: f32, _title: &str, _info: &str) {}
}

/// Final offline / production rendering engine for Hydra delegates.
pub struct FinalEngine {
    pub base: HydraEngine,
    pub aov_tokens: HashMap<String, String>,
}

impl FinalEngine {
    pub fn new(delegate_name: &str) -> Self {
        Self {
            base: HydraEngine::new(delegate_name),
            aov_tokens: HashMap::new(),
        }
    }

    pub fn render(&mut self) {
        self.base.render();
    }
}

/// Interactive 3D viewport rendering engine for Hydra delegates.
pub struct ViewportEngine {
    pub base: HydraEngine,
    pub time_begin: f64,
}

impl ViewportEngine {
    pub fn new(delegate_name: &str) -> Self {
        Self {
            base: HydraEngine::new(delegate_name),
            time_begin: 0.0,
        }
    }

    pub fn render(&mut self) {
        self.base.render();
    }
}

/// Material / asset preview rendering engine for Hydra delegates.
pub struct PreviewEngine {
    pub base: HydraEngine,
}

impl PreviewEngine {
    pub fn new(delegate_name: &str) -> Self {
        Self {
            base: HydraEngine::new(delegate_name),
        }
    }

    pub fn render(&mut self) {
        self.base.render();
    }
}
