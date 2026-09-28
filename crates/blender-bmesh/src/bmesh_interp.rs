//! Auto-transpiled C/C++ header module: bmesh_interp

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BMLoopInterpMultiresData {
    pub cd_loop_mdisp_offset: i32,
    pub res: i32,
    pub d: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LoopWalkCtx {
    pub r#type: i32,
    pub cd_layer_offset: i32,
    pub data_len: i32,
    pub weight_accum: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LoopGroupCD {
    pub data_len: i32,
}
