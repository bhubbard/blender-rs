//! Auto-transpiled C/C++ header module: grease_pencil_intern

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct InputSample {
    pub mouse_position: [f32; 2],
    pub pressure: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GreasePencilStrokeParams {
    pub layer_index: i32,
    pub frame_number: i32,
    pub multi_frame_falloff: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IndexMask;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IndexMaskMemory;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AutoMaskingInfo {
    pub point_mask: IndexMask,
    pub memory: IndexMaskMemory,
}
