/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::ffi::c_void;
use blender_dna::{Object, NodesModifierData};
use crate::DEG_depsgraph::Main;

#[repr(C)]
pub struct NodesModifierDataBlock {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct NodesModifierPackedBake {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct NodesModifierBake {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct NodesModifierUsageInferenceCache {
    input_values_hash: u64,
}

#[repr(C)]
pub struct NodesModifierRuntime {
    pub eval_log: *mut c_void,
    pub cache: *mut c_void,
    pub usage_cache: NodesModifierUsageInferenceCache,
}

extern "C" {
    pub fn MOD_nodes_update_interface(bmain: *mut Main, object: *mut Object, nmd: *mut NodesModifierData);
    pub fn nodes_modifier_data_block_destruct(data_block: *mut NodesModifierDataBlock, do_id_user: bool);
    pub fn nodes_modifier_packed_bake_copy(bake_dst: *mut NodesModifierBake, bake_src: *const NodesModifierBake);
    pub fn nodes_modifier_packed_bake_free(packed_bake: *mut NodesModifierPackedBake);
    pub fn nodes_modifier_bake_destruct(bake: *mut NodesModifierBake, do_id_user: bool);
}
