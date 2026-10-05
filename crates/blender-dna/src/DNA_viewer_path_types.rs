//! Auto-transpiled C/C++ header module: DNA_viewer_path_types

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ViewerPathElem {
    pub next: *mut ViewerPathElem,
    pub r#type: ViewerPathElemType,
    pub _pad: [i8; 4],
    pub ui_name: *mut i8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDViewerPathElem {
    pub base: ViewerPathElem,
    pub id: *mut ID,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ModifierViewerPathElem {
    pub base: ViewerPathElem,
    pub modifier_uid: i32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GroupNodeViewerPathElem {
    pub base: ViewerPathElem,
    pub node_id: i32,
    pub _pad1: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SimulationZoneViewerPathElem {
    pub base: ViewerPathElem,
    pub sim_output_node_id: i32,
    pub _pad1: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RepeatZoneViewerPathElem {
    pub base: ViewerPathElem,
    pub repeat_output_node_id: i32,
    pub iteration: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ForeachGeometryElementZoneViewerPathElem {
    pub base: ViewerPathElem,
    pub zone_output_node_id: i32,
    pub index: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ViewerNodeViewerPathElem {
    pub base: ViewerPathElem,
    pub node_id: i32,
    pub _pad1: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EvaluateClosureNodeViewerPathElem {
    pub base: ViewerPathElem,
    pub evaluate_node_id: i32,
    pub source_output_node_id: i32,
    pub source_node_tree: *mut bNodeTree,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ViewerPath {
    pub path: ListBaseT<ViewerPathElem>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ID {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bNodeTree {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewerPathElemType {
    VIEWER_PATH_ELEM_TYPE_ID = 0,
    VIEWER_PATH_ELEM_TYPE_MODIFIER = 1,
    VIEWER_PATH_ELEM_TYPE_GROUP_NODE = 2,
    VIEWER_PATH_ELEM_TYPE_SIMULATION_ZONE = 3,
    VIEWER_PATH_ELEM_TYPE_VIEWER_NODE = 4,
    VIEWER_PATH_ELEM_TYPE_REPEAT_ZONE = 5,
    VIEWER_PATH_ELEM_TYPE_FOREACH_GEOMETRY_ELEMENT_ZONE = 6,
    VIEWER_PATH_ELEM_TYPE_EVALUATE_CLOSURE = 7,
}

impl Default for ViewerPathElemType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const VIEWER_PATH_ELEM_TYPE_ID: i32 = ViewerPathElemType::VIEWER_PATH_ELEM_TYPE_ID as i32;
pub const VIEWER_PATH_ELEM_TYPE_MODIFIER: i32 = ViewerPathElemType::VIEWER_PATH_ELEM_TYPE_MODIFIER as i32;
pub const VIEWER_PATH_ELEM_TYPE_GROUP_NODE: i32 = ViewerPathElemType::VIEWER_PATH_ELEM_TYPE_GROUP_NODE as i32;
pub const VIEWER_PATH_ELEM_TYPE_SIMULATION_ZONE: i32 = ViewerPathElemType::VIEWER_PATH_ELEM_TYPE_SIMULATION_ZONE as i32;
pub const VIEWER_PATH_ELEM_TYPE_VIEWER_NODE: i32 = ViewerPathElemType::VIEWER_PATH_ELEM_TYPE_VIEWER_NODE as i32;
pub const VIEWER_PATH_ELEM_TYPE_REPEAT_ZONE: i32 = ViewerPathElemType::VIEWER_PATH_ELEM_TYPE_REPEAT_ZONE as i32;
pub const VIEWER_PATH_ELEM_TYPE_FOREACH_GEOMETRY_ELEMENT_ZONE: i32 = ViewerPathElemType::VIEWER_PATH_ELEM_TYPE_FOREACH_GEOMETRY_ELEMENT_ZONE as i32;
pub const VIEWER_PATH_ELEM_TYPE_EVALUATE_CLOSURE: i32 = ViewerPathElemType::VIEWER_PATH_ELEM_TYPE_EVALUATE_CLOSURE as i32;

