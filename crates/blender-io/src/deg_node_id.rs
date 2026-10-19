//! Auto-transpiled C/C++ header module: deg_node_id
//! Upstream: source/blender/depsgraph/intern/node/deg_node_id.hh

use crate::*;
use std::collections::HashMap;

pub type IDComponentsMask = u64;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum eDepsNode_LinkedState_Type {
    DEG_ID_LINKED_INDIRECTLY = 0,
    DEG_ID_LINKED_VIA_SET = 1,
    DEG_ID_LINKED_DIRECTLY = 2,
}

impl Default for eDepsNode_LinkedState_Type {
    fn default() -> Self {
        eDepsNode_LinkedState_Type::DEG_ID_LINKED_INDIRECTLY
    }
}

pub fn linked_state_as_string(linked_state: eDepsNode_LinkedState_Type) -> &'static str {
    match linked_state {
        eDepsNode_LinkedState_Type::DEG_ID_LINKED_INDIRECTLY => "INDIRECTLY",
        eDepsNode_LinkedState_Type::DEG_ID_LINKED_VIA_SET => "VIA_SET",
        eDepsNode_LinkedState_Type::DEG_ID_LINKED_DIRECTLY => "DIRECTLY",
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentIDKey {
    pub type_: NodeType,
    pub name: String,
}

impl ComponentIDKey {
    pub fn new(type_: NodeType, name: &str) -> Self {
        Self {
            type_,
            name: name.to_string(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct IDNode {
    pub base: DepsgraphNode,
    pub id_orig: *mut ID,
    pub id_cow: *mut ID,
    pub components: HashMap<ComponentIDKey, Box<ComponentNode>>,
    pub linked_state: eDepsNode_LinkedState_Type,
    pub eval_flags: u32,
    pub is_directly_visible: bool,
    pub is_eval_pose_needed: bool,
}

impl Default for IDNode {
    fn default() -> Self {
        Self {
            base: DepsgraphNode {
                type_: NodeType::ID_REF,
                ..Default::default()
            },
            id_orig: core::ptr::null_mut(),
            id_cow: core::ptr::null_mut(),
            components: HashMap::new(),
            linked_state: eDepsNode_LinkedState_Type::default(),
            eval_flags: 0,
            is_directly_visible: false,
            is_eval_pose_needed: false,
        }
    }
}

impl IDNode {
    pub fn new(id_orig: *mut ID) -> Self {
        Self {
            base: DepsgraphNode {
                type_: NodeType::ID_REF,
                ..Default::default()
            },
            id_orig,
            id_cow: core::ptr::null_mut(),
            components: HashMap::new(),
            linked_state: eDepsNode_LinkedState_Type::default(),
            eval_flags: 0,
            is_directly_visible: false,
            is_eval_pose_needed: false,
        }
    }

    pub fn find_component(&self, type_: NodeType, name: &str) -> Option<&ComponentNode> {
        let key = ComponentIDKey::new(type_, name);
        self.components.get(&key).map(|c| c.as_ref())
    }

    pub fn find_component_mut(&mut self, type_: NodeType, name: &str) -> Option<&mut ComponentNode> {
        let key = ComponentIDKey::new(type_, name);
        self.components.get_mut(&key).map(|c| c.as_mut())
    }

    pub fn add_component(&mut self, type_: NodeType, name: &str) -> &mut ComponentNode {
        let key = ComponentIDKey::new(type_, name);
        let node = Box::new(ComponentNode::new(type_, name));
        self.components.entry(key.clone()).or_insert(node);
        self.components.get_mut(&key).unwrap().as_mut()
    }
}
