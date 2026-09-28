use std::sync::Mutex;

//! Auto-transpiled C/C++ header module: AS_asset_library

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AssetStorage {
    pub external_assets_mutex: Mutex,
    pub local_id_assets_mutex: Mutex,
}
