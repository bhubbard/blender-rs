//! Auto-transpiled C/C++ header module: GEO_mesh_triangulate

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriangulateNGonMode {
    Beauty = 0,
    EarClip = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriangulateQuadMode {
    Beauty = 0,
    Fixed = 1,
    Alternate = 2,
    ShortEdge = 3,
    LongEdge = 4,
}
