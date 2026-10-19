//! Auto-transpiled C/C++ header module: GEO_mesh_to_curve
//! Source: upstream/source/blender/geometry/GEO_mesh_to_curve.hh

use blender_dna::DNA_mesh_types::Mesh;
use blender_dna::DNA_curves_types::CurvesGeometry;
use blender_dna::BKE_attribute_filter::AttributeFilter;
use blender_math::cpp_types::IndexMask;

extern "C" {
    pub fn mesh_edges_to_curves_convert(
        mesh: *const Mesh,
        selection: *const IndexMask,
        attribute_filter: *const AttributeFilter,
    ) -> CurvesGeometry;

    pub fn create_curve_from_vert_indices(
        mesh_attributes: *const core::ffi::c_void,
        vert_indices: *const i32,
        vert_indices_len: usize,
        curve_offsets: *const i32,
        curve_offsets_len: usize,
        cyclic_start: usize,
        cyclic_size: usize,
        attribute_filter: *const AttributeFilter,
    ) -> CurvesGeometry;

    pub fn mesh_faces_to_curves_convert(
        mesh: *const Mesh,
        selection: *const IndexMask,
        attribute_filter: *const AttributeFilter,
    ) -> CurvesGeometry;
}
