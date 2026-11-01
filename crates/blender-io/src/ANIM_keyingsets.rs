/* SPDX-FileCopyrightText: 2024 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Keying set identifiers and registration structures.

pub const ANIM_KS_LOCATION_ID: &str = "Location";
pub const ANIM_KS_ROTATION_ID: &str = "Rotation";
pub const ANIM_KS_SCALING_ID: &str = "Scaling";
pub const ANIM_KS_LOC_ROT_SCALE_ID: &str = "LocRotScale";
pub const ANIM_KS_LOC_ROT_SCALE_CPROP_ID: &str = "LocRotScaleCProp";
pub const ANIM_KS_AVAILABLE_ID: &str = "Available";
pub const ANIM_KS_WHOLE_CHARACTER_ID: &str = "WholeCharacter";
pub const ANIM_KS_WHOLE_CHARACTER_SELECTED_ID: &str = "WholeCharacterSelected";

/// Metadata for procedural/builtin keying set handlers.
#[derive(Debug, Clone)]
pub struct KeyingSetInfo {
    pub idname: String,
    pub name: String,
    pub description: String,
}

impl KeyingSetInfo {
    pub fn new(idname: &str, name: &str, description: &str) -> Self {
        Self {
            idname: idname.to_string(),
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}
