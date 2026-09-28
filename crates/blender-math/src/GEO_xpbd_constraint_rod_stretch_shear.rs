//! Auto-transpiled C/C++ header module: GEO_xpbd_constraint_rod_stretch_shear

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RodStretchAndShearConstraintResult {
    pub delta_lambda_pos: [f32; 3],
    pub delta_lambda_rot: [f32; 3],
    pub residual_error_squared: f32,
}
