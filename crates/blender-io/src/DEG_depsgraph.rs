//! Auto-transpiled C/C++ header module: DEG_depsgraph

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DEGEditorUpdateContext {
    pub bmain: *mut Main,
    pub depsgraph: *mut Depsgraph,
    pub scene: *mut Scene,
    pub view_layer: *mut ViewLayer,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Depsgraph {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Main {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Scene {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ViewLayer {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eEvaluationMode {
    DAG_EVAL_VIEWPORT = 0,
    DAG_EVAL_RENDER = 1,
}

impl Default for eEvaluationMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const DAG_EVAL_VIEWPORT: i32 = eEvaluationMode::DAG_EVAL_VIEWPORT as i32;
pub const DAG_EVAL_RENDER: i32 = eEvaluationMode::DAG_EVAL_RENDER as i32;

pub const DAG_EVAL_NEED_CURVE_PATH: i32 = (1 << 0);
pub const DAG_EVAL_NEED_SHRINKWRAP_BOUNDARY: i32 = (1 << 1);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepsgraphEvaluateSyncWriteback {
    DEG_EVALUATE_SYNC_WRITEBACK_NO,
    DEG_EVALUATE_SYNC_WRITEBACK_YES,
}

impl Default for DepsgraphEvaluateSyncWriteback {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const DEG_EVALUATE_SYNC_WRITEBACK_NO: i32 = DepsgraphEvaluateSyncWriteback::DEG_EVALUATE_SYNC_WRITEBACK_NO as i32;
pub const DEG_EVALUATE_SYNC_WRITEBACK_YES: i32 = DepsgraphEvaluateSyncWriteback::DEG_EVALUATE_SYNC_WRITEBACK_YES as i32;
