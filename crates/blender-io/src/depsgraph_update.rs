//! Auto-transpiled C/C++ header module: depsgraph_update
//! Upstream: source/blender/depsgraph/intern/depsgraph_update.hh

use crate::*;

extern "C" {
    pub fn DEG_deg_editors_id_update(update_ctx: *const DEGEditorUpdateContext, id: *mut ID);
    pub fn DEG_deg_editors_scene_update(update_ctx: *const DEGEditorUpdateContext, updated: bool);
}
