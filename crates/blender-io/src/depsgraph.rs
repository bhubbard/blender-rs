//! Auto-transpiled C/C++ header module: depsgraph
//! Upstream: source/blender/depsgraph/intern/depsgraph.hh

use crate::*;
use crate::DEG_depsgraph::{Main, Scene, ViewLayer, eEvaluationMode};
use std::collections::HashMap;
use std::ffi::c_char;

#[repr(C)]
#[derive(Debug, Default)]
pub struct DepsgraphInternal {
    pub bmain: *mut Main,
    pub scene: *mut Scene,
    pub view_layer: *mut ViewLayer,
    pub mode: eEvaluationMode,
    pub id_hash: HashMap<*const ID, Box<IDNode>>,
    pub id_nodes: Vec<*mut IDNode>,
    pub time_source: Option<Box<TimeSourceNode>>,
    pub entry_tags: Vec<*mut OperationNode>,
    pub relations: Vec<Box<Relation>>,
}

impl DepsgraphInternal {
    pub fn new(bmain: *mut Main, scene: *mut Scene, view_layer: *mut ViewLayer, mode: eEvaluationMode) -> Self {
        Self {
            bmain,
            scene,
            view_layer,
            mode,
            id_hash: HashMap::new(),
            id_nodes: Vec::new(),
            time_source: None,
            entry_tags: Vec::new(),
            relations: Vec::new(),
        }
    }

    pub fn add_time_source(&mut self) -> &mut TimeSourceNode {
        if self.time_source.is_none() {
            self.time_source = Some(Box::new(TimeSourceNode::new()));
        }
        self.time_source.as_mut().unwrap().as_mut()
    }

    pub fn find_time_source(&self) -> Option<&TimeSourceNode> {
        self.time_source.as_ref().map(|b| b.as_ref())
    }

    pub fn tag_time_source(&mut self) {
        if let Some(ts) = self.time_source.as_mut() {
            ts.tag_update();
        }
    }

    pub fn find_id_node(&self, id: *const ID) -> Option<&IDNode> {
        self.id_hash.get(&id).map(|b| b.as_ref())
    }

    pub fn find_id_node_mut(&mut self, id: *const ID) -> Option<&mut IDNode> {
        self.id_hash.get_mut(&id).map(|b| b.as_mut())
    }

    pub fn add_id_node(&mut self, id: *mut ID) -> &mut IDNode {
        let node = Box::new(IDNode::new(id));
        self.id_hash.entry(id as *const ID).or_insert(node);
        let ptr = self.id_hash.get_mut(&(id as *const ID)).unwrap().as_mut() as *mut IDNode;
        if !self.id_nodes.contains(&ptr) {
            self.id_nodes.push(ptr);
        }
        self.id_hash.get_mut(&(id as *const ID)).unwrap().as_mut()
    }

    pub fn add_new_relation(
        &mut self,
        from: *mut DepsgraphNode,
        to: *mut DepsgraphNode,
        description: *const c_char,
        flags: i32,
    ) -> &mut Relation {
        let rel = Box::new(Relation {
            from,
            to,
            name: description,
            flag: flags,
        });
        self.relations.push(rel);
        let idx = self.relations.len() - 1;
        let rel_ptr = self.relations[idx].as_mut() as *mut Relation;
        unsafe {
            if !from.is_null() {
                (*from).outlinks.push(rel_ptr);
            }
            if !to.is_null() {
                (*to).inlinks.push(rel_ptr);
            }
        }
        self.relations[idx].as_mut()
    }

    pub fn add_entry_tag(&mut self, node: *mut OperationNode) {
        if !self.entry_tags.contains(&node) {
            self.entry_tags.push(node);
        }
    }

    pub fn clear_all_nodes(&mut self) {
        self.id_hash.clear();
        self.id_nodes.clear();
        self.time_source = None;
        self.entry_tags.clear();
        self.relations.clear();
    }
}
