/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::ffi::c_void;
use blender_dna::{RenderData, Scene};
use crate::DEG_depsgraph::Main;
use crate::RE_pipeline::Render;

#[repr(C)]
pub struct CompositorInputData {
    pub render: *mut Render,
    pub main: *const Main,
    pub scene: *const Scene,
    pub render_data: *const RenderData,
    pub view_name: *const std::os::raw::c_char,
    pub render_context: *mut c_void,
    pub triggered_by_user: bool,
}

extern "C" {
    pub fn RE_compositor_execute(input_data: CompositorInputData);
    pub fn RE_compositor_free(render: *mut Render);
}
