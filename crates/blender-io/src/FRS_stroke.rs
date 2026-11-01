/* SPDX-FileCopyrightText: 2004-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Freestyle Stroke creation, chaining, vertex shaders, and rasterization pipeline.

/// Vertex along a stylized artistic stroke.
#[derive(Debug, Clone, Copy)]
pub struct StrokeVertex {
    pub point: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 4],
    pub thickness: [f32; 2], // Left/right thickness
    pub u: f32,              // Normalized arc length parameter [0, 1]
}

impl Default for StrokeVertex {
    fn default() -> Self {
        Self {
            point: [0.0; 3],
            normal: [0.0, 0.0, 1.0],
            color: [0.0, 0.0, 0.0, 1.0],
            thickness: [1.0, 1.0],
            u: 0.0,
        }
    }
}

/// A continuous 1D artistic stroke made of vertices.
#[derive(Debug, Clone)]
pub struct Stroke {
    pub vertices: Vec<StrokeVertex>,
    pub id: usize,
    pub length_2d: f32,
}

impl Stroke {
    pub fn new(id: usize) -> Self {
        Self {
            vertices: Vec::new(),
            id,
            length_2d: 0.0,
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn push_vertex(&mut self, v: StrokeVertex) {
        self.vertices.push(v);
    }
}

/// Renderable 2D/3D polygon mesh representation of a stroke with width and color.
pub struct StrokeRep {
    pub stroke_id: usize,
    pub vertices: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 4]>,
    pub indices: Vec<u32>,
}

impl StrokeRep {
    pub fn new(stroke_id: usize) -> Self {
        Self {
            stroke_id,
            vertices: Vec::new(),
            colors: Vec::new(),
            indices: Vec::new(),
        }
    }
}

/// Stroke shader trait for procedural styling (thickness, color, geometry displacement).
pub trait StrokeShader: Send + Sync {
    fn shade(&self, stroke: &mut Stroke);
}

/// Chaining operator for connecting contiguous view edges into longer strokes.
pub struct Chain {
    pub view_edge_ids: Vec<usize>,
}

impl Chain {
    pub fn new() -> Self {
        Self {
            view_edge_ids: Vec::new(),
        }
    }
}

impl Default for Chain {
    fn default() -> Self {
        Self::new()
    }
}

/// Layer of stylized strokes sharing render parameters.
pub struct StrokeLayer {
    pub name: String,
    pub strokes: Vec<Stroke>,
    pub shaders: Vec<Box<dyn StrokeShader>>,
}

impl StrokeLayer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            strokes: Vec::new(),
            shaders: Vec::new(),
        }
    }

    pub fn add_shader<S: StrokeShader + 'static>(&mut self, shader: S) {
        self.shaders.push(Box::new(shader));
    }

    pub fn execute_shaders(&mut self) {
        for stroke in &mut self.strokes {
            for shader in &self.shaders {
                shader.shade(stroke);
            }
        }
    }
}
