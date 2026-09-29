//! Auto-transpiled C/C++ header module: node_item

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Any = 0,
    Empty,
    Multioutput,
    String,
    Filename,
    Boolean,
    Integer,
    Float,
    Vector2,
    Vector3,
    Color3,
    Vector4,
    Color4,
    BSDF,
    EDF,
    DisplacementShader,
    SurfaceShader,
    Material,
    SurfaceOpacity,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOp {
    Less = 0, LessEq, Eq, GreaterEq, Greater, NotEq,
}
