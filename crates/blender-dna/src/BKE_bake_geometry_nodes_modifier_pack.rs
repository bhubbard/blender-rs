//! Auto-transpiled C/C++ header module: BKE_bake_geometry_nodes_modifier_pack

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackGeometryNodesBakeResult {
    NoDataFound,
    PackedAlready,
    Success,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnpackGeometryNodesBakeResult {
    BlendFileNotSaved,
    NoPackedData,
    Error,
    Success,
}
