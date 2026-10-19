//! Auto-transpiled C/C++ header module: GEO_reorder
//! Source: upstream/source/blender/geometry/GEO_reorder.hh

use blender_dna::DNA_mesh_types::Mesh;
use blender_dna::DNA_pointcloud_types::PointCloud;
use blender_dna::DNA_curves_types::CurvesGeometry;
use blender_dna::BKE_attribute_filter::AttributeFilter;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttrDomain {
    Auto = -1,
    Point = 0,
    Edge = 1,
    Face = 2,
    Corner = 3,
    Curve = 4,
    Instance = 5,
    Layer = 6,
}

extern "C" {
    pub fn reorder_mesh(
        src_mesh: *const Mesh,
        old_by_new_map: *const i32,
        old_by_new_map_len: usize,
        domain: AttrDomain,
        attribute_filter: *const AttributeFilter,
    ) -> *mut Mesh;

    pub fn reorder_points(
        src_pointcloud: *const PointCloud,
        old_by_new_map: *const i32,
        old_by_new_map_len: usize,
        attribute_filter: *const AttributeFilter,
    ) -> *mut PointCloud;

    pub fn reorder_curves_geometry(
        src_curves: *const CurvesGeometry,
        old_by_new_map: *const i32,
        old_by_new_map_len: usize,
        attribute_filter: *const AttributeFilter,
    ) -> CurvesGeometry;
}
