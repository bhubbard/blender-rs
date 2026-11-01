/* SPDX-FileCopyrightText: 2023-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Compositor execution result types, intermediate data buffers, and pixel formats.

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResultType {
    Float = 0,
    Float2 = 1,
    Float3 = 2,
    Float4 = 3,
    Color = 4,
    Int = 5,
    Int2 = 6,
    Int3 = 7,
    Int4 = 8,
    Bool = 9,
}

/// Buffer or single value resulting from a node operation.
#[derive(Debug, Clone)]
pub struct Result {
    pub result_type: ResultType,
    pub width: usize,
    pub height: usize,
    pub data: Vec<f32>,
    pub is_single_value: bool,
}

impl Result {
    pub fn new_single_value(result_type: ResultType, val: f32) -> Self {
        Self {
            result_type,
            width: 1,
            height: 1,
            data: vec![val],
            is_single_value: true,
        }
    }

    pub fn new_buffer(result_type: ResultType, width: usize, height: usize, channels: usize) -> Self {
        Self {
            result_type,
            width,
            height,
            data: vec![0.0; width * height * channels],
            is_single_value: false,
        }
    }
}
