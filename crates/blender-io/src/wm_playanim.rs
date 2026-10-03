//! Auto-transpiled C/C++ header module: wm_playanim

use crate::*;

pub const PLAY_FRAME_CACHE_MAX: i32 = 30;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GHOST_ISystem {
    pub _marker: core::marker::PhantomData<u8>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GHOST_IWindow {
    pub _marker: core::marker::PhantomData<u8>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GPUContext {
    pub _marker: core::marker::PhantomData<u8>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ColorManagedViewSettings {
    pub _marker: core::marker::PhantomData<u8>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ColorManagedDisplaySettings {
    pub _marker: core::marker::PhantomData<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListBaseT<T> {
    pub _marker: core::marker::PhantomData<T>,
}
impl<T> Default for ListBaseT<T> {
    fn default() -> Self {
        Self {
            _marker: core::marker::PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ImBuf {
    pub _marker: core::marker::PhantomData<u8>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MovieReader {
    pub _marker: core::marker::PhantomData<u8>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LinkData {
    pub _marker: core::marker::PhantomData<u8>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GhostData {
    pub system: *mut GHOST_ISystem,
    pub window: *mut GHOST_IWindow,
    pub gpu_context: *mut GPUContext,
    pub qual: eWS_Qual,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PlayArgs {
    pub argc: i32,
    pub argv: *mut *mut i8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PlayDisplayContext {
    pub view_settings: ColorManagedViewSettings,
    pub display_settings: ColorManagedDisplaySettings,
    pub ui_scale: f32,
    pub size: [i32; 2],
    pub use_window_csd: bool,
    pub ui_window_csd_alpha: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PlayState {
    pub display_ctx: PlayDisplayContext,
    pub zoom: f32,
    pub direction: i16,
    pub next_frame: i16,
    pub once: bool,
    pub pingpong: bool,
    pub no_frame_skip: bool,
    pub show_frame_indicator: bool,
    pub single_step: bool,
    pub wait: bool,
    pub stopped: bool,
    pub go: bool,
    pub loading: bool,
    pub draw_flip: [bool; 2],
    pub frame_step: i32,
    pub picsbase: ListBaseT<PlayAnimPict>,
    pub picture: *mut PlayAnimPict,
    pub ibuf_size: [i32; 2],
    pub font_id: i32,
    pub font_size: i32,
    pub argc_next: i32,
    pub argv_next: *mut *mut i8,
    pub need_frame_update: bool,
    pub frame_cursor_x: i32,
    pub ghost_data: GhostData,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PlayAnimPict {
    pub next: *mut PlayAnimPict,
    pub prev: *mut PlayAnimPict,
    pub mem: *mut u8,
    pub size: usize,
    pub filepath: *mut i8,
    pub error_message: *mut i8,
    pub ibuf: *mut ImBuf,
    pub anim: *mut MovieReader,
    pub frame: i32,
    pub frame_cache_node: *mut LinkData,
    pub size_in_memory: usize,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eWS_Qual {
    WS_QUAL_LSHIFT = (1 << 0),
    WS_QUAL_RSHIFT = (1 << 1),
    WS_QUAL_LALT = (1 << 2),
    WS_QUAL_RALT = (1 << 3),
    WS_QUAL_LCTRL = (1 << 4),
    WS_QUAL_RCTRL = (1 << 5),
    WS_QUAL_LCMD = (1 << 6),
    WS_QUAL_RCMD = (1 << 7),
    WS_QUAL_LMOUSE = (1 << 16),
    WS_QUAL_MMOUSE = (1 << 17),
    WS_QUAL_RMOUSE = (1 << 18),
}

impl Default for eWS_Qual {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const WS_QUAL_LSHIFT: i32 = eWS_Qual::WS_QUAL_LSHIFT as i32;
pub const WS_QUAL_RSHIFT: i32 = eWS_Qual::WS_QUAL_RSHIFT as i32;
pub const WS_QUAL_LALT: i32 = eWS_Qual::WS_QUAL_LALT as i32;
pub const WS_QUAL_RALT: i32 = eWS_Qual::WS_QUAL_RALT as i32;
pub const WS_QUAL_LCTRL: i32 = eWS_Qual::WS_QUAL_LCTRL as i32;
pub const WS_QUAL_RCTRL: i32 = eWS_Qual::WS_QUAL_RCTRL as i32;
pub const WS_QUAL_LCMD: i32 = eWS_Qual::WS_QUAL_LCMD as i32;
pub const WS_QUAL_RCMD: i32 = eWS_Qual::WS_QUAL_RCMD as i32;
pub const WS_QUAL_LMOUSE: i32 = eWS_Qual::WS_QUAL_LMOUSE as i32;
pub const WS_QUAL_MMOUSE: i32 = eWS_Qual::WS_QUAL_MMOUSE as i32;
pub const WS_QUAL_RMOUSE: i32 = eWS_Qual::WS_QUAL_RMOUSE as i32;
