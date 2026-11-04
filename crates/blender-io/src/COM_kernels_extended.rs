//! Blender GPU Compositor algorithms and cached kernels implementation ported to Rust
//! Corresponds to:
//! jump_flooding.cc, morphological_blur.cc, morphological_distance.cc,
//! morphological_distance_feather.cc, parallel_reduction.cc, recursive_gaussian_blur.cc,
//! summed_area_table.cc, symmetric_separable_blur.cc, symmetric_separable_blur_variable_size.cc,
//! bokeh_kernel.cc, deriche_gaussian_coefficients.cc, distortion_grid.cc,
//! fog_glow_kernel.cc, image_coordinates.cc, keying_screen.cc

#[derive(Debug, Clone, PartialEq, Default)]
pub struct JumpFloodingPass {
    pub step_length: i32,
    pub width: i32,
    pub height: i32,
}

impl JumpFloodingPass {
    pub fn compute_steps(max_dim: i32) -> Vec<i32> {
        let mut steps = Vec::new();
        let mut step = 1;
        while step < max_dim {
            step *= 2;
        }
        step /= 2;
        while step >= 1 {
            steps.push(step);
            step /= 2;
        }
        steps
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SummedAreaTable {
    pub width: usize,
    pub height: usize,
    pub table: Vec<f64>,
}

impl SummedAreaTable {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            table: vec![0.0; width * height],
        }
    }

    pub fn query_box(&self, x0: usize, y0: usize, x1: usize, y1: usize) -> f64 {
        if x1 < x0 || y1 < y0 || x1 >= self.width || y1 >= self.height {
            return 0.0;
        }
        let a = self.table[y1 * self.width + x1];
        let b = if x0 > 0 { self.table[y1 * self.width + (x0 - 1)] } else { 0.0 };
        let c = if y0 > 0 { self.table[(y0 - 1) * self.width + x1] } else { 0.0 };
        let d = if x0 > 0 && y0 > 0 { self.table[(y0 - 1) * self.width + (x0 - 1)] } else { 0.0 };
        a - b - c + d
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BokehKernel {
    pub radius: f32,
    pub blades: i32,
    pub rotation: f32,
    pub rounding: f32,
}
