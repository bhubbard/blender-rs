//! Blender IO subsystem headers ported to Rust
//! Corresponds to:
//! abc_customdata.h, abc_reader_object.h, IO_abstract_hierarchy_iterator.h, IO_orientation.hh,
//! IO_path_util.hh, IO_string_utils.hh, IO_types.hh, IO_validate.hh, dupli_parent_finder.hh,
//! IO_csv.hh, IO_fbx.hh, fbx_import.hh, fbx_import_anim.hh, fbx_import_armature.hh,
//! fbx_import_material.hh, fbx_import_mesh.hh, fbx_import_util.hh, grease_pencil_io.hh,
//! grease_pencil_io_intern.hh, IO_ply.hh, ply_data.hh, IO_spz.hh, spz_import.hh,
//! spz_header.hh, spz_read.hh, spz_read_common.hh, IO_stl.hh, camera_delegate.hh,
//! curves.hh, image.hh, instancer.hh, light.hh, IO_wavefront_obj.hh, obj_importer.hh

use blender_dna::*;
use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOAxis {
    X = 0,
    Y = 1,
    Z = 2,
    NegativeX = 3,
    NegativeY = 4,
    NegativeZ = 5,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IOOrientationSettings {
    pub forward_axis: i32,
    pub up_axis: i32,
    pub global_scale: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FbxImportParams {
    pub use_custom_normals: bool,
    pub use_image_search: bool,
    pub use_alpha_decals: bool,
    pub use_anim: bool,
    pub anim_offset: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpzHeader {
    pub magic: [u8; 4],
    pub version: u32,
    pub num_points: u32,
    pub sh_degree: u8,
    pub fractional_bits: u8,
    pub flags: u8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PlyData {
    pub vertex_count: usize,
    pub face_count: usize,
    pub has_normals: bool,
    pub has_colors: bool,
    pub has_uvs: bool,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct HydraDelegateContext {
    pub is_active: bool,
    pub sample_count: i32,
}
