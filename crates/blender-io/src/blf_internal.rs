/* SPDX-FileCopyrightText: 2009 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use crate::blf_internal_types::{FontBLF, GlyphBLF, GlyphCacheBLF};

pub const BLF_MAX_FONT: usize = 64;
pub const BLF_CACHE_MAX_FACES: usize = 8;
pub const BLF_CACHE_MAX_SIZES: usize = 16;

extern "C" {
    pub fn blf_font_new(
        name: *const std::os::raw::c_char,
        filename: *const std::os::raw::c_char,
    ) -> *mut FontBLF;
    pub fn blf_font_free(font: *mut FontBLF);
    pub fn blf_glyph_cache_find(font: *mut FontBLF, size: f32, dpi: u32) -> *mut GlyphCacheBLF;
    pub fn blf_glyph_cache_new(font: *mut FontBLF) -> *mut GlyphCacheBLF;
    pub fn blf_glyph_find(font: *mut FontBLF, c: u32) -> *mut GlyphBLF;
}
