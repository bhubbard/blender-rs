/* SPDX-FileCopyrightText: 2019 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::ffi::c_void;
use blender_dna::ListBase;
use crate::MOD_lineart::LineartStaticMemPool;

extern "C" {
    pub fn lineart_list_append_pointer_pool(
        h: *mut ListBase,
        smp: *mut LineartStaticMemPool,
        data: *mut c_void,
    ) -> *mut c_void;

    pub fn lineart_list_append_pointer_pool_sized(
        h: *mut ListBase,
        smp: *mut LineartStaticMemPool,
        data: *mut c_void,
        size: i32,
    ) -> *mut c_void;

    pub fn lineart_list_append_pointer_pool_thread(
        h: *mut ListBase,
        smp: *mut LineartStaticMemPool,
        data: *mut c_void,
    ) -> *mut c_void;

    pub fn lineart_list_append_pointer_pool_sized_thread(
        h: *mut ListBase,
        smp: *mut LineartStaticMemPool,
        data: *mut c_void,
        size: i32,
    ) -> *mut c_void;

    pub fn list_push_pointer_static(
        h: *mut ListBase,
        smp: *mut LineartStaticMemPool,
        p: *mut c_void,
    ) -> *mut c_void;
}
