//! Auto-transpiled C/C++ header module: BLI_color_types

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eAlpha {
    Straight,
    Premultiplied,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eSpace {
    Theme,
    SceneLinear,
    SceneLinearByteEncoded,
}
