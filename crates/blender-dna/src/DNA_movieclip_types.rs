//! Auto-transpiled C/C++ header module: DNA_movieclip_types

use core::ffi::c_void;
use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MovieClipUser {
    pub framenr: i32,
    pub render_size: eMovieClipProxy_RenderSize,
    pub render_flag: eMovieClipProxy_RenderFlag,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct MovieClipProxy {
    pub dir: [i8; 768],
    pub quality: i16,
    pub build_size_flag: eMovieClipProxy_Size,
}

impl Default for MovieClipProxy {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MovieClip_RuntimeGPUTexture {
    pub next: *mut core::ffi::c_void,
    pub user: MovieClipUser,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MovieClip_Runtime {
    pub gputextures: ListBaseT<MovieClip_RuntimeGPUTexture>,
    pub last_update: u64,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct MovieClip {
    pub id: ID,
    pub adt: *mut AnimData,
    pub filepath: [i8; 1024],
    pub source: MovieClipSource,
    pub lastsize: [i32; 2],
    pub aspx: f32,
    pub _pad: i32,
    pub anim: *mut MovieReader,
    pub cache: *mut MovieClipCache,
    pub gpd: *mut bGPdata,
    pub tracking: MovieTracking,
    pub tracking_context: *mut core::ffi::c_void,
    pub proxy: MovieClipProxy,
    pub flag: MovieClipFlag,
    pub len: i32,
    pub start_frame: i32,
    pub frame_offset: i32,
    pub _pad1: i32,
    pub colorspace_settings: ColorManagedColorspaceSettings,
    pub runtime: MovieClip_Runtime,
}

impl Default for MovieClip {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MovieClipScopes {
    pub ok: i16,
    pub use_track_mask: i16,
    pub track_preview_height: i32,
    pub frame_width: i32,
    pub undist_marker: MovieTrackingMarker,
    pub track_search: *mut ImBuf,
    pub track_preview: *mut ImBuf,
    pub track_pos: [f32; 2],
    pub track_disabled: i16,
    pub track_locked: i16,
    pub scene_framenr: i32,
    pub track: *mut MovieTrackingTrack,
    pub marker: *mut MovieTrackingMarker,
    pub slide_scale: [f32; 2],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnimData {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImBuf {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MovieReader {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MovieTrackingMarker {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MovieTrackingTrack {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bGPdata {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Texture {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MovieClipCache {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MovieTracking {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMovieClipProxy_Size {
    MCLIP_PROXY_SIZE_25 = (1 << 0),
    MCLIP_PROXY_SIZE_50 = (1 << 1),
    MCLIP_PROXY_SIZE_75 = (1 << 2),
    MCLIP_PROXY_SIZE_100 = (1 << 3),
    MCLIP_PROXY_UNDISTORTED_SIZE_25 = (1 << 4),
    MCLIP_PROXY_UNDISTORTED_SIZE_50 = (1 << 5),
    MCLIP_PROXY_UNDISTORTED_SIZE_75 = (1 << 6),
    MCLIP_PROXY_UNDISTORTED_SIZE_100 = (1 << 7),
}

impl Default for eMovieClipProxy_Size {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MCLIP_PROXY_SIZE_25: i32 = eMovieClipProxy_Size::MCLIP_PROXY_SIZE_25 as i32;
pub const MCLIP_PROXY_SIZE_50: i32 = eMovieClipProxy_Size::MCLIP_PROXY_SIZE_50 as i32;
pub const MCLIP_PROXY_SIZE_75: i32 = eMovieClipProxy_Size::MCLIP_PROXY_SIZE_75 as i32;
pub const MCLIP_PROXY_SIZE_100: i32 = eMovieClipProxy_Size::MCLIP_PROXY_SIZE_100 as i32;
pub const MCLIP_PROXY_UNDISTORTED_SIZE_25: i32 = eMovieClipProxy_Size::MCLIP_PROXY_UNDISTORTED_SIZE_25 as i32;
pub const MCLIP_PROXY_UNDISTORTED_SIZE_50: i32 = eMovieClipProxy_Size::MCLIP_PROXY_UNDISTORTED_SIZE_50 as i32;
pub const MCLIP_PROXY_UNDISTORTED_SIZE_75: i32 = eMovieClipProxy_Size::MCLIP_PROXY_UNDISTORTED_SIZE_75 as i32;
pub const MCLIP_PROXY_UNDISTORTED_SIZE_100: i32 = eMovieClipProxy_Size::MCLIP_PROXY_UNDISTORTED_SIZE_100 as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovieClipSource {
    MCLIP_SRC_SEQUENCE = 1,
    MCLIP_SRC_MOVIE = 2,
}

impl Default for MovieClipSource {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MCLIP_SRC_SEQUENCE: i32 = MovieClipSource::MCLIP_SRC_SEQUENCE as i32;
pub const MCLIP_SRC_MOVIE: i32 = MovieClipSource::MCLIP_SRC_MOVIE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovieClipFlag {
    MCLIP_USE_PROXY = (1 << 0),
    MCLIP_USE_PROXY_CUSTOM_DIR = (1 << 1),
    MCLIP_DATA_EXPAND = (1 << 3),
    MCLIP_PROXY_FLAGS = (MCLIP_USE_PROXY | MCLIP_USE_PROXY_CUSTOM_DIR),
}

impl Default for MovieClipFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MCLIP_USE_PROXY: i32 = MovieClipFlag::MCLIP_USE_PROXY as i32;
pub const MCLIP_USE_PROXY_CUSTOM_DIR: i32 = MovieClipFlag::MCLIP_USE_PROXY_CUSTOM_DIR as i32;
pub const MCLIP_DATA_EXPAND: i32 = MovieClipFlag::MCLIP_DATA_EXPAND as i32;
pub const MCLIP_PROXY_FLAGS: i32 = MovieClipFlag::MCLIP_PROXY_FLAGS as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMovieClipProxy_RenderSize {
    MCLIP_PROXY_RENDER_SIZE_FULL = 0,
    MCLIP_PROXY_RENDER_SIZE_25 = 1,
    MCLIP_PROXY_RENDER_SIZE_50 = 2,
    MCLIP_PROXY_RENDER_SIZE_75 = 3,
    MCLIP_PROXY_RENDER_SIZE_100 = 4,
}

impl Default for eMovieClipProxy_RenderSize {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MCLIP_PROXY_RENDER_SIZE_FULL: i32 = eMovieClipProxy_RenderSize::MCLIP_PROXY_RENDER_SIZE_FULL as i32;
pub const MCLIP_PROXY_RENDER_SIZE_25: i32 = eMovieClipProxy_RenderSize::MCLIP_PROXY_RENDER_SIZE_25 as i32;
pub const MCLIP_PROXY_RENDER_SIZE_50: i32 = eMovieClipProxy_RenderSize::MCLIP_PROXY_RENDER_SIZE_50 as i32;
pub const MCLIP_PROXY_RENDER_SIZE_75: i32 = eMovieClipProxy_RenderSize::MCLIP_PROXY_RENDER_SIZE_75 as i32;
pub const MCLIP_PROXY_RENDER_SIZE_100: i32 = eMovieClipProxy_RenderSize::MCLIP_PROXY_RENDER_SIZE_100 as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMovieClipProxy_RenderFlag {
    MCLIP_PROXY_RENDER_UNDISTORT = 1,
    MCLIP_PROXY_RENDER_USE_FALLBACK_RENDER = 2,
}

impl Default for eMovieClipProxy_RenderFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MCLIP_PROXY_RENDER_UNDISTORT: i32 = eMovieClipProxy_RenderFlag::MCLIP_PROXY_RENDER_UNDISTORT as i32;
pub const MCLIP_PROXY_RENDER_USE_FALLBACK_RENDER: i32 = eMovieClipProxy_RenderFlag::MCLIP_PROXY_RENDER_USE_FALLBACK_RENDER as i32;
