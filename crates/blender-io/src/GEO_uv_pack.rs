//! Auto-transpiled C/C++ header module: GEO_uv_pack
//! Source: upstream/source/blender/geometry/GEO_uv_pack.hh

use blender_math::Vec2;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UVPackIslandMarginMethod {
    Scaled = 0,
    Add = 1,
    Fraction = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UVPackIslandRotationMethod {
    None = 0,
    AxisAligned = 1,
    AxisAlignedX = 2,
    AxisAlignedY = 3,
    Cardinal = 4,
    Any = 5,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UVPackIslandShapeMethod {
    Aabb = 0,
    Convex = 1,
    Concave = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UVPackIslandPinMethod {
    None = 0,
    Ignore = 1,
    LockRotation = 2,
    LockRotationScale = 3,
    LockScale = 4,
    LockAll = 5,
}

#[derive(Debug, Clone)]
pub struct UVPackIslandParams {
    pub rotate_method: UVPackIslandRotationMethod,
    pub scale_to_fit: bool,
    pub only_selected_uvs: bool,
    pub only_selected_faces: bool,
    pub use_seams: bool,
    pub correct_aspect: bool,
    pub pin_method: UVPackIslandPinMethod,
    pub pin_unselected: bool,
    pub merge_overlap: bool,
    pub margin: f32,
    pub margin_method: UVPackIslandMarginMethod,
    pub udim_base_offset: [f32; 2],
    pub target_extent: f32,
    pub target_aspect_y: f32,
    pub shape_method: UVPackIslandShapeMethod,
}

impl Default for UVPackIslandParams {
    fn default() -> Self {
        Self {
            rotate_method: UVPackIslandRotationMethod::None,
            scale_to_fit: true,
            only_selected_uvs: false,
            only_selected_faces: false,
            use_seams: false,
            correct_aspect: false,
            pin_method: UVPackIslandPinMethod::None,
            pin_unselected: false,
            merge_overlap: false,
            margin: 0.001,
            margin_method: UVPackIslandMarginMethod::Scaled,
            udim_base_offset: [0.0, 0.0],
            target_extent: 1.0,
            target_aspect_y: 1.0,
            shape_method: UVPackIslandShapeMethod::Aabb,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PackIsland {
    pub aspect_y: f32,
    pub pinned: bool,
    pub pre_translate: Vec2,
    pub angle: f32,
    pub caller_index: i32,
    pub pivot: Vec2,
    pub half_diagonal: Vec2,
    pub pre_rotate: f32,
    pub triangle_vertices: Vec<Vec2>,
}

extern "C" {
    pub fn pack_islands(
        islands: *const *mut PackIsland,
        islands_len: usize,
        params: *const UVPackIslandParams,
    ) -> f32;
}
