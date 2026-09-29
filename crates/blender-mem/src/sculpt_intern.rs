//! Auto-transpiled C/C++ header module: sculpt_intern

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StrokeExtension {
    pub is_first: bool,
    pub mouse_position: [f32; 2],
    pub pressure: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CurvesBrush3D {
    pub position_cu: [f32; 3],
    pub radius_cu: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MoveAndResampleBuffers {

}
