/* SPDX-FileCopyrightText: 2025-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! OpenColorIO scope space transformations (waveform, vectorscope, histogram).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OCIOScopeType {
    Waveform = 0,
    Vectorscope = 1,
    Histogram = 2,
}

pub struct OCIOScopeTransform {
    pub scope_type: OCIOScopeType,
    pub is_hdr: bool,
}

impl OCIOScopeTransform {
    pub fn new(scope_type: OCIOScopeType, is_hdr: bool) -> Self {
        Self { scope_type, is_hdr }
    }
}
