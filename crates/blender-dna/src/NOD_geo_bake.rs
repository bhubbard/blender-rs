use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bNode {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PointerRNA {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BakeDrawContext {
    pub node: *mut bNode,
    pub snode: *mut SpaceNode,
    pub object: *mut Object,
    pub nmd: *mut NodesModifierData,
    pub bake: *mut NodesModifierBake,
    pub bake_rna: PointerRNA,
    pub bake_still: bool,
    pub is_baked: bool,
    pub is_bakeable_in_current_context: bool,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NodesModifierData {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NodesModifierBake {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceNode {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Object {
    pub _opaque: [u8; 0],
}
