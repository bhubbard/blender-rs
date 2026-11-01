/* SPDX-FileCopyrightText: 2023-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Layered Action evaluation: property identifiers and animated property states.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PropIdentifier {
    pub rna_path: String,
    pub array_index: i32,
}

impl PropIdentifier {
    pub fn new(rna_path: &str, array_index: i32) -> Self {
        Self {
            rna_path: rna_path.to_string(),
            array_index,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnimatedProperty {
    pub value: f32,
    pub prop_identifier: PropIdentifier,
}

impl AnimatedProperty {
    pub fn new(value: f32, prop_identifier: PropIdentifier) -> Self {
        Self {
            value,
            prop_identifier,
        }
    }
}
