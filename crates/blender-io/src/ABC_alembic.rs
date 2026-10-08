//! Auto-transpiled C/C++ header module: ABC_alembic

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct AlembicExportParams {
    pub frame_start: f64,
    pub frame_end: f64,
    pub frame_samples_xform: u32,
    pub frame_samples_shape: u32,
    pub shutter_open: f64,
    pub shutter_close: f64,
    pub selected_only: bool,
    pub uvs: bool,
    pub normals: bool,
    pub vcolors: bool,
    pub orcos: bool,
    pub apply_subdiv: bool,
    pub curves_as_mesh: bool,
    pub flatten_hierarchy: bool,
    pub face_sets: bool,
    pub use_subdiv_schema: bool,
    pub packuv: bool,
    pub triangulate: bool,
    pub export_hair: bool,
    pub export_particles: bool,
    pub export_custom_properties: bool,
    pub use_instancing: bool,
    pub evaluation_mode: eEvaluationMode,
    pub quad_method: i32,
    pub ngon_method: i32,
    pub global_scale: f32,
    pub collection: [i8; MAX_ID_NAME - 2],
}

impl Default for AlembicExportParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AlembicImportParams {
    pub global_scale: f32,
    pub sequence_max_frame: i32,
    pub sequence_min_frame: i32,
    pub is_sequence: bool,
    pub set_frame_range: bool,
    pub validate_meshes: bool,
    pub always_add_cache_reader: bool,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ABCReadParams {
    pub time: f64,
    pub read_flags: i32,
    pub velocity_name: *mut i8,
    pub velocity_scale: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CacheArchiveHandle {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CacheFileLayer {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CacheObjectPath {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CacheReader {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Main {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Mesh {
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
pub struct bContext {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GeometrySet {
    pub _opaque: [u8; 0],
}
