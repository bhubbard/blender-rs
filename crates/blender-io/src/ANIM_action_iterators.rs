/* SPDX-FileCopyrightText: 2024 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Action iteration helpers for traversing layered and legacy animation curves.

use blender_dna::{bAction, FCurve};
use crate::ANIM_pose::SlotHandle;

/// Iterate over all FCurves in an action.
pub fn foreach_fcurve_in_action<F>(_action: *mut bAction, mut _callback: F)
where
    F: FnMut(*mut FCurve),
{
}

/// Iterate over all FCurves for a specific action slot handle.
pub fn foreach_fcurve_in_action_slot<F>(_action: *mut bAction, _handle: SlotHandle, mut _callback: F)
where
    F: FnMut(*mut FCurve),
{
}

/// Iterate over editable FCurves for a specific action slot handle.
pub fn foreach_fcurve_in_action_slot_editable<F>(
    _action: *mut bAction,
    _handle: SlotHandle,
    mut _callback: F,
) where
    F: FnMut(*mut FCurve),
{
}
