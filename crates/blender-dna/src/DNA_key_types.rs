//! Auto-transpiled C/C++ header module: DNA_key_types

use core::ffi::c_void;
use crate::*;

pub const KEYELEM_FLOAT_LEN_COORD: i32 = 3;
pub const KEYELEM_ELEM_SIZE_CURVE: i32 = 3;
pub const KEYELEM_ELEM_LEN_BPOINT: i32 = 2;
pub const KEYELEM_ELEM_LEN_BEZTRIPLE: i32 = 4;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct KeyBlock {
    pub next: *mut KeyBlock,
    pub pos: f32,
    pub curval: f32,
    pub r#type: KeyInterpolationType,
    pub _pad1: [i8; 2],
    pub relative: i16,
    pub flag: KeyBlockFlag,
    pub totelem: i32,
    pub uid: i32,
    pub data: *mut core::ffi::c_void,
    pub name: [i8; 64],
    pub vgroup: [i8; 64],
    pub slidermin: f32,
    pub slidermax: f32,
}

impl Default for KeyBlock {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Key {
    pub id: ID,
    pub adt: *mut AnimData,
    pub refkey: *mut KeyBlock,
    pub elemstr: [i8; 32],
    pub elemsize: i32,
    pub _pad: [i8; 4],
    pub block: ListBaseT<KeyBlock>,
    pub from: *mut ID,
    pub totkey: i32,
    pub flag: ShapekeyContainerFlag,
    pub r#type: ShapekeyContainerType,
    pub _pad2: i8,
    pub ctime: f32,
    pub uidgen: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnimData {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct that {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapekeyContainerType {
    KEY_NORMAL = 0,
    KEY_RELATIVE = 1,
}

impl Default for ShapekeyContainerType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const KEY_NORMAL: i32 = ShapekeyContainerType::KEY_NORMAL as i32;
pub const KEY_RELATIVE: i32 = ShapekeyContainerType::KEY_RELATIVE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapekeyContainerFlag {
    KEY_DS_EXPAND = 1,
}

impl Default for ShapekeyContainerFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const KEY_DS_EXPAND: i32 = ShapekeyContainerFlag::KEY_DS_EXPAND as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyInterpolationType {
    KEY_LINEAR = 0,
    KEY_CARDINAL = 1,
    KEY_BSPLINE = 2,
    KEY_CATMULL_ROM = 3,
}

impl Default for KeyInterpolationType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const KEY_LINEAR: i32 = KeyInterpolationType::KEY_LINEAR as i32;
pub const KEY_CARDINAL: i32 = KeyInterpolationType::KEY_CARDINAL as i32;
pub const KEY_BSPLINE: i32 = KeyInterpolationType::KEY_BSPLINE as i32;
pub const KEY_CATMULL_ROM: i32 = KeyInterpolationType::KEY_CATMULL_ROM as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyBlockFlag {
    KEYBLOCK_MUTE = (1 << 0),
    KEYBLOCK_SEL = (1 << 1),
    KEYBLOCK_LOCKED = (1 << 2),
    KEYBLOCK_LOCKED_SHAPE = (1 << 3),
}

impl Default for KeyBlockFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const KEYBLOCK_MUTE: i32 = KeyBlockFlag::KEYBLOCK_MUTE as i32;
pub const KEYBLOCK_SEL: i32 = KeyBlockFlag::KEYBLOCK_SEL as i32;
pub const KEYBLOCK_LOCKED: i32 = KeyBlockFlag::KEYBLOCK_LOCKED as i32;
pub const KEYBLOCK_LOCKED_SHAPE: i32 = KeyBlockFlag::KEYBLOCK_LOCKED_SHAPE as i32;

