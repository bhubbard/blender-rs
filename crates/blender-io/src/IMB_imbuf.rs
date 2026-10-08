//! Auto-transpiled C/C++ header module: IMB_imbuf

use crate::*;

pub const FILTER_MASK_NULL: i32 = 0;
pub const FILTER_MASK_MARGIN: i32 = 1;
pub const FILTER_MASK_USED: i32 = 2;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImBuf {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct rctf {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct rcti {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ColorManagedColorspaceSettings {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImageFormatData {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Stereo3dFormat {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Changes {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IMBThumbLoadFlags {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IMBScaleFilter {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GPUTextureCreateFlags {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IMBThumbLoadFlags_2 {
    Zero = 0,
    LoadLargeFiles = (1 << 0),
}

impl Default for IMBThumbLoadFlags_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const Zero: i32 = IMBThumbLoadFlags_2::Zero as i32;
pub const LoadLargeFiles: i32 = IMBThumbLoadFlags_2::LoadLargeFiles as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IMB_BlendMode {
    IMB_BLEND_MIX = 0,
    IMB_BLEND_ADD = 1,
    IMB_BLEND_SUB = 2,
    IMB_BLEND_MUL = 3,
    IMB_BLEND_LIGHTEN = 4,
    IMB_BLEND_DARKEN = 5,
    IMB_BLEND_ERASE_ALPHA = 6,
    IMB_BLEND_ADD_ALPHA = 7,
    IMB_BLEND_OVERLAY = 8,
    IMB_BLEND_HARDLIGHT = 9,
    IMB_BLEND_COLORBURN = 10,
    IMB_BLEND_LINEARBURN = 11,
    IMB_BLEND_COLORDODGE = 12,
    IMB_BLEND_SCREEN = 13,
    IMB_BLEND_SOFTLIGHT = 14,
    IMB_BLEND_PINLIGHT = 15,
    IMB_BLEND_VIVIDLIGHT = 16,
    IMB_BLEND_LINEARLIGHT = 17,
    IMB_BLEND_DIFFERENCE = 18,
    IMB_BLEND_EXCLUSION = 19,
    IMB_BLEND_HUE = 20,
    IMB_BLEND_SATURATION = 21,
    IMB_BLEND_LUMINOSITY = 22,
    IMB_BLEND_COLOR = 23,
    IMB_BLEND_INTERPOLATE = 24,
    IMB_BLEND_COPY_RGB = 1001,
    IMB_BLEND_COPY_ALPHA = 1002,
}

impl Default for IMB_BlendMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const IMB_BLEND_MIX: i32 = IMB_BlendMode::IMB_BLEND_MIX as i32;
pub const IMB_BLEND_ADD: i32 = IMB_BlendMode::IMB_BLEND_ADD as i32;
pub const IMB_BLEND_SUB: i32 = IMB_BlendMode::IMB_BLEND_SUB as i32;
pub const IMB_BLEND_MUL: i32 = IMB_BlendMode::IMB_BLEND_MUL as i32;
pub const IMB_BLEND_LIGHTEN: i32 = IMB_BlendMode::IMB_BLEND_LIGHTEN as i32;
pub const IMB_BLEND_DARKEN: i32 = IMB_BlendMode::IMB_BLEND_DARKEN as i32;
pub const IMB_BLEND_ERASE_ALPHA: i32 = IMB_BlendMode::IMB_BLEND_ERASE_ALPHA as i32;
pub const IMB_BLEND_ADD_ALPHA: i32 = IMB_BlendMode::IMB_BLEND_ADD_ALPHA as i32;
pub const IMB_BLEND_OVERLAY: i32 = IMB_BlendMode::IMB_BLEND_OVERLAY as i32;
pub const IMB_BLEND_HARDLIGHT: i32 = IMB_BlendMode::IMB_BLEND_HARDLIGHT as i32;
pub const IMB_BLEND_COLORBURN: i32 = IMB_BlendMode::IMB_BLEND_COLORBURN as i32;
pub const IMB_BLEND_LINEARBURN: i32 = IMB_BlendMode::IMB_BLEND_LINEARBURN as i32;
pub const IMB_BLEND_COLORDODGE: i32 = IMB_BlendMode::IMB_BLEND_COLORDODGE as i32;
pub const IMB_BLEND_SCREEN: i32 = IMB_BlendMode::IMB_BLEND_SCREEN as i32;
pub const IMB_BLEND_SOFTLIGHT: i32 = IMB_BlendMode::IMB_BLEND_SOFTLIGHT as i32;
pub const IMB_BLEND_PINLIGHT: i32 = IMB_BlendMode::IMB_BLEND_PINLIGHT as i32;
pub const IMB_BLEND_VIVIDLIGHT: i32 = IMB_BlendMode::IMB_BLEND_VIVIDLIGHT as i32;
pub const IMB_BLEND_LINEARLIGHT: i32 = IMB_BlendMode::IMB_BLEND_LINEARLIGHT as i32;
pub const IMB_BLEND_DIFFERENCE: i32 = IMB_BlendMode::IMB_BLEND_DIFFERENCE as i32;
pub const IMB_BLEND_EXCLUSION: i32 = IMB_BlendMode::IMB_BLEND_EXCLUSION as i32;
pub const IMB_BLEND_HUE: i32 = IMB_BlendMode::IMB_BLEND_HUE as i32;
pub const IMB_BLEND_SATURATION: i32 = IMB_BlendMode::IMB_BLEND_SATURATION as i32;
pub const IMB_BLEND_LUMINOSITY: i32 = IMB_BlendMode::IMB_BLEND_LUMINOSITY as i32;
pub const IMB_BLEND_COLOR: i32 = IMB_BlendMode::IMB_BLEND_COLOR as i32;
pub const IMB_BLEND_INTERPOLATE: i32 = IMB_BlendMode::IMB_BLEND_INTERPOLATE as i32;
pub const IMB_BLEND_COPY_RGB: i32 = IMB_BlendMode::IMB_BLEND_COPY_RGB as i32;
pub const IMB_BLEND_COPY_ALPHA: i32 = IMB_BlendMode::IMB_BLEND_COPY_ALPHA as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eIMBInterpolationFilterMode {
    IMB_FILTER_NEAREST,
    IMB_FILTER_BILINEAR,
    IMB_FILTER_CUBIC_BSPLINE,
    IMB_FILTER_CUBIC_MITCHELL,
    IMB_FILTER_BOX,
}

impl Default for eIMBInterpolationFilterMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const IMB_FILTER_NEAREST: i32 = eIMBInterpolationFilterMode::IMB_FILTER_NEAREST as i32;
pub const IMB_FILTER_BILINEAR: i32 = eIMBInterpolationFilterMode::IMB_FILTER_BILINEAR as i32;
pub const IMB_FILTER_CUBIC_BSPLINE: i32 = eIMBInterpolationFilterMode::IMB_FILTER_CUBIC_BSPLINE as i32;
pub const IMB_FILTER_CUBIC_MITCHELL: i32 = eIMBInterpolationFilterMode::IMB_FILTER_CUBIC_MITCHELL as i32;
pub const IMB_FILTER_BOX: i32 = eIMBInterpolationFilterMode::IMB_FILTER_BOX as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IMBScaleFilter_2 {
    Nearest,
    Bilinear,
    Box,
}

impl Default for IMBScaleFilter_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const Nearest: i32 = IMBScaleFilter_2::Nearest as i32;
pub const Bilinear: i32 = IMBScaleFilter_2::Bilinear as i32;
pub const Box: i32 = IMBScaleFilter_2::Box as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eIMBTransformMode {
    IMB_TRANSFORM_MODE_REGULAR = 0,
    IMB_TRANSFORM_MODE_CROP_SRC = 1,
    IMB_TRANSFORM_MODE_WRAP_REPEAT = 2,
}

impl Default for eIMBTransformMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const IMB_TRANSFORM_MODE_REGULAR: i32 = eIMBTransformMode::IMB_TRANSFORM_MODE_REGULAR as i32;
pub const IMB_TRANSFORM_MODE_CROP_SRC: i32 = eIMBTransformMode::IMB_TRANSFORM_MODE_CROP_SRC as i32;
pub const IMB_TRANSFORM_MODE_WRAP_REPEAT: i32 = eIMBTransformMode::IMB_TRANSFORM_MODE_WRAP_REPEAT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GPUTextureCreateFlags_2 {
    HighBitDepth = 1 << 0,
    Premultiplied = 1 << 1,
    LimitSize = 1 << 2,
    EnableMipmaps = 1 << 3,
}

impl Default for GPUTextureCreateFlags_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const HighBitDepth: i32 = GPUTextureCreateFlags_2::HighBitDepth as i32;
pub const Premultiplied: i32 = GPUTextureCreateFlags_2::Premultiplied as i32;
pub const LimitSize: i32 = GPUTextureCreateFlags_2::LimitSize as i32;
pub const EnableMipmaps: i32 = GPUTextureCreateFlags_2::EnableMipmaps as i32;
