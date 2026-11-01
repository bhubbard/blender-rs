/* SPDX-FileCopyrightText: 2024 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::ffi::c_void;
use blender_dna::{Link, ListBaseT};

#[repr(C)]
pub struct LineartModifierRuntime {
    pub object_dependencies: *mut c_void,
}

#[repr(C)]
pub struct LineartStaticMemPoolNode {
    pub item: Link,
    pub size: usize,
    pub used_byte: usize,
}

#[repr(C)]
pub struct LineartStaticMemPool {
    pub pools: ListBaseT<LineartStaticMemPoolNode>,
    pub lock_mem: u32,
}

#[repr(C)]
pub struct LineartEdge {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct LineartTriangleAdjacent {
    pub e: [*mut LineartEdge; 3],
}

extern "C" {
    pub fn MOD_lineart_destroy_render_data(custom_data: *mut c_void);
}
