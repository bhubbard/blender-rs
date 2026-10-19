//! Auto-transpiled C/C++ header module: depsgraph_tag
//! Upstream: source/blender/depsgraph/intern/depsgraph_tag.hh

use crate::*;
use crate::DEG_depsgraph::{Depsgraph, Main};
use std::os::raw::c_uint;

extern "C" {
    pub fn DEG_geometry_tag_to_component(id: *const ID) -> NodeType;
    pub fn DEG_id_tag_update(bmain: *mut Main, id: *mut ID, flags: c_uint, update_source: eUpdateSource);
    pub fn DEG_graph_id_tag_update(
        bmain: *mut Main,
        graph: *mut Depsgraph,
        id: *mut ID,
        flags: c_uint,
        update_source: eUpdateSource,
    );
    pub fn DEG_graph_tag_ids_for_visible_update(graph: *mut Depsgraph);
}
