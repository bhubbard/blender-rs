//! Auto-transpiled C/C++ header module: GEO_join_geometries
//! Source: upstream/source/blender/geometry/GEO_join_geometries.hh

use blender_dna::BKE_attribute_filter::AttributeFilter;
use crate::ABC_alembic::GeometrySet;

extern "C" {
    pub fn join_geometries(
        geometries: *const GeometrySet,
        geometries_len: usize,
        attribute_filter: *const AttributeFilter,
        component_types_to_join: *const i32,
        component_types_to_join_len: usize,
        allow_merging_instance_references: bool,
    ) -> GeometrySet;
}
