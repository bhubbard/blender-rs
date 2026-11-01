/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::ffi::c_void;
use std::os::raw::c_char;

pub const FRAME_SNAP_EPS: f32 = 0.0001;

extern "C" {
    pub fn MOD_meshcache_read_mdd_index(
        fp: *mut c_void,
        vertex_cos: *mut [f32; 3],
        verts_tot: i32,
        index: i32,
        factor: f32,
        r_err_str: *mut *const c_char,
    ) -> bool;

    pub fn MOD_meshcache_read_mdd_frame(
        fp: *mut c_void,
        vertex_cos: *mut [f32; 3],
        verts_tot: i32,
        interp: c_char,
        frame: f32,
        r_err_str: *mut *const c_char,
    ) -> bool;

    pub fn MOD_meshcache_read_mdd_times(
        filepath: *const c_char,
        vertex_cos: *mut [f32; 3],
        verts_tot: i32,
        interp: c_char,
        time: f32,
        fps: f32,
        time_mode: c_char,
        r_err_str: *mut *const c_char,
    ) -> bool;

    pub fn MOD_meshcache_read_pc2_index(
        fp: *mut c_void,
        vertex_cos: *mut [f32; 3],
        verts_tot: i32,
        index: i32,
        factor: f32,
        r_err_str: *mut *const c_char,
    ) -> bool;

    pub fn MOD_meshcache_read_pc2_frame(
        fp: *mut c_void,
        vertex_cos: *mut [f32; 3],
        verts_tot: i32,
        interp: c_char,
        frame: f32,
        r_err_str: *mut *const c_char,
    ) -> bool;

    pub fn MOD_meshcache_read_pc2_times(
        filepath: *const c_char,
        vertex_cos: *mut [f32; 3],
        verts_tot: i32,
        interp: c_char,
        time: f32,
        fps: f32,
        time_mode: c_char,
        r_err_str: *mut *const c_char,
    ) -> bool;

    pub fn MOD_meshcache_calc_range(
        frame: f32,
        interp: c_char,
        frame_tot: i32,
        r_index_range: *mut [i32; 2],
        r_factor: *mut f32,
    );
}
