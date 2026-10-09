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
pub struct ColorManagedDisplaySpace(pub i32);

impl ColorManagedDisplaySpace {
    pub const DISPLAY_SPACE_DRAW: Self = Self(0 as i32);
    pub const DISPLAY_SPACE_IMAGE_OUTPUT: Self = Self(1 as i32);
    pub const DISPLAY_SPACE_VIDEO_OUTPUT: Self = Self(2 as i32);
    pub const DISPLAY_SPACE_COLOR_INSPECTION: Self = Self(3 as i32);
    pub const DISPLAY_SPACE_SCOPE: Self = Self(4 as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ColorManagedFileOutput(pub i32);

impl ColorManagedFileOutput {
    pub const Image: Self = Self(0 as i32);
    pub const Video: Self = Self(1 as i32);
}

