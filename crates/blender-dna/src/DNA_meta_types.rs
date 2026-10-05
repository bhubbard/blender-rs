//! Auto-transpiled C/C++ header module: DNA_meta_types

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MetaElem {
    pub next: *mut MetaElem,
    pub bb: *mut BoundBox,
    pub r#type: eMetaElem_Type,
    pub flag: eMetaElem_Flag,
    pub _pad: [i8; 4],
    pub x: f32,
    pub quat: [f32; 4],
    pub expx: f32,
    pub expy: f32,
    pub expz: f32,
    pub rad: f32,
    pub rad2: f32,
    pub s: f32,
    pub len: f32,
    pub mat: *mut f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MetaBall {
    pub id: ID,
    pub adt: *mut AnimData,
    pub elems: ListBaseT<MetaElem>,
    pub editelems: *mut ListBaseT<MetaElem>,
    pub mat: *mut *mut Material,
    pub flag: eMetaBall_Flag,
    pub flag2: eMetaBall_Flag2,
    pub totcol: i16,
    pub texspace_flag: i8,
    pub _pad: [i8; 2],
    pub needs_flush_to_id: i8,
    pub texspace_location: [f32; 3],
    pub texspace_size: [f32; 3],
    pub wiresize: f32,
    pub thresh: f32,
    pub _pad0: [i8; 4],
    pub lastelem: *mut MetaElem,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnimData {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BoundBox {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Material {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMetaBall_TexSpaceFlag {
    MB_TEXSPACE_FLAG_AUTO = 1 << 0,
}

impl Default for eMetaBall_TexSpaceFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MB_TEXSPACE_FLAG_AUTO: i32 = eMetaBall_TexSpaceFlag::MB_TEXSPACE_FLAG_AUTO as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMetaBall_Flag {
    MB_UPDATE_ALWAYS = 0,
    MB_UPDATE_HALFRES = 1,
    MB_UPDATE_FAST = 2,
    MB_UPDATE_NEVER = 3,
}

impl Default for eMetaBall_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MB_UPDATE_ALWAYS: i32 = eMetaBall_Flag::MB_UPDATE_ALWAYS as i32;
pub const MB_UPDATE_HALFRES: i32 = eMetaBall_Flag::MB_UPDATE_HALFRES as i32;
pub const MB_UPDATE_FAST: i32 = eMetaBall_Flag::MB_UPDATE_FAST as i32;
pub const MB_UPDATE_NEVER: i32 = eMetaBall_Flag::MB_UPDATE_NEVER as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMetaBall_Flag2 {
    MB_DS_EXPAND = 1 << 0,
}

impl Default for eMetaBall_Flag2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MB_DS_EXPAND: i32 = eMetaBall_Flag2::MB_DS_EXPAND as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMetaElem_Type {
    MB_BALL = 0,
    MB_TUBEX = 1,
    MB_TUBEY = 2,
    MB_TUBEZ = 3,
    MB_TUBE = 4,
    MB_PLANE = 5,
    MB_ELIPSOID = 6,
    MB_CUBE = 7,
}

impl Default for eMetaElem_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MB_BALL: i32 = eMetaElem_Type::MB_BALL as i32;
pub const MB_TUBEX: i32 = eMetaElem_Type::MB_TUBEX as i32;
pub const MB_TUBEY: i32 = eMetaElem_Type::MB_TUBEY as i32;
pub const MB_TUBEZ: i32 = eMetaElem_Type::MB_TUBEZ as i32;
pub const MB_TUBE: i32 = eMetaElem_Type::MB_TUBE as i32;
pub const MB_PLANE: i32 = eMetaElem_Type::MB_PLANE as i32;
pub const MB_ELIPSOID: i32 = eMetaElem_Type::MB_ELIPSOID as i32;
pub const MB_CUBE: i32 = eMetaElem_Type::MB_CUBE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMetaElem_Flag {
    MB_SELECT = (1 << 0),
    MB_NEGATIVE = 1 << 1,
    MB_HIDE = 1 << 3,
    MB_SCALE_RAD = 1 << 4,
}

impl Default for eMetaElem_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MB_SELECT: i32 = eMetaElem_Flag::MB_SELECT as i32;
pub const MB_NEGATIVE: i32 = eMetaElem_Flag::MB_NEGATIVE as i32;
pub const MB_HIDE: i32 = eMetaElem_Flag::MB_HIDE as i32;
pub const MB_SCALE_RAD: i32 = eMetaElem_Flag::MB_SCALE_RAD as i32;

