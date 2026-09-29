//! Auto-transpiled C/C++ header module: ANIM_action

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Keyframe = 0,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flags {
    Enabled = (1 << 0),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MixMode {
    Replace = 0,
    Offset = 1,
    Add = 2,
    Subtract = 3,
    Multiply = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flags_2 {
    Expanded = (1 << 0),
    Selected = (1 << 1),
    Active = (1 << 2),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionSlotAssignmentResult {
    OK = 0,
    SlotNotFromAction = 1,
    SlotNotSuitable = 2,
    MissingAction = 3,
}
