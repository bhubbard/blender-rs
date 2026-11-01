/* SPDX-FileCopyrightText: 2025 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::ffi::c_void;

#[repr(C)]
pub struct ThumbnailProvider {
    _unused: [u8; 0],
}

extern "C" {
    pub fn thumbnail_provider_generate(
        provider: *mut ThumbnailProvider,
        file_path: *const std::os::raw::c_char,
        width: i32,
        height: i32,
    ) -> *mut c_void;
}
