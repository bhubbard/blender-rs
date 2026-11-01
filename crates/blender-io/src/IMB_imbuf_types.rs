/* SPDX-FileCopyrightText: 2001-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Extended image buffer types, options, and flags.

pub const OPENEXR_HALF: i32 = 1 << 8;
pub const OPENEXR_MULTIPART: i32 = 1 << 9;
pub const OPENEXR_CODEC_MASK: i32 = 0xF;

pub const CINEON_LOG: i32 = 1 << 8;

pub const IB_RECT: i32 = 1 << 0;
pub const IB_RECT_FLOAT: i32 = 1 << 1;
pub const IB_ZBUF: i32 = 1 << 2;
pub const IB_ZBUF_FLOAT: i32 = 1 << 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImBufProfile {
    Linear = 0,
    SRGB = 1,
    Custom = 2,
}

#[derive(Debug, Clone)]
pub struct ImBufViewData {
    pub name: String,
    pub rect: Vec<u8>,
    pub rect_float: Vec<f32>,
}
