//! Auto-transpiled C/C++ header module: BKE_main

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BlendThumbnail {
    pub rect: [i8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct id_pointer {

}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MainIDRelationsEntry {
    pub session_uid: u32,
    pub tags: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MainIDRelations {
    pub flag: i16,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MainColorspace {
    pub is_missing_opencolorio_config: bool,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MainMergeReport {
    pub num_merged_ids: i32,
    pub num_unknown_ids: i32,
    pub num_remapped_ids: i32,
    pub num_remapped_libraries: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMainIDRelationsEntryTags {
    MAINIDRELATIONS_ENTRY_TAGS_DOIT = 1 << 0,
    MAINIDRELATIONS_ENTRY_TAGS_PROCESSED_TO = 1 << 4,
    MAINIDRELATIONS_ENTRY_TAGS_PROCESSED_FROM = 1 << 5,
    MAINIDRELATIONS_ENTRY_TAGS_PROCESSED = (1 << 4) | (1 << 5),
    MAINIDRELATIONS_ENTRY_TAGS_INPROGRESS_TO = 1 << 8,
    MAINIDRELATIONS_ENTRY_TAGS_INPROGRESS_FROM = 1 << 9,
    MAINIDRELATIONS_ENTRY_TAGS_INPROGRESS = (1 << 8) | (1 << 9),
}
