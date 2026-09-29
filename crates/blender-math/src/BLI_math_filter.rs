//! Auto-transpiled C/C++ header module: BLI_math_filter

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterKernel {
    Box,
    Tent,
    Quad,
    Cubic,
    Catrom,
    Gauss,
    Mitch,
}
