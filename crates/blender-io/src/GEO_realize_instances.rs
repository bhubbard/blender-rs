//! Auto-transpiled C/C++ header module: GEO_realize_instances
//! Source: upstream/source/blender/geometry/GEO_realize_instances.hh

use blender_dna::BKE_attribute_filter::AttributeFilter;
use blender_math::cpp_types::IndexMask;
use crate::ABC_alembic::GeometrySet;

#[derive(Debug, Clone)]
pub struct RealizeInstancesOptions {
    pub keep_original_ids: bool,
    pub realize_instance_attributes: bool,
    pub realize_to_point_domain: bool,
    pub preserve_normals: bool,
    pub attribute_filter: AttributeFilter,
}

impl Default for RealizeInstancesOptions {
    fn default() -> Self {
        Self {
            keep_original_ids: false,
            realize_instance_attributes: true,
            realize_to_point_domain: true,
            preserve_normals: false,
            attribute_filter: AttributeFilter::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct VariedDepthOptions {
    pub selection: IndexMask,
    pub depths: Vec<i32>,
}

impl VariedDepthOptions {
    pub const MAX_DEPTH: i32 = -1;
}

#[derive(Debug, Clone, Default)]
pub struct RealizeInstancesResult {
    pub geometry: GeometrySet,
    pub errors: Vec<String>,
}

extern "C" {
    pub fn realize_instances(
        geometry_set: GeometrySet,
        options: *const RealizeInstancesOptions,
    ) -> RealizeInstancesResult;

    pub fn realize_instances_varied_depth(
        geometry_set: GeometrySet,
        options: *const RealizeInstancesOptions,
        varied_depth_option: *const VariedDepthOptions,
    ) -> RealizeInstancesResult;
}
