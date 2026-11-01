/* SPDX-FileCopyrightText: 2001-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Internal rendering data structures, pipeline interfaces, tile highlighting, and z-buffer spans.

use std::collections::HashSet;
use std::os::raw::{c_char, c_void};
use std::sync::Mutex;
use blender_dna::{rcti, RenderData, Scene};
use crate::RE_compositor::CompositorInputData;
use crate::RE_pipeline::{Render, RenderResult};

/// Maximum coordinate range for vector passes.
pub const PASS_VECTOR_MAX: f32 = 10000.0;

/// Tile highlight coordinate tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileRect {
    pub xmin: i32,
    pub xmax: i32,
    pub ymin: i32,
    pub ymax: i32,
}

impl From<rcti> for TileRect {
    fn from(r: rcti) -> Self {
        Self {
            xmin: r.xmin,
            xmax: r.xmax,
            ymin: r.ymin,
            ymax: r.ymax,
        }
    }
}

impl Into<rcti> for TileRect {
    fn into(self) -> rcti {
        rcti {
            xmin: self.xmin,
            xmax: self.xmax,
            ymin: self.ymin,
            ymax: self.ymax,
        }
    }
}

/// Thread-safe active tile highlight manager.
#[derive(Default)]
pub struct TilesHighlight {
    tiles: Mutex<HashSet<TileRect>>,
}

impl TilesHighlight {
    pub fn new() -> Self {
        Self {
            tiles: Mutex::new(HashSet::new()),
        }
    }

    pub fn highlight_tile(&self, x: i32, y: i32, width: i32, height: i32) {
        let mut set = self.tiles.lock().unwrap();
        set.insert(TileRect {
            xmin: x,
            xmax: x + width,
            ymin: y,
            ymax: y + height,
        });
    }

    pub fn unhighlight_tile(&self, x: i32, y: i32, width: i32, height: i32) {
        let mut set = self.tiles.lock().unwrap();
        set.remove(&TileRect {
            xmin: x,
            xmax: x + width,
            ymin: y,
            ymax: y + height,
        });
    }

    pub fn clear(&self) {
        let mut set = self.tiles.lock().unwrap();
        set.clear();
    }

    pub fn get_all_highlighted_tiles(&self) -> Vec<rcti> {
        let set = self.tiles.lock().unwrap();
        set.iter().map(|&t| t.into()).collect()
    }
}

/// Rendering display callbacks and status hooks.
pub struct RenderDisplay {
    pub display_update: Option<Box<dyn FnMut(*mut RenderResult) + Send + Sync>>,
    pub stats_draw: Option<Box<dyn FnMut(&str) + Send + Sync>>,
    pub progress: Option<Box<dyn FnMut(f32) + Send + Sync>>,
}

impl Default for RenderDisplay {
    fn default() -> Self {
        Self {
            display_update: None,
            stats_draw: None,
            progress: None,
        }
    }
}

/// Abstract base render interface for full render and viewport renderers.
pub trait BaseRender: Send + Sync {
    fn get_tile_highlight(&self) -> Option<&TilesHighlight>;
    fn compositor_execute(&mut self, input_data: CompositorInputData);
    fn compositor_free(&mut self);
}

/// Span fill method used for localized rasterization and Z-buffering.
#[repr(C)]
pub struct ZSpan {
    pub rectx: i32,
    pub recty: i32,
    pub miny1: i32,
    pub maxy1: i32,
    pub miny2: i32,
    pub maxy2: i32,
    pub minp1: *const f32,
    pub maxp1: *const f32,
    pub minp2: *const f32,
    pub maxp2: *const f32,
    pub span1: *mut f32,
    pub span2: *mut f32,
}

impl Default for ZSpan {
    fn default() -> Self {
        Self {
            rectx: 0,
            recty: 0,
            miny1: 0,
            maxy1: 0,
            miny2: 0,
            maxy2: 0,
            minp1: std::ptr::null(),
            maxp1: std::ptr::null(),
            minp2: std::ptr::null(),
            maxp2: std::ptr::null(),
            span1: std::ptr::null_mut(),
            span2: std::ptr::null_mut(),
        }
    }
}

/// Brightness and contrast adjustment helpers for procedural textures.
pub fn bricont_sample(val: f32, bright: f32, contrast: f32, clamp: bool) -> f32 {
    let mut res = (val - 0.5) * contrast + bright - 0.5;
    if clamp {
        res = res.clamp(0.0, 1.0);
    }
    res
}

pub fn bricont_rgb_sample(rgba: &mut [f32; 4], bright: f32, contrast: f32, rfac: f32, gfac: f32, bfac: f32, clamp: bool) {
    rgba[0] = rfac * ((rgba[0] - 0.5) * contrast + bright - 0.5);
    rgba[1] = gfac * ((rgba[1] - 0.5) * contrast + bright - 0.5);
    rgba[2] = bfac * ((rgba[2] - 0.5) * contrast + bright - 0.5);
    if clamp {
        rgba[0] = rgba[0].clamp(0.0, 1.0);
        rgba[1] = rgba[1].clamp(0.0, 1.0);
        rgba[2] = rgba[2].clamp(0.0, 1.0);
    }
}

extern "C" {
    pub fn render_result_new(
        re: *mut Render,
        partrct: *const rcti,
        layername: *const c_char,
        viewname: *const c_char,
    ) -> *mut RenderResult;

    pub fn render_result_passes_allocated_ensure(rr: *mut RenderResult);

    pub fn render_copy_renderdata(to: *mut RenderData, from: *const RenderData);

    pub fn zbuf_alloc_span(zspan: *mut ZSpan, rectx: i32, recty: i32);
    pub fn zbuf_free_span(zspan: *mut ZSpan);
    pub fn zspan_rasterize_triangle(
        zspan: *mut ZSpan,
        handle: *mut c_void,
        v1: *const f32,
        v2: *const f32,
        v3: *const f32,
        func: Option<extern "C" fn(*mut c_void, i32, i32, f32, f32)>,
    );
}
