use core::ffi::c_void;
use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImplicitSharingInfoHandle {
    pub shared_data: *mut c_void,
}

pub struct PackedFile {
    pub size: i32,
    pub seek: i32,
    pub data: *mut core::ffi::c_void,
    pub sharing_info: *mut ImplicitSharingInfoHandle,
}
