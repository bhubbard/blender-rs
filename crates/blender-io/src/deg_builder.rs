//! Auto-transpiled C/C++ header module: deg_builder
//! Upstream: source/blender/depsgraph/intern/builder/deg_builder.h

use crate::*;
use crate::DEG_depsgraph::{Depsgraph, Main};

#[repr(C)]
#[derive(Debug)]
pub struct DepsgraphBuilder {
    pub bmain: *mut Main,
    pub graph: *mut Depsgraph,
    pub cache: *mut DepsgraphBuilderCache,
}

impl DepsgraphBuilder {
    pub fn new(bmain: *mut Main, graph: *mut Depsgraph, cache: *mut DepsgraphBuilderCache) -> Self {
        Self {
            bmain,
            graph,
            cache,
        }
    }
}

extern "C" {
    pub fn DEG_deg_check_id_in_depsgraph(graph: *const Depsgraph, id_orig: *mut ID) -> bool;
    pub fn DEG_deg_check_base_in_depsgraph(graph: *const Depsgraph, base: *mut Base) -> bool;
    pub fn DEG_deg_graph_build_finalize(bmain: *mut Main, graph: *mut Depsgraph);
}
