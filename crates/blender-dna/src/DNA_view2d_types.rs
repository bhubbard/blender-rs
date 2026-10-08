//! Auto-transpiled C/C++ header module: DNA_view2d_types

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct View2D {
    pub tot: rctf,
    pub cur: rctf,
    pub vert: rcti,
    pub hor: rcti,
    pub mask: rcti,
    pub min: [f32; 2],
    pub minzoom: f32,
    pub scroll: eView2D_Scroll,
    pub scroll_ui: eView2D_ScrollUI,
    pub keeptot: eView2D_KeepTot,
    pub keepzoom: eView2D_KeepZoom,
    pub keepofs: eView2D_KeepOfs,
    pub flag: eView2D_Flag,
    pub align: eView2D_Align,
    pub winx: i16,
    pub oldwinx: i16,
    pub around: i16,
    pub alpha_vert: i8,
    pub _pad: [i8; 2],
    pub page_size_y: f32,
    pub smooth_timer: *mut wmTimer,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SmoothView2DStore {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct wmTimer {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eView2D_KeepZoom {
    V2D_LIMITZOOM = (1 << 0),
    V2D_KEEPASPECT = (1 << 1),
    V2D_KEEPZOOM = (1 << 2),
    V2D_LOCKZOOM_X = (1 << 8),
    V2D_LOCKZOOM_Y = (1 << 9),
}

impl Default for eView2D_KeepZoom {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const V2D_LIMITZOOM: i32 = eView2D_KeepZoom::V2D_LIMITZOOM as i32;
pub const V2D_KEEPASPECT: i32 = eView2D_KeepZoom::V2D_KEEPASPECT as i32;
pub const V2D_KEEPZOOM: i32 = eView2D_KeepZoom::V2D_KEEPZOOM as i32;
pub const V2D_LOCKZOOM_X: i32 = eView2D_KeepZoom::V2D_LOCKZOOM_X as i32;
pub const V2D_LOCKZOOM_Y: i32 = eView2D_KeepZoom::V2D_LOCKZOOM_Y as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eView2D_KeepOfs {
    V2D_LOCKOFS_X = (1 << 1),
    V2D_LOCKOFS_Y = (1 << 2),
    V2D_KEEPOFS_X = (1 << 3),
    V2D_KEEPOFS_Y = (1 << 4),
}

impl Default for eView2D_KeepOfs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const V2D_LOCKOFS_X: i32 = eView2D_KeepOfs::V2D_LOCKOFS_X as i32;
pub const V2D_LOCKOFS_Y: i32 = eView2D_KeepOfs::V2D_LOCKOFS_Y as i32;
pub const V2D_KEEPOFS_X: i32 = eView2D_KeepOfs::V2D_KEEPOFS_X as i32;
pub const V2D_KEEPOFS_Y: i32 = eView2D_KeepOfs::V2D_KEEPOFS_Y as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eView2D_KeepTot {
    V2D_KEEPTOT_FREE = 0,
    V2D_KEEPTOT_BOUNDS = 1,
    V2D_KEEPTOT_STRICT = 2,
}

impl Default for eView2D_KeepTot {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const V2D_KEEPTOT_FREE: i32 = eView2D_KeepTot::V2D_KEEPTOT_FREE as i32;
pub const V2D_KEEPTOT_BOUNDS: i32 = eView2D_KeepTot::V2D_KEEPTOT_BOUNDS as i32;
pub const V2D_KEEPTOT_STRICT: i32 = eView2D_KeepTot::V2D_KEEPTOT_STRICT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eView2D_Flag {
    V2D_VIEWSYNC_SCREEN_TIME = (1 << 0),
    V2D_VIEWSYNC_AREA_VERTICAL = (1 << 1),
    V2D_PIXELOFS_X = (1 << 2),
    V2D_PIXELOFS_Y = (1 << 3),
    V2D_IS_NAVIGATING = (1 << 9),
    V2D_IS_INIT = (1 << 10),
    V2D_SNAP_TO_PAGESIZE_Y = (1 << 11),
    V2D_ZOOM_IGNORE_KEEPOFS = (1 << 12),
}

impl Default for eView2D_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const V2D_VIEWSYNC_SCREEN_TIME: i32 = eView2D_Flag::V2D_VIEWSYNC_SCREEN_TIME as i32;
pub const V2D_VIEWSYNC_AREA_VERTICAL: i32 = eView2D_Flag::V2D_VIEWSYNC_AREA_VERTICAL as i32;
pub const V2D_PIXELOFS_X: i32 = eView2D_Flag::V2D_PIXELOFS_X as i32;
pub const V2D_PIXELOFS_Y: i32 = eView2D_Flag::V2D_PIXELOFS_Y as i32;
pub const V2D_IS_NAVIGATING: i32 = eView2D_Flag::V2D_IS_NAVIGATING as i32;
pub const V2D_IS_INIT: i32 = eView2D_Flag::V2D_IS_INIT as i32;
pub const V2D_SNAP_TO_PAGESIZE_Y: i32 = eView2D_Flag::V2D_SNAP_TO_PAGESIZE_Y as i32;
pub const V2D_ZOOM_IGNORE_KEEPOFS: i32 = eView2D_Flag::V2D_ZOOM_IGNORE_KEEPOFS as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eView2D_Scroll {
    V2D_SCROLL_LEFT = (1 << 0),
    V2D_SCROLL_RIGHT = (1 << 1),
    V2D_SCROLL_VERTICAL = (V2D_SCROLL_LEFT | V2D_SCROLL_RIGHT),
    V2D_SCROLL_TOP = (1 << 2),
    V2D_SCROLL_BOTTOM = (1 << 3),
    V2D_SCROLL_HORIZONTAL = (V2D_SCROLL_TOP | V2D_SCROLL_BOTTOM),
    V2D_SCROLL_VERTICAL_HANDLES = (1 << 5),
    V2D_SCROLL_HORIZONTAL_HANDLES = (1 << 6),
    V2D_SCROLL_VERTICAL_HIDE = (1 << 7),
    V2D_SCROLL_HORIZONTAL_HIDE = (1 << 8),
    V2D_SCROLL_VERTICAL_FULLR = (1 << 9),
    V2D_SCROLL_HORIZONTAL_FULLR = (1 << 10),
}

impl Default for eView2D_Scroll {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const V2D_SCROLL_LEFT: i32 = eView2D_Scroll::V2D_SCROLL_LEFT as i32;
pub const V2D_SCROLL_RIGHT: i32 = eView2D_Scroll::V2D_SCROLL_RIGHT as i32;
pub const V2D_SCROLL_VERTICAL: i32 = eView2D_Scroll::V2D_SCROLL_VERTICAL as i32;
pub const V2D_SCROLL_TOP: i32 = eView2D_Scroll::V2D_SCROLL_TOP as i32;
pub const V2D_SCROLL_BOTTOM: i32 = eView2D_Scroll::V2D_SCROLL_BOTTOM as i32;
pub const V2D_SCROLL_HORIZONTAL: i32 = eView2D_Scroll::V2D_SCROLL_HORIZONTAL as i32;
pub const V2D_SCROLL_VERTICAL_HANDLES: i32 = eView2D_Scroll::V2D_SCROLL_VERTICAL_HANDLES as i32;
pub const V2D_SCROLL_HORIZONTAL_HANDLES: i32 = eView2D_Scroll::V2D_SCROLL_HORIZONTAL_HANDLES as i32;
pub const V2D_SCROLL_VERTICAL_HIDE: i32 = eView2D_Scroll::V2D_SCROLL_VERTICAL_HIDE as i32;
pub const V2D_SCROLL_HORIZONTAL_HIDE: i32 = eView2D_Scroll::V2D_SCROLL_HORIZONTAL_HIDE as i32;
pub const V2D_SCROLL_VERTICAL_FULLR: i32 = eView2D_Scroll::V2D_SCROLL_VERTICAL_FULLR as i32;
pub const V2D_SCROLL_HORIZONTAL_FULLR: i32 = eView2D_Scroll::V2D_SCROLL_HORIZONTAL_FULLR as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eView2D_ScrollUI {
    V2D_SCROLL_H_ACTIVE = (1 << 0),
    V2D_SCROLL_V_ACTIVE = (1 << 1),
}

impl Default for eView2D_ScrollUI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const V2D_SCROLL_H_ACTIVE: i32 = eView2D_ScrollUI::V2D_SCROLL_H_ACTIVE as i32;
pub const V2D_SCROLL_V_ACTIVE: i32 = eView2D_ScrollUI::V2D_SCROLL_V_ACTIVE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eView2D_Align {
    V2D_ALIGN_FREE = 0,
    V2D_ALIGN_NO_POS_X = (1 << 0),
    V2D_ALIGN_NO_NEG_X = (1 << 1),
    V2D_ALIGN_NO_POS_Y = (1 << 2),
    V2D_ALIGN_NO_NEG_Y = (1 << 3),
}

impl Default for eView2D_Align {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const V2D_ALIGN_FREE: i32 = eView2D_Align::V2D_ALIGN_FREE as i32;
pub const V2D_ALIGN_NO_POS_X: i32 = eView2D_Align::V2D_ALIGN_NO_POS_X as i32;
pub const V2D_ALIGN_NO_NEG_X: i32 = eView2D_Align::V2D_ALIGN_NO_NEG_X as i32;
pub const V2D_ALIGN_NO_POS_Y: i32 = eView2D_Align::V2D_ALIGN_NO_POS_Y as i32;
pub const V2D_ALIGN_NO_NEG_Y: i32 = eView2D_Align::V2D_ALIGN_NO_NEG_Y as i32;
