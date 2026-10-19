//! Auto-transpiled C/C++ header module: GEO_curve_constraints
//! Source: upstream/source/blender/geometry/GEO_curve_constraints.hh

use blender_dna::DNA_mesh_types::Mesh;
use blender_math::cpp_types::IndexMask;
use blender_math::Vec3;

extern "C" {
    pub fn compute_segment_lengths(
        points_by_curve: *const usize,
        points_by_curve_len: usize,
        positions: *const Vec3,
        positions_len: usize,
        curve_selection: *const IndexMask,
        r_segment_lengths: *mut f32,
    );

    pub fn solve_length_constraints(
        points_by_curve: *const usize,
        points_by_curve_len: usize,
        curve_selection: *const IndexMask,
        segment_lengths: *const f32,
        segment_lengths_len: usize,
        positions: *mut Vec3,
        positions_len: usize,
    );

    pub fn solve_length_and_collision_constraints(
        points_by_curve: *const usize,
        points_by_curve_len: usize,
        curve_selection: *const IndexMask,
        segment_lengths: *const f32,
        segment_lengths_len: usize,
        start_positions: *const Vec3,
        start_positions_len: usize,
        surface: *const Mesh,
        transforms: *const core::ffi::c_void,
        positions: *mut Vec3,
        positions_len: usize,
        surface_collision_distance: f32,
    );
}
