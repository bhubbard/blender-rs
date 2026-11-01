/* SPDX-FileCopyrightText: 2024 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::ffi::c_void;
use blender_dna::Object;

#[repr(C)]
pub struct GreasePencilModifierInfluenceData {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct LayerDrawingInfo {
    pub drawing: *mut c_void,
    pub layer_index: i32,
}

#[repr(C)]
pub struct FrameDrawingInfo {
    pub drawing: *mut c_void,
    pub start_frame_number: i32,
}

pub type IDWalkFunc = Option<unsafe extern "C" fn(user_data: *mut c_void, id: *mut c_void)>;

extern "C" {
    pub fn init_influence_data(
        influence_data: *mut GreasePencilModifierInfluenceData,
        has_custom_curve: bool,
    );
    pub fn copy_influence_data(
        influence_data_src: *const GreasePencilModifierInfluenceData,
        influence_data_dst: *mut GreasePencilModifierInfluenceData,
        flag: i32,
    );
    pub fn free_influence_data(influence_data: *mut GreasePencilModifierInfluenceData);
    pub fn foreach_influence_ID_link(
        influence_data: *mut GreasePencilModifierInfluenceData,
        ob: *mut Object,
        walk: IDWalkFunc,
        user_data: *mut c_void,
    );
    pub fn write_influence_data(
        writer: *mut c_void,
        influence_data: *const GreasePencilModifierInfluenceData,
    );
    pub fn read_influence_data(
        reader: *mut c_void,
        influence_data: *mut GreasePencilModifierInfluenceData,
    );
    pub fn draw_layer_filter_settings(c: *const c_void, layout: *mut c_void, ptr: *mut c_void);
    pub fn draw_material_filter_settings(c: *const c_void, layout: *mut c_void, ptr: *mut c_void);
    pub fn draw_vertex_group_settings(c: *const c_void, layout: *mut c_void, ptr: *mut c_void);
    pub fn draw_custom_curve_settings(c: *const c_void, layout: *mut c_void, ptr: *mut c_void);
}
