/* SPDX-FileCopyrightText: 2008 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::ffi::c_void;

pub const BLF_VARIATIONS_MAX: usize = 16;

#[repr(C)]
pub struct FontBLF {
    pub name: [std::os::raw::c_char; 256],
    pub filename: [std::os::raw::c_char; 1024],
    pub glyph_cache: *mut c_void,
}

#[repr(C)]
pub struct GlyphCacheBLF {
    pub size: f32,
    pub dpi: u32,
}

#[repr(C)]
pub struct GlyphBLF {
    pub c: u32,
    pub width: i32,
    pub height: i32,
    pub advance: f32,
}
