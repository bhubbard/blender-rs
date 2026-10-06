use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DictionaryEntryParser {
    pub result: Vec<PrimitiveType>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDPropertySerializer {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDPStringSerializer {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDPBoolSerializer {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDPIntSerializer {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDPFloatSerializer {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDPDoubleSerializer {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDPArraySerializer {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDPGroupSerializer {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDPUnknownSerializer {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PrimitiveType {
    pub value: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Vector<PrimitiveType> {
    pub elements: Vec<PrimitiveType>,
}
