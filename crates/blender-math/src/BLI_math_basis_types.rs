//! Auto-transpiled C/C++ header module: BLI_math_basis_types

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CartesianBasis {

}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    X = 0,
    Y,
    Z,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value_2 {
    X_POS = 0,
    Y_POS = 1,
    Z_POS = 2,
    X_NEG = 3,
    Y_NEG = 4,
    Z_NEG = 5,
}
