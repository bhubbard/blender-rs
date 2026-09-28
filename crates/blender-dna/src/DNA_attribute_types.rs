//! Auto-transpiled C/C++ header module: DNA_attribute_types

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AttributeArray {
    pub size: i64,
    pub is_single: i8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AttributeSingle {

}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Attribute {
    pub data_type: i16,
    pub domain: i8,
    pub storage_type: i8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AttributeStorage {
    pub dna_attributes_num: i32,
}
