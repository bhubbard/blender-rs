/* SPDX-FileCopyrightText: 2004-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Freestyle 2D/3D geometry utilities, spatial subdivision grids, Bezier curves, and noise.

use std::collections::HashMap;

/// 3D Axis-Aligned Bounding Box.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BBox3D {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl BBox3D {
    pub fn new(min: [f32; 3], max: [f32; 3]) -> Self {
        Self { min, max }
    }

    pub fn empty() -> Self {
        Self {
            min: [f32::MAX; 3],
            max: [f32::MIN; 3],
        }
    }

    pub fn extend(&mut self, p: [f32; 3]) {
        for i in 0..3 {
            if p[i] < self.min[i] {
                self.min[i] = p[i];
            }
            if p[i] > self.max[i] {
                self.max[i] = p[i];
            }
        }
    }

    pub fn contains(&self, p: [f32; 3]) -> bool {
        (0..3).all(|i| p[i] >= self.min[i] && p[i] <= self.max[i])
    }
}

/// Cubic Bezier curve evaluator.
pub struct BezierCurve3D {
    pub p0: [f32; 3],
    pub p1: [f32; 3],
    pub p2: [f32; 3],
    pub p3: [f32; 3],
}

impl BezierCurve3D {
    pub fn new(p0: [f32; 3], p1: [f32; 3], p2: [f32; 3], p3: [f32; 3]) -> Self {
        Self { p0, p1, p2, p3 }
    }

    pub fn evaluate(&self, t: f32) -> [f32; 3] {
        let u = 1.0 - t;
        let tt = t * t;
        let uu = u * u;
        let uuu = uu * u;
        let ttt = tt * t;

        let mut res = [0.0; 3];
        for i in 0..3 {
            res[i] = uuu * self.p0[i]
                + 3.0 * uu * t * self.p1[i]
                + 3.0 * u * tt * self.p2[i]
                + ttt * self.p3[i];
        }
        res
    }
}

/// Fast uniform spatial grid for ray/segment collision and proximity queries.
pub struct FastGrid<T> {
    pub cell_size: f32,
    pub cells: HashMap<(i32, i32, i32), Vec<T>>,
}

impl<T> FastGrid<T> {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size: if cell_size <= 0.0 { 1.0 } else { cell_size },
            cells: HashMap::new(),
        }
    }

    pub fn cell_coords(&self, p: [f32; 3]) -> (i32, i32, i32) {
        (
            (p[0] / self.cell_size).floor() as i32,
            (p[1] / self.cell_size).floor() as i32,
            (p[2] / self.cell_size).floor() as i32,
        )
    }

    pub fn insert(&mut self, p: [f32; 3], item: T) {
        let coords = self.cell_coords(p);
        self.cells.entry(coords).or_default().push(item);
    }
}

/// Procedural Perlin/Simplex-style pseudo noise for stroke waviness.
pub struct FrsNoise {
    pub seed: u32,
}

impl FrsNoise {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }

    pub fn sample_1d(&self, x: f32) -> f32 {
        let n = (x * 100.0) as i32 ^ (self.seed as i32);
        let nn = (n.wrapping_mul(n.wrapping_mul(15731) + 789221) + 1376312589) & 0x7fffffff;
        1.0 - (nn as f32 / 1073741824.0)
    }
}
