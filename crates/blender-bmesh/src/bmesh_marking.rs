//! Auto-transpiled C/C++ header module: bmesh_marking

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SelectionCountChunkData {
    pub selection_len: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SelectionFlushChunkData {
    pub delta_selection_len: i32,
}
