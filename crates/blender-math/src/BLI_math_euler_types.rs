//! Auto-transpiled C/C++ header module: BLI_math_euler_types

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EulerBase {

}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EulerOrder {
    XYZ = 1,
    XZY,
    YXZ,
    YZX,
    ZXY,
    ZYX,
}
