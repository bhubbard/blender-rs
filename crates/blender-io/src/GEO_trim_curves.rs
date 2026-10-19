//! Auto-transpiled C/C++ header module: GEO_trim_curves

use core::ffi::c_void;
use blender_dna::DNA_curves_types::CurvesGeometry;
use blender_dna::DNA_node_types::GeometryNodeCurveSampleMode;
use blender_math::cpp_types::IndexMask;

extern "C" {
    pub fn trim_curves(
        src_curves: *const CurvesGeometry,
        selection: *const IndexMask,
        starts: *const c_void,
        ends: *const c_void,
        mode: GeometryNodeCurveSampleMode,
        attribute_filter: *const c_void,
    ) -> CurvesGeometry;
}
