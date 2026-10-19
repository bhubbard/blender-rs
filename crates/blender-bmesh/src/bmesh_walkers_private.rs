//! Auto-transpiled C/C++ header module: bmesh_walkers_private

use crate::bmesh_class::*;
use blender_dna::Link;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMwGenericWalker {
    pub link: Link,
    pub depth: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct BMwShellWalker {
    pub header: BMwGenericWalker,
    pub curedge: *mut BMEdge,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct BMwLoopShellWalker {
    pub header: BMwGenericWalker,
    pub curloop: *mut BMLoop,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct BMwLoopShellWireWalker {
    pub header: BMwGenericWalker,
    pub curelem: *mut BMElem,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct BMwIslandboundWalker {
    pub header: BMwGenericWalker,
    pub base: *mut BMLoop,
    pub lastv: *mut BMVert,
    pub curloop: *mut BMLoop,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct BMwIslandWalker {
    pub header: BMwGenericWalker,
    pub cur: *mut BMFace,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct BMwEdgeLoopWalker {
    pub header: BMwGenericWalker,
    pub cur: *mut BMEdge,
    pub start: *mut BMEdge,
    pub lastv: *mut BMVert,
    pub startv: *mut BMVert,
    pub f_hub: *mut BMFace,
    pub is_boundary: bool,
    pub is_single: bool,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct BMwFaceLoopWalker {
    pub header: BMwGenericWalker,
    pub l: *mut BMLoop,
    pub no_calc: bool,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct BMwEdgeringWalker {
    pub header: BMwGenericWalker,
    pub l: *mut BMLoop,
    pub wireedge: *mut BMEdge,
    pub no_calc: bool,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct BMwEdgeboundaryWalker {
    pub header: BMwGenericWalker,
    pub e: *mut BMEdge,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct BMwNonManifoldEdgeLoopWalker {
    pub header: BMwGenericWalker,
    pub start: *mut BMEdge,
    pub cur: *mut BMEdge,
    pub startv: *mut BMVert,
    pub lastv: *mut BMVert,
    pub face_count: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct BMwUVEdgeWalker {
    pub header: BMwGenericWalker,
    pub l: *mut BMLoop,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct BMwConnectedVertexWalker {
    pub header: BMwGenericWalker,
    pub curvert: *mut BMVert,
}
