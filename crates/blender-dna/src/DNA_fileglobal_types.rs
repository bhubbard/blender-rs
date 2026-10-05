//! Auto-transpiled C/C++ header module: DNA_fileglobal_types

use core::ffi::c_void;
use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct FileGlobal {
    pub subvstr: [i8; 4],
    pub subversion: i16,
    pub minversion: i16,
    pub _pad: [i8; 6],
    pub curscreen: *mut bScreen,
    pub curscene: *mut Scene,
    pub cur_view_layer: *mut ViewLayer,
    pub _pad1: *mut core::ffi::c_void,
    pub fileflags: i32,
    pub globalf: i32,
    pub build_commit_timestamp: u64,
    pub build_hash: [i8; 16],
    pub filepath: [i8; 1024],
    pub colorspace_scene_linear_name: [i8; 64],
    pub colorspace_scene_linear_to_xyz: [[f32; 3]; 3],
    pub _pad2: [i32; 3],
}

impl Default for FileGlobal {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bScreen {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Scene {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ViewLayer {
    pub _opaque: [u8; 0],
}

