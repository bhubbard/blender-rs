//! Auto-transpiled C/C++ header module: DNA_ID

use core::ffi::c_void;
use crate::*;

pub const MAX_IDPROP_NAME: i32 = 64;
pub const DEFAULT_ALLOC_FOR_NULL_STRINGS: i32 = 64;
pub const MAX_ID_NAME: i32 = 258;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDPropertyUIData {
    pub description: *mut i8,
    pub rna_subtype: i32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDPropertyUIDataEnumItem {
    pub identifier: *mut i8,
    pub name: *mut i8,
    pub description: *mut i8,
    pub value: i32,
    pub icon: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDPropertyUIDataInt {
    pub base: IDPropertyUIData,
    pub default_array: *mut i32,
    pub default_array_len: i32,
    pub min: i32,
    pub max: i32,
    pub soft_min: i32,
    pub soft_max: i32,
    pub step: i32,
    pub default_value: i32,
    pub enum_items_num: i32,
    pub enum_items: *mut IDPropertyUIDataEnumItem,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDPropertyUIDataBool {
    pub base: IDPropertyUIData,
    pub default_array: *mut i8,
    pub default_array_len: i32,
    pub _pad: [i8; 3],
    pub default_value: i8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDPropertyUIDataFloat {
    pub base: IDPropertyUIData,
    pub default_array: *mut f64,
    pub default_array_len: i32,
    pub _pad: [i8; 4],
    pub step: f32,
    pub precision: i32,
    pub min: f64,
    pub max: f64,
    pub soft_min: f64,
    pub soft_max: f64,
    pub default_value: f64,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDPropertyUIDataString {
    pub base: IDPropertyUIData,
    pub default_value: *mut i8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDPropertyUIDataID {
    pub base: IDPropertyUIData,
    pub id_type: i16,
    pub _pad: [i8; 6],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDPropertyData {
    pub pointer: *mut core::ffi::c_void,
    pub group: ListBaseT<IDProperty>,
    pub val: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct IDProperty {
    pub next: *mut IDProperty,
    pub r#type: eIDPropertyType,
    pub subtype: i8,
    pub flag: eIDPropertyFlag,
    pub name: [i8; 64],
    pub _pad0: [i8; 4],
    pub data: IDPropertyData,
    pub len: i32,
    pub totallen: i32,
    pub ui_data: *mut IDPropertyUIData,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDOverrideLibraryPropertyOperation {
    pub next: *mut IDOverrideLibraryPropertyOperation,
    pub operation: eID_OverrideLib_Op,
    pub flag: eID_OverrideLib_OpFlag,
    pub tag: eID_OverrideLib_PropTag,
    pub _pad0: [i8; 2],
    pub subitem_reference_name: *mut i8,
    pub subitem_local_name: *mut i8,
    pub subitem_reference_index: i32,
    pub subitem_local_index: i32,
    pub subitem_reference_id: *mut ID,
    pub subitem_local_id: *mut ID,
    pub label: *mut i8,
    pub tooltip: *mut i8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDOverrideLibraryProperty {
    pub next: *mut IDOverrideLibraryProperty,
    pub rna_path: *mut i8,
    pub operations: ListBaseT<IDOverrideLibraryPropertyOperation>,
    pub tag: eID_OverrideLib_PropTag,
    pub _pad: [i8; 2],
    pub rna_prop_type: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDOverrideLibrary {
    pub reference: *mut ID,
    pub properties: ListBaseT<IDOverrideLibraryProperty>,
    pub hierarchy_root: *mut ID,
    pub runtime: *mut IDOverrideLibraryRuntime,
    pub flag: eID_OverrideLib_Flag,
    pub _pad_1: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDHash {
    pub data: [i8; 16],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct ID {
    pub next: *mut core::ffi::c_void,
    pub newid: *mut ID,
    pub lib: *mut Library,
    pub asset_data: *mut AssetMetaData,
    pub name: [i8; 258],
    pub flag: i16,
    pub tag: i32,
    pub us: i32,
    pub icon_id: i32,
    pub recalc: u32,
    pub recalc_up_to_undo_push: u32,
    pub recalc_after_undo_push: u32,
    pub session_uid: u32,
    pub deep_hash: IDHash,
    pub properties: *mut IDProperty,
    pub system_properties: *mut IDProperty,
    pub _pad1: *mut core::ffi::c_void,
    pub override_library: *mut IDOverrideLibrary,
    pub orig_id: *mut ID,
    pub py_instance: *mut core::ffi::c_void,
    pub library_weak_reference: *mut LibraryWeakReference,
}

impl Default for ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Library {
    pub id: ID,
    pub filepath: [i8; 1024],
    pub flag: u16,
    pub undo_runtime_tag: u16,
    pub _pad: [i8; 4],
    pub archive_parent_library: *mut Library,
    pub packedfile: *mut PackedFile,
    pub _pad2: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct LibraryWeakReference {
    pub library_filepath: [i8; 1024],
    pub library_id_name: [i8; 258],
    pub _pad: [i8; 2],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PreviewImage {
    pub w: [u32; 2],
    pub h: [u32; 2],
    pub flag: [i16; 2],
    pub changed_timestamp: [i16; 2],
    pub rect: [*mut u32; 2],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ID_Runtime {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PreviewImageRuntime {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDPropertyGroupChildrenSet {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LibraryRuntime {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FileData {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GHash {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PackedFile {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct UniqueName_Map {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDOverrideLibraryRuntime {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AssetMetaData {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eID_OverrideLib_Op {
    LIBOVERRIDE_OP_NOOP = 0,
    LIBOVERRIDE_OP_REPLACE = 1,
    LIBOVERRIDE_OP_ADD = 101,
    LIBOVERRIDE_OP_SUBTRACT = 102,
    LIBOVERRIDE_OP_MULTIPLY = 103,
    LIBOVERRIDE_OP_INSERT_AFTER = 201,
    LIBOVERRIDE_OP_INSERT_BEFORE = 202,
    LIBOVERRIDE_OP_CUSTOM = 255,
}

impl Default for eID_OverrideLib_Op {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LIBOVERRIDE_OP_NOOP: i32 = eID_OverrideLib_Op::LIBOVERRIDE_OP_NOOP as i32;
pub const LIBOVERRIDE_OP_REPLACE: i32 = eID_OverrideLib_Op::LIBOVERRIDE_OP_REPLACE as i32;
pub const LIBOVERRIDE_OP_ADD: i32 = eID_OverrideLib_Op::LIBOVERRIDE_OP_ADD as i32;
pub const LIBOVERRIDE_OP_SUBTRACT: i32 = eID_OverrideLib_Op::LIBOVERRIDE_OP_SUBTRACT as i32;
pub const LIBOVERRIDE_OP_MULTIPLY: i32 = eID_OverrideLib_Op::LIBOVERRIDE_OP_MULTIPLY as i32;
pub const LIBOVERRIDE_OP_INSERT_AFTER: i32 = eID_OverrideLib_Op::LIBOVERRIDE_OP_INSERT_AFTER as i32;
pub const LIBOVERRIDE_OP_INSERT_BEFORE: i32 = eID_OverrideLib_Op::LIBOVERRIDE_OP_INSERT_BEFORE as i32;
pub const LIBOVERRIDE_OP_CUSTOM: i32 = eID_OverrideLib_Op::LIBOVERRIDE_OP_CUSTOM as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eID_OverrideLib_OpFlag {
    LIBOVERRIDE_OP_FLAG_MANDATORY = 1 << 0,
    LIBOVERRIDE_OP_FLAG_LOCKED = 1 << 1,
    LIBOVERRIDE_OP_FLAG_IDPOINTER_MATCH_REFERENCE = 1 << 8,
    LIBOVERRIDE_OP_FLAG_IDPOINTER_ITEM_USE_ID = 1 << 9,
}

impl Default for eID_OverrideLib_OpFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LIBOVERRIDE_OP_FLAG_MANDATORY: i32 = eID_OverrideLib_OpFlag::LIBOVERRIDE_OP_FLAG_MANDATORY as i32;
pub const LIBOVERRIDE_OP_FLAG_LOCKED: i32 = eID_OverrideLib_OpFlag::LIBOVERRIDE_OP_FLAG_LOCKED as i32;
pub const LIBOVERRIDE_OP_FLAG_IDPOINTER_MATCH_REFERENCE: i32 = eID_OverrideLib_OpFlag::LIBOVERRIDE_OP_FLAG_IDPOINTER_MATCH_REFERENCE as i32;
pub const LIBOVERRIDE_OP_FLAG_IDPOINTER_ITEM_USE_ID: i32 = eID_OverrideLib_OpFlag::LIBOVERRIDE_OP_FLAG_IDPOINTER_ITEM_USE_ID as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eID_OverrideLib_PropTag {
    LIBOVERRIDE_PROP_OP_TAG_UNUSED = 1 << 0,
    LIBOVERRIDE_PROP_TAG_NEEDS_RETORE = 1 << 1,
}

impl Default for eID_OverrideLib_PropTag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LIBOVERRIDE_PROP_OP_TAG_UNUSED: i32 = eID_OverrideLib_PropTag::LIBOVERRIDE_PROP_OP_TAG_UNUSED as i32;
pub const LIBOVERRIDE_PROP_TAG_NEEDS_RETORE: i32 = eID_OverrideLib_PropTag::LIBOVERRIDE_PROP_TAG_NEEDS_RETORE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eID_OverrideLib_Flag {
    LIBOVERRIDE_FLAG_NO_HIERARCHY = 1 << 0,
    LIBOVERRIDE_FLAG_SYSTEM_DEFINED = 1 << 1,
}

impl Default for eID_OverrideLib_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LIBOVERRIDE_FLAG_NO_HIERARCHY: i32 = eID_OverrideLib_Flag::LIBOVERRIDE_FLAG_NO_HIERARCHY as i32;
pub const LIBOVERRIDE_FLAG_SYSTEM_DEFINED: i32 = eID_OverrideLib_Flag::LIBOVERRIDE_FLAG_SYSTEM_DEFINED as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eID_RemapStatus {
    ID_REMAP_IS_LINKED_DIRECT = 1 << 0,
    ID_REMAP_IS_USER_ONE_SKIPPED = 1 << 1,
}

impl Default for eID_RemapStatus {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const ID_REMAP_IS_LINKED_DIRECT: i32 = eID_RemapStatus::ID_REMAP_IS_LINKED_DIRECT as i32;
pub const ID_REMAP_IS_USER_ONE_SKIPPED: i32 = eID_RemapStatus::ID_REMAP_IS_USER_ONE_SKIPPED as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ePreviewImage_Flag {
    PRV_CHANGED = (1 << 0),
    PRV_USER_EDITED = (1 << 1),
    PRV_RENDERING = (1 << 2),
}

impl Default for ePreviewImage_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const PRV_CHANGED: i32 = ePreviewImage_Flag::PRV_CHANGED as i32;
pub const PRV_USER_EDITED: i32 = ePreviewImage_Flag::PRV_USER_EDITED as i32;
pub const PRV_RENDERING: i32 = ePreviewImage_Flag::PRV_RENDERING as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ePreviewImage_Tag {
    PRV_TAG_DEFERRED_RENDERING = (1 << 1),
    PRV_TAG_DEFERRED_DELETE = (1 << 2),
    PRV_TAG_DEFERRED_INVALID = (1 << 3),
    PRV_TAG_RESTART_RENDERING = (1 << 4),
}

impl Default for ePreviewImage_Tag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const PRV_TAG_DEFERRED_RENDERING: i32 = ePreviewImage_Tag::PRV_TAG_DEFERRED_RENDERING as i32;
pub const PRV_TAG_DEFERRED_DELETE: i32 = ePreviewImage_Tag::PRV_TAG_DEFERRED_DELETE as i32;
pub const PRV_TAG_DEFERRED_INVALID: i32 = ePreviewImage_Tag::PRV_TAG_DEFERRED_INVALID as i32;
pub const PRV_TAG_RESTART_RENDERING: i32 = ePreviewImage_Tag::PRV_TAG_RESTART_RENDERING as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eID_Flag {
    ID_FLAG_FAKEUSER = 1 << 9,
    ID_FLAG_EMBEDDED_DATA = 1 << 10,
    ID_FLAG_INDIRECT_WEAK_LINK = 1 << 11,
    ID_FLAG_EMBEDDED_DATA_LIB_OVERRIDE = 1 << 12,
    ID_FLAG_LIB_OVERRIDE_RESYNC_LEFTOVER = 1 << 13,
    ID_FLAG_CLIPBOARD_MARK = 1 << 14,
    ID_FLAG_LINKED_AND_PACKED = 1 << 15,
}

impl Default for eID_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const ID_FLAG_FAKEUSER: i32 = eID_Flag::ID_FLAG_FAKEUSER as i32;
pub const ID_FLAG_EMBEDDED_DATA: i32 = eID_Flag::ID_FLAG_EMBEDDED_DATA as i32;
pub const ID_FLAG_INDIRECT_WEAK_LINK: i32 = eID_Flag::ID_FLAG_INDIRECT_WEAK_LINK as i32;
pub const ID_FLAG_EMBEDDED_DATA_LIB_OVERRIDE: i32 = eID_Flag::ID_FLAG_EMBEDDED_DATA_LIB_OVERRIDE as i32;
pub const ID_FLAG_LIB_OVERRIDE_RESYNC_LEFTOVER: i32 = eID_Flag::ID_FLAG_LIB_OVERRIDE_RESYNC_LEFTOVER as i32;
pub const ID_FLAG_CLIPBOARD_MARK: i32 = eID_Flag::ID_FLAG_CLIPBOARD_MARK as i32;
pub const ID_FLAG_LINKED_AND_PACKED: i32 = eID_Flag::ID_FLAG_LINKED_AND_PACKED as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eID_Tag {
    ID_TAG_LOCAL = 0,
    ID_TAG_EXTERN = 1 << 0,
    ID_TAG_INDIRECT = 1 << 1,
    ID_TAG_RUNTIME = 1 << 2,
    ID_TAG_MISSING = 1 << 3,
    ID_TAG_EXTRAUSER = 1 << 4,
    ID_TAG_EXTRAUSER_SET = 1 << 5,
    ID_TAG_LIBOVERRIDE_REFOK = 1 << 6,
    ID_TAG_LIBOVERRIDE_AUTOREFRESH = 1 << 7,
    ID_TAG_LIBOVERRIDE_NEED_RESYNC = 1 << 8,
    ID_TAG_NEW = 1 << 12,
    ID_TAG_PRE_EXISTING = 1 << 13,
    ID_TAG_UNDO_OLD_ID_REUSED_UNCHANGED = 1 << 17,
    ID_TAG_UNDO_OLD_ID_REUSED_NOUNDO = 1 << 18,
    ID_TAG_UNDO_OLD_ID_REREAD_IN_PLACE = 1 << 19,
    ID_TAG_TEMP_MAIN = 1 << 20,
    ID_TAG_NO_MAIN = 1 << 21,
    ID_TAG_LOCALIZED = 1 << 22,
    ID_TAG_COPIED_ON_EVAL = 1 << 23,
    ID_TAG_NO_USER_REFCOUNT = 1 << 25,
    ID_TAG_NOT_ALLOCATED = 1 << 26,
    ID_TAG_DOIT = i32::MIN,
}

impl Default for eID_Tag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const ID_TAG_LOCAL: i32 = eID_Tag::ID_TAG_LOCAL as i32;
pub const ID_TAG_EXTERN: i32 = eID_Tag::ID_TAG_EXTERN as i32;
pub const ID_TAG_INDIRECT: i32 = eID_Tag::ID_TAG_INDIRECT as i32;
pub const ID_TAG_RUNTIME: i32 = eID_Tag::ID_TAG_RUNTIME as i32;
pub const ID_TAG_MISSING: i32 = eID_Tag::ID_TAG_MISSING as i32;
pub const ID_TAG_EXTRAUSER: i32 = eID_Tag::ID_TAG_EXTRAUSER as i32;
pub const ID_TAG_EXTRAUSER_SET: i32 = eID_Tag::ID_TAG_EXTRAUSER_SET as i32;
pub const ID_TAG_LIBOVERRIDE_REFOK: i32 = eID_Tag::ID_TAG_LIBOVERRIDE_REFOK as i32;
pub const ID_TAG_LIBOVERRIDE_AUTOREFRESH: i32 = eID_Tag::ID_TAG_LIBOVERRIDE_AUTOREFRESH as i32;
pub const ID_TAG_LIBOVERRIDE_NEED_RESYNC: i32 = eID_Tag::ID_TAG_LIBOVERRIDE_NEED_RESYNC as i32;
pub const ID_TAG_NEW: i32 = eID_Tag::ID_TAG_NEW as i32;
pub const ID_TAG_PRE_EXISTING: i32 = eID_Tag::ID_TAG_PRE_EXISTING as i32;
pub const ID_TAG_UNDO_OLD_ID_REUSED_UNCHANGED: i32 = eID_Tag::ID_TAG_UNDO_OLD_ID_REUSED_UNCHANGED as i32;
pub const ID_TAG_UNDO_OLD_ID_REUSED_NOUNDO: i32 = eID_Tag::ID_TAG_UNDO_OLD_ID_REUSED_NOUNDO as i32;
pub const ID_TAG_UNDO_OLD_ID_REREAD_IN_PLACE: i32 = eID_Tag::ID_TAG_UNDO_OLD_ID_REREAD_IN_PLACE as i32;
pub const ID_TAG_TEMP_MAIN: i32 = eID_Tag::ID_TAG_TEMP_MAIN as i32;
pub const ID_TAG_NO_MAIN: i32 = eID_Tag::ID_TAG_NO_MAIN as i32;
pub const ID_TAG_LOCALIZED: i32 = eID_Tag::ID_TAG_LOCALIZED as i32;
pub const ID_TAG_COPIED_ON_EVAL: i32 = eID_Tag::ID_TAG_COPIED_ON_EVAL as i32;
pub const ID_TAG_NO_USER_REFCOUNT: i32 = eID_Tag::ID_TAG_NO_USER_REFCOUNT as i32;
pub const ID_TAG_NOT_ALLOCATED: i32 = eID_Tag::ID_TAG_NOT_ALLOCATED as i32;
pub const ID_TAG_DOIT: i32 = eID_Tag::ID_TAG_DOIT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IDRecalcFlag {
    ID_RECALC_TRANSFORM = (1 << 0),
    ID_RECALC_GEOMETRY = (1 << 1),
    ID_RECALC_ANIMATION = (1 << 2),
    ID_RECALC_PSYS_REDO = (1 << 3),
    ID_RECALC_PSYS_RESET = (1 << 4),
    ID_RECALC_PSYS_CHILD = (1 << 5),
    ID_RECALC_PSYS_PHYS = (1 << 6),
    ID_RECALC_SHADING = (1 << 7),
    ID_RECALC_SELECT = (1 << 9),
    ID_RECALC_BASE_FLAGS = (1 << 10),
    ID_RECALC_POINT_CACHE = (1 << 11),
    ID_RECALC_EDITORS = (1 << 12),
    ID_RECALC_SYNC_TO_EVAL = (1 << 13),
    ID_RECALC_SEQUENCER_STRIPS = (1 << 14),
    ID_RECALC_FRAME_CHANGE = (1 << 15),
    ID_RECALC_AUDIO_FPS = (1 << 16),
    ID_RECALC_AUDIO_VOLUME = (1 << 17),
    ID_RECALC_AUDIO_MUTE = (1 << 18),
    ID_RECALC_AUDIO_LISTENER = (1 << 19),
    ID_RECALC_AUDIO = (1 << 20),
    ID_RECALC_PARAMETERS = (1 << 21),
    ID_RECALC_SOURCE = (1 << 23),
    ID_RECALC_TAG_FOR_UNDO = (1 << 24),
    ID_RECALC_NTREE_OUTPUT = (1 << 25),
    ID_RECALC_HIERARCHY = (1 << 26),
    ID_RECALC_COMPOSITOR = (1 << 27),
    ID_RECALC_PROVISION_28 = (1 << 28),
    ID_RECALC_PROVISION_29 = (1 << 29),
    ID_RECALC_PROVISION_30 = (1 << 30),
    ID_RECALC_PROVISION_31 = (i32::MIN),
    ID_RECALC_GEOMETRY_ALL_MODES = ID_RECALC_GEOMETRY | ID_RECALC_SYNC_TO_EVAL,
    ID_RECALC_ALL = (-1i32),
    ID_RECALC_PSYS_ALL = (ID_RECALC_PSYS_REDO | ID_RECALC_PSYS_RESET | ID_RECALC_PSYS_CHILD | ID_RECALC_PSYS_PHYS),
}

impl Default for IDRecalcFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const ID_RECALC_TRANSFORM: i32 = IDRecalcFlag::ID_RECALC_TRANSFORM as i32;
pub const ID_RECALC_GEOMETRY: i32 = IDRecalcFlag::ID_RECALC_GEOMETRY as i32;
pub const ID_RECALC_ANIMATION: i32 = IDRecalcFlag::ID_RECALC_ANIMATION as i32;
pub const ID_RECALC_PSYS_REDO: i32 = IDRecalcFlag::ID_RECALC_PSYS_REDO as i32;
pub const ID_RECALC_PSYS_RESET: i32 = IDRecalcFlag::ID_RECALC_PSYS_RESET as i32;
pub const ID_RECALC_PSYS_CHILD: i32 = IDRecalcFlag::ID_RECALC_PSYS_CHILD as i32;
pub const ID_RECALC_PSYS_PHYS: i32 = IDRecalcFlag::ID_RECALC_PSYS_PHYS as i32;
pub const ID_RECALC_SHADING: i32 = IDRecalcFlag::ID_RECALC_SHADING as i32;
pub const ID_RECALC_SELECT: i32 = IDRecalcFlag::ID_RECALC_SELECT as i32;
pub const ID_RECALC_BASE_FLAGS: i32 = IDRecalcFlag::ID_RECALC_BASE_FLAGS as i32;
pub const ID_RECALC_POINT_CACHE: i32 = IDRecalcFlag::ID_RECALC_POINT_CACHE as i32;
pub const ID_RECALC_EDITORS: i32 = IDRecalcFlag::ID_RECALC_EDITORS as i32;
pub const ID_RECALC_SYNC_TO_EVAL: i32 = IDRecalcFlag::ID_RECALC_SYNC_TO_EVAL as i32;
pub const ID_RECALC_SEQUENCER_STRIPS: i32 = IDRecalcFlag::ID_RECALC_SEQUENCER_STRIPS as i32;
pub const ID_RECALC_FRAME_CHANGE: i32 = IDRecalcFlag::ID_RECALC_FRAME_CHANGE as i32;
pub const ID_RECALC_AUDIO_FPS: i32 = IDRecalcFlag::ID_RECALC_AUDIO_FPS as i32;
pub const ID_RECALC_AUDIO_VOLUME: i32 = IDRecalcFlag::ID_RECALC_AUDIO_VOLUME as i32;
pub const ID_RECALC_AUDIO_MUTE: i32 = IDRecalcFlag::ID_RECALC_AUDIO_MUTE as i32;
pub const ID_RECALC_AUDIO_LISTENER: i32 = IDRecalcFlag::ID_RECALC_AUDIO_LISTENER as i32;
pub const ID_RECALC_AUDIO: i32 = IDRecalcFlag::ID_RECALC_AUDIO as i32;
pub const ID_RECALC_PARAMETERS: i32 = IDRecalcFlag::ID_RECALC_PARAMETERS as i32;
pub const ID_RECALC_SOURCE: i32 = IDRecalcFlag::ID_RECALC_SOURCE as i32;
pub const ID_RECALC_TAG_FOR_UNDO: i32 = IDRecalcFlag::ID_RECALC_TAG_FOR_UNDO as i32;
pub const ID_RECALC_NTREE_OUTPUT: i32 = IDRecalcFlag::ID_RECALC_NTREE_OUTPUT as i32;
pub const ID_RECALC_HIERARCHY: i32 = IDRecalcFlag::ID_RECALC_HIERARCHY as i32;
pub const ID_RECALC_COMPOSITOR: i32 = IDRecalcFlag::ID_RECALC_COMPOSITOR as i32;
pub const ID_RECALC_PROVISION_28: i32 = IDRecalcFlag::ID_RECALC_PROVISION_28 as i32;
pub const ID_RECALC_PROVISION_29: i32 = IDRecalcFlag::ID_RECALC_PROVISION_29 as i32;
pub const ID_RECALC_PROVISION_30: i32 = IDRecalcFlag::ID_RECALC_PROVISION_30 as i32;
pub const ID_RECALC_PROVISION_31: i32 = IDRecalcFlag::ID_RECALC_PROVISION_31 as i32;
pub const ID_RECALC_ANIMATION_NO_FLUSH: i32 = ID_RECALC_SYNC_TO_EVAL;
pub const ID_RECALC_GEOMETRY_ALL_MODES: i32 = IDRecalcFlag::ID_RECALC_GEOMETRY_ALL_MODES as i32;
pub const ID_RECALC_ALL: i32 = IDRecalcFlag::ID_RECALC_ALL as i32;
pub const ID_RECALC_PSYS_ALL: i32 = IDRecalcFlag::ID_RECALC_PSYS_ALL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eID_Index {
    INDEX_ID_LI = 0,
    INDEX_ID_AC,
    INDEX_ID_GD_LEGACY,
    INDEX_ID_NT,
    INDEX_ID_VF,
    INDEX_ID_TXT,
    INDEX_ID_SO,
    INDEX_ID_MSK,
    INDEX_ID_IM,
    INDEX_ID_MC,
    INDEX_ID_TE,
    INDEX_ID_MA,
    INDEX_ID_LS,
    INDEX_ID_WO,
    INDEX_ID_CF,
    INDEX_ID_PA,
    INDEX_ID_KE,
    INDEX_ID_AR,
    INDEX_ID_ME,
    INDEX_ID_CU_LEGACY,
    INDEX_ID_MB,
    INDEX_ID_CV,
    INDEX_ID_PT,
    INDEX_ID_VO,
    INDEX_ID_LT,
    INDEX_ID_LA,
    INDEX_ID_CA,
    INDEX_ID_SPK,
    INDEX_ID_LP,
    INDEX_ID_GP,
    INDEX_ID_OB,
    INDEX_ID_GR,
    INDEX_ID_PAL,
    INDEX_ID_PC,
    INDEX_ID_BR,
    INDEX_ID_SCE,
    INDEX_ID_SCR,
    INDEX_ID_WS,
    INDEX_ID_WM,
    INDEX_ID_NULL,
}

impl Default for eID_Index {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const INDEX_ID_LI: i32 = eID_Index::INDEX_ID_LI as i32;
pub const INDEX_ID_AC: i32 = eID_Index::INDEX_ID_AC as i32;
pub const INDEX_ID_GD_LEGACY: i32 = eID_Index::INDEX_ID_GD_LEGACY as i32;
pub const INDEX_ID_NT: i32 = eID_Index::INDEX_ID_NT as i32;
pub const INDEX_ID_VF: i32 = eID_Index::INDEX_ID_VF as i32;
pub const INDEX_ID_TXT: i32 = eID_Index::INDEX_ID_TXT as i32;
pub const INDEX_ID_SO: i32 = eID_Index::INDEX_ID_SO as i32;
pub const INDEX_ID_MSK: i32 = eID_Index::INDEX_ID_MSK as i32;
pub const INDEX_ID_IM: i32 = eID_Index::INDEX_ID_IM as i32;
pub const INDEX_ID_MC: i32 = eID_Index::INDEX_ID_MC as i32;
pub const INDEX_ID_TE: i32 = eID_Index::INDEX_ID_TE as i32;
pub const INDEX_ID_MA: i32 = eID_Index::INDEX_ID_MA as i32;
pub const INDEX_ID_LS: i32 = eID_Index::INDEX_ID_LS as i32;
pub const INDEX_ID_WO: i32 = eID_Index::INDEX_ID_WO as i32;
pub const INDEX_ID_CF: i32 = eID_Index::INDEX_ID_CF as i32;
pub const INDEX_ID_PA: i32 = eID_Index::INDEX_ID_PA as i32;
pub const INDEX_ID_KE: i32 = eID_Index::INDEX_ID_KE as i32;
pub const INDEX_ID_AR: i32 = eID_Index::INDEX_ID_AR as i32;
pub const INDEX_ID_ME: i32 = eID_Index::INDEX_ID_ME as i32;
pub const INDEX_ID_CU_LEGACY: i32 = eID_Index::INDEX_ID_CU_LEGACY as i32;
pub const INDEX_ID_MB: i32 = eID_Index::INDEX_ID_MB as i32;
pub const INDEX_ID_CV: i32 = eID_Index::INDEX_ID_CV as i32;
pub const INDEX_ID_PT: i32 = eID_Index::INDEX_ID_PT as i32;
pub const INDEX_ID_VO: i32 = eID_Index::INDEX_ID_VO as i32;
pub const INDEX_ID_LT: i32 = eID_Index::INDEX_ID_LT as i32;
pub const INDEX_ID_LA: i32 = eID_Index::INDEX_ID_LA as i32;
pub const INDEX_ID_CA: i32 = eID_Index::INDEX_ID_CA as i32;
pub const INDEX_ID_SPK: i32 = eID_Index::INDEX_ID_SPK as i32;
pub const INDEX_ID_LP: i32 = eID_Index::INDEX_ID_LP as i32;
pub const INDEX_ID_GP: i32 = eID_Index::INDEX_ID_GP as i32;
pub const INDEX_ID_OB: i32 = eID_Index::INDEX_ID_OB as i32;
pub const INDEX_ID_GR: i32 = eID_Index::INDEX_ID_GR as i32;
pub const INDEX_ID_PAL: i32 = eID_Index::INDEX_ID_PAL as i32;
pub const INDEX_ID_PC: i32 = eID_Index::INDEX_ID_PC as i32;
pub const INDEX_ID_BR: i32 = eID_Index::INDEX_ID_BR as i32;
pub const INDEX_ID_SCE: i32 = eID_Index::INDEX_ID_SCE as i32;
pub const INDEX_ID_SCR: i32 = eID_Index::INDEX_ID_SCR as i32;
pub const INDEX_ID_WS: i32 = eID_Index::INDEX_ID_WS as i32;
pub const INDEX_ID_WM: i32 = eID_Index::INDEX_ID_WM as i32;
pub const INDEX_ID_NULL: i32 = eID_Index::INDEX_ID_NULL as i32;
