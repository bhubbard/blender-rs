/* SPDX-FileCopyrightText: 2004-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Freestyle Winged-Edge topological mesh representation, edge classification, and curvatures.

/// Edge topological nature flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeNature {
    Point = 0,
    Silhouette = 1,
    Border = 2,
    Crease = 4,
    Ridge = 8,
    Valley = 16,
    SuggestiveContour = 32,
    MaterialBoundary = 64,
}

/// Winged-Edge representation of a mesh edge.
#[derive(Debug, Clone)]
pub struct WEdge {
    pub a: usize,
    pub b: usize,
    pub left_face: Option<usize>,
    pub right_face: Option<usize>,
    pub a_next: Option<usize>,
    pub a_prev: Option<usize>,
    pub b_next: Option<usize>,
    pub b_prev: Option<usize>,
    pub nature: u32,
}

impl WEdge {
    pub fn new(a: usize, b: usize) -> Self {
        Self {
            a,
            b,
            left_face: None,
            right_face: None,
            a_next: None,
            a_prev: None,
            b_next: None,
            b_prev: None,
            nature: EdgeNature::Border as u32,
        }
    }

    pub fn is_silhouette(&self) -> bool {
        (self.nature & (EdgeNature::Silhouette as u32)) != 0
    }

    pub fn is_crease(&self) -> bool {
        (self.nature & (EdgeNature::Crease as u32)) != 0
    }
}

/// Mesh vertex curvature tensor for ridge/valley detection.
#[derive(Debug, Clone, Copy)]
pub struct Curvature {
    pub k1: f32,
    pub k2: f32,
    pub e1: [f32; 3],
    pub e2: [f32; 3],
}

impl Default for Curvature {
    fn default() -> Self {
        Self {
            k1: 0.0,
            k2: 0.0,
            e1: [1.0, 0.0, 0.0],
            e2: [0.0, 1.0, 0.0],
        }
    }
}

/// Winged edge topology graph.
pub struct WingedEdgeGraph {
    pub vertices: Vec<[f32; 3]>,
    pub edges: Vec<WEdge>,
}

impl WingedEdgeGraph {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, v: [f32; 3]) -> usize {
        let idx = self.vertices.len();
        self.vertices.push(v);
        idx
    }

    pub fn add_edge(&mut self, a: usize, b: usize) -> usize {
        let idx = self.edges.len();
        self.edges.push(WEdge::new(a, b));
        idx
    }
}

impl Default for WingedEdgeGraph {
    fn default() -> Self {
        Self::new()
    }
}
