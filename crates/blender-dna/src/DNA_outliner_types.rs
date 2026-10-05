//! Auto-transpiled C/C++ header module: DNA_outliner_types

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TreeStoreElem {
    pub r#type: eTreeStoreElemType,
    pub nr: i16,
    pub flag: eTreeStoreElem_Flag,
    pub used: i16,
    pub id: *mut ID,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TreeStore {
    pub totelem: i32,
    pub usedelem: i32,
    pub data: *mut TreeStoreElem,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ID {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTreeStoreElem_Flag {
    TSE_CLOSED = (1 << 0),
    TSE_SELECTED = (1 << 1),
    TSE_TEXTBUT = (1 << 2),
    TSE_CHILDSEARCH = (1 << 3),
    TSE_SEARCHMATCH = (1 << 4),
    TSE_HIGHLIGHTED = (1 << 5),
    TSE_DRAG_INTO = (1 << 6),
    TSE_DRAG_BEFORE = (1 << 7),
    TSE_DRAG_AFTER = (1 << 8),
    TSE_ACTIVE = (1 << 9),
    TSE_HIGHLIGHTED_ICON = (1 << 11),
    TSE_DRAG_ANY = (TSE_DRAG_INTO | TSE_DRAG_BEFORE | TSE_DRAG_AFTER),
    TSE_HIGHLIGHTED_ANY = (TSE_HIGHLIGHTED | TSE_HIGHLIGHTED_ICON),
}

impl Default for eTreeStoreElem_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TSE_CLOSED: i32 = eTreeStoreElem_Flag::TSE_CLOSED as i32;
pub const TSE_SELECTED: i32 = eTreeStoreElem_Flag::TSE_SELECTED as i32;
pub const TSE_TEXTBUT: i32 = eTreeStoreElem_Flag::TSE_TEXTBUT as i32;
pub const TSE_CHILDSEARCH: i32 = eTreeStoreElem_Flag::TSE_CHILDSEARCH as i32;
pub const TSE_SEARCHMATCH: i32 = eTreeStoreElem_Flag::TSE_SEARCHMATCH as i32;
pub const TSE_HIGHLIGHTED: i32 = eTreeStoreElem_Flag::TSE_HIGHLIGHTED as i32;
pub const TSE_DRAG_INTO: i32 = eTreeStoreElem_Flag::TSE_DRAG_INTO as i32;
pub const TSE_DRAG_BEFORE: i32 = eTreeStoreElem_Flag::TSE_DRAG_BEFORE as i32;
pub const TSE_DRAG_AFTER: i32 = eTreeStoreElem_Flag::TSE_DRAG_AFTER as i32;
pub const TSE_ACTIVE: i32 = eTreeStoreElem_Flag::TSE_ACTIVE as i32;
pub const TSE_HIGHLIGHTED_ICON: i32 = eTreeStoreElem_Flag::TSE_HIGHLIGHTED_ICON as i32;
pub const TSE_DRAG_ANY: i32 = eTreeStoreElem_Flag::TSE_DRAG_ANY as i32;
pub const TSE_HIGHLIGHTED_ANY: i32 = eTreeStoreElem_Flag::TSE_HIGHLIGHTED_ANY as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTreeStoreElemType {
    TSE_SOME_ID = 0,
    TSE_NLA = 1,
    TSE_NLA_ACTION = 2,
    TSE_DEFGROUP_BASE = 3,
    TSE_DEFGROUP = 4,
    TSE_BONE = 5,
    TSE_EBONE = 6,
    TSE_CONSTRAINT_BASE = 7,
    TSE_CONSTRAINT = 8,
    TSE_MODIFIER_BASE = 9,
    TSE_MODIFIER = 10,
    TSE_LINKED_OB = 11,
    TSE_POSE_BASE = 13,
    TSE_POSE_CHANNEL = 14,
    TSE_ANIM_DATA = 15,
    TSE_DRIVER_BASE = 16,
    TSE_R_LAYER_BASE = 19,
    TSE_R_LAYER = 20,
    TSE_BONE_COLLECTION_BASE = 24,
    TSE_BONE_COLLECTION = 25,
    TSE_STRIP = 26,
    TSE_STRIP_DATA = 27,
    TSE_STRIP_DUP = 28,
    TSE_LINKED_PSYS = 29,
    TSE_RNA_STRUCT = 30,
    TSE_RNA_PROPERTY = 31,
    TSE_RNA_ARRAY_ELEM = 32,
    TSE_NLA_TRACK = 33,
    TSE_ID_BASE = 36,
    TSE_GP_LAYER = 37,
    TSE_LAYER_COLLECTION = 38,
    TSE_SCENE_COLLECTION_BASE = 39,
    TSE_VIEW_COLLECTION_BASE = 40,
    TSE_SCENE_OBJECTS_BASE = 41,
    TSE_GPENCIL_EFFECT_BASE = 42,
    TSE_GPENCIL_EFFECT = 43,
    TSE_LIBRARY_OVERRIDE_BASE = 44,
    TSE_LIBRARY_OVERRIDE = 45,
    TSE_LIBRARY_OVERRIDE_OPERATION = 46,
    TSE_GENERIC_LABEL = 47,
    TSE_GREASE_PENCIL_NODE = 48,
    TSE_LINKED_NODE_TREE = 49,
    TSE_ACTION_SLOT = 50,
    TSE_SHAPE_KEY_BLOCK = 51,
    TSE_SHAPE_KEY_BASE = 52,
}

impl Default for eTreeStoreElemType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TSE_SOME_ID: i32 = eTreeStoreElemType::TSE_SOME_ID as i32;
pub const TSE_NLA: i32 = eTreeStoreElemType::TSE_NLA as i32;
pub const TSE_NLA_ACTION: i32 = eTreeStoreElemType::TSE_NLA_ACTION as i32;
pub const TSE_DEFGROUP_BASE: i32 = eTreeStoreElemType::TSE_DEFGROUP_BASE as i32;
pub const TSE_DEFGROUP: i32 = eTreeStoreElemType::TSE_DEFGROUP as i32;
pub const TSE_BONE: i32 = eTreeStoreElemType::TSE_BONE as i32;
pub const TSE_EBONE: i32 = eTreeStoreElemType::TSE_EBONE as i32;
pub const TSE_CONSTRAINT_BASE: i32 = eTreeStoreElemType::TSE_CONSTRAINT_BASE as i32;
pub const TSE_CONSTRAINT: i32 = eTreeStoreElemType::TSE_CONSTRAINT as i32;
pub const TSE_MODIFIER_BASE: i32 = eTreeStoreElemType::TSE_MODIFIER_BASE as i32;
pub const TSE_MODIFIER: i32 = eTreeStoreElemType::TSE_MODIFIER as i32;
pub const TSE_LINKED_OB: i32 = eTreeStoreElemType::TSE_LINKED_OB as i32;
pub const TSE_POSE_BASE: i32 = eTreeStoreElemType::TSE_POSE_BASE as i32;
pub const TSE_POSE_CHANNEL: i32 = eTreeStoreElemType::TSE_POSE_CHANNEL as i32;
pub const TSE_ANIM_DATA: i32 = eTreeStoreElemType::TSE_ANIM_DATA as i32;
pub const TSE_DRIVER_BASE: i32 = eTreeStoreElemType::TSE_DRIVER_BASE as i32;
pub const TSE_R_LAYER_BASE: i32 = eTreeStoreElemType::TSE_R_LAYER_BASE as i32;
pub const TSE_R_LAYER: i32 = eTreeStoreElemType::TSE_R_LAYER as i32;
pub const TSE_BONE_COLLECTION_BASE: i32 = eTreeStoreElemType::TSE_BONE_COLLECTION_BASE as i32;
pub const TSE_BONE_COLLECTION: i32 = eTreeStoreElemType::TSE_BONE_COLLECTION as i32;
pub const TSE_STRIP: i32 = eTreeStoreElemType::TSE_STRIP as i32;
pub const TSE_STRIP_DATA: i32 = eTreeStoreElemType::TSE_STRIP_DATA as i32;
pub const TSE_STRIP_DUP: i32 = eTreeStoreElemType::TSE_STRIP_DUP as i32;
pub const TSE_LINKED_PSYS: i32 = eTreeStoreElemType::TSE_LINKED_PSYS as i32;
pub const TSE_RNA_STRUCT: i32 = eTreeStoreElemType::TSE_RNA_STRUCT as i32;
pub const TSE_RNA_PROPERTY: i32 = eTreeStoreElemType::TSE_RNA_PROPERTY as i32;
pub const TSE_RNA_ARRAY_ELEM: i32 = eTreeStoreElemType::TSE_RNA_ARRAY_ELEM as i32;
pub const TSE_NLA_TRACK: i32 = eTreeStoreElemType::TSE_NLA_TRACK as i32;
pub const TSE_ID_BASE: i32 = eTreeStoreElemType::TSE_ID_BASE as i32;
pub const TSE_GP_LAYER: i32 = eTreeStoreElemType::TSE_GP_LAYER as i32;
pub const TSE_LAYER_COLLECTION: i32 = eTreeStoreElemType::TSE_LAYER_COLLECTION as i32;
pub const TSE_SCENE_COLLECTION_BASE: i32 = eTreeStoreElemType::TSE_SCENE_COLLECTION_BASE as i32;
pub const TSE_VIEW_COLLECTION_BASE: i32 = eTreeStoreElemType::TSE_VIEW_COLLECTION_BASE as i32;
pub const TSE_SCENE_OBJECTS_BASE: i32 = eTreeStoreElemType::TSE_SCENE_OBJECTS_BASE as i32;
pub const TSE_GPENCIL_EFFECT_BASE: i32 = eTreeStoreElemType::TSE_GPENCIL_EFFECT_BASE as i32;
pub const TSE_GPENCIL_EFFECT: i32 = eTreeStoreElemType::TSE_GPENCIL_EFFECT as i32;
pub const TSE_LIBRARY_OVERRIDE_BASE: i32 = eTreeStoreElemType::TSE_LIBRARY_OVERRIDE_BASE as i32;
pub const TSE_LIBRARY_OVERRIDE: i32 = eTreeStoreElemType::TSE_LIBRARY_OVERRIDE as i32;
pub const TSE_LIBRARY_OVERRIDE_OPERATION: i32 = eTreeStoreElemType::TSE_LIBRARY_OVERRIDE_OPERATION as i32;
pub const TSE_GENERIC_LABEL: i32 = eTreeStoreElemType::TSE_GENERIC_LABEL as i32;
pub const TSE_GREASE_PENCIL_NODE: i32 = eTreeStoreElemType::TSE_GREASE_PENCIL_NODE as i32;
pub const TSE_LINKED_NODE_TREE: i32 = eTreeStoreElemType::TSE_LINKED_NODE_TREE as i32;
pub const TSE_ACTION_SLOT: i32 = eTreeStoreElemType::TSE_ACTION_SLOT as i32;
pub const TSE_SHAPE_KEY_BLOCK: i32 = eTreeStoreElemType::TSE_SHAPE_KEY_BLOCK as i32;
pub const TSE_SHAPE_KEY_BASE: i32 = eTreeStoreElemType::TSE_SHAPE_KEY_BASE as i32;

