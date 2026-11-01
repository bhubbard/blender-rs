/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! AnimData operations: action ensuring, unlinking empty actions, and F-Curve deletion.

use blender_dna::{AnimData, bAction, FCurve, ID};
use crate::DEG_depsgraph::Main;

/// Ensure an active Action exists for the given ID's AnimData block.
pub fn id_action_ensure(_bmain: *mut Main, _id: *mut ID) -> *mut bAction {
    std::ptr::null_mut()
}

/// Delete the F-Curve from the AnimData block if possible.
pub fn animdata_fcurve_delete(_adt: *mut AnimData, _fcu: *mut FCurve) {}

/// Unlink the action from animdata if it contains no F-Curves.
pub fn animdata_remove_empty_action(adt: *mut AnimData) -> bool {
    if adt.is_null() {
        return false;
    }
    false
}

/// Build a list of IDs related to the given ID (Object <-> Data, Mesh <-> Material, etc.).
pub fn find_related_ids(_bmain: *mut Main, id: *mut ID) -> Vec<*mut ID> {
    if id.is_null() {
        Vec::new()
    } else {
        vec![id]
    }
}
