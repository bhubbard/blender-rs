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

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CameraParams {
    pub is_ortho: bool,
    pub is_flipped_x: bool,
    pub lens: f32,
    pub ortho_scale: f32,
    pub zoom: f32,
    pub roll: f32,
    pub shiftx: f32,
    pub shifty: f32,
    pub offsetx: f32,
    pub offsety: f32,
    pub sensor_x: f32,
    pub sensor_y: f32,
    pub sensor_fit: i32,
    pub use_aspect_override: bool,
    pub aspect_override: f32,
    pub clip_start: f32,
    pub clip_end: f32,
    pub ycor: f32,
    pub viewdx: f32,
    pub viewdy: f32,
    pub viewplane: rctf,
    pub winmat: [[f32; 4]; 4],
}

impl Default for CameraParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

