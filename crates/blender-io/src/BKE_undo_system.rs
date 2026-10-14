//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[allow(non_camel_case_types)]
type int32_t = i32;
#[allow(non_camel_case_types)]
type uint32_t = u32;
#[allow(non_camel_case_types)]
type int16_t = i16;
#[allow(non_camel_case_types)]
type uint16_t = u16;
#[allow(non_camel_case_types)]
type int64_t = i64;
#[allow(non_camel_case_types)]
type uint64_t = u64;
#[allow(non_camel_case_types)]
type int8_t = i8;
#[allow(non_camel_case_types)]
type uint8_t = u8;
#[allow(non_camel_case_types)]
type uchar = u8;
#[allow(non_camel_case_types)]
type ushort = u16;
#[allow(non_camel_case_types)]
type uint = u32;
#[allow(non_camel_case_types)]
type ulong = u64;
#[allow(non_camel_case_types)]
type int = i32;
#[allow(non_camel_case_types)]
type UString = String;
#[allow(non_camel_case_types)]
type PropertyFlag = u32;
#[allow(non_camel_case_types)]
type PropertyOverrideFlag = u32;
#[allow(non_camel_case_types)]
type ParameterFlag = u32;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct UndoEncodeHints(pub i32);

impl UndoEncodeHints {
    pub const None: Self = Self((0) as i32);
    pub const PreMemFileChanges: Self = Self((1 << 1) as i32);
    pub const HasMemFileChangesOnly: Self = Self((1 << 2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eUndoStepDir(pub i32);

impl eUndoStepDir {
    pub const STEP_REDO: Self = Self((1) as i32);
    pub const STEP_UNDO: Self = Self((-1) as i32);
    pub const STEP_INVALID: Self = Self((0) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eUndoPushReturn(pub i32);

impl eUndoPushReturn {
    pub const UNDO_PUSH_RET_FAILURE: Self = Self((0) as i32);
    pub const UNDO_PUSH_RET_SUCCESS: Self = Self(((1 << 0)) as i32);
    pub const UNDO_PUSH_RET_OVERRIDE_CHANGED: Self = Self(((1 << 1)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eUndoTypeFlags(pub i32);

impl eUndoTypeFlags {
    pub const UNDOTYPE_FLAG_NEED_CONTEXT_FOR_ENCODE: Self = Self((1 << 0) as i32);
    pub const UNDOTYPE_FLAG_DECODE_ACTIVE_STEP: Self = Self((1 << 1) as i32);
    pub const UNDOTYPE_FLAG_ENCODE_PRE_MEMFILE_SUPPORTED: Self = Self((1 << 2) as i32);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct UndoRefID {
    pub ptr: *mut core::ffi::c_void,
    pub name: [u8; MAX_ID_NAME],
    pub library_filepath_abs: [u8; FILE_MAX],
}

impl Default for UndoRefID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct UndoStack {
    pub steps: ListBaseT<UndoStep>,
    pub step_active: *mut core::ffi::c_void,
    pub step_active_memfile: *mut core::ffi::c_void,
    pub step_init: *mut core::ffi::c_void,
    pub group_level: i32,
}

impl Default for UndoStack {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct UndoStep {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub name: [u8; 64],
    pub r#type: *mut core::ffi::c_void,
    pub data_size: usize,
    pub skip: bool,
    pub use_memfile_step: bool,
    pub use_old_bmain_data: bool,
    pub is_applied: bool,
}

impl Default for UndoStep {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct UndoType {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub identifier: *mut core::ffi::c_void,
    pub flags: u32,
    pub step_size: usize,
}

impl Default for UndoType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

