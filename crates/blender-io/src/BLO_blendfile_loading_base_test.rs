//! Auto-transpiled C/C++ header module: BLO_blendfile_loading_base_test
//! Upstream: source/blender/blenloader/tests/blendfile_loading_base_test.h

use crate::*;
use crate::DEG_depsgraph::Depsgraph;

#[repr(C)]
#[derive(Debug)]
pub struct BlendfileLoadingBaseTest {
    pub bfile: *mut BlendFileData,
    pub depsgraph: *mut Depsgraph,
}

impl Default for BlendfileLoadingBaseTest {
    fn default() -> Self {
        Self {
            bfile: core::ptr::null_mut(),
            depsgraph: core::ptr::null_mut(),
        }
    }
}
