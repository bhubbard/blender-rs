//! Auto-transpiled C/C++ header module: deg_builder_map
//! Upstream: source/blender/depsgraph/intern/builder/deg_builder_map.hh

use crate::*;
use std::collections::HashMap;

pub const DEG_BUILDER_TAG_ANIMATION: i32 = 1 << 0;
pub const DEG_BUILDER_TAG_PARAMETERS: i32 = 1 << 1;
pub const DEG_BUILDER_TAG_TRANSFORM: i32 = 1 << 2;
pub const DEG_BUILDER_TAG_GEOMETRY: i32 = 1 << 3;
pub const DEG_BUILDER_TAG_SCENE_COMPOSITOR: i32 = 1 << 4;
pub const DEG_BUILDER_TAG_SCENE_SEQUENCER: i32 = 1 << 5;
pub const DEG_BUILDER_TAG_SCENE_AUDIO: i32 = 1 << 6;
pub const DEG_BUILDER_TAG_COLLECTION_CHILDREN_HIERARCHY: i32 = 1 << 7;
pub const DEG_BUILDER_TAG_COLLECTION_PROPERTIES: i32 = 1 << 8;
pub const DEG_BUILDER_TAG_COMPLETE: i32 = DEG_BUILDER_TAG_ANIMATION
    | DEG_BUILDER_TAG_PARAMETERS
    | DEG_BUILDER_TAG_TRANSFORM
    | DEG_BUILDER_TAG_GEOMETRY
    | DEG_BUILDER_TAG_SCENE_COMPOSITOR
    | DEG_BUILDER_TAG_SCENE_SEQUENCER
    | DEG_BUILDER_TAG_SCENE_AUDIO
    | DEG_BUILDER_TAG_COLLECTION_PROPERTIES;

#[derive(Debug, Clone, Default)]
pub struct BuilderMap {
    pub tags: HashMap<*const ID, i32>,
}

impl BuilderMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn check_is_built(&self, id: *const ID, tag: i32) -> bool {
        self.tags.get(&id).map_or(false, |&t| (t & tag) == tag)
    }

    pub fn tag_built(&mut self, id: *const ID, tag: i32) {
        *self.tags.entry(id).or_insert(0) |= tag;
    }

    pub fn check_is_built_and_tag(&mut self, id: *const ID, tag: i32) -> bool {
        if self.check_is_built(id, tag) {
            true
        } else {
            self.tag_built(id, tag);
            false
        }
    }
}
