/* SPDX-FileCopyrightText: 2023-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Freestyle Blender data model interface: file loader, stroke renderer, and style modules.

use std::collections::HashMap;
use blender_dna::{Material, Mesh, Object, Scene, ViewLayer};
use crate::DEG_depsgraph::{Depsgraph, Main};
use crate::RE_pipeline::Render;

/// Geometry loader bounding box and coordinate state.
#[derive(Debug, Clone)]
pub struct LoaderState {
    pub pv: [f32; 3],
    pub pn: [f32; 3],
    pub pm: [Option<usize>; 3],
    pub current_index: usize,
    pub current_m_index: usize,
    pub min_bbox: [f32; 3],
    pub max_bbox: [f32; 3],
}

impl Default for LoaderState {
    fn default() -> Self {
        Self {
            pv: [0.0; 3],
            pn: [0.0; 3],
            pm: [None; 3],
            current_index: 0,
            current_m_index: 0,
            min_bbox: [f32::MAX; 3],
            max_bbox: [f32::MIN; 3],
        }
    }
}

/// Loader connecting Blender scene objects and meshes into Freestyle render graphs.
pub struct BlenderFileLoader {
    pub num_faces_read: usize,
    pub min_edge_size: f32,
    pub smooth: bool,
    pub z_near: f32,
    pub z_far: f32,
    pub z_offset: f32,
}

impl BlenderFileLoader {
    pub fn new() -> Self {
        Self {
            num_faces_read: 0,
            min_edge_size: 0.0,
            smooth: true,
            z_near: 0.1,
            z_far: 1000.0,
            z_offset: 0.0,
        }
    }

    pub fn num_faces_read(&self) -> usize {
        self.num_faces_read
    }

    pub fn min_edge_size(&self) -> f32 {
        self.min_edge_size
    }

    pub fn test_degenerate_triangle(&self, v1: [f32; 3], v2: [f32; 3], v3: [f32; 3]) -> bool {
        let e1 = [v2[0] - v1[0], v2[1] - v1[1], v2[2] - v1[2]];
        let e2 = [v3[0] - v1[0], v3[1] - v1[1], v3[2] - v1[2]];
        let cross = [
            e1[1] * e2[2] - e1[2] * e2[1],
            e1[2] * e2[0] - e1[0] * e2[2],
            e1[0] * e2[1] - e1[1] * e2[0],
        ];
        let len_sq = cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2];
        len_sq < 1e-12
    }
}

impl Default for BlenderFileLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Group of generated stroke geometry sharing render materials.
#[derive(Debug, Clone)]
pub struct StrokeGroup {
    pub totvert: i32,
    pub totedge: i32,
    pub faces_num: i32,
    pub totloop: i32,
    pub materials: HashMap<usize, i32>,
}

impl StrokeGroup {
    pub fn new() -> Self {
        Self {
            totvert: 0,
            totedge: 0,
            faces_num: 0,
            totloop: 0,
            materials: HashMap::new(),
        }
    }
}

impl Default for StrokeGroup {
    fn default() -> Self {
        Self::new()
    }
}

/// Stroke renderer outputting lines directly into Blender meshes or render buffers.
pub struct BlenderStrokeRenderer {
    pub render_count: i32,
    pub stroke_groups: Vec<StrokeGroup>,
    pub textured_stroke_groups: Vec<StrokeGroup>,
}

impl BlenderStrokeRenderer {
    pub fn new(render_count: i32) -> Self {
        Self {
            render_count,
            stroke_groups: Vec::new(),
            textured_stroke_groups: Vec::new(),
        }
    }
}

/// Style module definition hook for controlling stroke styling rules.
pub trait BlenderStyleModule {
    fn name(&self) -> &str;
    fn execute(&mut self);
}
