//! Auto-transpiled C/C++ header module: workbench_shadow_bsl

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VertIn {
    pub lP: [f32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VertOut {
    pub lP: [f32; 3],
    pub frontPosition: [f32; 4],
    pub backPosition: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GeomOut {
    pub gpu_position: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FragOut {

}
