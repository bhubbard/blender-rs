/* SPDX-FileCopyrightText: 2010 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use blender_dna::{Mesh, MultiresModifierData};

#[repr(C)]
pub struct MultiresBakeRender {
    pub base_mesh: *mut Mesh,
    pub multires_modifier: *mut MultiresModifierData,
    pub bake_margin: i32,
    pub bake_margin_type: i32,
    pub type_: i32,
    pub displacement_space: i32,
    pub use_low_resolution_mesh: bool,
    pub num_total_objects: i32,
    pub num_baked_objects: i32,
}

extern "C" {
    pub fn RE_multires_bake_images(bake: *mut MultiresBakeRender);
}
