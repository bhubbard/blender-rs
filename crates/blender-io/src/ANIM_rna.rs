/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! RNA animation integration helpers: path extraction and rotation mode detection.

pub const ROT_MODE_QUAT: i32 = 0;
pub const ROT_MODE_EUL: i32 = 1;
pub const ROT_MODE_AXISANGLE: i32 = 2;

/// Get RNA path component for a rotation mode.
pub fn get_rotation_mode_path(rotation_mode: i32) -> &'static str {
    match rotation_mode {
        ROT_MODE_QUAT => "rotation_quaternion",
        ROT_MODE_AXISANGLE => "rotation_axis_angle",
        _ => "rotation_euler",
    }
}

/// Extract bone name from an RNA path like `pose.bones["Bone"].location`.
pub fn pose_bone_name_from_rna_path(path: &str) -> Option<String> {
    if let Some(start) = path.find("pose.bones[\"") {
        let sub = &path[start + 12..];
        if let Some(end) = sub.find("\"]") {
            return Some(sub[..end].to_string());
        }
    }
    None
}
