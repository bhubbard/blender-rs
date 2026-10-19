//! Auto-transpiled C/C++ header module: GEO_curves_remove_and_split

use blender_dna::DNA_curves_types::CurvesGeometry;
use blender_math::cpp_types::IndexMask;

extern "C" {
    pub fn remove_points_and_split(
        curves: *const CurvesGeometry,
        mask: *const IndexMask,
    ) -> CurvesGeometry;

    pub fn grease_pencil_remove_points_and_split(
        curves: *const CurvesGeometry,
        mask: *const IndexMask,
    ) -> CurvesGeometry;
}
