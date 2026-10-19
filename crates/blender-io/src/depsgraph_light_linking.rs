//! Auto-transpiled C/C++ header module: depsgraph_light_linking
//! Upstream: source/blender/depsgraph/intern/depsgraph_light_linking.hh

use crate::*;
use blender_dna::Object;
use std::collections::HashMap;

pub const LIGHT_SET_MAX_ID: i32 = 63;
pub const LIGHT_SET_DEFAULT_ID: i32 = 0;
pub const SET_MEMBERSHIP_ALL: u64 = !0u64;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LightSet {
    pub include_collection_mask: u64,
    pub exclude_collection_mask: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EmitterSetMembership {
    pub set_membership: u64,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReceiverKey {
    pub receiver: *mut Object,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LightLinking {
    pub receiver_sets: HashMap<ReceiverKey, LightSet>,
}
