//! Auto-transpiled C/C++ header module: BLO_writefile
//! Upstream: source/blender/blenloader/BLO_writefile.hh

use crate::*;
use crate::DEG_depsgraph::Main;
use crate::BKE_asset_edit::ReportList;
use std::ffi::c_char;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBLO_WritePathRemap {
    BLO_WRITE_PATH_REMAP_NONE = 0,
    BLO_WRITE_PATH_REMAP_RELATIVE = 1,
    BLO_WRITE_PATH_REMAP_RELATIVE_ALL = 2,
    BLO_WRITE_PATH_REMAP_ABSOLUTE = 3,
}

impl Default for eBLO_WritePathRemap {
    fn default() -> Self {
        eBLO_WritePathRemap::BLO_WRITE_PATH_REMAP_RELATIVE
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BlendFileWriteParams {
    pub remap_mode: eBLO_WritePathRemap,
    pub use_save_versions: bool,
    pub use_save_as_copy: bool,
    pub use_userdef: bool,
    pub is_copypaste_buffer: bool,
}

extern "C" {
    pub fn BLO_write_file(
        mainvar: *mut Main,
        filepath: *const c_char,
        write_flags: i32,
        params: *const BlendFileWriteParams,
        reports: *mut ReportList,
    ) -> bool;
}
