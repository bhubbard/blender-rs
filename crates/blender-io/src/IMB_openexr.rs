use crate::*;

pub const EXR_LAY_MAXNAME: i32 = 64;
pub const EXR_PASS_MAXNAME: i32 = 64;
pub const EXR_VIEW_MAXNAME: i32 = 64;
pub const EXR_TOT_MAXNAME: i32 = 64;
pub const EXR_PASS_MAXCHAN: i32 = 24;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ExrPassInfo {
    pub layer: StringRefNull,
    pub pass: StringRefNull,
    pub view: StringRefNull,
    pub chan_id: StringRefNull,
    pub channels: i32,
    pub ibuf: *mut ImBuf,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImBuf {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StampData {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ExrReadHandle {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ExrWriteHandle {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StringRefNull {
    pub _opaque: [u8; 0],
}
