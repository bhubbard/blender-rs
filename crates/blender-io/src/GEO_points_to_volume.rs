//! Auto-transpiled C/C++ header module: GEO_points_to_volume
//! Source: upstream/source/blender/geometry/GEO_points_to_volume.hh

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointRasterizeType {
    Scalar = 0,
    Vector = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelType {
    NearestPoint = 0,
    Linear = 1,
    Quadratic = 2,
    Cubic = 3,
}

#[derive(Debug, Clone)]
pub struct PointDataGridAttributeInfo {
    pub name: String,
    pub data_ptr: *const core::ffi::c_void,
    pub data_len: usize,
}
