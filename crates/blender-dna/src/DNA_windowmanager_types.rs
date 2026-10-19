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
#[allow(non_camel_case_types)]
type int = i32;
#[allow(non_camel_case_types)]
type UString = String;
#[allow(non_camel_case_types)]
type PropertyFlag = u32;
#[allow(non_camel_case_types)]
type PropertyOverrideFlag = u32;
#[allow(non_camel_case_types)]
type ParameterFlag = u32;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eWM_ExtensionsUpdates(pub i32);

impl eWM_ExtensionsUpdates {
    pub const WM_EXTENSIONS_UPDATE_UNSET: Self = Self((-2) as i32);
    pub const WM_EXTENSIONS_UPDATE_CHECKING: Self = Self((-1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eWM_InitFlag(pub u8);

impl eWM_InitFlag {
    pub const WM_INIT_FLAG_WINDOW: Self = Self(((1 << 0)) as u8);
    pub const WM_INIT_FLAG_KEYCONFIG: Self = Self(((1 << 1)) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eWM_OutlinerSyncSelectDirty(pub i16);

impl eWM_OutlinerSyncSelectDirty {
    pub const WM_OUTLINER_SYNC_SELECT_FROM_OBJECT: Self = Self(((1 << 0)) as i16);
    pub const WM_OUTLINER_SYNC_SELECT_FROM_EDIT_BONE: Self = Self(((1 << 1)) as i16);
    pub const WM_OUTLINER_SYNC_SELECT_FROM_POSE_BONE: Self = Self(((1 << 2)) as i16);
    pub const WM_OUTLINER_SYNC_SELECT_FROM_SEQUENCE: Self = Self(((1 << 3)) as i16);
    pub const WM_OUTLINER_SYNC_SELECT_FROM_COLLECTION: Self = Self(((1 << 4)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eKMI_Flag(pub u8);

impl eKMI_Flag {
    pub const KMI_INACTIVE: Self = Self(((1 << 0)) as u8);
    pub const KMI_EXPANDED: Self = Self(((1 << 1)) as u8);
    pub const KMI_USER_MODIFIED: Self = Self(((1 << 2)) as u8);
    pub const KMI_UPDATE: Self = Self(((1 << 3)) as u8);
    pub const KMI_REPEAT_IGNORE: Self = Self(((1 << 4)) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eKMI_MapType(pub u8);

impl eKMI_MapType {
    pub const KMI_TYPE_KEYBOARD: Self = Self((0) as u8);
    pub const KMI_TYPE_MOUSE: Self = Self((1) as u8);
    pub const KMI_TYPE_TEXTINPUT: Self = Self((3) as u8);
    pub const KMI_TYPE_TIMER: Self = Self((4) as u8);
    pub const KMI_TYPE_NDOF: Self = Self((5) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eKeyMap_Flag(pub i16);

impl eKeyMap_Flag {
    pub const KEYMAP_MODAL: Self = Self(((1 << 0)) as i16);
    pub const KEYMAP_USER: Self = Self(((1 << 1)) as i16);
    pub const KEYMAP_EXPANDED: Self = Self(((1 << 2)) as i16);
    pub const KEYMAP_CHILDREN_EXPANDED: Self = Self(((1 << 3)) as i16);
    pub const KEYMAP_DIFF: Self = Self(((1 << 4)) as i16);
    pub const KEYMAP_USER_MODIFIED: Self = Self(((1 << 5)) as i16);
    pub const KEYMAP_UPDATE: Self = Self(((1 << 6)) as i16);
    pub const KEYMAP_TOOL: Self = Self(((1 << 7)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eKeyConf_Flag(pub i16);

impl eKeyConf_Flag {
    pub const KEYCONF_USER: Self = Self(((1 << 1)) as i16);
    pub const KEYCONF_INIT_DEFAULT: Self = Self(((1 << 2)) as i16);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ReportTimerInfo {
    pub widthfac: f32,
    pub flash_progress: f32,
}

impl Default for ReportTimerInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct wmXrData {
    pub runtime: *mut core::ffi::c_void,
    pub session_settings: XrSessionSettings,
}

impl Default for wmXrData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct wmWindowManager {
    pub windows: ListBaseT<wmWindow>,
    pub nullptr: ListBaseT<wmWindow>,
}

impl Default for wmWindowManager {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct wmWindow {
    pub parent: *mut core::ffi::c_void,
    pub scene: *mut core::ffi::c_void,
    pub new_scene: *mut core::ffi::c_void,
    pub view_layer_name: [u8; 64],
    pub unpinned_scene: *mut core::ffi::c_void,
    pub workspace_hook: *mut core::ffi::c_void,
    pub global_areas: ScrAreaMap,
    pub winid: i32,
    pub posx: i16,
    pub posy: i16,
    pub sizex: i16,
    pub sizey: i16,
    pub windowstate: i8,
    pub active: i8,
    pub cursor: i16,
    pub lastcursor: i16,
    pub modalcursor: i16,
    pub grabcursor: i16,
    pub pie_event_type_lock: i16,
    pub pie_event_type_last: i16,
    pub tag_cursor_refresh: i8,
    pub event_queue_check_click: i8,
    pub event_queue_check_drag: i8,
    pub event_queue_check_drag_handled: i8,
    pub event_queue_consecutive_gesture_type: i16,
    pub event_queue_consecutive_gesture_xy: [i32; 2],
}

impl Default for wmWindow {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct wmOperatorTypeMacro {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub idname: [u8; 64],
    pub properties: *mut core::ffi::c_void,
    pub ptr: *mut core::ffi::c_void,
}

impl Default for wmOperatorTypeMacro {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct wmKeyMapItem {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub idname: [u8; 64],
    pub properties: *mut core::ffi::c_void,
    pub propvalue_str: [u8; 64],
    pub propvalue: i16,
    pub r#type: i16,
    pub val: i8,
    pub direction: i8,
    pub shift: i8,
    pub ctrl: i8,
    pub alt: i8,
    pub oskey: i8,
    pub hyper: i8,
    pub _pad0: [u8; 7],
}

impl Default for wmKeyMapItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct wmKeyMapDiffItem {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub remove_item: *mut core::ffi::c_void,
    pub add_item: *mut core::ffi::c_void,
}

impl Default for wmKeyMapDiffItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct wmKeyMap {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub items: ListBaseT<wmKeyMapItem>,
    pub nullptr: ListBaseT<wmKeyMapItem>,
}

impl Default for wmKeyMap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct wmKeyConfigPref {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub idname: [u8; 64],
    pub prop: *mut core::ffi::c_void,
}

impl Default for wmKeyConfigPref {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct wmKeyConfig {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub idname: [u8; 64],
    pub basename: [u8; 64],
    pub keymaps: ListBaseT<wmKeyMap>,
    pub nullptr: ListBaseT<wmKeyMap>,
}

impl Default for wmKeyConfig {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct wmOperator {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub idname: [u8; 64],
    pub properties: *mut core::ffi::c_void,
    pub r#type: *mut core::ffi::c_void,
    pub customdata: *mut core::ffi::c_void,
    pub py_instance: *mut core::ffi::c_void,
    pub ptr: *mut core::ffi::c_void,
    pub reports: *mut core::ffi::c_void,
    pub r#macro: ListBaseT<wmOperator>,
    pub nullptr: ListBaseT<wmOperator>,
}

impl Default for wmOperator {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

