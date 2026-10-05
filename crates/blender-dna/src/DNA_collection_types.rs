//! Auto-transpiled C/C++ header module: DNA_collection_types

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CollectionLightLinking {
    pub link_state: eCollectionLightLinkingState,
    pub _pad: [u8; 3],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CollectionObject {
    pub next: *mut CollectionObject,
    pub ob: *mut Object,
    pub light_linking: CollectionLightLinking,
    pub sort_index: i32,
    pub parented_sort_index: i32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CollectionChild {
    pub next: *mut CollectionChild,
    pub collection: *mut Collection,
    pub light_linking: CollectionLightLinking,
    pub sort_index: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct CollectionImport {
    pub fh_idname: [i8; 64],
    pub import_properties: *mut IDProperty,
    pub flag: u32,
    pub _pad0: u32,
}

impl Default for CollectionImport {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct CollectionExport {
    pub next: *mut CollectionExport,
    pub fh_idname: [i8; 64],
    pub name: [i8; 64],
    pub export_properties: *mut IDProperty,
    pub flag: u32,
    pub _pad0: u32,
    pub layout_panel_states: ListBaseT<LayoutPanelState>,
}

impl Default for CollectionExport {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Collection {
    pub id: ID,
    pub owner_id: *mut ID,
    pub gobject: ListBaseT<CollectionObject>,
    pub children: ListBaseT<CollectionChild>,
    pub _pad0: [i8; 4],
    pub active_exporter_index: i32,
    pub exporters: ListBaseT<CollectionExport>,
    pub importer: *mut CollectionImport,
    pub preview: *mut PreviewImage,
    pub layer: u32,
    pub instance_offset: [f32; 3],
    pub flag: eCollection_Flag,
    pub color_tag: CollectionColorTag,
    pub _pad1: [i8; 2],
    pub lineart_usage: eCollectionLineArt_Usage,
    pub lineart_flags: eCollectionLineArt_Flags,
    pub lineart_intersection_mask: u8,
    pub lineart_intersection_priority: u8,
    pub view_layer: *mut ViewLayer,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CollectionRuntime {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Object {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GHash {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LayoutPanelState {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PreviewImage {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ViewLayer {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOHandlerPanelFlag {
    IO_HANDLER_PANEL_OPEN = 1 << 0,
}

impl Default for IOHandlerPanelFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const IO_HANDLER_PANEL_OPEN: i32 = IOHandlerPanelFlag::IO_HANDLER_PANEL_OPEN as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCollectionLightLinkingState {
    COLLECTION_LIGHT_LINKING_STATE_INCLUDE = 0,
    COLLECTION_LIGHT_LINKING_STATE_EXCLUDE = 1,
}

impl Default for eCollectionLightLinkingState {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const COLLECTION_LIGHT_LINKING_STATE_INCLUDE: i32 = eCollectionLightLinkingState::COLLECTION_LIGHT_LINKING_STATE_INCLUDE as i32;
pub const COLLECTION_LIGHT_LINKING_STATE_EXCLUDE: i32 = eCollectionLightLinkingState::COLLECTION_LIGHT_LINKING_STATE_EXCLUDE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCollectionLineArt_Usage {
    COLLECTION_LRT_INCLUDE = 0,
    COLLECTION_LRT_OCCLUSION_ONLY = (1 << 0),
    COLLECTION_LRT_EXCLUDE = (1 << 1),
    COLLECTION_LRT_INTERSECTION_ONLY = (1 << 2),
    COLLECTION_LRT_NO_INTERSECTION = (1 << 3),
    COLLECTION_LRT_FORCE_INTERSECTION = (1 << 4),
}

impl Default for eCollectionLineArt_Usage {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const COLLECTION_LRT_INCLUDE: i32 = eCollectionLineArt_Usage::COLLECTION_LRT_INCLUDE as i32;
pub const COLLECTION_LRT_OCCLUSION_ONLY: i32 = eCollectionLineArt_Usage::COLLECTION_LRT_OCCLUSION_ONLY as i32;
pub const COLLECTION_LRT_EXCLUDE: i32 = eCollectionLineArt_Usage::COLLECTION_LRT_EXCLUDE as i32;
pub const COLLECTION_LRT_INTERSECTION_ONLY: i32 = eCollectionLineArt_Usage::COLLECTION_LRT_INTERSECTION_ONLY as i32;
pub const COLLECTION_LRT_NO_INTERSECTION: i32 = eCollectionLineArt_Usage::COLLECTION_LRT_NO_INTERSECTION as i32;
pub const COLLECTION_LRT_FORCE_INTERSECTION: i32 = eCollectionLineArt_Usage::COLLECTION_LRT_FORCE_INTERSECTION as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCollectionLineArt_Flags {
    COLLECTION_LRT_USE_INTERSECTION_MASK = (1 << 0),
    COLLECTION_LRT_USE_INTERSECTION_PRIORITY = (1 << 1),
}

impl Default for eCollectionLineArt_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const COLLECTION_LRT_USE_INTERSECTION_MASK: i32 = eCollectionLineArt_Flags::COLLECTION_LRT_USE_INTERSECTION_MASK as i32;
pub const COLLECTION_LRT_USE_INTERSECTION_PRIORITY: i32 = eCollectionLineArt_Flags::COLLECTION_LRT_USE_INTERSECTION_PRIORITY as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCollection_Flag {
    COLLECTION_HIDE_VIEWPORT = (1 << 0),
    COLLECTION_HIDE_SELECT = (1 << 1),
    COLLECTION_HIDE_RENDER = (1 << 3),
    COLLECTION_HAS_OBJECT_CACHE = (1 << 4),
    COLLECTION_IS_MASTER = (1 << 5),
    COLLECTION_HAS_OBJECT_CACHE_INSTANCED = (1 << 6),
}

impl Default for eCollection_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const COLLECTION_HIDE_VIEWPORT: i32 = eCollection_Flag::COLLECTION_HIDE_VIEWPORT as i32;
pub const COLLECTION_HIDE_SELECT: i32 = eCollection_Flag::COLLECTION_HIDE_SELECT as i32;
pub const COLLECTION_HIDE_RENDER: i32 = eCollection_Flag::COLLECTION_HIDE_RENDER as i32;
pub const COLLECTION_HAS_OBJECT_CACHE: i32 = eCollection_Flag::COLLECTION_HAS_OBJECT_CACHE as i32;
pub const COLLECTION_IS_MASTER: i32 = eCollection_Flag::COLLECTION_IS_MASTER as i32;
pub const COLLECTION_HAS_OBJECT_CACHE_INSTANCED: i32 = eCollection_Flag::COLLECTION_HAS_OBJECT_CACHE_INSTANCED as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionColorTag {
    COLLECTION_COLOR_NONE = -1,
    COLLECTION_COLOR_01,
    COLLECTION_COLOR_02,
    COLLECTION_COLOR_03,
    COLLECTION_COLOR_04,
    COLLECTION_COLOR_05,
    COLLECTION_COLOR_06,
    COLLECTION_COLOR_07,
    COLLECTION_COLOR_08,
    COLLECTION_COLOR_TOT,
}

impl Default for CollectionColorTag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const COLLECTION_COLOR_NONE: i32 = CollectionColorTag::COLLECTION_COLOR_NONE as i32;
pub const COLLECTION_COLOR_01: i32 = CollectionColorTag::COLLECTION_COLOR_01 as i32;
pub const COLLECTION_COLOR_02: i32 = CollectionColorTag::COLLECTION_COLOR_02 as i32;
pub const COLLECTION_COLOR_03: i32 = CollectionColorTag::COLLECTION_COLOR_03 as i32;
pub const COLLECTION_COLOR_04: i32 = CollectionColorTag::COLLECTION_COLOR_04 as i32;
pub const COLLECTION_COLOR_05: i32 = CollectionColorTag::COLLECTION_COLOR_05 as i32;
pub const COLLECTION_COLOR_06: i32 = CollectionColorTag::COLLECTION_COLOR_06 as i32;
pub const COLLECTION_COLOR_07: i32 = CollectionColorTag::COLLECTION_COLOR_07 as i32;
pub const COLLECTION_COLOR_08: i32 = CollectionColorTag::COLLECTION_COLOR_08 as i32;
pub const COLLECTION_COLOR_TOT: i32 = CollectionColorTag::COLLECTION_COLOR_TOT as i32;

