//! Auto-transpiled C/C++ header module: GEO_fit_curves
//! Source: upstream/source/blender/geometry/GEO_fit_curves.hh

use blender_dna::DNA_curves_types::CurvesGeometry;
use blender_dna::BKE_attribute_filter::AttributeFilter;
use blender_math::cpp_types::IndexMask;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FitMethod {
    Refit = 0,
    Split = 1,
}

extern "C" {
    pub fn fit_poly_to_bezier_curves(
        src_curves: *const CurvesGeometry,
        curve_selection: *const IndexMask,
        thresholds: *const f32,
        corners: *const bool,
        method: FitMethod,
        attribute_filter: *const AttributeFilter,
    ) -> CurvesGeometry;
}
