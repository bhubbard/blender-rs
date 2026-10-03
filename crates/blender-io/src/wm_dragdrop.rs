//! Auto-transpiled C/C++ header module: wm_dragdrop

use crate::*;

pub const KMAP_MAX_NAME: usize = 64;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct wmDropBox {
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

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum eWM_DragDataType {
    #[default]
    WM_DRAG_DATA_TYPE_NONE = 0,
}

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

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct wmDragPrefetchHandler {
    pub drag_type: eWM_DragDataType,
}
