//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ebToolRef_Runtime_Flag(pub i32);

impl ebToolRef_Runtime_Flag {
    pub const TOOLREF_FLAG_FALLBACK_KEYMAP: Self = Self(((1 << 0)) as i32);
    pub const TOOLREF_FLAG_USE_BRUSHES: Self = Self(((1 << 1)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eWorkSpaceFlags(pub i32);

impl eWorkSpaceFlags {
    pub const WORKSPACE_USE_FILTER_BY_ORIGIN: Self = Self(((1 << 1)) as i32);
    pub const WORKSPACE_USE_PIN_SCENE: Self = Self(((1 << 2)) as i32);
    pub const WORKSPACE_SYNC_SCENE_TIME: Self = Self(((1 << 3)) as i32);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bToolRef_Runtime {
    pub cursor: i32,
    pub keymap: [u8; 64],
    pub gizmo_group: [u8; 64],
    pub data_block: [u8; 64],
    pub brush_type: i32,
    pub keymap_fallback: [u8; 64],
    pub op: [u8; 64],
    pub index: i32,
    pub flag: ebToolRef_Runtime_Flag,
}

impl Default for bToolRef_Runtime {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bToolRef {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub idname: [u8; 64],
    pub idname_fallback: [u8; 64],
    pub idname_pending: [u8; 64],
    pub tag: i16,
    pub space_type: i16,
    pub mode: i32,
    pub properties: *mut core::ffi::c_void,
    pub runtime: *mut core::ffi::c_void,
}

impl Default for bToolRef {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct WorkSpaceLayout {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub screen: *mut core::ffi::c_void,
    pub name: [u8; 64],
}

impl Default for WorkSpaceLayout {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct wmOwnerID {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub name: [u8; 128],
}

impl Default for wmOwnerID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct WorkSpace {
    pub layouts: ListBaseT<WorkSpaceLayout>,
    pub nullptr: ListBaseT<WorkSpaceLayout>,
}

impl Default for WorkSpace {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct WorkSpaceDataRelation {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub parent: *mut core::ffi::c_void,
    pub value: *mut core::ffi::c_void,
    pub parentid: i32,
    pub _pad_0: [u8; 4],
}

impl Default for WorkSpaceDataRelation {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct WorkSpaceInstanceHook {
    pub active: *mut core::ffi::c_void,
    pub act_layout: *mut core::ffi::c_void,
    pub temp_workspace_store: *mut core::ffi::c_void,
    pub temp_layout_store: *mut core::ffi::c_void,
}

impl Default for WorkSpaceInstanceHook {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

