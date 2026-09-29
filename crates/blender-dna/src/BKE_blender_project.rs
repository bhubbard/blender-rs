//! Auto-transpiled C/C++ header module: BKE_blender_project

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectVariableType {
    STRING = eIDPropertyType::IDP_STRING,
    INT = eIDPropertyType::IDP_INT,
    FLOAT = eIDPropertyType::IDP_FLOAT,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectVariableStringSubtype {
    NONE = 0,
    FILEPATH = 1,
}
