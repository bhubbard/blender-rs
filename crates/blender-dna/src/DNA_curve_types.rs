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

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BevPoint {
    pub vec: [f32; 3],
    pub tilt: f32,
    pub radius: f32,
    pub weight: f32,
    pub offset: f32,
    pub sina: f32,
    pub cosa: f32,
    pub dir: [f32; 3],
    pub tan: [f32; 3],
    pub quat: [f32; 4],
    pub dupe_tag: i16,
}

impl Default for BevPoint {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BevList {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub nr: i32,
    pub dupe_nr: i32,
    pub poly: i32,
    pub hole: i32,
    pub reversed: bool,
    pub charidx: i32,
    pub segbevcount: *mut core::ffi::c_void,
    pub seglen: *mut core::ffi::c_void,
    pub bevpoints: *mut core::ffi::c_void,
}

impl Default for BevList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BezTriple {
    pub vec: [[f32; 3]; 3],
    pub tilt: f32,
    pub weight: f32,
    pub radius: f32,
    pub ipo: eBezTriple_Interpolation,
    pub h1: eBezTriple_Handle,
    pub h2: eBezTriple_Handle,
    pub f1: eBezTriple_Flag,
    pub f2: eBezTriple_Flag,
    pub f3: eBezTriple_Flag,
    pub hide: i8,
    pub easing: eBezTriple_Easing,
    pub back: f32,
    pub amplitude: f32,
    pub period: f32,
    pub auto_handle_type: eBezTriple_Auto_Type,
    pub _pad: [u8; 3],
}

impl Default for BezTriple {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BPoint {
    pub vec: [f32; 4],
    pub tilt: f32,
    pub weight: f32,
    pub f1: u8,
    pub _pad1: [u8; 1],
    pub hide: i16,
    pub radius: f32,
    pub _pad: [u8; 4],
}

impl Default for BPoint {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Nurb {
    pub r#type: eNurbType,
    pub mat_nr: i16,
    pub hide: i16,
    pub flag: eNurbFlag,
}

impl Default for Nurb {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CharInfo {
    pub kern: f32,
    pub mat_nr: i16,
    pub flag: eCharInfoFlag,
}

impl Default for CharInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct TextBox {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Default for TextBox {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct EditNurb {

}

impl Default for EditNurb {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Curve {
    pub adt: *mut core::ffi::c_void,
    pub nurb: ListBaseT<Nurb>,
    pub nullptr: ListBaseT<Nurb>,
}

impl Default for Curve {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

