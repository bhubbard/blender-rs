//! Auto-transpiled C/C++ header module: bmesh_marking

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMEditSelection {
    pub htype: i8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BMSelectFlushFlag {
    None = 0,
    RecalcLenVert = (1 << 0),
    RecalcLenEdge = (1 << 1),
    RecalcLenFace = (1 << 2),
    Down = (1 << 3),
}
