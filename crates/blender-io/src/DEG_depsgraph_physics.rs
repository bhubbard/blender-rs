//! Auto-transpiled C/C++ header module: DEG_depsgraph_physics

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Collection {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CollisionRelation {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DepsNodeHandle {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Depsgraph {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EffectorRelation {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EffectorWeights {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ModifierData {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Object {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ePhysicsCollisionType {
    DEG_PHYSICS_COLLISION = 0,
    DEG_PHYSICS_SMOKE_COLLISION = 1,
    DEG_PHYSICS_DYNAMIC_BRUSH = 2,
    DEG_PHYSICS_COLLISION_NUM = 3,
}

impl Default for ePhysicsCollisionType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const DEG_PHYSICS_COLLISION: i32 = ePhysicsCollisionType::DEG_PHYSICS_COLLISION as i32;
pub const DEG_PHYSICS_SMOKE_COLLISION: i32 = ePhysicsCollisionType::DEG_PHYSICS_SMOKE_COLLISION as i32;
pub const DEG_PHYSICS_DYNAMIC_BRUSH: i32 = ePhysicsCollisionType::DEG_PHYSICS_DYNAMIC_BRUSH as i32;
pub const DEG_PHYSICS_COLLISION_NUM: i32 = ePhysicsCollisionType::DEG_PHYSICS_COLLISION_NUM as i32;
