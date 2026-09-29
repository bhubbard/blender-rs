//! Auto-transpiled C/C++ header module: mtl_texture

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TextureUpdateParams {
    pub mip_index: i32,
    pub extent: [i32; 3],
    pub offset: [i32; 3],
    pub unpack_row_length: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TextureReadParams {
    pub mip_index: i32,
    pub extent: [i32; 3],
    pub offset: [i32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepthTextureUpdateMode {
    MTL_DEPTH_UPDATE_MODE_FLOAT = 0,
    MTL_DEPTH_UPDATE_MODE_INT24 = 1,
    MTL_DEPTH_UPDATE_MODE_INT32 = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum resource_mode_ {
    MTL_TEXTURE_MODE_DEFAULT,
    MTL_TEXTURE_MODE_EXTERNAL,
    MTL_TEXTURE_MODE_VBO,
    MTL_TEXTURE_MODE_TEXTURE_VIEW,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureViewDirtyState {
    TEXTURE_VIEW_NOT_DIRTY = 0,
    TEXTURE_VIEW_SWIZZLE_DIRTY = (1 << 0),
    TEXTURE_VIEW_MIP_DIRTY = (1 << 1),
}
