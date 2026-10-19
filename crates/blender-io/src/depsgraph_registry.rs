//! Auto-transpiled C/C++ header module: depsgraph_registry
//! Upstream: source/blender/depsgraph/intern/depsgraph_registry.hh

use crate::*;
use crate::DEG_depsgraph::{Depsgraph, Main};

extern "C" {
    pub fn DEG_register_graph(depsgraph: *mut Depsgraph);
    pub fn DEG_unregister_graph(depsgraph: *mut Depsgraph);
    pub fn DEG_get_all_registered_graphs(bmain: *mut Main, count: *mut usize) -> *const *mut Depsgraph;
}
