//! Auto-transpiled C/C++ header module: IMB_thumbs

use crate::*;

pub const PREVIEW_RENDER_DEFAULT_HEIGHT: i32 = 128;
pub const PREVIEW_RENDER_LARGE_HEIGHT: i32 = 256;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ThumbCancellationToken {

}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImBuf {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThumbSize {
    THB_NORMAL,
    THB_LARGE,
    THB_FAIL,
}

impl Default for ThumbSize {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const THB_NORMAL: i32 = ThumbSize::THB_NORMAL as i32;
pub const THB_LARGE: i32 = ThumbSize::THB_LARGE as i32;
pub const THB_FAIL: i32 = ThumbSize::THB_FAIL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThumbSource {
    THB_SOURCE_IMAGE,
    THB_SOURCE_MOVIE,
    THB_SOURCE_BLEND,
    THB_SOURCE_FONT,
    THB_SOURCE_OBJECT_IO,
    THB_SOURCE_DIRECT,
}

impl Default for ThumbSource {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const THB_SOURCE_IMAGE: i32 = ThumbSource::THB_SOURCE_IMAGE as i32;
pub const THB_SOURCE_MOVIE: i32 = ThumbSource::THB_SOURCE_MOVIE as i32;
pub const THB_SOURCE_BLEND: i32 = ThumbSource::THB_SOURCE_BLEND as i32;
pub const THB_SOURCE_FONT: i32 = ThumbSource::THB_SOURCE_FONT as i32;
pub const THB_SOURCE_OBJECT_IO: i32 = ThumbSource::THB_SOURCE_OBJECT_IO as i32;
pub const THB_SOURCE_DIRECT: i32 = ThumbSource::THB_SOURCE_DIRECT as i32;
