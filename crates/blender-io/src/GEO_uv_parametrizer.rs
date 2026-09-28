//! Auto-transpiled C/C++ header module: GEO_uv_parametrizer

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ParamSlimOptions {
    pub weight_influence: f32,
    pub iterations: i32,
    pub no_flip: bool,
    pub skip_init: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PHandleState {
    PHANDLE_STATE_ALLOCATED,
    PHANDLE_STATE_CONSTRUCTED,
    PHANDLE_STATE_LSCM,
    PHANDLE_STATE_STRETCH,
}
