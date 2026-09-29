use crate::*;
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct FixedString {
    pub data: [i8; 10], // Placeholder for N
}
