//! Auto-transpiled C/C++ header module: BLO_versioning_common
//! Upstream: source/blender/blenloader/versioning_common.hh

use crate::*;
use std::ffi::c_char;

extern "C" {
    pub fn do_versions_new_to_old_idcode_get(id_code_new: i16) -> i16;
    pub fn do_versions_add_region_if_not_found(
        regionbase: *mut ListBaseT<ARegion>,
        region_type: i32,
        allocname: *const c_char,
        link_after_region_type: i32,
    ) -> *mut ARegion;
}
