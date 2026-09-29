use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMDataLayerLookup {
    pub offset: i32,
    pub layer: *mut CustomDataLayer,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LinkNode {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MemArena {
    pub _opaque: [u8; 0],
}

pub struct CustomDataLayer {
    pub data: Vec<u8>,
}
