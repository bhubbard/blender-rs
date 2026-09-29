//! Auto-transpiled C/C++ header module: bmesh_mesh

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMeshCreateParams {
    pub use_toolflags: bool,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMAllocTemplate {
    pub totvert: i32,
    pub totedge: i32,
    pub totloop: i32,
    pub totface: i32,
}
