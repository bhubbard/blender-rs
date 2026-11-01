/* SPDX-FileCopyrightText: 2025 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BHead {
    pub code: i32,
    pub sdnanr: i32,
    pub old: *const c_void,
    pub len: i64,
    pub nr: i64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BHead4 {
    pub code: i32,
    pub len: i32,
    pub old: u32,
    pub sdnanr: i32,
    pub nr: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SmallBHead8 {
    pub code: i32,
    pub len: i32,
    pub old: u64,
    pub sdnanr: i32,
    pub nr: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LargeBHead8 {
    pub code: i32,
    pub sdnanr: i32,
    pub old: u64,
    pub len: i64,
    pub nr: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BHeadType {
    BHead4,
    SmallBHead8,
    LargeBHead8,
}

pub const fn blend_make_id(a: u8, b: u8, c: u8, d: u8) -> i32 {
    #[cfg(target_endian = "big")]
    {
        ((a as i32) << 24) | ((b as i32) << 16) | ((c as i32) << 8) | (d as i32)
    }
    #[cfg(not(target_endian = "big"))]
    {
        ((d as i32) << 24) | ((c as i32) << 16) | ((b as i32) << 8) | (a as i32)
    }
}

pub const BLO_CODE_DATA: i32 = blend_make_id(b'D', b'A', b'T', b'A');
pub const BLO_CODE_GLOB: i32 = blend_make_id(b'G', b'L', b'O', b'B');
pub const BLO_CODE_DNA1: i32 = blend_make_id(b'D', b'N', b'A', b'1');
pub const BLO_CODE_TEST: i32 = blend_make_id(b'T', b'E', b'S', b'T');
pub const BLO_CODE_REND: i32 = blend_make_id(b'R', b'E', b'N', b'D');
pub const BLO_CODE_USER: i32 = blend_make_id(b'U', b'S', b'E', b'R');
pub const BLO_CODE_ENDB: i32 = blend_make_id(b'E', b'N', b'D', b'B');

#[inline]
pub fn uint32_from_uint64_ptr(ptr: u64) -> u32 {
    (ptr >> 3) as u32
}

extern "C" {
    pub fn BLO_readfile_read_bhead(file: *mut c_void, bhead_type: BHeadType) -> Option<BHead>;
}
