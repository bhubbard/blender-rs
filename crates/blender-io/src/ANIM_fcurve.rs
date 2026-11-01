/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! F-Curve query, keyframe settings, and creation descriptors.

use blender_dna::FCurve;

/// Information needed to look up or create an FCurve.
#[derive(Debug, Clone)]
pub struct FCurveDescriptor {
    pub rna_path: String,
    pub array_index: i32,
    pub channel_group: Option<String>,
}

/// Settings applied when inserting keyframes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyframeSettings {
    pub keyframe_type: i16,
    pub handle: i16,
    pub interpolation: i16,
}

impl Default for KeyframeSettings {
    fn default() -> Self {
        Self {
            keyframe_type: 0,
            handle: 0,
            interpolation: 0,
        }
    }
}

pub fn get_keyframe_settings(_from_userprefs: bool) -> KeyframeSettings {
    KeyframeSettings::default()
}

pub fn fcurve_find<'a>(
    _fcurves: &'a [*const FCurve],
    _descriptor: &FCurveDescriptor,
) -> Option<*const FCurve> {
    None
}
