//! Makesrna core types ported to Rust
//! Corresponds to RNA_types.hh, rna_action_tools.hh, rna_defaults.hh, rna_internal_types.hh

use blender_dna::*;
use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct AncestorPointerRNA {
    pub rna_type: *mut c_void,
    pub data: *mut c_void,
}

impl Default for AncestorPointerRNA {
    fn default() -> Self {
        Self {
            rna_type: std::ptr::null_mut(),
            data: std::ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct ExtendedPointerRNA {
    pub owner_id: *mut ID,
    pub rna_type: *mut c_void,
    pub data: *mut c_void,
    pub ancestors: Vec<AncestorPointerRNA>,
}

impl Default for ExtendedPointerRNA {
    fn default() -> Self {
        Self {
            owner_id: std::ptr::null_mut(),
            rna_type: std::ptr::null_mut(),
            data: std::ptr::null_mut(),
            ancestors: Vec::new(),
        }
    }
}

impl ExtendedPointerRNA {
    pub fn is_null(&self) -> bool {
        self.data.is_null()
    }
}

pub type RNAUpdateFn = Option<extern "C" fn(bmain: *mut c_void, scene: *mut Scene, ptr: *mut ExtendedPointerRNA)>;
