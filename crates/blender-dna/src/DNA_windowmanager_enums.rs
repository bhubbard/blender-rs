//! Auto-transpiled C/C++ header module: DNA_windowmanager_enums

use crate::*;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum wmOperatorStatus {
    OPERATOR_RUNNING_MODAL = (1 << 0),
    OPERATOR_CANCELLED = (1 << 1),
    OPERATOR_FINISHED = (1 << 2),
    OPERATOR_PASS_THROUGH = (1 << 3),
    OPERATOR_HANDLED = (1 << 4),
    OPERATOR_INTERFACE = (1 << 5),
}

impl Default for wmOperatorStatus {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const OPERATOR_RUNNING_MODAL: i32 = wmOperatorStatus::OPERATOR_RUNNING_MODAL as i32;
pub const OPERATOR_CANCELLED: i32 = wmOperatorStatus::OPERATOR_CANCELLED as i32;
pub const OPERATOR_FINISHED: i32 = wmOperatorStatus::OPERATOR_FINISHED as i32;
pub const OPERATOR_PASS_THROUGH: i32 = wmOperatorStatus::OPERATOR_PASS_THROUGH as i32;
pub const OPERATOR_HANDLED: i32 = wmOperatorStatus::OPERATOR_HANDLED as i32;
pub const OPERATOR_INTERFACE: i32 = wmOperatorStatus::OPERATOR_INTERFACE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eOperator_Flag {
    OP_IS_INVOKE = (1 << 0),
    OP_IS_REPEAT = (1 << 1),
    OP_IS_REPEAT_LAST = (1 << 2),
    OP_IS_MODAL_GRAB_CURSOR = (1 << 3),
    OP_IS_MODAL_CURSOR_REGION = (1 << 4),
}

impl Default for eOperator_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const OP_IS_INVOKE: i32 = eOperator_Flag::OP_IS_INVOKE as i32;
pub const OP_IS_REPEAT: i32 = eOperator_Flag::OP_IS_REPEAT as i32;
pub const OP_IS_REPEAT_LAST: i32 = eOperator_Flag::OP_IS_REPEAT_LAST as i32;
pub const OP_IS_MODAL_GRAB_CURSOR: i32 = eOperator_Flag::OP_IS_MODAL_GRAB_CURSOR as i32;
pub const OP_IS_MODAL_CURSOR_REGION: i32 = eOperator_Flag::OP_IS_MODAL_CURSOR_REGION as i32;

