//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eImageUser_Flag(pub i16);

impl eImageUser_Flag {
    pub const IMA_ANIM_ALWAYS: Self = Self((1 << 0) as i16);
    pub const IMA_SHOW_SEQUENCER_SCENE: Self = Self((1 << 1) as i16);
    pub const IMA_NEED_FRAME_RECALC: Self = Self((1 << 3) as i16);
    pub const IMA_SHOW_STEREO: Self = Self((1 << 4) as i16);
    pub const IMA_USER_FRAME_IN_RANGE: Self = Self(((1 << 10)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eImage_Flag(pub i32);

impl eImage_Flag {
    pub const IMA_HIGH_BITDEPTH: Self = Self(((1 << 0)) as i32);
    pub const IMA_FLAG_UNUSED_1: Self = Self(((1 << 1)) as i32);
    pub const IMA_DO_PREMUL: Self = Self(((1 << 2)) as i32);
    pub const IMA_FLAG_UNUSED_4: Self = Self(((1 << 4)) as i32);
    pub const IMA_NOCOLLECT: Self = Self(((1 << 5)) as i32);
    pub const IMA_FLAG_UNUSED_6: Self = Self(((1 << 6)) as i32);
    pub const IMA_OLD_PREMUL: Self = Self(((1 << 7)) as i32);
    pub const IMA_FLAG_UNUSED_8: Self = Self(((1 << 8)) as i32);
    pub const IMA_USED_FOR_RENDER: Self = Self(((1 << 9)) as i32);
    pub const IMA_VIEW_AS_RENDER: Self = Self(((1 << 11)) as i32);
    pub const IMA_FLAG_UNUSED_12: Self = Self(((1 << 12)) as i32);
    pub const IMA_DEINTERLACE: Self = Self(((1 << 13)) as i32);
    pub const IMA_USE_VIEWS: Self = Self(((1 << 14)) as i32);
    pub const IMA_FLAG_UNUSED_15: Self = Self(((1 << 15)) as i32);
    pub const IMA_FLAG_UNUSED_16: Self = Self(((1 << 16)) as i32);
    pub const IMA_AUTOSAVE_TEMPPACK: Self = Self(((1 << 17)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eImageSource(pub i16);

impl eImageSource {
    pub const IMA_SRC_FILE: Self = Self((1) as i16);
    pub const IMA_SRC_SEQUENCE: Self = Self((2) as i16);
    pub const IMA_SRC_MOVIE: Self = Self((3) as i16);
    pub const IMA_SRC_GENERATED: Self = Self((4) as i16);
    pub const IMA_SRC_VIEWER: Self = Self((5) as i16);
    pub const IMA_SRC_TILED: Self = Self((6) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eImageType(pub i16);

impl eImageType {
    pub const IMA_TYPE_IMAGE: Self = Self((0) as i16);
    pub const IMA_TYPE_MULTILAYER: Self = Self((1) as i16);
    pub const IMA_TYPE_UV_TEST: Self = Self((2) as i16);
    pub const IMA_TYPE_R_RESULT: Self = Self((4) as i16);
    pub const IMA_TYPE_COMPOSITE: Self = Self((5) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eImageGenType(pub i8);

impl eImageGenType {
    pub const IMA_GENTYPE_BLANK: Self = Self((0) as i8);
    pub const IMA_GENTYPE_GRID: Self = Self((1) as i8);
    pub const IMA_GENTYPE_GRID_COLOR: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eImage_GenFlag(pub i8);

impl eImage_GenFlag {
    pub const IMA_GEN_FLOAT: Self = Self(((1 << 0)) as i8);
    pub const IMA_GEN_TILE: Self = Self(((1 << 1)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eImageAlphaMode(pub i8);

impl eImageAlphaMode {
    pub const IMA_ALPHA_STRAIGHT: Self = Self((0) as i8);
    pub const IMA_ALPHA_PREMUL: Self = Self((1) as i8);
    pub const IMA_ALPHA_CHANNEL_PACKED: Self = Self((2) as i8);
    pub const IMA_ALPHA_IGNORE: Self = Self((3) as i8);
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

