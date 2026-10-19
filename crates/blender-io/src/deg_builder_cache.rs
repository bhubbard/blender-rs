//! Auto-transpiled C/C++ header module: deg_builder_cache
//! Upstream: source/blender/depsgraph/intern/builder/deg_builder_cache.h

use crate::*;
use std::collections::HashSet;
use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AnimatedPropertyID {
    pub data: *const c_void,
    pub property_rna: *const c_void,
}

#[derive(Debug, Clone, Default)]
pub struct AnimatedPropertyStorage {
    pub animated_properties: HashSet<AnimatedPropertyID>,
}

impl AnimatedPropertyStorage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tag_property_as_animated(&mut self, property_id: AnimatedPropertyID) {
        self.animated_properties.insert(property_id);
    }

    pub fn is_property_animated(&self, property_id: &AnimatedPropertyID) -> bool {
        self.animated_properties.contains(property_id)
    }
}

#[derive(Debug, Clone, Default)]
pub struct DepsgraphBuilderCache {
    pub storage: AnimatedPropertyStorage,
}

impl DepsgraphBuilderCache {
    pub fn new() -> Self {
        Self::default()
    }
}
