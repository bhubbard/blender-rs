//! Auto-transpiled C/C++ header module: bake

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BakeDataZSpan {
    pub primitive_id: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TriTessFace {
    pub tspace: [[f32; 4]; 3],
    pub normal: [f32; 3],
    pub is_smooth: bool,
}
