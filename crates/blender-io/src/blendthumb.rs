//! Auto-transpiled C/C++ header module: blendthumb

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Thumbnail {
    pub width: i32,
    pub height: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eThumbStatus {
    BT_OK = 0,
    BT_FILE_ERR = 1,
    BT_COMPRES_ERR = 2,
    BT_DECOMPRESS_ERR = 3,
    BT_INVALID_FILE = 4,
    BT_EARLY_VERSION = 5,
    BT_INVALID_THUMB = 6,
    BT_ERROR = 9,
}
