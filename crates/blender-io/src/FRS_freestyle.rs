/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::ffi::c_void;
use blender_dna::DNA_freestyle_types::FreestyleConfig;
use blender_dna::DNA_linestyle_types::FreestyleLineStyle;
use blender_dna::{Material, Scene};
use crate::DEG_depsgraph::Main;

#[repr(C)]
pub struct Render {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct ViewLayer {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct FreestyleGlobals {
    pub scene: *mut Scene,
    pub viewpoint: [f32; 3],
    pub mv: [[f32; 4]; 4],
    pub proj: [[f32; 4]; 4],
    pub viewport: [i32; 4],
}

extern "C" {
    pub static mut g_freestyle: FreestyleGlobals;

    pub fn FRS_init();
    pub fn FRS_set_context(c: *mut c_void);
    pub fn FRS_is_freestyle_enabled(view_layer: *mut ViewLayer) -> i32;
    pub fn FRS_init_stroke_renderer(re: *mut Render);
    pub fn FRS_begin_stroke_rendering(re: *mut Render);
    pub fn FRS_do_stroke_rendering(re: *mut Render, view_layer: *mut ViewLayer);
    pub fn FRS_end_stroke_rendering(re: *mut Render);
    pub fn FRS_free_view_map_cache();
    pub fn FRS_composite_result(
        re: *mut Render,
        view_layer: *mut ViewLayer,
        freestyle_render: *mut Render,
    );
    pub fn FRS_exit();

    pub fn FRS_copy_active_lineset(config: *mut FreestyleConfig);
    pub fn FRS_paste_active_lineset(config: *mut FreestyleConfig);
    pub fn FRS_delete_active_lineset(config: *mut FreestyleConfig);
    pub fn FRS_move_active_lineset(config: *mut FreestyleConfig, direction: i32) -> bool;

    pub fn FRS_create_stroke_material(
        bmain: *mut Main,
        linestyle: *mut FreestyleLineStyle,
    ) -> *mut Material;
}
