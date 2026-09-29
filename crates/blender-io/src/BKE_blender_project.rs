//! Auto-transpiled C/C++ header module: BKE_blender_project

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Main {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectVariableType {
    STRING = eIDPropertyType::IDP_STRING,
    INT = eIDPropertyType::IDP_INT,
    FLOAT = eIDPropertyType::IDP_FLOAT,
}

impl Default for ProjectVariableType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectVariableStringSubtype {
    NONE = 0,
    FILEPATH = 1,
}

impl Default for ProjectVariableStringSubtype {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
