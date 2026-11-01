//! Blender Kernel mesh and evaluation headers ported to Rust
//! Corresponds to BKE_mesh.hh, BKE_mesh_sample.hh, BKE_mesh_wrapper.hh, BKE_volume_to_mesh.hh, BKE_pointcache.h, BKE_key.hh, BKE_deform.hh

use blender_dna::*;
use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MeshSampleResult {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub face_index: i32,
    pub weights: [f32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MeshWrapper {
    pub mesh: Option<Box<Mesh>>,
    pub is_editmode: bool,
    pub is_dirty: bool,
}

impl MeshWrapper {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn free(&mut self) {
        self.mesh = None;
        self.is_dirty = false;
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VolumeToMeshSettings {
    pub resolution: i32,
    pub threshold: f32,
    pub adaptivity: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PointCacheItem {
    pub frame: i32,
    pub flag: i32,
    pub data_size: usize,
}
