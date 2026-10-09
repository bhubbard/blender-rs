//! Auto-transpiled C/C++ header module: eevee_light_shape_display_bsl

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ShapeDisplayVertOut {
    pub lP: [f32; 2],
    pub P: [f32; 3],
    pub radiance: [f32; 3],
    pub light_index: i32,
    pub light_type: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ShapeDisplayFragOut {
    pub out_color: [f32; 4],
}
