//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i16)]
pub enum eImageUser_Flag {
    #[default]
    IMA_ANIM_ALWAYS = 1 << 0,
    IMA_SHOW_SEQUENCER_SCENE = 1 << 1,
    IMA_NEED_FRAME_RECALC = 1 << 3,
    IMA_SHOW_STEREO = 1 << 4,
    IMA_USER_FRAME_IN_RANGE = (1 << 10),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum eImage_Flag {
    #[default]
    IMA_HIGH_BITDEPTH = (1 << 0),
    IMA_FLAG_UNUSED_1 = (1 << 1),
    IMA_DO_PREMUL = (1 << 2),
    IMA_FLAG_UNUSED_4 = (1 << 4),
    IMA_NOCOLLECT = (1 << 5),
    IMA_FLAG_UNUSED_6 = (1 << 6),
    IMA_OLD_PREMUL = (1 << 7),
    IMA_FLAG_UNUSED_8 = (1 << 8),
    IMA_USED_FOR_RENDER = (1 << 9),
    IMA_VIEW_AS_RENDER = (1 << 11),
    IMA_FLAG_UNUSED_12 = (1 << 12),
    IMA_DEINTERLACE = (1 << 13),
    IMA_USE_VIEWS = (1 << 14),
    IMA_FLAG_UNUSED_15 = (1 << 15),
    IMA_FLAG_UNUSED_16 = (1 << 16),
    IMA_AUTOSAVE_TEMPPACK = (1 << 17),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i16)]
pub enum eImageSource {
    #[default]
    IMA_SRC_FILE = 1,
    IMA_SRC_SEQUENCE = 2,
    IMA_SRC_MOVIE = 3,
    IMA_SRC_GENERATED = 4,
    IMA_SRC_VIEWER = 5,
    IMA_SRC_TILED = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i16)]
pub enum eImageType {
    #[default]
    IMA_TYPE_IMAGE = 0,
    IMA_TYPE_MULTILAYER = 1,
    IMA_TYPE_UV_TEST = 2,
    IMA_TYPE_R_RESULT = 4,
    IMA_TYPE_COMPOSITE = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i8)]
pub enum eImageGenType {
    #[default]
    IMA_GENTYPE_BLANK = 0,
    IMA_GENTYPE_GRID = 1,
    IMA_GENTYPE_GRID_COLOR = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i8)]
pub enum eImage_GenFlag {
    #[default]
    IMA_GEN_FLOAT = (1 << 0),
    IMA_GEN_TILE = (1 << 1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i8)]
pub enum eImageAlphaMode {
    #[default]
    IMA_ALPHA_STRAIGHT = 0,
    IMA_ALPHA_PREMUL = 1,
    IMA_ALPHA_CHANNEL_PACKED = 2,
    IMA_ALPHA_IGNORE = 3,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ImageUser {
    pub scene: *mut core::ffi::c_void,
    pub framenr: i32,
    pub frames: i32,
    pub offset: i32,
    pub sfra: i32,
    pub cycl: i8,
    pub multiview_eye: i8,
    pub pass: i16,
    pub tile: i32,
    pub multi_index: i16,
    pub view: i16,
    pub layer: i16,
    pub flag: eImageUser_Flag,
}

impl Default for ImageUser {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ImageAnim {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub anim: *mut core::ffi::c_void,
}

impl Default for ImageAnim {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ImageView {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub name: [u8; 64],
    pub filepath: [u8; 1024],
}

impl Default for ImageView {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ImagePackedFile {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub packedfile: *mut core::ffi::c_void,
    pub view: i32,
    pub tile_number: i32,
    pub filepath: [u8; 1024],
}

impl Default for ImagePackedFile {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct RenderSlot {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub name: [u8; 64],
    pub render: *mut core::ffi::c_void,
}

impl Default for RenderSlot {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ImageTile_Runtime {
    pub tilearray_layer: i32,
    pub _pad: i32,
}

impl Default for ImageTile_Runtime {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ImageTile {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub runtime: ImageTile_Runtime,
    pub tile_number: i32,
    pub gen_x: i32,
    pub gen_y: i32,
    pub gen_type: eImageGenType,
    pub gen_flag: eImage_GenFlag,
}

impl Default for ImageTile {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Image {
    pub adt: *mut core::ffi::c_void,
    pub filepath: [u8; 1024],
    pub anims: ListBaseT<ImageAnim>,
    pub nullptr: ListBaseT<ImageAnim>,
}

impl Default for Image {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

