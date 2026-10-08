//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBoidRuleType(pub i32);

impl eBoidRuleType {
    pub const eBoidRuleType_None: Self = Self((0) as i32);
    pub const eBoidRuleType_Goal: Self = Self((1) as i32);
    pub const eBoidRuleType_Avoid: Self = Self((2) as i32);
    pub const eBoidRuleType_AvoidCollision: Self = Self((3) as i32);
    pub const eBoidRuleType_Separate: Self = Self((4) as i32);
    pub const eBoidRuleType_Flock: Self = Self((5) as i32);
    pub const eBoidRuleType_FollowLeader: Self = Self((6) as i32);
    pub const eBoidRuleType_AverageSpeed: Self = Self((7) as i32);
    pub const eBoidRuleType_Fight: Self = Self((8) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBoidRule_Flag(pub i32);

impl eBoidRule_Flag {
    pub const BOIDRULE_CURRENT: Self = Self((1 << 0) as i32);
    pub const BOIDRULE_IN_AIR: Self = Self((1 << 2) as i32);
    pub const BOIDRULE_ON_LAND: Self = Self((1 << 3) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBoidRuleGoalAvoid_Option(pub i32);

impl eBoidRuleGoalAvoid_Option {
    pub const BRULE_GOAL_AVOID_PREDICT: Self = Self((1 << 0) as i32);
    pub const BRULE_GOAL_AVOID_ARRIVE: Self = Self((1 << 1) as i32);
    pub const BRULE_GOAL_AVOID_SIGNAL: Self = Self((1 << 2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBoidRuleAvoidCollision_Option(pub i32);

impl eBoidRuleAvoidCollision_Option {
    pub const BRULE_ACOLL_WITH_BOIDS: Self = Self((1 << 0) as i32);
    pub const BRULE_ACOLL_WITH_DEFLECTORS: Self = Self((1 << 1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBoidMode(pub i16);

impl eBoidMode {
    pub const eBoidMode_InAir: Self = Self((0) as i16);
    pub const eBoidMode_OnLand: Self = Self((1) as i16);
    pub const eBoidMode_Climbing: Self = Self((2) as i16);
    pub const eBoidMode_Falling: Self = Self((3) as i16);
    pub const eBoidMode_Liftoff: Self = Self((4) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBoidRulesetType(pub i32);

impl eBoidRulesetType {
    pub const eBoidRulesetType_Fuzzy: Self = Self((0) as i32);
    pub const eBoidRulesetType_Random: Self = Self((1) as i32);
    pub const eBoidRulesetType_Average: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBoid_Option(pub i32);

impl eBoid_Option {
    pub const BOID_ALLOW_FLIGHT: Self = Self((1 << 0) as i32);
    pub const BOID_ALLOW_LAND: Self = Self((1 << 1) as i32);
    pub const BOID_ALLOW_CLIMB: Self = Self((1 << 2) as i32);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BoidRule {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub r#type: eBoidRuleType,
    pub flag: eBoidRule_Flag,
}

impl Default for BoidRule {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BoidRuleGoalAvoid {
    pub rule: BoidRule,
    pub ob: *mut core::ffi::c_void,
    pub options: eBoidRuleGoalAvoid_Option,
}

impl Default for BoidRuleGoalAvoid {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BoidRuleAvoidCollision {
    pub rule: BoidRule,
    pub options: eBoidRuleAvoidCollision_Option,
}

impl Default for BoidRuleAvoidCollision {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BoidRuleAverageSpeed {
    pub rule: BoidRule,
    pub wander: f32,
    pub level: f32,
    pub speed: f32,
    pub _pad0: [u8; 4],
}

impl Default for BoidRuleAverageSpeed {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BoidRuleFight {
    pub rule: BoidRule,
    pub distance: f32,
    pub flee_distance: f32,
}

impl Default for BoidRuleFight {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BoidData {
    pub health: f32,
    pub acc: [f32; 3],
}

impl Default for BoidData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BoidState {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub rules: ListBaseT<BoidRule>,
    pub nullptr: ListBaseT<BoidRule>,
}

impl Default for BoidState {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BoidSettings {
    pub options: eBoid_Option,
}

impl Default for BoidSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

