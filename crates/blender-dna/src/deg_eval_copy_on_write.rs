use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ValidateData {
    pub is_valid: bool,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RemapCallbackUserData {
    pub depsgraph: *mut Depsgraph,
}

// Placeholder for Depsgraph type
pub struct Depsgraph {
    // Placeholder fields
}
