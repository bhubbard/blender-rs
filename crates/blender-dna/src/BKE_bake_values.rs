//! Auto-transpiled C/C++ header module: BKE_bake_values

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Item {
    pub value: SocketValueVariant,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct InputValue {
    pub id: i32,
    pub value: SocketValueVariant,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct OutputKey {
    pub id: i32,
    pub r#type: eNodeSocketDatatype,
}
