//! Auto-transpiled C/C++ header module: deg_node
//! Upstream: source/blender/depsgraph/intern/node/deg_node.hh

use crate::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeClass {
    GENERIC = 0,
    COMPONENT = 1,
    OPERATION = 2,
}

impl Default for NodeClass {
    fn default() -> Self {
        NodeClass::GENERIC
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeType {
    UNDEFINED = 0,
    OPERATION,
    TIMESOURCE,
    ID_REF,
    PARAMETERS,
    ANIMATION,
    TRANSFORM,
    GEOMETRY,
    SEQUENCER,
    COMPOSITOR,
    LAYER_COLLECTIONS,
    COPY_ON_EVAL,
    OBJECT_FROM_LAYER,
    HIERARCHY,
    AUDIO,
    ARMATURE,
    GENERIC_DATABLOCK,
    SCENE,
    VISIBILITY,
    EVAL_POSE,
    BONE,
    PARTICLE_SYSTEM,
    PARTICLE_SETTINGS,
    SHADING,
    POINT_CACHE,
    IMAGE_ANIMATION,
    CACHE,
    BATCH_CACHE,
    INSTANCING,
    SYNCHRONIZATION,
    NTREE_OUTPUT,
    NTREE_GEOMETRY_PREPROCESS,
    NUM_TYPES,
}

impl Default for NodeType {
    fn default() -> Self {
        NodeType::UNDEFINED
    }
}

pub fn node_class_as_string(node_class: NodeClass) -> &'static str {
    match node_class {
        NodeClass::GENERIC => "GENERIC",
        NodeClass::COMPONENT => "COMPONENT",
        NodeClass::OPERATION => "OPERATION",
    }
}

pub fn node_type_as_string(node_type: NodeType) -> &'static str {
    match node_type {
        NodeType::UNDEFINED => "UNDEFINED",
        NodeType::OPERATION => "OPERATION",
        NodeType::TIMESOURCE => "TIMESOURCE",
        NodeType::ID_REF => "ID_REF",
        NodeType::PARAMETERS => "PARAMETERS",
        NodeType::ANIMATION => "ANIMATION",
        NodeType::TRANSFORM => "TRANSFORM",
        NodeType::GEOMETRY => "GEOMETRY",
        NodeType::SEQUENCER => "SEQUENCER",
        NodeType::COMPOSITOR => "COMPOSITOR",
        NodeType::LAYER_COLLECTIONS => "LAYER_COLLECTIONS",
        NodeType::COPY_ON_EVAL => "COPY_ON_EVAL",
        NodeType::OBJECT_FROM_LAYER => "OBJECT_FROM_LAYER",
        NodeType::HIERARCHY => "HIERARCHY",
        NodeType::AUDIO => "AUDIO",
        NodeType::ARMATURE => "ARMATURE",
        NodeType::GENERIC_DATABLOCK => "GENERIC_DATABLOCK",
        NodeType::SCENE => "SCENE",
        NodeType::VISIBILITY => "VISIBILITY",
        NodeType::EVAL_POSE => "EVAL_POSE",
        NodeType::BONE => "BONE",
        NodeType::PARTICLE_SYSTEM => "PARTICLE_SYSTEM",
        NodeType::PARTICLE_SETTINGS => "PARTICLE_SETTINGS",
        NodeType::SHADING => "SHADING",
        NodeType::POINT_CACHE => "POINT_CACHE",
        NodeType::IMAGE_ANIMATION => "IMAGE_ANIMATION",
        NodeType::CACHE => "CACHE",
        NodeType::BATCH_CACHE => "BATCH_CACHE",
        NodeType::INSTANCING => "INSTANCING",
        NodeType::SYNCHRONIZATION => "SYNCHRONIZATION",
        NodeType::NTREE_OUTPUT => "NTREE_OUTPUT",
        NodeType::NTREE_GEOMETRY_PREPROCESS => "NTREE_GEOMETRY_PREPROCESS",
        NodeType::NUM_TYPES => "NUM_TYPES",
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct TypeInfo {
    pub node_type: NodeType,
    pub type_name: *const c_char,
    pub id_recalc_tag: c_int,
}

impl Default for TypeInfo {
    fn default() -> Self {
        Self {
            node_type: NodeType::UNDEFINED,
            type_name: core::ptr::null(),
            id_recalc_tag: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct NodeStats {
    pub current_time: f64,
}

impl NodeStats {
    pub fn reset(&mut self) {
        self.current_time = 0.0;
    }
    pub fn reset_current(&mut self) {
        self.current_time = 0.0;
    }
}

pub use crate::depsgraph_relation::Relation;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct DepsgraphNode {
    pub name: String,
    pub type_: NodeType,
    pub inlinks: Vec<*mut Relation>,
    pub outlinks: Vec<*mut Relation>,
    pub stats: NodeStats,
    pub custom_flags: i32,
}

pub type Node = DepsgraphNode;

impl Default for DepsgraphNode {
    fn default() -> Self {
        Self {
            name: String::new(),
            type_: NodeType::UNDEFINED,
            inlinks: Vec::new(),
            outlinks: Vec::new(),
            stats: NodeStats::default(),
            custom_flags: 0,
        }
    }
}

impl DepsgraphNode {
    pub fn identifier(&self) -> String {
        format!("{}:{}", node_type_as_string(self.type_), self.name)
    }

    pub fn get_class(&self) -> NodeClass {
        match self.type_ {
            NodeType::OPERATION => NodeClass::OPERATION,
            NodeType::TIMESOURCE | NodeType::ID_REF => NodeClass::GENERIC,
            _ => NodeClass::COMPONENT,
        }
    }
}

extern "C" {
    pub fn deg_register_base_depsnodes();
}
