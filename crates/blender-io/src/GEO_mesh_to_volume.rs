//! Auto-transpiled C/C++ header module: GEO_mesh_to_volume
//! Source: upstream/source/blender/geometry/GEO_mesh_to_volume.hh

use blender_dna::DNA_modifier_types::MeshToVolumeModifierResolutionMode;
use blender_math::{Mat4, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct MeshToVolumeResolution {
    pub mode: MeshToVolumeModifierResolutionMode,
    pub voxel_size_or_amount: f32,
}

extern "C" {
    pub fn volume_compute_voxel_size(
        depsgraph: *const core::ffi::c_void,
        bounds_fn: extern "C" fn(*mut core::ffi::c_void, *mut [Vec3; 2]),
        bounds_data: *mut core::ffi::c_void,
        resolution: MeshToVolumeResolution,
        exterior_band_width: f32,
        transform: *const Mat4,
    ) -> f32;
}
