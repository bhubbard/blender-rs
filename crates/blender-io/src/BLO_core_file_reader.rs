/* SPDX-FileCopyrightText: 2025 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::ffi::c_void;

#[repr(C)]
pub struct FileReader {
    _unused: [u8; 0],
}

extern "C" {
    pub fn BLO_file_reader_uncompressed(rawfile: *mut FileReader) -> *mut FileReader;
    pub fn BLO_file_reader_uncompressed_from_descriptor(filedes: i32) -> *mut FileReader;
    pub fn BLO_file_reader_uncompressed_from_memory(
        mem: *const c_void,
        memsize: i32,
    ) -> *mut FileReader;
}
