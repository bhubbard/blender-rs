/* SPDX-FileCopyrightText: 2004-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Freestyle ViewMap: projected 2D edge visibility graph, silhouettes, and occlusion.

use crate::FRS_winged_edge::EdgeNature;

/// 0D point interface along projected silhouettes and creases.
#[derive(Debug, Clone)]
pub struct ViewVertex {
    pub point_2d: [f32; 2],
    pub point_3d: [f32; 3],
    pub id: usize,
}

impl ViewVertex {
    pub fn new(id: usize, point_2d: [f32; 2], point_3d: [f32; 3]) -> Self {
        Self { point_2d, point_3d, id }
    }
}

/// 1D edge interface representing continuous line segments in image space.
#[derive(Debug, Clone)]
pub struct ViewEdge {
    pub id: usize,
    pub nature: EdgeNature,
    pub vertices: Vec<ViewVertex>,
    pub occluded: bool,
    pub qi: i32, // Quantitative Invisibility count
}

impl ViewEdge {
    pub fn new(id: usize, nature: EdgeNature) -> Self {
        Self {
            id,
            nature,
            vertices: Vec::new(),
            occluded: false,
            qi: 0,
        }
    }

    pub fn is_visible(&self) -> bool {
        self.qi == 0
    }
}

/// The ViewMap data structure holding the 2D planar graph of visible/occluded feature lines.
pub struct ViewMap {
    pub view_edges: Vec<ViewEdge>,
    pub view_vertices: Vec<ViewVertex>,
}

impl ViewMap {
    pub fn new() -> Self {
        Self {
            view_edges: Vec::new(),
            view_vertices: Vec::new(),
        }
    }

    pub fn total_edges(&self) -> usize {
        self.view_edges.len()
    }

    pub fn total_vertices(&self) -> usize {
        self.view_vertices.len()
    }
}

impl Default for ViewMap {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder responsible for computing edge-edge visibility intersections in image space.
pub struct ViewMapBuilder {
    pub min_edge_length: f32,
}

impl ViewMapBuilder {
    pub fn new(min_edge_length: f32) -> Self {
        Self { min_edge_length }
    }

    pub fn build_view_map(&self) -> ViewMap {
        ViewMap::new()
    }
}

impl Default for ViewMapBuilder {
    fn default() -> Self {
        Self::new(0.0)
    }
}
