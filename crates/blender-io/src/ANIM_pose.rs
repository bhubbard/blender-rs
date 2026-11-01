/* SPDX-FileCopyrightText: 2024 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Pose evaluation, action blending, and bone animation application.

use blender_dna::{bAction, Object};

pub type SlotHandle = i32;

/// Opaque animation evaluation context.
#[repr(C)]
pub struct AnimationEvalContext {
    pub eval_time: f32,
}

/// Apply action to all bones ignoring selection state.
pub fn pose_apply_action_all_bones(
    _ob: *mut Object,
    _action: *mut bAction,
    _slot_handle: SlotHandle,
    _anim_eval_context: *const AnimationEvalContext,
) {
}

/// Evaluate action and blend the result into current pose based on blend_factor.
pub fn pose_apply_action_blend(
    _ob: *mut Object,
    _action: *mut bAction,
    _slot_handle: SlotHandle,
    _anim_eval_context: *const AnimationEvalContext,
    _blend_factor: f32,
) {
}

/// Apply action to all bones with blend factor regardless of selection.
pub fn pose_apply_action_blend_all_bones(
    _ob: *mut Object,
    _action: *mut bAction,
    _slot_handle: SlotHandle,
    _anim_eval_context: *const AnimationEvalContext,
    _blend_factor: f32,
) {
}
