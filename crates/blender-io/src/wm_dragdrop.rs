use crate::*;

pub const KMAP_MAX_NAME: usize = 64;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ListBaseT<T> {
    pub _marker: core::marker::PhantomData<T>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct wmDropBox {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum eWM_DragDataType {
    #[default]
    None = 0,
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
