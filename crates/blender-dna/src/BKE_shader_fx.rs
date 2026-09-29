//! Auto-transpiled C/C++ header module: BKE_shader_fx

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ShaderFxTypeInfo {
    pub name: [i8; 32],
    pub struct_name: [i8; 32],
    pub struct_size: i32,
    pub r#type: ShaderFxTypeType,
    pub flags: ShaderFxTypeFlag,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ARegionType {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BlendDataReader {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BlendWriter {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ID {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDTypeForeachColorFunctionCallback {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ModifierUpdateDepsgraphContext {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Object {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ShaderFxData {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderFxTypeType {
    eShaderFxType_NoneType,
    eShaderFxType_GpencilType,
}

impl Default for ShaderFxTypeType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderFxTypeFlag {
    eShaderFxTypeFlag_SupportsEditmode = (1 << 0),
    eShaderFxTypeFlag_EnableInEditmode = (1 << 2),
    eShaderFxTypeFlag_Single = (1 << 4),
    eShaderFxTypeFlag_NoUserAdd = (1 << 5),
}

impl Default for ShaderFxTypeFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
