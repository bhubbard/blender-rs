//! Auto-transpiled C/C++ header module: deg_builder_transitive
//! Upstream: source/blender/depsgraph/intern/builder/deg_builder_transitive.h

use crate::*;
use crate::DEG_depsgraph::Depsgraph;

extern "C" {
    pub fn DEG_deg_graph_transitive_reduction(graph: *mut Depsgraph);
}
