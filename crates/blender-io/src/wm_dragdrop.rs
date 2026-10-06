//! Auto-transpiled C/C++ header module: wm_dragdrop

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct wmDropBoxMap {
    pub next: *mut wmDropBoxMap,
    pub prev: *mut wmDropBoxMap,
    pub dropboxes: ListBaseT<wmDropBox>,
    pub spaceid: i16,
    pub regionid: i16,
    pub idname: [i8; KMAP_MAX_NAME],
}

impl Default for wmDropBoxMap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct wmDragPrefetchHandler {
    pub drag_type: eWM_DragDataType,
}
