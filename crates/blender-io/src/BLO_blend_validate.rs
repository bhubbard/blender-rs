//! Auto-transpiled C/C++ header module: BLO_blend_validate
//! Upstream: source/blender/blenloader/BLO_blend_validate.hh

use crate::*;
use crate::DEG_depsgraph::Main;
use crate::BKE_asset_edit::ReportList;

extern "C" {
    pub fn BLO_main_validate_libraries(bmain: *mut Main, reports: *mut ReportList) -> bool;
    pub fn BLO_main_validate_shapekeys(bmain: *mut Main, reports: *mut ReportList) -> bool;
    pub fn BLO_main_validate_embedded_liboverrides(bmain: *mut Main, reports: *mut ReportList);
    pub fn BLO_main_validate_embedded_flag(bmain: *mut Main, reports: *mut ReportList);
}
