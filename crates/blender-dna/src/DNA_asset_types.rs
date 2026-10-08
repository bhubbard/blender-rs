//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[allow(non_camel_case_types)]
type int32_t = i32;
#[allow(non_camel_case_types)]
type uint32_t = u32;
#[allow(non_camel_case_types)]
type int16_t = i16;
#[allow(non_camel_case_types)]
type uint16_t = u16;
#[allow(non_camel_case_types)]
type int64_t = i64;
#[allow(non_camel_case_types)]
type uint64_t = u64;
#[allow(non_camel_case_types)]
type int8_t = i8;
#[allow(non_camel_case_types)]
type uint8_t = u8;
#[allow(non_camel_case_types)]
type uchar = u8;
#[allow(non_camel_case_types)]
type ushort = u16;
#[allow(non_camel_case_types)]
type uint = u32;
#[allow(non_camel_case_types)]
type ulong = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub struct bUUID { pub data: [u8; 16] }

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eAssetLibraryType(pub i16);

impl eAssetLibraryType {
    pub const ASSET_LIBRARY_LOCAL: Self = Self((1) as i16);
    pub const ASSET_LIBRARY_ALL: Self = Self((2) as i16);
    pub const ASSET_LIBRARY_ESSENTIALS: Self = Self((3) as i16);
    pub const ASSET_LIBRARY_ONLINE_ESSENTIALS: Self = Self((4) as i16);
    pub const ASSET_LIBRARY_CUSTOM: Self = Self((100) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eAssetImportMethod(pub i32);

impl eAssetImportMethod {
    pub const ASSET_IMPORT_LINK: Self = Self((0) as i32);
    pub const ASSET_IMPORT_APPEND: Self = Self((1) as i32);
    pub const ASSET_IMPORT_APPEND_REUSE: Self = Self((2) as i32);
    pub const ASSET_IMPORT_PACK: Self = Self((3) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eAssetLibrary_Flag(pub i32);

impl eAssetLibrary_Flag {
    pub const ASSET_LIBRARY_RELATIVE_PATH: Self = Self(((1 << 0)) as i32);
    pub const ASSET_LIBRARY_DISABLED: Self = Self(((1 << 1)) as i32);
    pub const ASSET_LIBRARY_USE_REMOTE_URL: Self = Self(((1 << 2)) as i32);
    pub const ASSET_LIBRARY_USE_AUTH_TOKEN: Self = Self(((1 << 3)) as i32);
    pub const ASSET_LIBRARY_PROJECT_DEFINED: Self = Self(((1 << 4)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AssetAccess(pub i8);

impl AssetAccess {
    pub const OnlineAndOffline: Self = Self((0) as i8);
    pub const OnlyOnline: Self = Self((1) as i8);
    pub const OnlyOffline: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AssetMetaDataFlag(pub i32);

impl AssetMetaDataFlag {
    pub const ASSETDATA_USE_OWN_IMPORT_METHOD: Self = Self(((1 << 0)) as i32);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct AssetTag {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub name: [u8; 64],
}

impl Default for AssetTag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct AssetMetaData {
    pub local_type_info: *mut core::ffi::c_void,
    pub properties: *mut core::ffi::c_void,
    pub catalog_id: bUUID,
    pub catalog_simple_name: [u8; 64],
    pub author: *mut core::ffi::c_void,
    pub description: *mut core::ffi::c_void,
    pub copyright: *mut core::ffi::c_void,
    pub license: *mut core::ffi::c_void,
    pub tags: ListBaseT<AssetTag>,
    pub nullptr: ListBaseT<AssetTag>,
}

impl Default for AssetMetaData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct AssetImportSettings {
    pub method: eAssetImportMethod,
}

impl Default for AssetImportSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct AssetLibraryReference {
    pub r#type: eAssetLibraryType,
    pub _pad1: [u8; 2],
}

impl Default for AssetLibraryReference {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct AssetWeakReference {
    pub _pad: [u8; 6],
}

impl Default for AssetWeakReference {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct AssetCatalogPathLink {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub path: *mut core::ffi::c_void,
}

impl Default for AssetCatalogPathLink {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

