//! Auto-transpiled C/C++ header module: DEG_depsgraph_light_linking
//! Source: upstream/source/blender/depsgraph/DEG_depsgraph_light_linking.hh

use crate::DEG_depsgraph::Depsgraph;
use blender_dna::DNA_object_types::Object;

extern "C" {
    pub fn eval_runtime_data(depsgraph: *const Depsgraph, object_eval: *mut Object);
}
