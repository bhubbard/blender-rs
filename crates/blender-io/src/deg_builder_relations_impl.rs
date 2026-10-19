//! Auto-transpiled C/C++ header module: deg_builder_relations_impl
//! Upstream: source/blender/depsgraph/intern/builder/deg_builder_relations_impl.h

use crate::*;
use std::ffi::c_char;

impl DepsgraphInternal {
    pub fn add_relation_by_nodes(
        &mut self,
        from: *mut DepsgraphNode,
        to: *mut DepsgraphNode,
        description: *const c_char,
        flags: i32,
    ) -> Option<*mut Relation> {
        if from.is_null() || to.is_null() {
            return None;
        }
        Some(self.add_new_relation(from, to, description, flags))
    }
}
