//! Auto-transpiled C/C++ header module: BKE_bake_items_serialize

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BlobSlice {
    pub range: IndexRange,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StoredByRuntimeValue {
    pub sharing_info_version: i64,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct OutputStream {
    pub offset: i64,
}
