//! Auto-transpiled C/C++ header module: GEO_fillet_curves
//! Source: upstream/source/blender/geometry/GEO_fillet_curves.hh

use blender_dna::DNA_curves_types::CurvesGeometry;
use blender_dna::BKE_attribute_filter::AttributeFilter;
use blender_math::cpp_types::IndexMask;

extern "C" {
    pub fn fillet_curves_poly(
        src_curves: *const CurvesGeometry,
        curve_selection: *const IndexMask,
        radius: *const f32,
        counts: *const i32,
        limit_radius: bool,
        attribute_filter: *const AttributeFilter,
    ) -> CurvesGeometry;

    pub fn fillet_curves_bezier(
        src_curves: *const CurvesGeometry,
        curve_selection: *const IndexMask,
        radius: *const f32,
        limit_radius: bool,
        attribute_filter: *const AttributeFilter,
    ) -> CurvesGeometry;
}
