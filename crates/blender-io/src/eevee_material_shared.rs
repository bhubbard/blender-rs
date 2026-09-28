//! Auto-transpiled C/C++ header module: eevee_material_shared

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMaterialPipeline {
    MAT_PIPE_DEFERRED = 0,
    MAT_PIPE_FORWARD,
    MAT_PIPE_PREPASS_DEFERRED,
    MAT_PIPE_PREPASS_DEFERRED_VELOCITY,
    MAT_PIPE_PREPASS_FORWARD,
    MAT_PIPE_PREPASS_FORWARD_VELOCITY,
    MAT_PIPE_PREPASS_OVERLAP,
    MAT_PIPE_PREPASS_PLANAR,
    MAT_PIPE_VOLUME_OCCUPANCY,
    MAT_PIPE_VOLUME_MATERIAL,
    MAT_PIPE_SHADOW,
    MAT_PIPE_CAPTURE,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMaterialGeometry {
    MAT_GEOM_MESH = 0,
    MAT_GEOM_POINTCLOUD,
    MAT_GEOM_CURVES,
    MAT_GEOM_GSPLAT,
    MAT_GEOM_VOLUME,
    MAT_GEOM_WORLD,
}
