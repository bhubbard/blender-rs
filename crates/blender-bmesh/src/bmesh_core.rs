//! Auto-transpiled C/C++ header module: bmesh_core

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBMCreateFlag {
    BM_CREATE_NOP = 0,
    BM_CREATE_NO_DOUBLE = (1 << 1),
    BM_CREATE_SKIP_CD = (1 << 2),
}
