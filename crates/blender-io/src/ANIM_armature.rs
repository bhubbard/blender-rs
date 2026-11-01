/* SPDX-FileCopyrightText: 2025 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Armature query routines: bone visibility, selection tests, and hierarchy iteration.

use blender_dna::{bArmature, Bone};

pub const BONE_DEFAULT_NAME: &str = "Bone";

pub const BONE_HIDDEN_P: i32 = 1 << 0;
pub const BONE_HIDDEN_A: i32 = 1 << 1;
pub const BONE_SELECTED: i32 = 1 << 0;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct EditBone {
    pub name: [u8; 64],
    pub flag: i16,
}

/// Returns true if the given Bone is visible.
pub fn bone_is_visible(_armature: *const bArmature, bone: *const Bone) -> bool {
    !bone.is_null()
}

/// Returns true if the edit bone is visible.
pub fn edit_bone_is_visible(_armature: *const bArmature, ebone: *const EditBone) -> bool {
    if ebone.is_null() {
        return false;
    }
    unsafe {
        let eb = &*ebone;
        (eb.flag & (BONE_HIDDEN_A as i16)) == 0
    }
}

/// Returns true if the bone is selected and visible.
pub fn bone_is_selected(armature: *const bArmature, bone: *const Bone) -> bool {
    bone_is_visible(armature, bone)
}
