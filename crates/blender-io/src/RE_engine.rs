/* SPDX-FileCopyrightText: 2006 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::os::raw::c_char;
use blender_dna::{Scene, ViewLayer};
use crate::DEG_depsgraph::Depsgraph;

pub const RE_INTERNAL: i32 = 1 << 0;
pub const RE_USE_PREVIEW: i32 = 1 << 1;
pub const RE_USE_POSTPROCESS: i32 = 1 << 2;
pub const RE_USE_EEVEE_VIEWPORT: i32 = 1 << 3;
pub const RE_USE_SHADING_NODES_CUSTOM: i32 = 1 << 4;
pub const RE_USE_SPHERICAL_STEREO: i32 = 1 << 5;

#[repr(C)]
pub struct RenderEngineType {
    pub idname: [c_char; 32],
    pub name: [c_char; 64],
    pub flag: i32,
}

#[repr(C)]
pub struct RenderEngine {
    pub engine_type: *mut RenderEngineType,
    pub flag: i32,
    pub camera_override: *mut blender_dna::Object,
}

extern "C" {
    pub fn RE_engine_create(type_: *mut RenderEngineType) -> *mut RenderEngine;
    pub fn RE_engine_free(engine: *mut RenderEngine);
    pub fn RE_engine_render(engine: *mut RenderEngine, depsgraph: *mut Depsgraph);
    pub fn RE_engine_update(
        engine: *mut RenderEngine,
        scene: *mut Scene,
        view_layer: *mut ViewLayer,
    );
}
