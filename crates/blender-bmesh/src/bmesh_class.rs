//! Auto-transpiled C/C++ header module: bmesh_class

use crate::*;
use core::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMHeader {
    pub data: *mut core::ffi::c_void,
    pub index: i32,
    pub htype: i8,
    pub hflag: i8,
    pub api_flag: i8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMVert {
    pub head: BMHeader,
    pub co: [f32; 3],
    pub no: [f32; 3],
    pub e: *mut BMEdge,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMVert_OFlag {
    pub base: BMVert,
    pub oflags: *mut BMFlagLayer,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMDiskLink {
    pub next: *mut BMEdge,
    pub prev: *mut BMEdge,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMEdge {
    pub head: BMHeader,
    pub v1: *mut BMVert,
    pub v2: *mut BMVert,
    pub l: *mut BMLoop,
    pub v1_disk_link: BMDiskLink,
    pub v2_disk_link: BMDiskLink,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMEdge_OFlag {
    pub base: BMEdge,
    pub oflags: *mut BMFlagLayer,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMElemF {
    pub head: BMHeader,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMElem {
    pub head: BMHeader,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMLoopList {
    pub next: *mut BMLoopList,
    pub prev: *mut BMLoopList,
    pub first: *mut BMLoop,
    pub last: *mut BMLoop,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMFace {
    pub head: BMHeader,
    pub totbounds: i32,
    pub loops: ListBaseT<BMLoop>,
    pub l_first: *mut BMLoop,
    pub len: i32,
    pub no: [f32; 3],
    pub mat_nr: i16,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMFace_OFlag {
    pub base: BMFace,
    pub oflags: *mut BMFlagLayer,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMFlagLayer {
    pub f: i16,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMesh {
    pub totvert: i32,
    pub totedge: i32,
    pub totloop: i32,
    pub totface: i32,
    pub totvertsel: i32,
    pub totedgesel: i32,
    pub totfacesel: i32,
    pub elem_index_dirty: i8,
    pub elem_table_dirty: i8,
    pub vpool: *mut BLI_mempool,
    pub epool: *mut BLI_mempool,
    pub lpool: *mut BLI_mempool,
    pub fpool: *mut BLI_mempool,
    pub vtable: *mut *mut BMVert,
    pub etable: *mut *mut BMEdge,
    pub ftable: *mut *mut BMFace,
    pub vtable_tot: i32,
    pub etable_tot: i32,
    pub ftable_tot: i32,
    pub vtoolflagpool: *mut BLI_mempool,
    pub etoolflagpool: *mut BLI_mempool,
    pub ftoolflagpool: *mut BLI_mempool,
    pub use_toolflags: bool,
    pub uv_select_sync_valid: bool,
    pub toolflag_index: i32,
    pub vdata: CustomData,
    pub edata: CustomData,
    pub ldata: CustomData,
    pub pdata: CustomData,
    pub looplistpool: *mut BLI_mempool,
    pub lnor_spacearr: *mut MLoopNorSpaceArray,
    pub spacearr_dirty: i8,
    pub selectmode: i16,
    pub shapenr: i32,
    pub totflags: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMLoopNorEditData {
    pub loop_index: i32,
    pub r#loop: *mut BMLoop,
    pub niloc: [f32; 3],
    pub nloc: [f32; 3],
    pub loc: *mut f32,
    pub clnors_data: *mut i16,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMLoopNorEditDataArray {
    pub lnor_editdata: *mut BMLoopNorEditData,
    pub lidx_to_lnor_editdata: *mut *mut BMLoopNorEditData,
    pub cd_custom_normal_offset: i32,
    pub totloop: i32,
}
