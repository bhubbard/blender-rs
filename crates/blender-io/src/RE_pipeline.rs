/* SPDX-FileCopyrightText: 2006 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::os::raw::c_char;
use blender_dna::{RenderData, Scene, ViewLayer};
use crate::DEG_depsgraph::Main;

pub const RE_MAXNAME: usize = 64 + 10;

#[repr(C)]
pub struct RenderView {
    pub next: *mut RenderView,
    pub prev: *mut RenderView,
    pub name: [c_char; 64],
}

#[repr(C)]
pub struct RenderResult {
    pub rectf: *mut f32,
    pub rect32: *mut u32,
    pub rectx: i32,
    pub recty: i32,
}

#[repr(C)]
pub struct Render {
    _unused: [u8; 0],
}

extern "C" {
    pub fn RE_NewRender(name: *const c_char) -> *mut Render;
    pub fn RE_FreeRender(re: *mut Render);
    pub fn RE_InitRender(
        re: *mut Render,
        main: *mut Main,
        scene: *mut Scene,
        view_layer: *mut ViewLayer,
        rd: *mut RenderData,
        winx: i32,
        winy: i32,
    );
    pub fn RE_BlenderAnim(
        re: *mut Render,
        bmain: *mut Main,
        scene: *mut Scene,
        start: i32,
        end: i32,
    );
    pub fn RE_BlenderFrame(re: *mut Render, bmain: *mut Main, scene: *mut Scene, frame: i32);
    pub fn RE_AcquireResultRead(re: *mut Render) -> *mut RenderResult;
    pub fn RE_ReleaseResult(re: *mut Render);
}
