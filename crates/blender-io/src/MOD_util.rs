/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::os::raw::c_char;
use blender_dna::{Mesh, ModifierData, Object};
use blender_dna::DNA_meshdata_types::MDeformVert;

#[repr(C)]
pub struct MappingInfoModifierData {
    pub modifier: ModifierData,
    pub texmapping: i32,
    pub map_object: *mut Object,
    pub map_bone: [c_char; 64],
    pub uvlayer_name: [c_char; 64],
}

#[repr(C)]
pub struct ModifierEvalContext {
    _unused: [u8; 0],
}

extern "C" {
    pub fn MOD_init_texture(dmd: *mut MappingInfoModifierData, ctx: *const ModifierEvalContext);
    pub fn MOD_get_texture_coords(
        dmd: *mut MappingInfoModifierData,
        ctx: *const ModifierEvalContext,
        ob: *mut Object,
        mesh: *mut Mesh,
        cos: *mut [f32; 3],
        r_texco: *mut [f32; 3],
    );
    pub fn MOD_previous_vcos_store(md: *mut ModifierData, vert_coords: *const [f32; 3]);
    pub fn MOD_get_vgroup(
        ob: *const Object,
        mesh: *const Mesh,
        name: *const c_char,
        dvert: *mut *const MDeformVert,
    );
}
