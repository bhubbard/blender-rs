//! Auto-transpiled C/C++ header module: BLO_readfile
//! Upstream: source/blender/blenloader/BLO_readfile.hh

use crate::*;
use crate::DEG_depsgraph::{Main, Scene, ViewLayer};
use crate::BKE_asset_edit::ReportList;
use std::ffi::c_char;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBlenFileType {
    BLENFILETYPE_BLEND = 1,
}

#[repr(C)]
#[derive(Debug)]
pub struct BlendFileData {
    pub main: *mut Main,
    pub user: *mut UserDef,
    pub fileflags: i32,
    pub globalf: i32,
    pub filepath: [u8; 1024],
    pub curscreen: *mut bScreen,
    pub curscene: *mut Scene,
    pub cur_view_layer: *mut ViewLayer,
    pub file_type: eBlenFileType,
}

impl Default for BlendFileData {
    fn default() -> Self {
        Self {
            main: core::ptr::null_mut(),
            user: core::ptr::null_mut(),
            fileflags: 0,
            globalf: 0,
            filepath: [0u8; 1024],
            curscreen: core::ptr::null_mut(),
            curscene: core::ptr::null_mut(),
            cur_view_layer: core::ptr::null_mut(),
            file_type: eBlenFileType::BLENFILETYPE_BLEND,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct BlendFileReadParams {
    pub skip_flags: u32,
    pub is_startup: bool,
    pub is_factory_settings: bool,
    pub undo_direction: i32,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct BlendFileReadReport {
    pub reports: *mut ReportList,
}

extern "C" {
    pub fn BLO_read_from_file(
        filepath: *const c_char,
        params: *const BlendFileReadParams,
        reports: *mut BlendFileReadReport,
    ) -> *mut BlendFileData;

    pub fn BLO_read_from_memory(
        mem: *const u8,
        memsize: usize,
        params: *const BlendFileReadParams,
        reports: *mut BlendFileReadReport,
    ) -> *mut BlendFileData;

    pub fn BLO_blendfiledata_free(bfd: *mut BlendFileData);
}
