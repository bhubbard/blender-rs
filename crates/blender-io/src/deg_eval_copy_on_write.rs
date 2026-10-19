//! Auto-transpiled C/C++ header module: deg_eval_copy_on_write
//! Upstream: source/blender/depsgraph/intern/eval/deg_eval_copy_on_write.h

use crate::*;
use crate::DEG_depsgraph::Depsgraph;

extern "C" {
    pub fn DEG_deg_update_eval_copy_datablock(depsgraph: *const Depsgraph, id_node: *const IDNode) -> *mut ID;
    pub fn DEG_deg_free_eval_copy_datablock(id_cow: *mut ID);
    pub fn DEG_deg_create_eval_copy(depsgraph: *mut Depsgraph, id_node: *const IDNode);
    pub fn DEG_deg_validate_eval_copy_datablock(id_cow: *mut ID) -> bool;
    pub fn DEG_deg_tag_id_as_copy_on_eval(id: *mut ID);
    pub fn DEG_deg_tag_id_as_no_copy_on_eval(id: *mut ID);
    pub fn DEG_deg_is_id_copy_on_eval(id: *const ID) -> bool;
}
