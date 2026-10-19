//! Auto-transpiled C/C++ header module: GEO_resample_curves
//! Source: upstream/source/blender/geometry/GEO_resample_curves.hh

use blender_dna::DNA_curves_types::CurvesGeometry;
use blender_math::cpp_types::IndexMask;

#[derive(Debug, Clone, Default)]
pub struct ResampleCurvesOutputAttributeIDs {
    pub tangent_id: Option<String>,
    pub normal_id: Option<String>,
}

extern "C" {
    pub fn resample_to_count(
        src_curves: *const CurvesGeometry,
        selection: *const IndexMask,
        counts: *const i32,
        counts_len: usize,
        output_ids: *const ResampleCurvesOutputAttributeIDs,
    ) -> CurvesGeometry;

    pub fn resample_to_length(
        src_curves: *const CurvesGeometry,
        selection: *const IndexMask,
        sample_lengths: *const f32,
        sample_lengths_len: usize,
        output_ids: *const ResampleCurvesOutputAttributeIDs,
        keep_last_segment: bool,
    ) -> CurvesGeometry;

    pub fn resample_to_evaluated(
        src_curves: *const CurvesGeometry,
        selection: *const IndexMask,
        output_ids: *const ResampleCurvesOutputAttributeIDs,
    ) -> CurvesGeometry;
}
