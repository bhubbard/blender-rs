/* SPDX-FileCopyrightText: 2025 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use crate::BLO_core_bhead::BHeadType;
use std::ffi::c_void;

pub const BLEND_FILE_FORMAT_VERSION_0: i32 = 0;
pub const BLEND_FILE_FORMAT_VERSION_1: i32 = 1;
pub const MIN_SIZEOFBLENDERHEADER: usize = 12;
pub const MAX_SIZEOFBLENDERHEADER: usize = 17;
pub const SIZEOFBLENDERHEADER_VERSION_0: usize = 12;
pub const SIZEOFBLENDERHEADER_VERSION_1: usize = 17;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlenderHeader {
    pub pointer_size: i32,
    pub endian: i32,
    pub file_version: i32,
    pub file_format_version: i32,
}

impl BlenderHeader {
    pub fn bhead_type(&self) -> BHeadType {
        if self.file_format_version == BLEND_FILE_FORMAT_VERSION_1 {
            BHeadType::LargeBHead8
        } else if self.pointer_size == 4 {
            BHeadType::BHead4
        } else {
            BHeadType::SmallBHead8
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlenderHeaderUnknown;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlenderHeaderInvalid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlenderHeaderVariant {
    Invalid(BlenderHeaderInvalid),
    Unknown(BlenderHeaderUnknown),
    Valid(BlenderHeader),
}

extern "C" {
    pub fn BLO_readfile_blender_header_decode(file: *mut c_void) -> BlenderHeaderVariant;
}
