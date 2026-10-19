//! Auto-transpiled C/C++ header module: pipeline_compositor
//! Upstream: source/blender/depsgraph/intern/builder/pipeline_compositor.h

use crate::*;
use crate::DEG_depsgraph::Depsgraph;

#[repr(C)]
#[derive(Debug)]
pub struct CompositorBuilderPipeline {
    pub base: AbstractBuilderPipeline,
}

impl CompositorBuilderPipeline {
    pub fn new(graph: *mut Depsgraph) -> Self {
        Self {
            base: AbstractBuilderPipeline::new(graph),
        }
    }
}
