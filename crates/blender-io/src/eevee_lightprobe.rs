//! Auto-transpiled C/C++ header module: eevee_lightprobe

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LightProbe {
    pub used: bool,
    pub initialized: bool,
    pub updated: bool,
    pub viewport_display: bool,
    pub viewport_display_size: f32,
}
