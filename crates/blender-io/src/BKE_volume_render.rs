use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VolumeGridType {
    pub resolution: [i32; 3],
    pub voxel_size: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DenseFloatVolumeGrid {
    pub r#type: VolumeGridType,
    pub resolution: [i32; 3],
    pub texture_to_object: [[f32; 4]; 4],
    pub channels: i32,
    pub voxels: *mut f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Volume {
    pub _opaque: [u8; 0],
}
