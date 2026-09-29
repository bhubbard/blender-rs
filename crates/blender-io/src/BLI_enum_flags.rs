use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BitwiseNotEnumValue {
    pub value: u64,
}

fn main() {
    let value: u64 = 123;
    let bitwise_not_value = BitwiseNotEnumValue { value };
    println!("BitwiseNotEnumValue value: {}", bitwise_not_value.value);
}
