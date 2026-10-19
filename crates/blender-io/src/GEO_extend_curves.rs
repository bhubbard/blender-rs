//! Auto-transpiled C/C++ header module: GEO_extend_curves
//! Source: upstream/source/blender/geometry/GEO_extend_curves.hh

use blender_dna::DNA_curves_types::CurvesGeometry;
use blender_dna::BKE_attribute_filter::AttributeFilter;
use blender_math::cpp_types::IndexMask;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometryNodeCurveSampleMode {
    Factor = 0,
    Length = 1,
}

extern "C" {
    pub fn extend_curves(
        src_curves: *mut CurvesGeometry,
        selection: *const IndexMask,
        start_lengths: *const f32,
        end_lengths: *const f32,
        overshoot_fac: f32,
        follow_curvature: bool,
        point_density: f32,
        segment_influence: f32,
        max_angle: f32,
        invert_curvature: bool,
        sample_mode: GeometryNodeCurveSampleMode,
        attribute_filter: *const AttributeFilter,
    ) -> CurvesGeometry;
}
