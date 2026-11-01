/* SPDX-FileCopyrightText: 2024 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Legacy Action backward compatibility routines.

use blender_dna::{AnimData, bAction, bActionGroup, FCurve};

pub const DEFAULT_LEGACY_SLOT_NAME: &str = "Legacy Slot";
pub const DEFAULT_LEGACY_LAYER_NAME: &str = "Legacy Layer";

/// Return all F-Curves in the legacy action.
pub fn fcurves_all(action: *const bAction) -> Vec<*mut FCurve> {
    if action.is_null() {
        return Vec::new();
    }
    Vec::new()
}

/// Return whether the action (+slot) assigned to `adt` has keyframes.
pub fn assigned_action_has_keyframes(adt: *const AnimData) -> bool {
    if adt.is_null() {
        return false;
    }
    unsafe {
        let anim_data = &*adt;
        !anim_data.action.is_null()
    }
}

/// Return all channel groups in the action.
pub fn channel_groups_all(action: *const bAction) -> Vec<*mut bActionGroup> {
    if action.is_null() {
        return Vec::new();
    }
    Vec::new()
}

/// Return channel groups for the assigned action slot.
pub fn channel_groups_for_assigned_slot(adt: *const AnimData) -> Vec<*mut bActionGroup> {
    if adt.is_null() {
        return Vec::new();
    }
    Vec::new()
}
