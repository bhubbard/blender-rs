//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum eBoidRuleType {
    #[default]
    eBoidRuleType_None = 0,
    eBoidRuleType_Goal = 1,
    eBoidRuleType_Avoid = 2,
    eBoidRuleType_AvoidCollision = 3,
    eBoidRuleType_Separate = 4,
    eBoidRuleType_Flock = 5,
    eBoidRuleType_FollowLeader = 6,
    eBoidRuleType_AverageSpeed = 7,
    eBoidRuleType_Fight = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum eBoidRule_Flag {
    #[default]
    BOIDRULE_CURRENT = 1 << 0,
    BOIDRULE_IN_AIR = 1 << 2,
    BOIDRULE_ON_LAND = 1 << 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum eBoidRuleGoalAvoid_Option {
    #[default]
    BRULE_GOAL_AVOID_PREDICT = 1 << 0,
    BRULE_GOAL_AVOID_ARRIVE = 1 << 1,
    BRULE_GOAL_AVOID_SIGNAL = 1 << 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum eBoidRuleAvoidCollision_Option {
    #[default]
    BRULE_ACOLL_WITH_BOIDS = 1 << 0,
    BRULE_ACOLL_WITH_DEFLECTORS = 1 << 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i16)]
pub enum eBoidMode {
    #[default]
    eBoidMode_InAir = 0,
    eBoidMode_OnLand = 1,
    eBoidMode_Climbing = 2,
    eBoidMode_Falling = 3,
    eBoidMode_Liftoff = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum eBoidRulesetType {
    #[default]
    eBoidRulesetType_Fuzzy = 0,
    eBoidRulesetType_Random = 1,
    eBoidRulesetType_Average = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum eBoid_Option {
    #[default]
    BOID_ALLOW_FLIGHT = 1 << 0,
    BOID_ALLOW_LAND = 1 << 1,
    BOID_ALLOW_CLIMB = 1 << 2,
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

