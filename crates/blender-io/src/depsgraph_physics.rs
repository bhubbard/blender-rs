//! Auto-transpiled C/C++ header module: depsgraph_physics

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct T {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CollisionComponentFlag {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionComponentFlag_2 {
    None = 0,
    Transform = 1 << 0,
    Geometry = 1 << 1,
    EvalPose = 1 << 2,
}

impl Default for CollisionComponentFlag_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const None: i32 = CollisionComponentFlag_2::None as i32;
pub const Transform: i32 = CollisionComponentFlag_2::Transform as i32;
pub const Geometry: i32 = CollisionComponentFlag_2::Geometry as i32;
pub const EvalPose: i32 = CollisionComponentFlag_2::EvalPose as i32;
