//! Auto-transpiled C/C++ header module: BKE_keyconfig

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ExtensionRNA;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct wmKeyConfigPrefType_Runtime {
    pub idname: [i8; 64],
    pub rna_ext: ExtensionRNA,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct wmKeyConfigFilterItemParams {
    pub check_item: u32,
    pub check_diff_item_add: u32,
    pub check_diff_item_remove: u32,
}
