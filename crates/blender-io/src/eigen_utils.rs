/* SPDX-FileCopyrightText: Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

pub type Scalar = f32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triplet {
    pub row: usize,
    pub col: usize,
    pub value: Scalar,
}

impl Triplet {
    pub fn new(row: usize, col: usize, value: Scalar) -> Self {
        Self { row, col, value }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TripletList {
    pub triplets: Vec<Triplet>,
}

impl TripletList {
    pub fn new() -> Self {
        Self {
            triplets: Vec::new(),
        }
    }

    pub fn reserve(&mut self, capacity: usize) {
        self.triplets.reserve(capacity);
    }

    pub fn push(&mut self, triplet: Triplet) {
        self.triplets.push(triplet);
    }

    pub fn clear(&mut self) {
        self.triplets.clear();
    }
}
