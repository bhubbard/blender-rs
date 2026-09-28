//! Auto-transpiled C/C++ header module: DNA_gpu_types

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GPUDOFSettings {
    pub focus_distance: f32,
    pub fstop: f32,
    pub focal_length: f32,
    pub sensor: f32,
    pub rotation: f32,
    pub ratio: f32,
    pub num_blades: i32,
    pub high_quality: i32,
}
