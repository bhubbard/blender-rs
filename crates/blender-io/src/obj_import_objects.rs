use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GlobalVertices {
    pub vertices: Vec<f32>,
    pub uv_vertices: Vec<f32>,
    pub vert_normals: Vec<f32>,
    pub vertex_colors: Vec<f32>,
    pub vertex_weights: Vec<f32>,
    pub mrgb_block: Vec<f32>,
    pub start_of_block: i64,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FaceCorner {
    pub vert_index: i32,
    pub uv_vert_index: i32,
    pub vertex_normal_index: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FaceElem {
    pub vertex_group_index: i32,
    pub material_index: i32,
    pub shaded_smooth: bool,
    pub start_index_: i64,
    pub corner_count_: i64,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NurbsElement {
    pub degree: i32,
    pub range: Vec<f32>,
    pub curv_indices: Vec<i32>,
    pub parm: Vec<f32>,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGeometryType {
    MESH,
    CURVE,
}

impl Default for eGeometryType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GEOM_MESH: i32 = 0x0001;
pub const GEOM_CURVE: i32 = 0x0002;
