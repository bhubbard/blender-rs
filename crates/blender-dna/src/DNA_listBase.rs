use core::ffi::c_void;
use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Link {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LinkData {
    pub next: *mut LinkData,
    pub prev: *mut LinkData,
    pub data: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ListBase {
    pub first_: *mut c_void,
    pub last_: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ListBaseTIterator {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ListBaseEnumerateWrapper {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ListBaseMutableWrapper {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ListBaseBackwardWrapper {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ListBaseMutableBackwardWrapper {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ListBaseT {
    pub _opaque: [u8; 0],
}
