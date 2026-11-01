//! Blender Kernel general types ported to Rust
//! Corresponds to BKE_cpp_types.hh, BKE_main_invariants.hh, BKE_node_socket_value.hh,
//! BKE_object_types.hh, BKE_outliner_treehash.hh, BKE_path_templates.hh, BKE_report.hh,
//! BKE_vfontdata.hh, data_transfer_intern.hh, multires_reshape.hh, pbvh_uv_islands.hh

use blender_dna::*;
use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct KernelReport {
    pub message: String,
    pub report_type: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct KernelReportList {
    pub reports: Vec<KernelReport>,
    pub print_level: i32,
    pub store_level: i32,
}

impl KernelReportList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn report(&mut self, report_type: i32, msg: impl Into<String>) {
        self.reports.push(KernelReport {
            message: msg.into(),
            report_type,
        });
    }

    pub fn clear(&mut self) {
        self.reports.clear();
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct OutlinerTreeHash {
    pub hash_code: u64,
    pub num_elements: usize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VFontData {
    pub name: String,
    pub resolution: i32,
    pub glyphs_count: usize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MultiresReshapeContext {
    pub level: i32,
    pub total_levels: i32,
    pub is_modified: bool,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PbvhUvIsland {
    pub island_id: u32,
    pub num_faces: usize,
}
