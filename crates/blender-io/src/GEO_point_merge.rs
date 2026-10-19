//! Auto-transpiled C/C++ header module: GEO_point_merge
//! Source: upstream/source/blender/geometry/GEO_point_merge.hh

use blender_dna::DNA_pointcloud_types::PointCloud;
use blender_dna::BKE_attribute_filter::AttributeFilter;
use blender_math::cpp_types::IndexMask;

extern "C" {
    pub fn point_merge_by_distance(
        src_points: *const PointCloud,
        merge_distance: f32,
        selection: *const IndexMask,
        attribute_filter: *const AttributeFilter,
    ) -> *mut PointCloud;

    pub fn merge_points(
        src_points: *const PointCloud,
        selection: *const IndexMask,
        merge_ids: *const i32,
        merge_ids_len: usize,
        attribute_filter: *const AttributeFilter,
    ) -> *mut PointCloud;
}
