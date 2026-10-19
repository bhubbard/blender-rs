//! Auto-transpiled C/C++ header module: deg_builder_nodes
//! Upstream: source/blender/depsgraph/intern/builder/deg_builder_nodes.h

use crate::*;
use crate::DEG_depsgraph::Main;
use std::ffi::c_char;

#[repr(C)]
#[derive(Debug)]
pub struct DepsgraphNodeBuilder {
    pub bmain: *mut Main,
    pub graph: *mut DepsgraphInternal,
    pub built_map: BuilderMap,
}

impl DepsgraphNodeBuilder {
    pub fn new(bmain: *mut Main, graph: *mut DepsgraphInternal) -> Self {
        Self {
            bmain,
            graph,
            built_map: BuilderMap::new(),
        }
    }

    pub fn build_id(&mut self, id: *mut ID) -> Option<*mut IDNode> {
        if id.is_null() || self.graph.is_null() {
            return None;
        }
        unsafe {
            let id_node = (*self.graph).add_id_node(id);
            Some(id_node as *mut IDNode)
        }
    }

    pub fn add_component(&mut self, id: *mut ID, type_: NodeType, name: &str) -> Option<*mut ComponentNode> {
        let id_node_ptr = self.build_id(id)?;
        unsafe {
            let comp = (*id_node_ptr).add_component(type_, name);
            Some(comp as *mut ComponentNode)
        }
    }

    pub fn add_operation(
        &mut self,
        id: *mut ID,
        comp_type: NodeType,
        comp_name: &str,
        opcode: OperationCode,
        op_name: &str,
        name_tag: i32,
        op_cb: DepsEvalOperationCb,
    ) -> Option<*mut OperationNode> {
        let comp_ptr = self.add_component(id, comp_type, comp_name)?;
        unsafe {
            let op = (*comp_ptr).add_operation(opcode, op_name, name_tag, op_cb);
            Some(op as *mut OperationNode)
        }
    }
}
