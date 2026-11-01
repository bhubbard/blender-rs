/* SPDX-FileCopyrightText: 2006 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::ffi::c_void;
use blender_dna::DNA_texture_types::MTex;

extern "C" {
    pub fn RE_texture_evaluate(
        mtex: *const MTex,
        vec: *const [f32; 3],
        thread: i32,
        pool: *mut c_void,
        skip_load_image: bool,
        texnode_preview: bool,
        r_intensity: *mut f32,
        r_rgba: *mut [f32; 4],
    ) -> bool;
}
