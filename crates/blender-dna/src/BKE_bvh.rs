//! Auto-transpiled C/C++ header module: BKE_bvh

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ClosestPointResult {
    pub position: [f32; 3],
    pub bary_coord: [f32; 3],
    pub index: u32,
    pub geomID: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FallbackTree {

}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElemType {
    Tris,
    Points,
    Edges,
}
