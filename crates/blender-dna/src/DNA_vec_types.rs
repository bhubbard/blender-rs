//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct vec2s {
    pub x: i16,
    pub y: i16,
}

impl Default for vec2s {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct vec2f {
    pub x: f32,
    pub y: f32,
}

impl Default for vec2f {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct vec2i {
    pub x: i32,
    pub y: i32,
}

impl Default for vec2i {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct vec3i {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Default for vec3i {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for vec3f {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Default for vec4f {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct mat4x4f {
    pub value: [[f32; 4]; 4],
}

impl Default for mat4x4f {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct rctf {
    pub xmin: f32,
    pub xmax: f32,
    pub ymin: f32,
    pub ymax: f32,
}

impl Default for rctf {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct DualQuat {
    pub quat: [f32; 4],
}

impl Default for DualQuat {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

