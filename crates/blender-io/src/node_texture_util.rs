//! Safe Rust port of Blender's `source/blender/nodes/texture/node_texture_util.hh`.

use crate::*;

pub const MAX_SOCKET: usize = 512;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TexResult {
    pub tin: [f32; 4],
    pub tr: f32,
    pub tg: f32,
    pub tb: f32,
    pub ta: f32,
    pub talpha: i32,
}

pub type MTex = blender_dna::MTex;
pub type bNode = blender_dna::bNode;
pub type bNodeStack = blender_dna::bNodeStack;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TexCallData {
    pub target: *mut TexResult,
    pub co: *const f32,
    pub do_preview: bool,
    pub do_manage: bool,
    pub thread: i16,
    pub which_output: i16,
    pub cfra: i32,
    pub mtex: *const MTex,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TexParams {
    pub co: *const f32,
    pub previewco: *const f32,
    pub cfra: i32,
    pub mtex: *const MTex,
}

pub type TexFn = Option<unsafe extern "C" fn(out: *mut f32, params: *mut TexParams, node: *mut bNode, in_stacks: *mut *mut bNodeStack, thread: i16)>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TexDelegate {
    pub cdata: *mut TexCallData,
    pub fn_ptr: TexFn,
    pub node: *mut bNode,
    pub in_stacks: [*mut bNodeStack; MAX_SOCKET],
    pub node_type: i32,
}

impl Default for TexDelegate {
    fn default() -> Self {
        Self {
            cdata: core::ptr::null_mut(),
            fn_ptr: None,
            node: core::ptr::null_mut(),
            in_stacks: [core::ptr::null_mut(); MAX_SOCKET],
            node_type: 0,
        }
    }
}
