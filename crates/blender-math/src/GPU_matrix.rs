//! Auto-transpiled C/C++ header module: GPU_matrix

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct dims {
    pub model_inverted: [[f32; 4]; 4],
    pub view: [f32; 4],
    pub is_persp: bool,
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub zmin: f64,
    pub zmax: f64,
}
