//! Auto-transpiled C/C++ header module: BLI_serialize

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Value {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StringValue {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DictionaryValue {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ArrayValue {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct eValueType {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PrimitiveValue {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct is {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NullValue {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Formatter {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct JsonFormatter {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eValueType_2 {
    String,
    Int,
    Array,
    Null,
    Boolean,
    Double,
    Dictionary,
    Enum,
}

impl Default for eValueType_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const String: i32 = eValueType_2::String as i32;
pub const Int: i32 = eValueType_2::Int as i32;
pub const Array: i32 = eValueType_2::Array as i32;
pub const Null: i32 = eValueType_2::Null as i32;
pub const Boolean: i32 = eValueType_2::Boolean as i32;
pub const Double: i32 = eValueType_2::Double as i32;
pub const Dictionary: i32 = eValueType_2::Dictionary as i32;
pub const Enum: i32 = eValueType_2::Enum as i32;
