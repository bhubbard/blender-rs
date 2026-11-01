/* SPDX-FileCopyrightText: 2023-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Compositor operation input descriptors and realization rules.

use crate::COM_result::ResultType;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputRealizationMode {
    None = 0,
    Transforms = 1,
    OperationDomain = 2,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImplicitInputType {
    UniformImageCoordinates = 0,
    SceneFrame = 1,
}

#[derive(Debug, Clone)]
pub struct InputDescriptor {
    pub input_type: ResultType,
    pub realization_mode: InputRealizationMode,
    pub implicit_input: Option<ImplicitInputType>,
    pub domain_priority: i32,
    pub expects_single_value: bool,
}

impl InputDescriptor {
    pub fn new(input_type: ResultType) -> Self {
        Self {
            input_type,
            realization_mode: InputRealizationMode::OperationDomain,
            implicit_input: None,
            domain_priority: 0,
            expects_single_value: false,
        }
    }
}
