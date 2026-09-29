//! Auto-transpiled C/C++ header module: AS_asset_file_status

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteAssetFileStatus {
    UNSET = 0,
    NOT_ON_DISK = 1,
    MATCH = 2,
    NO_MATCH = 3,
}
