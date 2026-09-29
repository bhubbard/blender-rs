//! Auto-transpiled C/C++ header module: bmesh_iterators

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMIter__elem_of_mesh {
    pub pooliter: BLI_mempool_iter,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMIter__edge_of_vert {
    pub vdata: *mut BMVert,
    pub e_first: *mut BMEdge,
    pub e_next: *mut BMEdge,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMIter__face_of_vert {
    pub vdata: *mut BMVert,
    pub l_first: *mut BMLoop,
    pub l_next: *mut BMLoop,
    pub e_first: *mut BMEdge,
    pub e_next: *mut BMEdge,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMIter__loop_of_vert {
    pub vdata: *mut BMVert,
    pub l_first: *mut BMLoop,
    pub l_next: *mut BMLoop,
    pub e_first: *mut BMEdge,
    pub e_next: *mut BMEdge,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMIter__loop_of_edge {
    pub edata: *mut BMEdge,
    pub l_first: *mut BMLoop,
    pub l_next: *mut BMLoop,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMIter__loop_of_loop {
    pub ldata: *mut BMLoop,
    pub l_first: *mut BMLoop,
    pub l_next: *mut BMLoop,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMIter__face_of_edge {
    pub edata: *mut BMEdge,
    pub l_first: *mut BMLoop,
    pub l_next: *mut BMLoop,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMIter__vert_of_edge {
    pub edata: *mut BMEdge,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMIter__vert_of_face {
    pub pdata: *mut BMFace,
    pub l_first: *mut BMLoop,
    pub l_next: *mut BMLoop,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMIter__edge_of_face {
    pub pdata: *mut BMFace,
    pub l_first: *mut BMLoop,
    pub l_next: *mut BMLoop,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMIter__loop_of_face {
    pub pdata: *mut BMFace,
    pub l_first: *mut BMLoop,
    pub l_next: *mut BMLoop,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct data {
    pub elem_of_mesh: BMIter__elem_of_mesh,
    pub edge_of_vert: BMIter__edge_of_vert,
    pub face_of_vert: BMIter__face_of_vert,
    pub loop_of_vert: BMIter__loop_of_vert,
    pub loop_of_edge: BMIter__loop_of_edge,
    pub loop_of_loop: BMIter__loop_of_loop,
    pub face_of_edge: BMIter__face_of_edge,
    pub vert_of_edge: BMIter__vert_of_edge,
    pub vert_of_face: BMIter__vert_of_face,
    pub edge_of_face: BMIter__edge_of_face,
    pub loop_of_face: BMIter__loop_of_face,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BMIterType {
    BM_VERTS_OF_MESH = 1,
    BM_EDGES_OF_MESH = 2,
    BM_FACES_OF_MESH = 3,
    BM_EDGES_OF_VERT = 4,
    BM_FACES_OF_VERT = 5,
    BM_LOOPS_OF_VERT = 6,
    BM_VERTS_OF_EDGE = 7,
    BM_FACES_OF_EDGE = 8,
    BM_VERTS_OF_FACE = 9,
    BM_EDGES_OF_FACE = 10,
    BM_LOOPS_OF_FACE = 11,
    BM_LOOPS_OF_LOOP = 12,
    BM_LOOPS_OF_EDGE = 13,
}
