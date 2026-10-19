//! Auto-transpiled C/C++ header module: GEO_mesh_bevel
//! Source: upstream/source/blender/geometry/GEO_mesh_bevel.hh

use blender_dna::DNA_mesh_types::Mesh;
use blender_dna::BKE_attribute_filter::AttributeFilter;
use blender_math::cpp_types::IndexMask;
use blender_math::Vec2;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BevelAffect {
    Vertices = 0,
    Edges = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BevelMiterType {
    Sharp = 0,
    Patch = 1,
    Arc = 2,
}

#[derive(Debug, Clone, Default)]
pub struct BevelAttributeOutputs {
    pub vertex_face_id: Option<String>,
    pub edge_face_id: Option<String>,
    pub outer_edge_id: Option<String>,
    pub mid_edge_id: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct BevelParameters {
    pub segments: i32,
    pub shape: f32,
    pub affect_type: Option<BevelAffect>,
    pub custom_profile_samples: Vec<Vec2>,
    pub offsets: [Vec<f32>; 4],
    pub miter: Vec<bool>,
    pub spread: Vec<f32>,
    pub attribute_outputs: BevelAttributeOutputs,
}

extern "C" {
    pub fn mesh_bevel(
        src_mesh: *const Mesh,
        selection: *const IndexMask,
        params: *const BevelParameters,
        attribute_filter: *const AttributeFilter,
    ) -> *mut Mesh;
}
