//! Auto-transpiled C/C++ header module: DEG_depsgraph_build

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CacheFile {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Collection {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CustomData_MeshMasks {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Depsgraph {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DepsNodeHandle {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Main {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Object {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Scene {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bNodeTree {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VFont {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDepsSceneComponentType {
    DEG_SCENE_COMP_PARAMETERS,
    DEG_SCENE_COMP_ANIMATION,
    DEG_SCENE_COMP_SEQUENCER,
    DEG_SCENE_COMP_COMPOSITOR,
}

impl Default for eDepsSceneComponentType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const DEG_SCENE_COMP_PARAMETERS: i32 = eDepsSceneComponentType::DEG_SCENE_COMP_PARAMETERS as i32;
pub const DEG_SCENE_COMP_ANIMATION: i32 = eDepsSceneComponentType::DEG_SCENE_COMP_ANIMATION as i32;
pub const DEG_SCENE_COMP_SEQUENCER: i32 = eDepsSceneComponentType::DEG_SCENE_COMP_SEQUENCER as i32;
pub const DEG_SCENE_COMP_COMPOSITOR: i32 = eDepsSceneComponentType::DEG_SCENE_COMP_COMPOSITOR as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDepsObjectComponentType {
    DEG_OB_COMP_ANY,
    DEG_OB_COMP_PARAMETERS,
    DEG_OB_COMP_ANIMATION,
    DEG_OB_COMP_TRANSFORM,
    DEG_OB_COMP_GEOMETRY,
    DEG_OB_COMP_EVAL_POSE,
    DEG_OB_COMP_BONE,
    DEG_OB_COMP_SHADING,
    DEG_OB_COMP_CACHE,
}

impl Default for eDepsObjectComponentType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const DEG_OB_COMP_ANY: i32 = eDepsObjectComponentType::DEG_OB_COMP_ANY as i32;
pub const DEG_OB_COMP_PARAMETERS: i32 = eDepsObjectComponentType::DEG_OB_COMP_PARAMETERS as i32;
pub const DEG_OB_COMP_ANIMATION: i32 = eDepsObjectComponentType::DEG_OB_COMP_ANIMATION as i32;
pub const DEG_OB_COMP_TRANSFORM: i32 = eDepsObjectComponentType::DEG_OB_COMP_TRANSFORM as i32;
pub const DEG_OB_COMP_GEOMETRY: i32 = eDepsObjectComponentType::DEG_OB_COMP_GEOMETRY as i32;
pub const DEG_OB_COMP_EVAL_POSE: i32 = eDepsObjectComponentType::DEG_OB_COMP_EVAL_POSE as i32;
pub const DEG_OB_COMP_BONE: i32 = eDepsObjectComponentType::DEG_OB_COMP_BONE as i32;
pub const DEG_OB_COMP_SHADING: i32 = eDepsObjectComponentType::DEG_OB_COMP_SHADING as i32;
pub const DEG_OB_COMP_CACHE: i32 = eDepsObjectComponentType::DEG_OB_COMP_CACHE as i32;
