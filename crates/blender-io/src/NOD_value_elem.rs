use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PrimitiveValueElem {
    pub affected: bool,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VectorElem {
    pub x: FloatElem,
    pub y: FloatElem,
    pub z: FloatElem,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RotationElem {
    pub euler: VectorElem,
    pub axis: VectorElem,
    pub angle: FloatElem,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MatrixElem {
    pub translation: VectorElem,
    pub rotation: RotationElem,
    pub scale: VectorElem,
    pub any_non_transform: FloatElem,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ElemVariant {
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SocketElem {
    pub socket: *mut bNodeSocket,
    pub elem: ElemVariant,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GroupInputElem {
    pub group_input_index: i32,
    pub elem: ElemVariant,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ValueNodeElem {
    pub node: *mut bNode,
    pub elem: ElemVariant,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BoolElem {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FloatElem {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IntElem {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct to {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bNodeSocket {
    // Placeholder for Blender's bNodeSocket type
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bNode {
    // Placeholder for Blender's bNode type
}
