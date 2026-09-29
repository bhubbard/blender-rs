//! Auto-transpiled C/C++ header module: BKE_mesh

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MLoopNorSpace {
    pub vec_lnor: [f32; 3],
    pub vec_ref: [f32; 3],
    pub vec_ortho: [f32; 3],
    pub ref_alpha: f32,
    pub ref_beta: f32,
    pub flags: i8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MLoopNorSpaceArray {
    pub data_type: i8,
    pub spaces_num: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMeshBatchDirtyMode {
    BKE_MESH_BATCH_DIRTY_ALL = 0,
    BKE_MESH_BATCH_DIRTY_SELECT,
    BKE_MESH_BATCH_DIRTY_SELECT_PAINT,
    BKE_MESH_BATCH_DIRTY_SHADING,
    BKE_MESH_BATCH_DIRTY_UVEDIT_ALL,
    BKE_MESH_BATCH_DIRTY_UVEDIT_SELECT,
}
