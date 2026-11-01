/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::os::raw::c_char;

#[repr(C)]
pub struct ModifierTypeInfo {
    pub idname: [c_char; 64],
    pub name: [c_char; 64],
    pub struct_name: [c_char; 64],
    pub struct_size: i32,
    pub rna_type: *mut std::ffi::c_void,
    pub modifier_type: i32,
    pub flags: i32,
}

extern "C" {
    pub static mut modifierType_None: ModifierTypeInfo;
    pub static mut modifierType_Subsurf: ModifierTypeInfo;
    pub static mut modifierType_Lattice: ModifierTypeInfo;
    pub static mut modifierType_Curve: ModifierTypeInfo;
    pub static mut modifierType_Build: ModifierTypeInfo;
    pub static mut modifierType_Mirror: ModifierTypeInfo;
    pub static mut modifierType_Decimate: ModifierTypeInfo;
    pub static mut modifierType_Wave: ModifierTypeInfo;
    pub static mut modifierType_Armature: ModifierTypeInfo;
    pub static mut modifierType_Hook: ModifierTypeInfo;
    pub static mut modifierType_Softbody: ModifierTypeInfo;
    pub static mut modifierType_Boolean: ModifierTypeInfo;
    pub static mut modifierType_Array: ModifierTypeInfo;
    pub static mut modifierType_EdgeSplit: ModifierTypeInfo;
    pub static mut modifierType_Displace: ModifierTypeInfo;
    pub static mut modifierType_UVProject: ModifierTypeInfo;
    pub static mut modifierType_Smooth: ModifierTypeInfo;
    pub static mut modifierType_Cast: ModifierTypeInfo;
    pub static mut modifierType_MeshDeform: ModifierTypeInfo;
    pub static mut modifierType_ParticleSystem: ModifierTypeInfo;
    pub static mut modifierType_ParticleInstance: ModifierTypeInfo;
    pub static mut modifierType_Explode: ModifierTypeInfo;
    pub static mut modifierType_Cloth: ModifierTypeInfo;
    pub static mut modifierType_Collision: ModifierTypeInfo;
    pub static mut modifierType_Bevel: ModifierTypeInfo;
    pub static mut modifierType_Shrinkwrap: ModifierTypeInfo;
    pub static mut modifierType_Fluidsim: ModifierTypeInfo;
    pub static mut modifierType_Mask: ModifierTypeInfo;
    pub static mut modifierType_SimpleDeform: ModifierTypeInfo;
    pub static mut modifierType_Multires: ModifierTypeInfo;
    pub static mut modifierType_Surface: ModifierTypeInfo;
    pub static mut modifierType_Fluid: ModifierTypeInfo;
    pub static mut modifierType_ShapeKey: ModifierTypeInfo;
    pub static mut modifierType_Solidify: ModifierTypeInfo;
}
