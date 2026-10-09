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
pub struct ImBufFlags(pub i32);

impl ImBufFlags {
    pub const Zero: Self = Self((0) as i32);
    pub const ByteData: Self = Self((1 << 0) as i32);
    pub const Test: Self = Self((1 << 1) as i32);
    pub const FloatData: Self = Self((1 << 5) as i32);
    pub const MultiLayer: Self = Self((1 << 7) as i32);
    pub const Metadata: Self = Self((1 << 8) as i32);
    pub const Deinterlace: Self = Self((1 << 9) as i32);
    pub const UninitializedPixels: Self = Self((1 << 10) as i32);
    pub const AlphaPremul: Self = Self((1 << 12) as i32);
    pub const AlphaDetect: Self = Self((1 << 13) as i32);
    pub const AlphaChannelPacked: Self = Self((1 << 14) as i32);
    pub const AlphaIgnore: Self = Self((1 << 15) as i32);
    pub const Thumbnail: Self = Self((1 << 16) as i32);
    pub const HasDisplayWindow: Self = Self((1 << 17) as i32);
    pub const NoColorspaceConvert: Self = Self((1 << 18) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eImbFileType(pub i8);

impl eImbFileType {
    pub const IMB_FTYPE_NONE: Self = Self((0) as i8);
    pub const IMB_FTYPE_PNG: Self = Self((1) as i8);
    pub const IMB_FTYPE_TGA: Self = Self((2) as i8);
    pub const IMB_FTYPE_JPG: Self = Self((3) as i8);
    pub const IMB_FTYPE_BMP: Self = Self((4) as i8);
    pub const IMB_FTYPE_OPENEXR: Self = Self((5) as i8);
    pub const IMB_FTYPE_IRIS: Self = Self((6) as i8);
    pub const IMB_FTYPE_PSD: Self = Self((7) as i8);
    pub const IMB_FTYPE_JP2: Self = Self((8) as i8);
    pub const IMB_FTYPE_RADHDR: Self = Self((9) as i8);
    pub const IMB_FTYPE_TIF: Self = Self((10) as i8);
    pub const IMB_FTYPE_CINEON: Self = Self((11) as i8);
    pub const IMB_FTYPE_DPX: Self = Self((12) as i8);
    pub const IMB_FTYPE_DDS: Self = Self((13) as i8);
    pub const IMB_FTYPE_WEBP: Self = Self((14) as i8);
    pub const IMB_FTYPE_AVIF: Self = Self((15) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eImFileTypeCapability(pub u8);

impl eImFileTypeCapability {
    pub const Zero: Self = Self((0) as u8);
    pub const File: Self = Self(((1 << 0)) as u8);
    pub const Memory: Self = Self(((1 << 1)) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct IMB_Proxy_Size(pub i32);

impl IMB_Proxy_Size {
    pub const IMB_PROXY_NONE: Self = Self((0) as i32);
    pub const IMB_PROXY_25: Self = Self((1) as i32);
    pub const IMB_PROXY_50: Self = Self((2) as i32);
    pub const IMB_PROXY_75: Self = Self((4) as i32);
    pub const IMB_PROXY_100: Self = Self((8) as i32);
    pub const IMB_PROXY_MAX_SLOT: Self = Self((4) as i32);
}

