//! Auto-transpiled C/C++ header module: deg_builder_remove_noop
//! Upstream: source/blender/depsgraph/intern/builder/deg_builder_remove_noop.h

use crate::*;
use crate::DEG_depsgraph::Depsgraph;

extern "C" {
    pub fn DEG_deg_graph_remove_unused_noops(graph: *mut Depsgraph);
}
