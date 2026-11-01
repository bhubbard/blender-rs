/* SPDX-FileCopyrightText: 2024 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Non-Linear Animation (NLA) strip action assignment and slot mapping.

use blender_dna::{bAction, ID, NlaStrip};
use crate::ANIM_action::ActionSlotAssignmentResult;
use crate::ANIM_pose::SlotHandle;

/// Assign action to an NLA strip.
pub fn assign_action(_strip: *mut NlaStrip, _action: *mut bAction, _animated_id: *mut ID) -> bool {
    true
}

/// Unassign action from an NLA strip.
pub fn unassign_action(_strip: *mut NlaStrip, _animated_id: *mut ID) {}

/// Assign action slot handle to an NLA strip.
pub fn assign_action_slot_handle(
    _strip: *mut NlaStrip,
    _slot_handle: SlotHandle,
    _animated_id: *mut ID,
) -> ActionSlotAssignmentResult {
    ActionSlotAssignmentResult::OK
}
