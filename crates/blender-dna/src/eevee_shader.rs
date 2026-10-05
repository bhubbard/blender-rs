use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CallbackThunk {
    pub shader_module: *mut ShaderModule,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct to {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SlotAllocator {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ShaderModule {
    pub _opaque: [u8; 0],
}
