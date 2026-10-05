//! Auto-transpiled C/C++ header module: DNA_vfont_types

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct VFont {
    pub id: ID,
    pub filepath: [i8; 1024],
    pub data: *mut VFontData,
    pub packedfile: *mut PackedFile,
    pub temp_pf: *mut PackedFile,
}

impl Default for VFont {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PackedFile {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VFontData {
    pub _opaque: [u8; 0],
}

