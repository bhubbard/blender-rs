//! Blender Kernel bake subsystem headers ported to Rust
//! Corresponds to BKE_bake_geometry_nodes_modifier.hh, BKE_bake_items_serialize.hh, BKE_bake_values.hh

use blender_dna::*;
use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BakeModifierData {
    pub custom_name: String,
    pub frame_start: i32,
    pub frame_end: i32,
    pub is_baked: bool,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BakeItemSerializer {
    pub version: u32,
    pub data_size: usize,
}

impl BakeItemSerializer {
    pub fn new(version: u32) -> Self {
        Self {
            version,
            data_size: 0,
        }
    }

    pub fn serialize(&self, buffer: &mut Vec<u8>) -> usize {
        buffer.extend_from_slice(&self.version.to_le_bytes());
        buffer.len()
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BakeValues {
    pub float_values: Vec<f32>,
    pub int_values: Vec<i32>,
    pub string_values: Vec<String>,
}

impl BakeValues {
    pub fn clear(&mut self) {
        self.float_values.clear();
        self.int_values.clear();
        self.string_values.clear();
    }
}
