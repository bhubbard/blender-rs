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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eScreen_Flag(pub i16);

impl eScreen_Flag {
    pub const SCREEN_DEPRECATED: Self = Self((1) as i16);
    pub const SCREEN_COLLAPSE_STATUSBAR: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eScreen_State(pub i8);

impl eScreen_State {
    pub const SCREENNORMAL: Self = Self((0) as i8);
    pub const SCREENMAXIMIZED: Self = Self((1) as i8);
    pub const SCREENFULL: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eScreen_Fullscreen_Flag(pub i16);

impl eScreen_Fullscreen_Flag {
    pub const FULLSCREEN_RESTORE_GIZMO_NAVIGATE: Self = Self(((1 << 0)) as i16);
    pub const FULLSCREEN_RESTORE_TEXT: Self = Self(((1 << 1)) as i16);
    pub const FULLSCREEN_RESTORE_STATS: Self = Self(((1 << 2)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eScreen_Redraws_Flag(pub i16);

impl eScreen_Redraws_Flag {
    pub const TIME_REGION: Self = Self(((1 << 0)) as i16);
    pub const TIME_ALL_3D_WIN: Self = Self(((1 << 1)) as i16);
    pub const TIME_ALL_ANIM_WIN: Self = Self(((1 << 2)) as i16);
    pub const TIME_ALL_BUTS_WIN: Self = Self(((1 << 3)) as i16);
    pub const TIME_SEQ: Self = Self(((1 << 5)) as i16);
    pub const TIME_ALL_IMAGE_WIN: Self = Self(((1 << 6)) as i16);
    pub const TIME_NODES: Self = Self(((1 << 8)) as i16);
    pub const TIME_CLIPS: Self = Self(((1 << 9)) as i16);
    pub const TIME_SPREADSHEETS: Self = Self(((1 << 10)) as i16);
    pub const TIME_FOLLOW: Self = Self(9 as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LayoutPanelStateFlag(pub u8);

impl LayoutPanelStateFlag {
    pub const LAYOUT_PANEL_STATE_FLAG_OPEN: Self = Self(((1 << 0)) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct uiPanelDataExpansion(pub i32);

impl uiPanelDataExpansion {
    pub const UI_PANEL_DATA_EXPAND_ROOT: Self = Self(((1 << 0)) as i32);
    pub const UI_SUBPANEL_DATA_EXPAND_1: Self = Self(((1 << 1)) as i32);
    pub const UI_SUBPANEL_DATA_EXPAND_2: Self = Self(((1 << 2)) as i32);
    pub const UI_SUBPANEL_DATA_EXPAND_3: Self = Self(((1 << 3)) as i32);
    pub const UI_SUBPANEL_DATA_EXPAND_4: Self = Self(((1 << 4)) as i32);
    pub const UI_SUBPANEL_DATA_EXPAND_5: Self = Self(((1 << 5)) as i32);
    pub const UI_SUBPANEL_DATA_EXPAND_6: Self = Self(((1 << 6)) as i32);
    pub const UI_SUBPANEL_DATA_EXPAND_7: Self = Self(((1 << 7)) as i32);
    pub const UI_SUBPANEL_DATA_EXPAND_8: Self = Self(((1 << 8)) as i32);
    pub const UI_SUBPANEL_DATA_EXPAND_9: Self = Self(((1 << 9)) as i32);
    pub const UI_SUBPANEL_DATA_EXPAND_10: Self = Self(((1 << 10)) as i32);
    pub const UI_SUBPANEL_DATA_EXPAND_11: Self = Self(((1 << 11)) as i32);
    pub const UI_SUBPANEL_DATA_EXPAND_12: Self = Self(((1 << 12)) as i32);
    pub const UI_SUBPANEL_DATA_EXPAND_13: Self = Self(((1 << 13)) as i32);
    pub const UI_SUBPANEL_DATA_EXPAND_14: Self = Self(((1 << 14)) as i32);
    pub const UI_SUBPANEL_DATA_EXPAND_15: Self = Self(((1 << 15)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePanel_Flag(pub i16);

impl ePanel_Flag {
    pub const PNL_SELECT: Self = Self(((1 << 0)) as i16);
    pub const PNL_UNUSED_1: Self = Self(((1 << 1)) as i16);
    pub const PNL_CLOSED: Self = Self(((1 << 2)) as i16);
    pub const PNL_PIN: Self = Self(((1 << 5)) as i16);
    pub const PNL_POPOVER: Self = Self(((1 << 6)) as i16);
    pub const PNL_INSTANCED_LIST_ORDER_CHANGED: Self = Self(((1 << 7)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct euiList_LayoutType(pub i32);

impl euiList_LayoutType {
    pub const UILST_LAYOUT_DEFAULT: Self = Self((0) as i32);
    pub const UILST_LAYOUT_COMPACT: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct euiList_Flag(pub i32);

impl euiList_Flag {
    pub const UILST_SCROLL_TO_ACTIVE_ITEM: Self = Self((1 << 0) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct euiList_Filter(pub i32);

impl euiList_Filter {
    pub const UILST_FLT_ITEM_NEVER_SHOW: Self = Self(((1 << 16)) as i32);
    pub const UILST_FLT_ITEM: Self = Self((1 << 30) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct euiList_FilterFlag(pub i32);

impl euiList_FilterFlag {
    pub const UILST_FLT_SHOW: Self = Self((1 << 0) as i32);
    pub const UILST_FLT_EXCLUDE: Self = Self(1 as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct euiList_FilterSortFlag(pub i32);

impl euiList_FilterSortFlag {
    pub const UILST_FLT_SORT_ALPHA: Self = Self((1) as i32);
    pub const UILST_FLT_SORT_LOCK: Self = Self(1 as i32);
    pub const UILST_FLT_SORT_REVERSE: Self = Self(2 as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct uiViewStateFlag(pub u16);

impl uiViewStateFlag {
    pub const UI_VIEW_SHOW_FILTER_OPTIONS: Self = Self(((1 << 0)) as u16);
    pub const UI_VIEW_SORT_ALPHA: Self = Self(((1 << 1)) as u16);
    pub const UI_VIEW_FILTER_INVERT: Self = Self(((1 << 2)) as u16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct uiPreviewTag(pub i16);

impl uiPreviewTag {
    pub const UI_PREVIEW_TAG_DIRTY: Self = Self(((1 << 0)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GlobalAreaFlag(pub i16);

impl GlobalAreaFlag {
    pub const GLOBAL_AREA_IS_HIDDEN: Self = Self(((1 << 0)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GlobalAreaAlign(pub i16);

impl GlobalAreaAlign {
    pub const GLOBAL_AREA_ALIGN_TOP: Self = Self((0) as i16);
    pub const GLOBAL_AREA_ALIGN_BOTTOM: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eScrArea_Flag(pub i16);

impl eScrArea_Flag {
    pub const HEADER_NO_PULLDOWN: Self = Self(((1 << 0)) as i16);
    pub const AREA_TEMP_INFO: Self = Self(((1 << 3)) as i16);
    pub const AREA_FLAG_REGION_SIZE_UPDATE: Self = Self(((1 << 3)) as i16);
    pub const AREA_FLAG_ACTIVE_TOOL_UPDATE: Self = Self(((1 << 4)) as i16);
    pub const AREA_FLAG_UNUSED_6: Self = Self(((1 << 6)) as i16);
    pub const AREA_FLAG_STACKED_FULLSCREEN: Self = Self(((1 << 7)) as i16);
    pub const AREA_FLAG_ACTIONZONES_UPDATE: Self = Self(((1 << 8)) as i16);
    pub const AREA_FLAG_OFFSCREEN: Self = Self(((1 << 9)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRegion_Type(pub i16);

impl eRegion_Type {
    pub const RGN_TYPE_WINDOW: Self = Self((0) as i16);
    pub const RGN_TYPE_HEADER: Self = Self((1) as i16);
    pub const RGN_TYPE_CHANNELS: Self = Self((2) as i16);
    pub const RGN_TYPE_TEMPORARY: Self = Self((3) as i16);
    pub const RGN_TYPE_UI: Self = Self((4) as i16);
    pub const RGN_TYPE_TOOLS: Self = Self((5) as i16);
    pub const RGN_TYPE_TOOL_PROPS: Self = Self((6) as i16);
    pub const RGN_TYPE_PREVIEW: Self = Self((7) as i16);
    pub const RGN_TYPE_HUD: Self = Self((8) as i16);
    pub const RGN_TYPE_NAV_BAR: Self = Self((9) as i16);
    pub const RGN_TYPE_EXECUTE: Self = Self((10) as i16);
    pub const RGN_TYPE_FOOTER: Self = Self((11) as i16);
    pub const RGN_TYPE_TOOL_HEADER: Self = Self((12) as i16);
    pub const RGN_TYPE_XR: Self = Self((13) as i16);
    pub const RGN_TYPE_ASSET_SHELF: Self = Self((14) as i16);
    pub const RGN_TYPE_ASSET_SHELF_HEADER: Self = Self((15) as i16);
    pub const RGN_TYPE_SCRUBBING: Self = Self((16) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRegion_Alignment(pub i16);

impl eRegion_Alignment {
    pub const RGN_ALIGN_NONE: Self = Self((0) as i16);
    pub const RGN_ALIGN_TOP: Self = Self((1) as i16);
    pub const RGN_ALIGN_BOTTOM: Self = Self((2) as i16);
    pub const RGN_ALIGN_LEFT: Self = Self((3) as i16);
    pub const RGN_ALIGN_RIGHT: Self = Self((4) as i16);
    pub const RGN_ALIGN_HSPLIT: Self = Self((5) as i16);
    pub const RGN_ALIGN_VSPLIT: Self = Self((6) as i16);
    pub const RGN_ALIGN_FLOAT: Self = Self((7) as i16);
    pub const RGN_ALIGN_QSPLIT: Self = Self((8) as i16);
    pub const RGN_SPLIT_PREV: Self = Self((1 << 5) as i16);
    pub const RGN_SPLIT_SCALE_PREV: Self = Self((1 << 6) as i16);
    pub const RGN_ALIGN_HIDE_WITH_PREV: Self = Self((1 << 7) as i16);
    pub const RGN_STACK_ON_PREV: Self = Self((1 << 8) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRegion_Flag(pub i16);

impl eRegion_Flag {
    pub const RGN_FLAG_HIDDEN: Self = Self(((1 << 0)) as i16);
    pub const RGN_FLAG_TOO_SMALL: Self = Self(((1 << 1)) as i16);
    pub const RGN_FLAG_DYNAMIC_SIZE: Self = Self(((1 << 2)) as i16);
    pub const RGN_FLAG_TEMP_REGIONDATA: Self = Self(((1 << 3)) as i16);
    pub const RGN_FLAG_NO_USER_RESIZE: Self = Self(((1 << 4)) as i16);
    pub const RGN_FLAG_SIZE_CLAMP_X: Self = Self(((1 << 5)) as i16);
    pub const RGN_FLAG_SIZE_CLAMP_Y: Self = Self(((1 << 6)) as i16);
    pub const RGN_FLAG_HIDDEN_BY_USER: Self = Self(((1 << 7)) as i16);
    pub const RGN_FLAG_SEARCH_FILTER_ACTIVE: Self = Self(((1 << 8)) as i16);
    pub const RGN_FLAG_SEARCH_FILTER_UPDATE: Self = Self(((1 << 9)) as i16);
    pub const RGN_FLAG_POLL_FAILED: Self = Self(((1 << 10)) as i16);
    pub const RGN_FLAG_RESIZE_RESPECT_BUTTON_SECTIONS: Self = Self(((1 << 11)) as i16);
    pub const RGN_FLAG_INDICATE_OVERFLOW: Self = Self(((1 << 12)) as i16);
    pub const RGN_FLAG_SEARCH_FILTER_SHOW: Self = Self(((1 << 13)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRegion_DrawFlag(pub i32);

impl eRegion_DrawFlag {
    pub const RGN_DRAW: Self = Self((1) as i32);
    pub const RGN_DRAW_PARTIAL: Self = Self((2) as i32);
    pub const RGN_DRAW_NO_REBUILD: Self = Self((4) as i32);
    pub const RGN_DRAWING: Self = Self((8) as i32);
    pub const RGN_REFRESH_UI: Self = Self((16) as i32);
    pub const RGN_DRAW_EDITOR_OVERLAYS: Self = Self((32) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AssetShelfSettings_DisplayFlag(pub i16);

impl AssetShelfSettings_DisplayFlag {
    pub const ASSETSHELF_SHOW_NAMES: Self = Self(((1 << 0)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AssetShelf_InstanceFlag(pub i16);

impl AssetShelf_InstanceFlag {
    pub const ASSETSHELF_REGION_IS_HIDDEN: Self = Self(((1 << 0)) as i16);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bScreen {
    pub vertbase: ListBaseT<ScrVert>,
    pub nullptr: ListBaseT<ScrVert>,
}

impl Default for bScreen {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ScrVert {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub newv: *mut core::ffi::c_void,
    pub vec: vec2s,
}

impl Default for ScrVert {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ScrEdge {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub v1: *mut core::ffi::c_void,
    pub v2: *mut core::ffi::c_void,
    pub border: i16,
    pub flag: i16,
    pub _pad: [u8; 4],
}

impl Default for ScrEdge {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ScrAreaMap {
    pub vertbase: ListBaseT<ScrVert>,
    pub nullptr: ListBaseT<ScrVert>,
}

impl Default for ScrAreaMap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct LayoutPanelState {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub idname: *mut core::ffi::c_void,
    pub flag: LayoutPanelStateFlag,
}

impl Default for LayoutPanelState {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Panel {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub r#type: *mut core::ffi::c_void,
    pub layout: *mut core::ffi::c_void,
    pub panelname: [u8; 64],
    pub drawname: *mut core::ffi::c_void,
    pub ofsx: i32,
    pub ofsy: i32,
    pub sizex: i32,
    pub sizey: i32,
    pub blocksizex: i32,
    pub blocksizey: i32,
    pub labelofs: i16,
    pub flag: ePanel_Flag,
}

impl Default for Panel {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct PanelCategoryDyn {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub idname: [u8; 64],
    pub icon: i32,
}

impl Default for PanelCategoryDyn {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct PanelCategoryStack {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub idname: [u8; 64],
}

impl Default for PanelCategoryStack {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct uiListDyn {
    pub free_runtime_data_fn: uiListFreeRuntimeDataFunc,
}

impl Default for uiListDyn {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct uiList {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub r#type: *mut core::ffi::c_void,
    pub list_id: [u8; 256],
    pub layout_type: euiList_LayoutType,
    pub flag: euiList_Flag,
}

impl Default for uiList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct uiViewState {
    pub custom_height: i32,
    pub scroll_offset: i32,
    pub flag: uiViewStateFlag,
}

impl Default for uiViewState {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct uiViewStateLink {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub idname: [u8; 64],
    pub state: uiViewState,
}

impl Default for uiViewStateLink {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct TransformOrientation {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub name: [u8; 64],
    pub mat: [[f32; 3]; 3],
}

impl Default for TransformOrientation {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct uiPreview {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub preview_id: [u8; 64],
    pub height: i16,
    pub tag: uiPreviewTag,
}

impl Default for uiPreview {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct TextboxState {
    pub visible_lines: i32,
    pub scroll: i32,
}

impl Default for TextboxState {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct uiTextboxStateLink {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub idname: *mut core::ffi::c_void,
    pub state: TextboxState,
}

impl Default for uiTextboxStateLink {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ScrGlobalAreaData {
    pub cur_fixed_height: i16,
    pub size_min: i16,
    pub size_max: i16,
    pub align: GlobalAreaAlign,
    pub flag: GlobalAreaFlag,
}

impl Default for ScrGlobalAreaData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ScrArea_Runtime {
    pub tool: *mut core::ffi::c_void,
    pub is_tool_set: i8,
    pub _pad0: [u8; 7],
}

impl Default for ScrArea_Runtime {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ScrArea {
    pub v1: *mut core::ffi::c_void,
    pub v2: *mut core::ffi::c_void,
    pub v3: *mut core::ffi::c_void,
    pub v4: *mut core::ffi::c_void,
    pub full: *mut core::ffi::c_void,
    pub totrct: rcti,
}

impl Default for ScrArea {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ARegion {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub v2d: View2D,
    pub winrct: rcti,
}

impl Default for ARegion {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct AssetShelfSettings {
    pub asset_library_reference: AssetLibraryReference,
    pub enabled_catalog_paths: ListBaseT<AssetCatalogPathLink>,
    pub nullptr: ListBaseT<AssetCatalogPathLink>,
}

impl Default for AssetShelfSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct AssetShelf {
    pub idname: [u8; 64],
    pub r#type: *mut core::ffi::c_void,
    pub settings: AssetShelfSettings,
    pub preferred_row_count: i16,
    pub instance_flag: AssetShelf_InstanceFlag,
}

impl Default for AssetShelf {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct RegionAssetShelf {
    pub shelves: ListBaseT<AssetShelf>,
    pub nullptr: ListBaseT<AssetShelf>,
}

impl Default for RegionAssetShelf {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct FileHandler {

}

impl Default for FileHandler {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

