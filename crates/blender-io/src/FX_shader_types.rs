/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::os::raw::c_char;

#[repr(C)]
pub struct ShaderFxTypeInfo {
    pub idname: [c_char; 64],
    pub name: [c_char; 64],
    pub struct_name: [c_char; 64],
    pub struct_size: i32,
    pub rna_type: *mut std::ffi::c_void,
    pub fx_type: i32,
    pub flags: i32,
}

extern "C" {
    pub static mut shaderfx_Type_None: ShaderFxTypeInfo;
    pub static mut shaderfx_Type_Blur: ShaderFxTypeInfo;
    pub static mut shaderfx_Type_Colorize: ShaderFxTypeInfo;
    pub static mut shaderfx_Type_Flip: ShaderFxTypeInfo;
    pub static mut shaderfx_Type_Glow: ShaderFxTypeInfo;
    pub static mut shaderfx_Type_Pixel: ShaderFxTypeInfo;
    pub static mut shaderfx_Type_Rim: ShaderFxTypeInfo;
    pub static mut shaderfx_Type_Shadow: ShaderFxTypeInfo;
    pub static mut shaderfx_Type_Swirl: ShaderFxTypeInfo;
    pub static mut shaderfx_Type_Wave: ShaderFxTypeInfo;

    pub fn shaderfx_type_init(types: *mut *mut ShaderFxTypeInfo);
}
