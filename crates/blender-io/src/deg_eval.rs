//! Auto-transpiled C/C++ header module: deg_eval
//! Upstream: source/blender/depsgraph/intern/eval/deg_eval.h

use crate::*;
use crate::DEG_depsgraph::Depsgraph;

extern "C" {
    pub fn DEG_evaluate_on_refresh(graph: *mut Depsgraph);
}
