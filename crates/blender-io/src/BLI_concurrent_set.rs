use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Unwrap {
    pub key: String,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PtrHash {
    pub key: String,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PtrIsEqual {
    pub key: String,
}
