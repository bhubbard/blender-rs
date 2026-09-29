//! Auto-transpiled C/C++ header module: eevee_ray_trace_screen_lib_bsl

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ScreenTraceHitData {
    pub ss_hit_P: [f32; 3],
    pub v_hit_P: [f32; 3],
    pub time: f32,
    pub valid: bool,
    pub hit_backface: bool,
}
