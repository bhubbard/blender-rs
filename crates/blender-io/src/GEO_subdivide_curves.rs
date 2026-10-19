//! Auto-transpiled C/C++ header module: GEO_subdivide_curves
//! Source: upstream/source/blender/geometry/GEO_subdivide_curves.hh

use blender_dna::DNA_curves_types::CurvesGeometry;
use blender_dna::BKE_attribute_filter::AttributeFilter;
use blender_math::cpp_types::IndexMask;

extern "C" {
    pub fn subdivide_curves(
        src_curves: *const CurvesGeometry,
        selection: *const IndexMask,
        cuts: *const i32,
        cuts_len: usize,
        attribute_filter: *const AttributeFilter,
    ) -> CurvesGeometry;
}
