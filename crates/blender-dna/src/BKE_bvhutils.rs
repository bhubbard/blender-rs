//! Auto-transpiled C/C++ header module: BKE_bvhutils

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BVHTreeFromMesh {
    pub nearest_callback: BVHTree_NearestPointCallback,
    pub raycast_callback: BVHTree_RayCastCallback,
}
