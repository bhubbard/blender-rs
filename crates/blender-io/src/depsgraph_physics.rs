//! Auto-transpiled C/C++ header module: depsgraph_physics
//! Upstream: source/blender/depsgraph/intern/depsgraph_physics.hh

use crate::*;
use crate::DEG_depsgraph::Depsgraph;
use blender_dna::DNA_collection_types::Collection;
use std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CollisionRelation {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EffectorRelation {
    pub _opaque: [u8; 0],
}

extern "C" {
    pub fn DEG_build_effector_relations(
        graph: *mut Depsgraph,
        collection: *mut Collection,
    ) -> *mut ListBaseT<EffectorRelation>;

    pub fn DEG_build_collision_relations(
        graph: *mut Depsgraph,
        collection: *mut Collection,
        modifier_type: c_uint,
    ) -> *mut ListBaseT<CollisionRelation>;

    pub fn DEG_clear_physics_relations(graph: *mut Depsgraph);
}
