//! Auto-transpiled C/C++ header module: pipeline
//! Upstream: source/blender/depsgraph/intern/builder/pipeline.h

use crate::*;
use crate::DEG_depsgraph::{Depsgraph, Main, Scene, ViewLayer};

#[repr(C)]
#[derive(Debug)]
pub struct AbstractBuilderPipeline {
    pub deg_graph: *mut Depsgraph,
    pub bmain: *mut Main,
    pub scene: *mut Scene,
    pub view_layer: *mut ViewLayer,
}

impl AbstractBuilderPipeline {
    pub fn new(deg_graph: *mut Depsgraph) -> Self {
        Self {
            deg_graph,
            bmain: core::ptr::null_mut(),
            scene: core::ptr::null_mut(),
            view_layer: core::ptr::null_mut(),
        }
    }
}
