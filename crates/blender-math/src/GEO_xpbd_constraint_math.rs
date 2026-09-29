//! Auto-transpiled C/C++ header module: GEO_xpbd_constraint_math

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PlaneIntersection {
    pub position: [f32; 3],
    pub segment_lambda: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SegmentClosestToRay {
    pub segment_lambda: f32,
    pub ray_lambda: f32,
}
