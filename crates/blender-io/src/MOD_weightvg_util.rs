/* SPDX-FileCopyrightText: 2011 by Bastien Montagne. All rights reserved.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::ffi::c_void;
use std::os::raw::c_char;
use blender_dna::{Mesh, Object, Scene};
use blender_dna::DNA_meshdata_types::{MDeformVert, MDeformWeight};
use blender_dna::DNA_texture_types::Tex;
use crate::MOD_util::ModifierEvalContext;

pub const MOD_WVG_ZEROFLOOR: f32 = 1.0e-32;

extern "C" {
    pub fn weightvg_do_map(
        num: i32,
        new_w: *mut f32,
        falloff_type: i16,
        do_invert: bool,
        cmap: *mut c_void,
        rng: *mut c_void,
    );
    pub fn weightvg_do_mask(
        ctx: *const ModifierEvalContext,
        num: i32,
        indices: *const i32,
        org_w: *mut f32,
        new_w: *const f32,
        ob: *mut Object,
        mesh: *mut Mesh,
        fact: f32,
        defgrp_name: *const c_char,
        scene: *mut Scene,
        texture: *mut Tex,
        tex_use_channel: i32,
        tex_mapping: i32,
        tex_map_object: *mut Object,
        text_map_bone: *const c_char,
        tex_uvlayer_name: *const c_char,
        invert_vgroup_mask: bool,
    );
    pub fn weightvg_update_vg(
        dvert: *mut MDeformVert,
        defgrp_idx: i32,
        dws: *mut *mut MDeformWeight,
        num: i32,
        indices: *const i32,
        weights: *const f32,
        do_add: bool,
        add_thresh: f32,
        do_rem: bool,
        rem_thresh: f32,
        do_normalize: bool,
    );
    pub fn weightvg_ui_common(
        c: *const c_void,
        ob_ptr: *mut c_void,
        ptr: *mut c_void,
        layout: *mut c_void,
    );
}
