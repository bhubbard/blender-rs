//! Auto-transpiled C/C++ header module: DNA_attribute_types

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AttributeArray {
    pub size: i64,
    pub is_single: int8_t,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AttributeSingle {

}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Attribute {
    pub data_type: i16,
    pub domain: int8_t,
    pub storage_type: int8_t,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AttributeStorage {
    pub dna_attributes_num: i32,
}
