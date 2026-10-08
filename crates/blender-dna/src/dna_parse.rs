//! Auto-transpiled C/C++ header module: dna_parse

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ParsedMember {
    pub alignment: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ParsedStruct {
    pub members: Vector<ParsedMember>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ParsedEnum {

}
