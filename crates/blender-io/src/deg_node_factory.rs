//! Auto-transpiled C/C++ header module: deg_node_factory
//! Upstream: source/blender/depsgraph/intern/node/deg_node_factory.hh

use crate::*;
use std::ffi::c_char;

pub trait DepsNodeFactory {
    fn type_(&self) -> NodeType;
    fn type_name(&self) -> *const c_char;
    fn id_recalc_tag(&self) -> i32;
    fn create_node(&self, id: *const ID, subdata: *const c_char, name: &str) -> Box<DepsgraphNode>;
}

extern "C" {
    pub fn register_node_typeinfo(factory: *mut ());
    pub fn type_get_factory(node_type: NodeType) -> *mut ();
}
