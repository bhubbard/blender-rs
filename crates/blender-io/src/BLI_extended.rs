//! Blender Library (blenlib) utilities, math, and data structures ported to Rust
//! Corresponds to:
//! BLI_alloca.hh, BLI_any.hh, BLI_asan.hh, BLI_bit_bool_conversion.hh, BLI_bit_span_ops.hh,
//! BLI_build_config.hh, BLI_colorspace.hh, BLI_compiler_typecheck.hh, BLI_fftw.hh,
//! BLI_fileops.hh, BLI_fixed_width_int_str.hh, BLI_generic_array.hh, BLI_generic_virtual_array.hh,
//! BLI_implicit_sharing.hh, BLI_link_utils.hh, BLI_linklist_stack.hh, BLI_math_axis_angle.hh,
//! BLI_math_base.hh, BLI_math_base_c.hh, BLI_math_base_safe.hh, BLI_math_boolean.hh,
//! BLI_math_color.hh, BLI_math_euler.hh, BLI_math_geom.hh, BLI_math_half.hh,
//! BLI_math_matrix.hh, BLI_math_quaternion.hh, BLI_math_rotation.hh, BLI_math_rotation_legacy.hh,
//! BLI_math_solvers.hh, BLI_math_vector_mpq_types.hh, BLI_math_vector_unroll.hh,
//! BLI_memory_cache_file_load.hh, BLI_mutex.hh, BLI_session_uid.hh, BLI_system.hh,
//! BLI_tempfile.hh, BLI_timecode.hh, BLI_unroll.hh, BLI_utildefines.hh,
//! BLI_utildefines_iter.hh, BLI_utility_mixins.hh, BLI_uuid.hh, BLI_vector.hh,
//! BLI_vector_set_slots.hh, BLI_virtual_array.hh, radial_tiling_shared.hh

use blender_dna::*;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct SessionUid(pub u64);

static SESSION_COUNTER: AtomicU64 = AtomicU64::new(1);

impl SessionUid {
    pub fn generate() -> Self {
        Self(SESSION_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TempFile {
    pub filepath: String,
    pub auto_remove: bool,
}

impl TempFile {
    pub fn new(filepath: impl Into<String>) -> Self {
        Self {
            filepath: filepath.into(),
            auto_remove: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeCode {
    pub hours: i32,
    pub minutes: i32,
    pub seconds: i32,
    pub frames: i32,
}

impl Default for TimeCode {
    fn default() -> Self {
        Self {
            hours: 0,
            minutes: 0,
            seconds: 0,
            frames: 0,
        }
    }
}

impl TimeCode {
    pub fn from_frames(frames: i64, fps: f64) -> Self {
        let total_secs = if fps > 0.0 { (frames as f64 / fps) as i64 } else { 0 };
        let rem_frames = if fps > 0.0 { (frames as f64 % fps) as i32 } else { 0 };
        let hours = (total_secs / 3600) as i32;
        let minutes = ((total_secs % 3600) / 60) as i32;
        let seconds = (total_secs % 60) as i32;
        Self {
            hours,
            minutes,
            seconds,
            frames: rem_frames,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RadialTilingShared {
    pub count: u32,
    pub radius: f32,
    pub offset_angle: f32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImplicitSharingData {
    pub user_count: usize,
}
