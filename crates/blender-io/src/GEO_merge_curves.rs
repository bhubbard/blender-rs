//! Auto-transpiled C/C++ header module: GEO_merge_curves
//! Source: upstream/source/blender/geometry/GEO_merge_curves.hh

use blender_dna::DNA_curves_types::CurvesGeometry;
use blender_dna::BKE_attribute_filter::AttributeFilter;

extern "C" {
    pub fn curves_merge_endpoints(
        src_curves: *const CurvesGeometry,
        connect_to_curve: *const i32,
        connect_to_curve_len: usize,
        flip_direction: *const bool,
        flip_direction_len: usize,
        attribute_filter: *const AttributeFilter,
    ) -> CurvesGeometry;
}
