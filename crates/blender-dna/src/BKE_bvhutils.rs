//! Auto-transpiled C/C++ header module: BKE_bvhutils

pub type BVHTree_NearestPointCallback = Option<unsafe extern "C" fn(*const std::ffi::c_void, *const std::ffi::c_void)>;
pub type BVHTree_RayCastCallback = Option<unsafe extern "C" fn(*const std::ffi::c_void, *const std::ffi::c_void)>;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BVHTreeFromMesh {
    pub nearest_callback: BVHTree_NearestPointCallback,
    pub raycast_callback: BVHTree_RayCastCallback,
}
