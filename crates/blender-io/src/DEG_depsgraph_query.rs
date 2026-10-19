//! Auto-transpiled C/C++ header module: DEG_depsgraph_query
//! Source: upstream/source/blender/depsgraph/DEG_depsgraph_query.hh

use crate::DEG_depsgraph::{Depsgraph, Main, Scene, ViewLayer, eEvaluationMode};
use blender_dna::DNA_ID::ID;
use blender_dna::DNA_object_types::Object;

extern "C" {
    pub fn DEG_get_input_scene(graph: *const Depsgraph) -> *mut Scene;
    pub fn DEG_get_input_view_layer(graph: *const Depsgraph) -> *mut ViewLayer;
    pub fn DEG_get_bmain(graph: *const Depsgraph) -> *mut Main;
    pub fn DEG_get_mode(graph: *const Depsgraph) -> eEvaluationMode;
    pub fn DEG_get_ctime(graph: *const Depsgraph) -> f32;

    pub fn DEG_id_type_updated(depsgraph: *const Depsgraph, id_type: i16) -> bool;
    pub fn DEG_id_type_any_updated(depsgraph: *const Depsgraph) -> bool;
    pub fn DEG_id_is_user_modified(graph: *const Depsgraph, id: *const ID) -> bool;
    pub fn DEG_id_type_any_exists(depsgraph: *const Depsgraph, id_type: i16) -> bool;
    pub fn DEG_get_eval_flags_for_id(graph: *const Depsgraph, id: *const ID) -> u32;

    pub fn DEG_get_evaluated_scene(graph: *const Depsgraph) -> *mut Scene;
    pub fn DEG_get_evaluated_view_layer(graph: *const Depsgraph) -> *mut ViewLayer;
    pub fn DEG_get_evaluated_id(graph: *const Depsgraph, id: *const ID) -> *mut ID;
    pub fn DEG_get_evaluated_object(graph: *const Depsgraph, object: *const Object) -> *mut Object;
    pub fn DEG_get_original_id(id: *const ID) -> *mut ID;
    pub fn DEG_get_original_object(object: *const Object) -> *mut Object;
    pub fn DEG_is_evaluated_id(id: *const ID) -> bool;
    pub fn DEG_is_evaluated_object(object: *const Object) -> bool;
    pub fn DEG_is_active(graph: *const Depsgraph) -> bool;
}
