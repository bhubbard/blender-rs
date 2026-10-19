//! Auto-transpiled C/C++ header module: DNA_userdef_types

use crate::*;
pub use crate::DNA_colorband_types::ColorBand;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bAddon {
    pub next: *mut bAddon,
    pub module: [i8; 128],
    pub prop: *mut IDProperty,
}

impl Default for bAddon {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bPathCompare {
    pub next: *mut bPathCompare,
    pub path: [i8; 768],
    pub flag: ePathCompare_Flag,
    pub _pad0: [i8; 7],
}

impl Default for bPathCompare {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bUserMenu {
    pub next: *mut bUserMenu,
    pub space_type: i8,
    pub _pad0: [i8; 7],
    pub context: [i8; 64],
    pub items: ListBaseT<bUserMenuItem>,
}

impl Default for bUserMenu {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bUserMenuItem {
    pub next: *mut bUserMenuItem,
    pub ui_name: [i8; 64],
    pub r#type: eUserMenu_Type,
    pub _pad0: [i8; 7],
}

impl Default for bUserMenuItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bUserMenuItem_Op {
    pub item: bUserMenuItem,
    pub op_idname: [i8; 64],
    pub prop: *mut IDProperty,
    pub op_prop_enum: [i8; 64],
    pub opcontext: i8,
    pub _pad0: [i8; 7],
}

impl Default for bUserMenuItem_Op {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bUserMenuItem_Menu {
    pub item: bUserMenuItem,
    pub mt_idname: [i8; 64],
}

impl Default for bUserMenuItem_Menu {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bUserMenuItem_Prop {
    pub item: bUserMenuItem,
    pub context_data_path: [i8; 256],
    pub prop_id: [i8; 64],
    pub prop_index: i32,
    pub _pad0: [i8; 4],
}

impl Default for bUserMenuItem_Prop {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bUserAssetLibrary {
    pub next: *mut bUserAssetLibrary,
    pub name: [i8; 64],
    pub dirpath: [i8; 1024],
    pub resolved_dirpath: [i8; 1024],
    pub remote_url: [i8; 1024],
    pub auth_token: *mut i8,
    pub uuid: bUUID,
    pub invalid_uuid: *mut i8,
    pub import_method: i16,
    pub flag: i16,
    pub _pad0: [i8; 4],
}

impl Default for bUserAssetLibrary {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bUserExtensionRepo {
    pub next: *mut bUserExtensionRepo,
    pub name: [i8; 64],
    pub module: [i8; 48],
    pub access_token: *mut i8,
    pub custom_dirpath: [i8; 1024],
    pub remote_url: [i8; 1024],
    pub flag: eUserExtensionRepo_Flag,
    pub source: eUserExtensionRepo_Source,
    pub _pad0: [i8; 6],
}

impl Default for bUserExtensionRepo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SolidLight {
    pub flag: i32,
    pub smooth: f32,
    pub col: [f32; 4],
    pub spec: [f32; 4],
    pub vec: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WalkNavigation {
    pub mouse_speed: f32,
    pub walk_speed: f32,
    pub walk_speed_factor: f32,
    pub view_height: f32,
    pub jump_height: f32,
    pub teleport_time: f32,
    pub flag: eWalkNavigation_Flag,
    pub _pad0: [i8; 6],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct XrNavigation {
    pub vignette_intensity: f32,
    pub flag: eUserpref_XrNavigationFlags,
    pub _pad0: [i8; 2],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct UserDef_Runtime {
    pub is_dirty: i8,
    pub _pad0: [i8; 7],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct UserDef_SpaceData {
    pub section_active: eUserPref_Section,
    pub flag: eUserPref_SpaceData_Flag,
    pub _pad0: [i8; 6],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct UserDef_FileSpaceData {
    pub display_type: eFileDisplayType,
    pub sort_type: eFileSortType,
    pub thumbnail_size: i32,
    pub _pad: [i8; 5],
    pub flag: eFileSel_Params_Flag,
    pub filter_id: u64,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct UserDef_TempWinBounds {
    pub file: rctf,
    pub userpref: rctf,
    pub image: rctf,
    pub graph: rctf,
    pub info: rctf,
    pub project: rctf,
    pub outliner: rctf,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct UserDef_Experimental {
    pub use_undo_legacy: i8,
    pub no_override_auto_resync: i8,
    pub use_cycles_debug: i8,
    pub use_eevee_debug: i8,
    pub show_asset_debug_info: i8,
    pub no_asset_indexing: i8,
    pub use_viewport_debug: i8,
    pub use_all_linked_data_direct: i8,
    pub use_extensions_debug: i8,
    pub use_recompute_usercount_on_save_debug: i8,
    pub write_legacy_blend_file_format: i8,
    pub no_data_block_packing: i8,
    pub use_paint_debug: i8,
    pub SANITIZE_AFTER_HERE: i8,
    pub use_new_curves_tools: i8,
    pub use_extended_asset_browser: i8,
    pub use_3d_texture_paint: i8,
    pub use_shader_node_previews: i8,
    pub use_remote_asset_libraries: i8,
    pub use_collection_importer: i8,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bUserScriptDirectory {
    pub next: *mut bUserScriptDirectory,
    pub name: [i8; 64],
    pub dir_path: [i8; 768],
}

impl Default for bUserScriptDirectory {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bUserAssetShelfSettings {
    pub next: *mut bUserAssetShelfSettings,
    pub shelf_idname: [i8; 64],
    pub enabled_catalog_paths: ListBaseT<AssetCatalogPathLink>,
}

impl Default for bUserAssetShelfSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct UserDef {
    pub versionfile: i32,
    pub subversionfile: i32,
    pub flag: eUserPref_Flag,
    pub dupflag: u32,
    pub pref_flag: eUserPref_PrefFlag,
    pub savetime: i8,
    pub mouse_emulate_3_button_modifier: eUserpref_EmulateMMBMod,
    pub trackpad_scroll_direction: eUserpref_TrackpadScrollDir,
    pub tempdir: [i8; 768],
    pub fontdir: [i8; 768],
    pub renderdir: [i8; 1024],
    pub render_cachedir: [i8; 768],
    pub textudir: [i8; 768],
    pub texture_cachedir: [i8; 768],
    pub pythondir_legacy: [i8; 768],
    pub sounddir: [i8; 768],
    pub i18ndir: [i8; 768],
    pub image_editor: [i8; 1024],
    pub text_editor: [i8; 1024],
    pub text_editor_args: [i8; 256],
    pub anim_player: [i8; 1024],
    pub anim_player_preset: i32,
    pub v2d_min_gridsize: i16,
    pub timecode_style: eTimecodeStyles,
    pub versions: i16,
    pub dbl_click_time: i16,
    pub asset_access: AssetAccess,
    pub _pad0: i8,
    pub border_width: i8,
    pub mini_axis_type: eUserpref_MiniAxisType,
    pub uiflag2: eUserpref_UI_Flag2,
    pub date_format: i8,
    pub time_format: i8,
    pub _pad8: [i8; 4],
    pub app_flag: eUserpref_APP_Flag,
    pub viewzoom: eViewZoom_Style,
    pub language: eUserpref_Language,
    pub mixbufsize: i32,
    pub audiodevice: i32,
    pub audiorate: i32,
    pub audioformat: i32,
    pub audiochannels: i32,
    pub ui_scale: f32,
    pub ui_line_width: i32,
    pub dpi: i32,
    pub scale_factor: f32,
    pub inv_scale_factor: f32,
    pub pixelsize: f32,
    pub virtual_pixel: eUserpref_VirtualPixel,
    pub scrollback: i32,
    pub node_margin: i8,
    pub node_preview_res: i8,
    pub menuthreshold1: i16,
    pub app_template: [i8; 64],
    pub themes: ListBaseT<bTheme>,
    pub uifonts: ListBaseT<uiFont>,
    pub uistyles: ListBaseT<uiStyle>,
    pub user_keymaps: ListBaseT<wmKeyMap>,
    pub user_keyconfig_prefs: ListBaseT<wmKeyConfigPref>,
    pub addons: ListBaseT<bAddon>,
    pub autoexec_paths: ListBaseT<bPathCompare>,
    pub script_directories: ListBaseT<bUserScriptDirectory>,
    pub user_menus: ListBaseT<bUserMenu>,
    pub asset_libraries: ListBaseT<bUserAssetLibrary>,
    pub extension_repos: ListBaseT<bUserExtensionRepo>,
    pub asset_shelves_settings: ListBaseT<bUserAssetShelfSettings>,
    pub keyconfigstr: [i8; 64],
    pub active_extension_repo: i16,
    pub extension_flag: eUserPref_ExtensionFlag,
    pub network_timeout: u8,
    pub network_connection_limit: u8,
    pub _pad2: [i8; 1],
    pub active_asset_library: i16,
    pub asset_flag: eUserPref_AssetFlag,
    pub _pad14: [i8; 1],
    pub undosteps: i16,
    pub undomemory: i32,
    pub gpu_viewport_quality: f32,
    pub gp_manhattandist: i16,
    pub gp_settings: eGP_UserdefSettings,
    pub _pad13: [i8; 4],
    pub light_param: [SolidLight; 4],
    pub light_ambient: [f32; 3],
    pub gizmo_flag: eUserpref_Gizmo_Flag,
    pub gizmo_size: i8,
    pub gizmo_size_navigate_v3d: i8,
    pub _pad3: [i8; 5],
    pub edit_studio_light: i16,
    pub lookdev_sphere_size: i16,
    pub vbotimeout: i16,
    pub textimeout: i16,
    pub memcachelimit: i32,
    pub nodes_stack_limit: i32,
    pub prefetchframes: i32,
    pub pad_rot_angle: f32,
    pub rvisize: i16,
    pub rvibright: i16,
    pub recent_files: i16,
    pub smooth_viewtx: i16,
    pub glreslimit: i16,
    pub color_picker_type: eColorPicker_Types,
    pub auto_smoothing_new: eFCurve_Smoothing,
    pub ipo_new: eBezTriple_Interpolation,
    pub keyhandles_new: eBezTriple_Handle,
    pub _pad11: [i8; 4],
    pub view_frame_type: eZoomFrame_Mode,
    pub view_frame_keyframes: i32,
    pub view_frame_seconds: f32,
    pub gpu_preferred_index: i32,
    pub gpu_preferred_vendor_id: u32,
    pub gpu_preferred_device_id: u32,
    pub gpu_shader_workers: i16,
    pub shader_compilation_method: eUserpref_ShaderCompileMethod,
    pub _pad16: [i8; 2],
    pub gpu_backend: eUserPref_GPUBackendType,
    pub playback_fps_samples: i16,
    pub widget_unit: i16,
    pub anisotropic_filter: i16,
    pub tablet_api: eUserpref_TabletAPI,
    pub pressure_threshold_max: f32,
    pub pressure_softness: f32,
    pub tablet_flag: eUserPref_Tablet_Flags,
    pub ndof_translation_sensitivity: f32,
    pub ndof_rotation_sensitivity: f32,
    pub ndof_deadzone: f32,
    pub ndof_flag: eNdof_Flag,
    pub ndof_navigation_mode: eNdof_Navigation_Mode,
    pub _pad17: [i8; 3],
    pub glalphaclip: f32,
    pub _pad15: [i8; 2],
    pub animation_flag: eUserpref_Anim_Flags,
    pub text_render: eText_Draw_Options,
    pub navigation_mode: eViewNavigation_Method,
    pub view_rotate_sensitivity_trackball: f32,
    pub coba_weight: ColorBand,
    pub sculpt_paint_overlay_col: [f32; 3],
    pub gpencil_new_layer_col: [f32; 4],
    pub drag_threshold_mouse: i8,
    pub drag_threshold_tablet: i8,
    pub drag_threshold: i8,
    pub move_threshold: i8,
    pub font_path_ui: [i8; 1024],
    pub font_path_ui_mono: [i8; 1024],
    pub compute_device_type: i32,
    pub fcu_inactive_alpha: f32,
    pub pie_tap_timeout: i16,
    pub pie_initial_timeout: i16,
    pub pie_animation_timeout: i16,
    pub pie_menu_confirm: i16,
    pub pie_menu_radius: i16,
    pub pie_menu_threshold: i16,
    pub sequencer_editor_flag: eUserpref_SeqEditorFlags,
    pub factor_display_type: eUserpref_FactorDisplay,
    pub viewport_aa: i8,
    pub render_display_type: eUserpref_RenderDisplayType,
    pub filebrowser_display_type: eUserpref_TempSpaceDisplayType,
    pub preferences_display_type: eUserpref_TempSpaceDisplayType,
    pub _pad18: [i8; 3],
    pub sequencer_default_strip_length: f32,
    pub sequencer_proxy_setup: eUserpref_SeqProxySetup,
    pub _pad1: i16,
    pub collection_instance_empty_size: f32,
    pub text_flag: eTextEdit_Flags,
    pub save_modified_images: eUserpref_Save_Modified_Images,
    pub file_preview_type: eUserpref_File_Preview_Type,
    pub statusbar_flag: eUserpref_StatusBar_Flag,
    pub walk_navigation: WalkNavigation,
    pub xr_navigation: XrNavigation,
    pub space_data: UserDef_SpaceData,
    pub file_space_data: UserDef_FileSpaceData,
    pub stored_bounds: UserDef_TempWinBounds,
    pub experimental: UserDef_Experimental,
    pub runtime: UserDef_Runtime,
}

impl Default for UserDef {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}


#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserPref_Flag {
    USER_AUTOSAVE = (1 << 0),
    USER_FLAG_NUMINPUT_ADVANCED = (1 << 1),
    USER_FLAG_RECENT_SEARCHES_DISABLE = (1 << 2),
    USER_MENU_CLOSE_LEAVE = (1 << 3),
    USER_FLAG_UNUSED_4 = (1 << 4),
    USER_TRACKBALL = (1 << 5),
    USER_FLAG_UNUSED_6 = (1 << 6),
    USER_FLAG_UNUSED_7 = (1 << 7),
    USER_MAT_ON_OB = (1 << 8),
    USER_INTERNET_ALLOW = (1 << 9),
    USER_DEVELOPER_UI = (1 << 10),
    USER_TOOLTIPS = (1 << 11),
    USER_TWOBUTTONMOUSE = (1 << 12),
    USER_NONUMPAD = (1 << 13),
    USER_ADD_CURSORALIGNED = (1 << 14),
    USER_FILECOMPRESS = (1 << 15),
    USER_FLAG_UNUSED_5 = (1 << 16),
    USER_CUSTOM_RANGE = (1 << 17),
    USER_ADD_EDITMODE = (1 << 18),
    USER_ADD_VIEWALIGNED = (1 << 19),
    USER_RELPATHS = (1 << 20),
    USER_RELEASECONFIRM = (1 << 21),
    USER_SCRIPT_AUTOEXEC_DISABLE = (1 << 22),
    USER_FILENOUI = (1 << 23),
    USER_NONEGFRAMES = (1 << 24),
    USER_TXT_TABSTOSPACES_DISABLE = (1 << 25),
    USER_TOOLTIPS_PYTHON = (1 << 26),
    USER_FLAG_UNUSED_27 = (1 << 27),
    USER_HIDE_DOT_DATABLOCK = (1 << 28),
}

impl Default for eUserPref_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_AUTOSAVE: i32 = eUserPref_Flag::USER_AUTOSAVE as i32;
pub const USER_FLAG_NUMINPUT_ADVANCED: i32 = eUserPref_Flag::USER_FLAG_NUMINPUT_ADVANCED as i32;
pub const USER_FLAG_RECENT_SEARCHES_DISABLE: i32 = eUserPref_Flag::USER_FLAG_RECENT_SEARCHES_DISABLE as i32;
pub const USER_MENU_CLOSE_LEAVE: i32 = eUserPref_Flag::USER_MENU_CLOSE_LEAVE as i32;
pub const USER_FLAG_UNUSED_4: i32 = eUserPref_Flag::USER_FLAG_UNUSED_4 as i32;
pub const USER_TRACKBALL: i32 = eUserPref_Flag::USER_TRACKBALL as i32;
pub const USER_FLAG_UNUSED_6: i32 = eUserPref_Flag::USER_FLAG_UNUSED_6 as i32;
pub const USER_FLAG_UNUSED_7: i32 = eUserPref_Flag::USER_FLAG_UNUSED_7 as i32;
pub const USER_MAT_ON_OB: i32 = eUserPref_Flag::USER_MAT_ON_OB as i32;
pub const USER_INTERNET_ALLOW: i32 = eUserPref_Flag::USER_INTERNET_ALLOW as i32;
pub const USER_DEVELOPER_UI: i32 = eUserPref_Flag::USER_DEVELOPER_UI as i32;
pub const USER_TOOLTIPS: i32 = eUserPref_Flag::USER_TOOLTIPS as i32;
pub const USER_TWOBUTTONMOUSE: i32 = eUserPref_Flag::USER_TWOBUTTONMOUSE as i32;
pub const USER_NONUMPAD: i32 = eUserPref_Flag::USER_NONUMPAD as i32;
pub const USER_ADD_CURSORALIGNED: i32 = eUserPref_Flag::USER_ADD_CURSORALIGNED as i32;
pub const USER_FILECOMPRESS: i32 = eUserPref_Flag::USER_FILECOMPRESS as i32;
pub const USER_FLAG_UNUSED_5: i32 = eUserPref_Flag::USER_FLAG_UNUSED_5 as i32;
pub const USER_CUSTOM_RANGE: i32 = eUserPref_Flag::USER_CUSTOM_RANGE as i32;
pub const USER_ADD_EDITMODE: i32 = eUserPref_Flag::USER_ADD_EDITMODE as i32;
pub const USER_ADD_VIEWALIGNED: i32 = eUserPref_Flag::USER_ADD_VIEWALIGNED as i32;
pub const USER_RELPATHS: i32 = eUserPref_Flag::USER_RELPATHS as i32;
pub const USER_RELEASECONFIRM: i32 = eUserPref_Flag::USER_RELEASECONFIRM as i32;
pub const USER_SCRIPT_AUTOEXEC_DISABLE: i32 = eUserPref_Flag::USER_SCRIPT_AUTOEXEC_DISABLE as i32;
pub const USER_FILENOUI: i32 = eUserPref_Flag::USER_FILENOUI as i32;
pub const USER_NONEGFRAMES: i32 = eUserPref_Flag::USER_NONEGFRAMES as i32;
pub const USER_TXT_TABSTOSPACES_DISABLE: i32 = eUserPref_Flag::USER_TXT_TABSTOSPACES_DISABLE as i32;
pub const USER_TOOLTIPS_PYTHON: i32 = eUserPref_Flag::USER_TOOLTIPS_PYTHON as i32;
pub const USER_FLAG_UNUSED_27: i32 = eUserPref_Flag::USER_FLAG_UNUSED_27 as i32;
pub const USER_HIDE_DOT_DATABLOCK: i32 = eUserPref_Flag::USER_HIDE_DOT_DATABLOCK as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserPref_AssetFlag {
    USER_ASSETS_USE_ONLINE_ESSENTIALS = 1 << 0,
}

impl Default for eUserPref_AssetFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_ASSETS_USE_ONLINE_ESSENTIALS: i32 = eUserPref_AssetFlag::USER_ASSETS_USE_ONLINE_ESSENTIALS as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserPref_ExtensionFlag {
    USER_EXTENSION_FLAG_ONLINE_ACCESS_HANDLED = 1 << 0,
}

impl Default for eUserPref_ExtensionFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_EXTENSION_FLAG_ONLINE_ACCESS_HANDLED: i32 = eUserPref_ExtensionFlag::USER_EXTENSION_FLAG_ONLINE_ACCESS_HANDLED as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_File_Preview_Type {
    USER_FILE_PREVIEW_NONE = 0,
    USER_FILE_PREVIEW_AUTO,
    USER_FILE_PREVIEW_SCREENSHOT,
    USER_FILE_PREVIEW_CAMERA,
}

impl Default for eUserpref_File_Preview_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_FILE_PREVIEW_NONE: i32 = eUserpref_File_Preview_Type::USER_FILE_PREVIEW_NONE as i32;
pub const USER_FILE_PREVIEW_AUTO: i32 = eUserpref_File_Preview_Type::USER_FILE_PREVIEW_AUTO as i32;
pub const USER_FILE_PREVIEW_SCREENSHOT: i32 = eUserpref_File_Preview_Type::USER_FILE_PREVIEW_SCREENSHOT as i32;
pub const USER_FILE_PREVIEW_CAMERA: i32 = eUserpref_File_Preview_Type::USER_FILE_PREVIEW_CAMERA as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_Save_Modified_Images {
    USER_SAVE_MODIFIED_IMAGES_ASK = 0,
    USER_SAVE_MODIFIED_IMAGES_ALWAYS,
    USER_SAVE_MODIFIED_IMAGES_NEVER,
}

impl Default for eUserpref_Save_Modified_Images {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_SAVE_MODIFIED_IMAGES_ASK: i32 = eUserpref_Save_Modified_Images::USER_SAVE_MODIFIED_IMAGES_ASK as i32;
pub const USER_SAVE_MODIFIED_IMAGES_ALWAYS: i32 = eUserpref_Save_Modified_Images::USER_SAVE_MODIFIED_IMAGES_ALWAYS as i32;
pub const USER_SAVE_MODIFIED_IMAGES_NEVER: i32 = eUserpref_Save_Modified_Images::USER_SAVE_MODIFIED_IMAGES_NEVER as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserPref_PrefFlag {
    USER_PREF_FLAG_SAVE = (1 << 0),
    USER_PREF_FLAG_PROJECT_SAVE = (1 << 1),
}

impl Default for eUserPref_PrefFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_PREF_FLAG_SAVE: i32 = eUserPref_PrefFlag::USER_PREF_FLAG_SAVE as i32;
pub const USER_PREF_FLAG_PROJECT_SAVE: i32 = eUserPref_PrefFlag::USER_PREF_FLAG_PROJECT_SAVE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eViewZoom_Style {
    USER_ZOOM_CONTINUE = 0,
    USER_ZOOM_SCALE = 1,
    USER_ZOOM_DOLLY = 2,
}

impl Default for eViewZoom_Style {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_ZOOM_CONTINUE: i32 = eViewZoom_Style::USER_ZOOM_CONTINUE as i32;
pub const USER_ZOOM_SCALE: i32 = eViewZoom_Style::USER_ZOOM_SCALE as i32;
pub const USER_ZOOM_DOLLY: i32 = eViewZoom_Style::USER_ZOOM_DOLLY as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eViewNavigation_Method {
    VIEW_NAVIGATION_WALK = 0,
    VIEW_NAVIGATION_FLY = 1,
}

impl Default for eViewNavigation_Method {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const VIEW_NAVIGATION_WALK: i32 = eViewNavigation_Method::VIEW_NAVIGATION_WALK as i32;
pub const VIEW_NAVIGATION_FLY: i32 = eViewNavigation_Method::VIEW_NAVIGATION_FLY as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_MiniAxisType {
    USER_MINI_AXIS_TYPE_GIZMO = 0,
    USER_MINI_AXIS_TYPE_MINIMAL = 1,
    USER_MINI_AXIS_TYPE_NONE = 2,
}

impl Default for eUserpref_MiniAxisType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_MINI_AXIS_TYPE_GIZMO: i32 = eUserpref_MiniAxisType::USER_MINI_AXIS_TYPE_GIZMO as i32;
pub const USER_MINI_AXIS_TYPE_MINIMAL: i32 = eUserpref_MiniAxisType::USER_MINI_AXIS_TYPE_MINIMAL as i32;
pub const USER_MINI_AXIS_TYPE_NONE: i32 = eUserpref_MiniAxisType::USER_MINI_AXIS_TYPE_NONE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eWalkNavigation_Flag {
    USER_WALK_GRAVITY = (1 << 0),
    USER_WALK_MOUSE_REVERSE = (1 << 1),
}

impl Default for eWalkNavigation_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_WALK_GRAVITY: i32 = eWalkNavigation_Flag::USER_WALK_GRAVITY as i32;
pub const USER_WALK_MOUSE_REVERSE: i32 = eWalkNavigation_Flag::USER_WALK_MOUSE_REVERSE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_UI_Flag {
    USER_NO_MULTITOUCH_GESTURES = (1 << 0),
    USER_REDUCE_MOTION = (1 << 1),
    USER_WHEELZOOMDIR = (1 << 2),
    USER_FILTERFILEEXTS = (1 << 3),
    USER_DRAWVIEWINFO = (1 << 4),
    USER_PLAINMENUS = (1 << 5),
    USER_LOCK_CURSOR_ADJUST = (1 << 6),
    USER_HEADER_BOTTOM = (1 << 7),
    USER_HEADER_FROM_PREF = (1 << 8),
    USER_MENUOPENAUTO = (1 << 9),
    USER_DEPTH_CURSOR = (1 << 10),
    USER_AUTOPERSP = (1 << 11),
    USER_NODE_AUTO_OFFSET = (1 << 12),
    USER_GLOBALUNDO = (1 << 13),
    USER_ORBIT_SELECTION = (1 << 14),
    USER_DEPTH_NAVIGATE = (1 << 15),
    USER_HIDE_DOT = (1 << 16),
    USER_SHOW_GIZMO_NAVIGATE = (1 << 17),
    USER_SHOW_VIEWPORTNAME = (1 << 18),
    USER_AREA_CORNER_HANDLE = (1 << 19),
    USER_ZOOM_TO_MOUSEPOS = (1 << 20),
    USER_SHOW_FPS = (1 << 21),
    USER_REGISTER_ALL_USERS = (1 << 22),
    USER_FILTER_BRUSHES_BY_TOOL = (1 << 23),
    USER_CONTINUOUS_MOUSE = (1 << 24),
    USER_ZOOM_INVERT = (1 << 25),
    USER_ZOOM_HORIZ = (1 << 26),
    USER_SPLASH_DISABLE = (1 << 27),
    USER_HIDE_RECENT = (1 << 28),
    USER_SHOW_THUMBNAILS = (1 << 29),
    USER_SAVE_PROMPT = (1 << 30),
    USER_HIDE_SYSTEM_BOOKMARKS = (i32::MIN),
}

impl Default for eUserpref_UI_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_NO_MULTITOUCH_GESTURES: i32 = eUserpref_UI_Flag::USER_NO_MULTITOUCH_GESTURES as i32;
pub const USER_REDUCE_MOTION: i32 = eUserpref_UI_Flag::USER_REDUCE_MOTION as i32;
pub const USER_WHEELZOOMDIR: i32 = eUserpref_UI_Flag::USER_WHEELZOOMDIR as i32;
pub const USER_FILTERFILEEXTS: i32 = eUserpref_UI_Flag::USER_FILTERFILEEXTS as i32;
pub const USER_DRAWVIEWINFO: i32 = eUserpref_UI_Flag::USER_DRAWVIEWINFO as i32;
pub const USER_PLAINMENUS: i32 = eUserpref_UI_Flag::USER_PLAINMENUS as i32;
pub const USER_LOCK_CURSOR_ADJUST: i32 = eUserpref_UI_Flag::USER_LOCK_CURSOR_ADJUST as i32;
pub const USER_HEADER_BOTTOM: i32 = eUserpref_UI_Flag::USER_HEADER_BOTTOM as i32;
pub const USER_HEADER_FROM_PREF: i32 = eUserpref_UI_Flag::USER_HEADER_FROM_PREF as i32;
pub const USER_MENUOPENAUTO: i32 = eUserpref_UI_Flag::USER_MENUOPENAUTO as i32;
pub const USER_DEPTH_CURSOR: i32 = eUserpref_UI_Flag::USER_DEPTH_CURSOR as i32;
pub const USER_AUTOPERSP: i32 = eUserpref_UI_Flag::USER_AUTOPERSP as i32;
pub const USER_NODE_AUTO_OFFSET: i32 = eUserpref_UI_Flag::USER_NODE_AUTO_OFFSET as i32;
pub const USER_GLOBALUNDO: i32 = eUserpref_UI_Flag::USER_GLOBALUNDO as i32;
pub const USER_ORBIT_SELECTION: i32 = eUserpref_UI_Flag::USER_ORBIT_SELECTION as i32;
pub const USER_DEPTH_NAVIGATE: i32 = eUserpref_UI_Flag::USER_DEPTH_NAVIGATE as i32;
pub const USER_HIDE_DOT: i32 = eUserpref_UI_Flag::USER_HIDE_DOT as i32;
pub const USER_SHOW_GIZMO_NAVIGATE: i32 = eUserpref_UI_Flag::USER_SHOW_GIZMO_NAVIGATE as i32;
pub const USER_SHOW_VIEWPORTNAME: i32 = eUserpref_UI_Flag::USER_SHOW_VIEWPORTNAME as i32;
pub const USER_AREA_CORNER_HANDLE: i32 = eUserpref_UI_Flag::USER_AREA_CORNER_HANDLE as i32;
pub const USER_ZOOM_TO_MOUSEPOS: i32 = eUserpref_UI_Flag::USER_ZOOM_TO_MOUSEPOS as i32;
pub const USER_SHOW_FPS: i32 = eUserpref_UI_Flag::USER_SHOW_FPS as i32;
pub const USER_REGISTER_ALL_USERS: i32 = eUserpref_UI_Flag::USER_REGISTER_ALL_USERS as i32;
pub const USER_FILTER_BRUSHES_BY_TOOL: i32 = eUserpref_UI_Flag::USER_FILTER_BRUSHES_BY_TOOL as i32;
pub const USER_CONTINUOUS_MOUSE: i32 = eUserpref_UI_Flag::USER_CONTINUOUS_MOUSE as i32;
pub const USER_ZOOM_INVERT: i32 = eUserpref_UI_Flag::USER_ZOOM_INVERT as i32;
pub const USER_ZOOM_HORIZ: i32 = eUserpref_UI_Flag::USER_ZOOM_HORIZ as i32;
pub const USER_SPLASH_DISABLE: i32 = eUserpref_UI_Flag::USER_SPLASH_DISABLE as i32;
pub const USER_HIDE_RECENT: i32 = eUserpref_UI_Flag::USER_HIDE_RECENT as i32;
pub const USER_SHOW_THUMBNAILS: i32 = eUserpref_UI_Flag::USER_SHOW_THUMBNAILS as i32;
pub const USER_SAVE_PROMPT: i32 = eUserpref_UI_Flag::USER_SAVE_PROMPT as i32;
pub const USER_HIDE_SYSTEM_BOOKMARKS: i32 = eUserpref_UI_Flag::USER_HIDE_SYSTEM_BOOKMARKS as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_UI_Flag2 {
    USER_ALWAYS_SHOW_NUMBER_ARROWS = (1 << 0),
    USER_REGION_OVERLAP = (1 << 1),
    USER_UIFLAG2_UNUSED_2 = (1 << 2),
    USER_UIFLAG2_UNUSED_3 = (1 << 3),
    USER_UIFLAG2_UNUSED_4 = (1 << 4),
    USER_UIFLAG2_PANEL_TABS_COMPACT = (1 << 5),
}

impl Default for eUserpref_UI_Flag2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_ALWAYS_SHOW_NUMBER_ARROWS: i32 = eUserpref_UI_Flag2::USER_ALWAYS_SHOW_NUMBER_ARROWS as i32;
pub const USER_REGION_OVERLAP: i32 = eUserpref_UI_Flag2::USER_REGION_OVERLAP as i32;
pub const USER_UIFLAG2_UNUSED_2: i32 = eUserpref_UI_Flag2::USER_UIFLAG2_UNUSED_2 as i32;
pub const USER_UIFLAG2_UNUSED_3: i32 = eUserpref_UI_Flag2::USER_UIFLAG2_UNUSED_3 as i32;
pub const USER_UIFLAG2_UNUSED_4: i32 = eUserpref_UI_Flag2::USER_UIFLAG2_UNUSED_4 as i32;
pub const USER_UIFLAG2_PANEL_TABS_COMPACT: i32 = eUserpref_UI_Flag2::USER_UIFLAG2_PANEL_TABS_COMPACT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_GPU_Flag {
    USER_GPU_FLAG_WORKBENCH_RT_SHADOWS = (1 << 0),
    USER_GPU_FLAG_NO_EDIT_MODE_SMOOTH_WIRE = (1 << 1),
    USER_GPU_FLAG_OVERLAY_SMOOTH_WIRE = (1 << 2),
    USER_GPU_FLAG_SUBDIVISION_EVALUATION = (1 << 3),
    USER_GPU_FLAG_FRESNEL_EDIT = (1 << 4),
}

impl Default for eUserpref_GPU_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_GPU_FLAG_WORKBENCH_RT_SHADOWS: i32 = eUserpref_GPU_Flag::USER_GPU_FLAG_WORKBENCH_RT_SHADOWS as i32;
pub const USER_GPU_FLAG_NO_EDIT_MODE_SMOOTH_WIRE: i32 = eUserpref_GPU_Flag::USER_GPU_FLAG_NO_EDIT_MODE_SMOOTH_WIRE as i32;
pub const USER_GPU_FLAG_OVERLAY_SMOOTH_WIRE: i32 = eUserpref_GPU_Flag::USER_GPU_FLAG_OVERLAY_SMOOTH_WIRE as i32;
pub const USER_GPU_FLAG_SUBDIVISION_EVALUATION: i32 = eUserpref_GPU_Flag::USER_GPU_FLAG_SUBDIVISION_EVALUATION as i32;
pub const USER_GPU_FLAG_FRESNEL_EDIT: i32 = eUserpref_GPU_Flag::USER_GPU_FLAG_FRESNEL_EDIT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserPref_GPUBackendType {
    USER_GPU_BACKEND_OPENGL = 1 << 0,
    USER_GPU_BACKEND_METAL = 1 << 1,
    USER_GPU_BACKEND_VULKAN = 1 << 3,
}

impl Default for eUserPref_GPUBackendType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_GPU_BACKEND_OPENGL: i32 = eUserPref_GPUBackendType::USER_GPU_BACKEND_OPENGL as i32;
pub const USER_GPU_BACKEND_METAL: i32 = eUserPref_GPUBackendType::USER_GPU_BACKEND_METAL as i32;
pub const USER_GPU_BACKEND_VULKAN: i32 = eUserPref_GPUBackendType::USER_GPU_BACKEND_VULKAN as i32;
pub const USER_GPU_BACKEND_DEFAULT: i32 = USER_GPU_BACKEND_METAL;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_TabletAPI {
    USER_TABLET_AUTOMATIC = 0,
    USER_TABLET_NATIVE = 1,
    USER_TABLET_WINTAB = 2,
}

impl Default for eUserpref_TabletAPI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_TABLET_AUTOMATIC: i32 = eUserpref_TabletAPI::USER_TABLET_AUTOMATIC as i32;
pub const USER_TABLET_NATIVE: i32 = eUserpref_TabletAPI::USER_TABLET_NATIVE as i32;
pub const USER_TABLET_WINTAB: i32 = eUserpref_TabletAPI::USER_TABLET_WINTAB as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserPref_Tablet_Flags {
    USER_TABLET_SHOW_DEBUG_VALUES = (1 << 0),
}

impl Default for eUserPref_Tablet_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_TABLET_SHOW_DEBUG_VALUES: i32 = eUserPref_Tablet_Flags::USER_TABLET_SHOW_DEBUG_VALUES as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_APP_Flag {
    USER_APP_LOCK_CORNER_SPLIT = (1 << 0),
    USER_APP_HIDE_REGION_TOGGLE = (1 << 1),
    USER_APP_LOCK_EDGE_RESIZE = (1 << 2),
}

impl Default for eUserpref_APP_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_APP_LOCK_CORNER_SPLIT: i32 = eUserpref_APP_Flag::USER_APP_LOCK_CORNER_SPLIT as i32;
pub const USER_APP_HIDE_REGION_TOGGLE: i32 = eUserpref_APP_Flag::USER_APP_HIDE_REGION_TOGGLE as i32;
pub const USER_APP_LOCK_EDGE_RESIZE: i32 = eUserpref_APP_Flag::USER_APP_LOCK_EDGE_RESIZE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_StatusBar_Flag {
    STATUSBAR_SHOW_MEMORY = (1 << 0),
    STATUSBAR_SHOW_VRAM = (1 << 1),
    STATUSBAR_SHOW_STATS = (1 << 2),
    STATUSBAR_SHOW_VERSION = (1 << 3),
    STATUSBAR_SHOW_SCENE_DURATION = (1 << 4),
    STATUSBAR_SHOW_EXTENSIONS_UPDATES = (1 << 5),
}

impl Default for eUserpref_StatusBar_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const STATUSBAR_SHOW_MEMORY: i32 = eUserpref_StatusBar_Flag::STATUSBAR_SHOW_MEMORY as i32;
pub const STATUSBAR_SHOW_VRAM: i32 = eUserpref_StatusBar_Flag::STATUSBAR_SHOW_VRAM as i32;
pub const STATUSBAR_SHOW_STATS: i32 = eUserpref_StatusBar_Flag::STATUSBAR_SHOW_STATS as i32;
pub const STATUSBAR_SHOW_VERSION: i32 = eUserpref_StatusBar_Flag::STATUSBAR_SHOW_VERSION as i32;
pub const STATUSBAR_SHOW_SCENE_DURATION: i32 = eUserpref_StatusBar_Flag::STATUSBAR_SHOW_SCENE_DURATION as i32;
pub const STATUSBAR_SHOW_EXTENSIONS_UPDATES: i32 = eUserpref_StatusBar_Flag::STATUSBAR_SHOW_EXTENSIONS_UPDATES as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eAutokey_Mode {
    AUTOKEY_ON = 1,
    AUTOKEY_MODE_NORMAL = 3,
    AUTOKEY_MODE_EDITKEYS = 5,
}

impl Default for eAutokey_Mode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const AUTOKEY_ON: i32 = eAutokey_Mode::AUTOKEY_ON as i32;
pub const AUTOKEY_MODE_NORMAL: i32 = eAutokey_Mode::AUTOKEY_MODE_NORMAL as i32;
pub const AUTOKEY_MODE_EDITKEYS: i32 = eAutokey_Mode::AUTOKEY_MODE_EDITKEYS as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eZoomFrame_Mode {
    ZOOM_FRAME_MODE_KEEP_RANGE = 0,
    ZOOM_FRAME_MODE_SECONDS = 1,
    ZOOM_FRAME_MODE_KEYFRAMES = 2,
}

impl Default for eZoomFrame_Mode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const ZOOM_FRAME_MODE_KEEP_RANGE: i32 = eZoomFrame_Mode::ZOOM_FRAME_MODE_KEEP_RANGE as i32;
pub const ZOOM_FRAME_MODE_SECONDS: i32 = eZoomFrame_Mode::ZOOM_FRAME_MODE_SECONDS as i32;
pub const ZOOM_FRAME_MODE_KEYFRAMES: i32 = eZoomFrame_Mode::ZOOM_FRAME_MODE_KEYFRAMES as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eKeying_Flag {
    KEYING_FLAG_VISUALKEY = (1 << 2),
    KEYING_FLAG_XYZ2RGB = (1 << 3),
    KEYING_FLAG_CYCLEAWARE = (1 << 8),
    AUTOKEY_FLAG_INSERTAVAILABLE = (1 << 0),
    AUTOKEY_FLAG_INSERTNEEDED = (1 << 1),
    AUTOKEY_FLAG_ONLYKEYINGSET = (1 << 6),
    AUTOKEY_FLAG_NOWARNING = (1 << 7),
    AUTOKEY_FLAG_LAYERED_RECORD = (1 << 10),
    MANUALKEY_FLAG_INSERTNEEDED = (1 << 11),
}

impl Default for eKeying_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const KEYING_FLAG_VISUALKEY: i32 = eKeying_Flag::KEYING_FLAG_VISUALKEY as i32;
pub const KEYING_FLAG_XYZ2RGB: i32 = eKeying_Flag::KEYING_FLAG_XYZ2RGB as i32;
pub const KEYING_FLAG_CYCLEAWARE: i32 = eKeying_Flag::KEYING_FLAG_CYCLEAWARE as i32;
pub const AUTOKEY_FLAG_INSERTAVAILABLE: i32 = eKeying_Flag::AUTOKEY_FLAG_INSERTAVAILABLE as i32;
pub const AUTOKEY_FLAG_INSERTNEEDED: i32 = eKeying_Flag::AUTOKEY_FLAG_INSERTNEEDED as i32;
pub const AUTOKEY_FLAG_ONLYKEYINGSET: i32 = eKeying_Flag::AUTOKEY_FLAG_ONLYKEYINGSET as i32;
pub const AUTOKEY_FLAG_NOWARNING: i32 = eKeying_Flag::AUTOKEY_FLAG_NOWARNING as i32;
pub const AUTOKEY_FLAG_LAYERED_RECORD: i32 = eKeying_Flag::AUTOKEY_FLAG_LAYERED_RECORD as i32;
pub const MANUALKEY_FLAG_INSERTNEEDED: i32 = eKeying_Flag::MANUALKEY_FLAG_INSERTNEEDED as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eKeyInsertChannels {
    USER_ANIM_KEY_CHANNEL_LOCATION = (1 << 0),
    USER_ANIM_KEY_CHANNEL_ROTATION = (1 << 1),
    USER_ANIM_KEY_CHANNEL_SCALE = (1 << 2),
    USER_ANIM_KEY_CHANNEL_ROTATION_MODE = (1 << 3),
    USER_ANIM_KEY_CHANNEL_CUSTOM_PROPERTIES = (1 << 4),
}

impl Default for eKeyInsertChannels {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_ANIM_KEY_CHANNEL_LOCATION: i32 = eKeyInsertChannels::USER_ANIM_KEY_CHANNEL_LOCATION as i32;
pub const USER_ANIM_KEY_CHANNEL_ROTATION: i32 = eKeyInsertChannels::USER_ANIM_KEY_CHANNEL_ROTATION as i32;
pub const USER_ANIM_KEY_CHANNEL_SCALE: i32 = eKeyInsertChannels::USER_ANIM_KEY_CHANNEL_SCALE as i32;
pub const USER_ANIM_KEY_CHANNEL_ROTATION_MODE: i32 = eKeyInsertChannels::USER_ANIM_KEY_CHANNEL_ROTATION_MODE as i32;
pub const USER_ANIM_KEY_CHANNEL_CUSTOM_PROPERTIES: i32 = eKeyInsertChannels::USER_ANIM_KEY_CHANNEL_CUSTOM_PROPERTIES as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_Anim_Flags {
    USER_ANIM_SHOW_CHANNEL_GROUP_COLORS = (1 << 0),
    USER_ANIM_ONLY_SHOW_SELECTED_CURVE_KEYS = (1 << 1),
    USER_ANIM_HIGH_QUALITY_DRAWING = (1 << 2),
}

impl Default for eUserpref_Anim_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_ANIM_SHOW_CHANNEL_GROUP_COLORS: i32 = eUserpref_Anim_Flags::USER_ANIM_SHOW_CHANNEL_GROUP_COLORS as i32;
pub const USER_ANIM_ONLY_SHOW_SELECTED_CURVE_KEYS: i32 = eUserpref_Anim_Flags::USER_ANIM_ONLY_SHOW_SELECTED_CURVE_KEYS as i32;
pub const USER_ANIM_HIGH_QUALITY_DRAWING: i32 = eUserpref_Anim_Flags::USER_ANIM_HIGH_QUALITY_DRAWING as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFixToCam_Flags {
    FIX_TO_CAM_FLAG_USE_LOC = (1 << 0),
    FIX_TO_CAM_FLAG_USE_ROT = (1 << 1),
    FIX_TO_CAM_FLAG_USE_SCALE = (1 << 2),
}

impl Default for eFixToCam_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FIX_TO_CAM_FLAG_USE_LOC: i32 = eFixToCam_Flags::FIX_TO_CAM_FLAG_USE_LOC as i32;
pub const FIX_TO_CAM_FLAG_USE_ROT: i32 = eFixToCam_Flags::FIX_TO_CAM_FLAG_USE_ROT as i32;
pub const FIX_TO_CAM_FLAG_USE_SCALE: i32 = eFixToCam_Flags::FIX_TO_CAM_FLAG_USE_SCALE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_Translation_Flags {
    USER_TR_TOOLTIPS = (1 << 0),
    USER_TR_IFACE = (1 << 1),
    USER_TR_REPORTS = (1 << 2),
    USER_TR_UNUSED_3 = (1 << 3),
    USER_TR_UNUSED_4 = (1 << 4),
    USER_DOTRANSLATE_DEPRECATED = (1 << 5),
    USER_TR_UNUSED_6 = (1 << 6),
    USER_TR_UNUSED_7 = (1 << 7),
    USER_TR_NEWDATANAME = (1 << 8),
}

impl Default for eUserpref_Translation_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_TR_TOOLTIPS: i32 = eUserpref_Translation_Flags::USER_TR_TOOLTIPS as i32;
pub const USER_TR_IFACE: i32 = eUserpref_Translation_Flags::USER_TR_IFACE as i32;
pub const USER_TR_REPORTS: i32 = eUserpref_Translation_Flags::USER_TR_REPORTS as i32;
pub const USER_TR_UNUSED_3: i32 = eUserpref_Translation_Flags::USER_TR_UNUSED_3 as i32;
pub const USER_TR_UNUSED_4: i32 = eUserpref_Translation_Flags::USER_TR_UNUSED_4 as i32;
pub const USER_DOTRANSLATE_DEPRECATED: i32 = eUserpref_Translation_Flags::USER_DOTRANSLATE_DEPRECATED as i32;
pub const USER_TR_UNUSED_6: i32 = eUserpref_Translation_Flags::USER_TR_UNUSED_6 as i32;
pub const USER_TR_UNUSED_7: i32 = eUserpref_Translation_Flags::USER_TR_UNUSED_7 as i32;
pub const USER_TR_NEWDATANAME: i32 = eUserpref_Translation_Flags::USER_TR_NEWDATANAME as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTextEdit_Flags {
    USER_TEXT_EDIT_AUTO_CLOSE = (1 << 0),
}

impl Default for eTextEdit_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_TEXT_EDIT_AUTO_CLOSE: i32 = eTextEdit_Flags::USER_TEXT_EDIT_AUTO_CLOSE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eText_Draw_Options {
    USER_TEXT_DISABLE_AA = (1 << 0),
    USER_TEXT_HINTING_NONE = (1 << 1),
    USER_TEXT_HINTING_SLIGHT = (1 << 2),
    USER_TEXT_HINTING_FULL = (1 << 3),
    USER_TEXT_RENDER_SUBPIXELAA = (1 << 4),
}

impl Default for eText_Draw_Options {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_TEXT_DISABLE_AA: i32 = eText_Draw_Options::USER_TEXT_DISABLE_AA as i32;
pub const USER_TEXT_HINTING_NONE: i32 = eText_Draw_Options::USER_TEXT_HINTING_NONE as i32;
pub const USER_TEXT_HINTING_SLIGHT: i32 = eText_Draw_Options::USER_TEXT_HINTING_SLIGHT as i32;
pub const USER_TEXT_HINTING_FULL: i32 = eText_Draw_Options::USER_TEXT_HINTING_FULL as i32;
pub const USER_TEXT_RENDER_SUBPIXELAA: i32 = eText_Draw_Options::USER_TEXT_RENDER_SUBPIXELAA as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGP_UserdefSettings {
    GP_PAINT_UNUSED_0 = (1 << 0),
}

impl Default for eGP_UserdefSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const GP_PAINT_UNUSED_0: i32 = eGP_UserdefSettings::GP_PAINT_UNUSED_0 as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_Gizmo_Flag {
    USER_GIZMO_DRAW = (1 << 0),
}

impl Default for eUserpref_Gizmo_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_GIZMO_DRAW: i32 = eUserpref_Gizmo_Flag::USER_GIZMO_DRAW as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eColorPicker_Types {
    USER_CP_CIRCLE_HSV = 0,
    USER_CP_SQUARE_SV = 1,
    USER_CP_SQUARE_HS = 2,
    USER_CP_SQUARE_HV = 3,
    USER_CP_CIRCLE_HSL = 4,
}

impl Default for eColorPicker_Types {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_CP_CIRCLE_HSV: i32 = eColorPicker_Types::USER_CP_CIRCLE_HSV as i32;
pub const USER_CP_SQUARE_SV: i32 = eColorPicker_Types::USER_CP_SQUARE_SV as i32;
pub const USER_CP_SQUARE_HS: i32 = eColorPicker_Types::USER_CP_SQUARE_HS as i32;
pub const USER_CP_SQUARE_HV: i32 = eColorPicker_Types::USER_CP_SQUARE_HV as i32;
pub const USER_CP_CIRCLE_HSL: i32 = eColorPicker_Types::USER_CP_CIRCLE_HSL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eTimecodeStyles {
    USER_TIMECODE_MINIMAL = 0,
    USER_TIMECODE_SMPTE_MSF = 1,
    USER_TIMECODE_SMPTE_FULL = 2,
    USER_TIMECODE_MILLISECONDS = 3,
    USER_TIMECODE_SECONDS_ONLY = 4,
    USER_TIMECODE_SUBRIP = 100,
}

impl Default for eTimecodeStyles {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_TIMECODE_MINIMAL: i32 = eTimecodeStyles::USER_TIMECODE_MINIMAL as i32;
pub const USER_TIMECODE_SMPTE_MSF: i32 = eTimecodeStyles::USER_TIMECODE_SMPTE_MSF as i32;
pub const USER_TIMECODE_SMPTE_FULL: i32 = eTimecodeStyles::USER_TIMECODE_SMPTE_FULL as i32;
pub const USER_TIMECODE_MILLISECONDS: i32 = eTimecodeStyles::USER_TIMECODE_MILLISECONDS as i32;
pub const USER_TIMECODE_SECONDS_ONLY: i32 = eTimecodeStyles::USER_TIMECODE_SECONDS_ONLY as i32;
pub const USER_TIMECODE_SUBRIP: i32 = eTimecodeStyles::USER_TIMECODE_SUBRIP as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eNdof_Flag {
    NDOF_SHOW_GUIDE_ORBIT_AXIS = (1 << 0),
    NDOF_FLY_HELICOPTER = (1 << 1),
    NDOF_LOCK_HORIZON = (1 << 2),
    NDOF_SHOULD_PAN = (1 << 3),
    NDOF_SHOULD_ZOOM = (1 << 4),
    NDOF_SHOULD_ROTATE = (1 << 5),
    NDOF_SWAP_YZ_AXIS = (1 << 7),
    NDOF_ROTX_INVERT_AXIS = (1 << 9),
    NDOF_ROTY_INVERT_AXIS = (1 << 10),
    NDOF_ROTZ_INVERT_AXIS = (1 << 11),
    NDOF_PANX_INVERT_AXIS = (1 << 12),
    NDOF_PANY_INVERT_AXIS = (1 << 13),
    NDOF_PANZ_INVERT_AXIS = (1 << 14),
    NDOF_TURNTABLE = (1 << 15),
    NDOF_CAMERA_PAN_ZOOM = (1 << 16),
    NDOF_ORBIT_CENTER_AUTO = (1 << 17),
    NDOF_ORBIT_CENTER_SELECTED = (1 << 18),
    NDOF_SHOW_GUIDE_ORBIT_CENTER = (1 << 19),
    NDOF_FLY_SPEED_AUTO = (1 << 20),
}

impl Default for eNdof_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const NDOF_SHOW_GUIDE_ORBIT_AXIS: i32 = eNdof_Flag::NDOF_SHOW_GUIDE_ORBIT_AXIS as i32;
pub const NDOF_FLY_HELICOPTER: i32 = eNdof_Flag::NDOF_FLY_HELICOPTER as i32;
pub const NDOF_LOCK_HORIZON: i32 = eNdof_Flag::NDOF_LOCK_HORIZON as i32;
pub const NDOF_SHOULD_PAN: i32 = eNdof_Flag::NDOF_SHOULD_PAN as i32;
pub const NDOF_SHOULD_ZOOM: i32 = eNdof_Flag::NDOF_SHOULD_ZOOM as i32;
pub const NDOF_SHOULD_ROTATE: i32 = eNdof_Flag::NDOF_SHOULD_ROTATE as i32;
pub const NDOF_SWAP_YZ_AXIS: i32 = eNdof_Flag::NDOF_SWAP_YZ_AXIS as i32;
pub const NDOF_ROTX_INVERT_AXIS: i32 = eNdof_Flag::NDOF_ROTX_INVERT_AXIS as i32;
pub const NDOF_ROTY_INVERT_AXIS: i32 = eNdof_Flag::NDOF_ROTY_INVERT_AXIS as i32;
pub const NDOF_ROTZ_INVERT_AXIS: i32 = eNdof_Flag::NDOF_ROTZ_INVERT_AXIS as i32;
pub const NDOF_PANX_INVERT_AXIS: i32 = eNdof_Flag::NDOF_PANX_INVERT_AXIS as i32;
pub const NDOF_PANY_INVERT_AXIS: i32 = eNdof_Flag::NDOF_PANY_INVERT_AXIS as i32;
pub const NDOF_PANZ_INVERT_AXIS: i32 = eNdof_Flag::NDOF_PANZ_INVERT_AXIS as i32;
pub const NDOF_TURNTABLE: i32 = eNdof_Flag::NDOF_TURNTABLE as i32;
pub const NDOF_CAMERA_PAN_ZOOM: i32 = eNdof_Flag::NDOF_CAMERA_PAN_ZOOM as i32;
pub const NDOF_ORBIT_CENTER_AUTO: i32 = eNdof_Flag::NDOF_ORBIT_CENTER_AUTO as i32;
pub const NDOF_ORBIT_CENTER_SELECTED: i32 = eNdof_Flag::NDOF_ORBIT_CENTER_SELECTED as i32;
pub const NDOF_SHOW_GUIDE_ORBIT_CENTER: i32 = eNdof_Flag::NDOF_SHOW_GUIDE_ORBIT_CENTER as i32;
pub const NDOF_FLY_SPEED_AUTO: i32 = eNdof_Flag::NDOF_FLY_SPEED_AUTO as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eNdof_Navigation_Mode {
    NDOF_NAVIGATION_MODE_OBJECT = 0,
    NDOF_NAVIGATION_MODE_FLY = 1,
    NDOF_NAVIGATION_MODE_DRONE = 2,
}

impl Default for eNdof_Navigation_Mode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const NDOF_NAVIGATION_MODE_OBJECT: i32 = eNdof_Navigation_Mode::NDOF_NAVIGATION_MODE_OBJECT as i32;
pub const NDOF_NAVIGATION_MODE_FLY: i32 = eNdof_Navigation_Mode::NDOF_NAVIGATION_MODE_FLY as i32;
pub const NDOF_NAVIGATION_MODE_DRONE: i32 = eNdof_Navigation_Mode::NDOF_NAVIGATION_MODE_DRONE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMultiSample_Type {
    USER_MULTISAMPLE_NONE = 0,
    USER_MULTISAMPLE_2 = 2,
    USER_MULTISAMPLE_4 = 4,
    USER_MULTISAMPLE_8 = 8,
    USER_MULTISAMPLE_16 = 16,
}

impl Default for eMultiSample_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_MULTISAMPLE_NONE: i32 = eMultiSample_Type::USER_MULTISAMPLE_NONE as i32;
pub const USER_MULTISAMPLE_2: i32 = eMultiSample_Type::USER_MULTISAMPLE_2 as i32;
pub const USER_MULTISAMPLE_4: i32 = eMultiSample_Type::USER_MULTISAMPLE_4 as i32;
pub const USER_MULTISAMPLE_8: i32 = eMultiSample_Type::USER_MULTISAMPLE_8 as i32;
pub const USER_MULTISAMPLE_16: i32 = eMultiSample_Type::USER_MULTISAMPLE_16 as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_VirtualPixel {
    VIRTUAL_PIXEL_NATIVE = 0,
    VIRTUAL_PIXEL_DOUBLE = 1,
}

impl Default for eUserpref_VirtualPixel {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const VIRTUAL_PIXEL_NATIVE: i32 = eUserpref_VirtualPixel::VIRTUAL_PIXEL_NATIVE as i32;
pub const VIRTUAL_PIXEL_DOUBLE: i32 = eUserpref_VirtualPixel::VIRTUAL_PIXEL_DOUBLE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_FactorDisplay {
    USER_FACTOR_AS_FACTOR = 0,
    USER_FACTOR_AS_PERCENTAGE = 1,
}

impl Default for eUserpref_FactorDisplay {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_FACTOR_AS_FACTOR: i32 = eUserpref_FactorDisplay::USER_FACTOR_AS_FACTOR as i32;
pub const USER_FACTOR_AS_PERCENTAGE: i32 = eUserpref_FactorDisplay::USER_FACTOR_AS_PERCENTAGE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_XrNavigationFlags {
    USER_XR_NAV_SNAP_TURN = (1 << 0),
    USER_XR_NAV_INVERT_ROTATION = (1 << 1),
}

impl Default for eUserpref_XrNavigationFlags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_XR_NAV_SNAP_TURN: i32 = eUserpref_XrNavigationFlags::USER_XR_NAV_SNAP_TURN as i32;
pub const USER_XR_NAV_INVERT_ROTATION: i32 = eUserpref_XrNavigationFlags::USER_XR_NAV_INVERT_ROTATION as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_RenderDisplayType {
    USER_RENDER_DISPLAY_NONE = 0,
    USER_RENDER_DISPLAY_SCREEN = 1,
    USER_RENDER_DISPLAY_AREA = 2,
    USER_RENDER_DISPLAY_WINDOW = 3,
}

impl Default for eUserpref_RenderDisplayType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_RENDER_DISPLAY_NONE: i32 = eUserpref_RenderDisplayType::USER_RENDER_DISPLAY_NONE as i32;
pub const USER_RENDER_DISPLAY_SCREEN: i32 = eUserpref_RenderDisplayType::USER_RENDER_DISPLAY_SCREEN as i32;
pub const USER_RENDER_DISPLAY_AREA: i32 = eUserpref_RenderDisplayType::USER_RENDER_DISPLAY_AREA as i32;
pub const USER_RENDER_DISPLAY_WINDOW: i32 = eUserpref_RenderDisplayType::USER_RENDER_DISPLAY_WINDOW as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_TempSpaceDisplayType {
    USER_TEMP_SPACE_DISPLAY_FULLSCREEN = 0,
    USER_TEMP_SPACE_DISPLAY_WINDOW = 1,
}

impl Default for eUserpref_TempSpaceDisplayType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_TEMP_SPACE_DISPLAY_FULLSCREEN: i32 = eUserpref_TempSpaceDisplayType::USER_TEMP_SPACE_DISPLAY_FULLSCREEN as i32;
pub const USER_TEMP_SPACE_DISPLAY_WINDOW: i32 = eUserpref_TempSpaceDisplayType::USER_TEMP_SPACE_DISPLAY_WINDOW as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_EmulateMMBMod {
    USER_EMU_MMB_MOD_ALT = 0,
    USER_EMU_MMB_MOD_OSKEY = 1,
}

impl Default for eUserpref_EmulateMMBMod {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_EMU_MMB_MOD_ALT: i32 = eUserpref_EmulateMMBMod::USER_EMU_MMB_MOD_ALT as i32;
pub const USER_EMU_MMB_MOD_OSKEY: i32 = eUserpref_EmulateMMBMod::USER_EMU_MMB_MOD_OSKEY as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_TrackpadScrollDir {
    USER_TRACKPAD_SCROLL_DIR_TRADITIONAL = 0,
    USER_TRACKPAD_SCROLL_DIR_NATURAL = 1,
}

impl Default for eUserpref_TrackpadScrollDir {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_TRACKPAD_SCROLL_DIR_TRADITIONAL: i32 = eUserpref_TrackpadScrollDir::USER_TRACKPAD_SCROLL_DIR_TRADITIONAL as i32;
pub const USER_TRACKPAD_SCROLL_DIR_NATURAL: i32 = eUserpref_TrackpadScrollDir::USER_TRACKPAD_SCROLL_DIR_NATURAL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_DiskCacheCompression {
    USER_SEQ_DISK_CACHE_COMPRESSION_NONE = 0,
    USER_SEQ_DISK_CACHE_COMPRESSION_LOW = 1,
    USER_SEQ_DISK_CACHE_COMPRESSION_HIGH = 2,
}

impl Default for eUserpref_DiskCacheCompression {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_SEQ_DISK_CACHE_COMPRESSION_NONE: i32 = eUserpref_DiskCacheCompression::USER_SEQ_DISK_CACHE_COMPRESSION_NONE as i32;
pub const USER_SEQ_DISK_CACHE_COMPRESSION_LOW: i32 = eUserpref_DiskCacheCompression::USER_SEQ_DISK_CACHE_COMPRESSION_LOW as i32;
pub const USER_SEQ_DISK_CACHE_COMPRESSION_HIGH: i32 = eUserpref_DiskCacheCompression::USER_SEQ_DISK_CACHE_COMPRESSION_HIGH as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_SeqProxySetup {
    USER_SEQ_PROXY_SETUP_MANUAL = 0,
    USER_SEQ_PROXY_SETUP_AUTOMATIC = 1,
}

impl Default for eUserpref_SeqProxySetup {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_SEQ_PROXY_SETUP_MANUAL: i32 = eUserpref_SeqProxySetup::USER_SEQ_PROXY_SETUP_MANUAL as i32;
pub const USER_SEQ_PROXY_SETUP_AUTOMATIC: i32 = eUserpref_SeqProxySetup::USER_SEQ_PROXY_SETUP_AUTOMATIC as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_SeqEditorFlags {
    USER_SEQ_ED_UNUSED_0 = (1 << 0),
    USER_SEQ_ED_CONNECT_STRIPS_BY_DEFAULT = (1 << 1),
    USER_SEQ_ED_CLAMP_STRIPS_BY_DEFAULT = (1 << 2),
}

impl Default for eUserpref_SeqEditorFlags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_SEQ_ED_UNUSED_0: i32 = eUserpref_SeqEditorFlags::USER_SEQ_ED_UNUSED_0 as i32;
pub const USER_SEQ_ED_CONNECT_STRIPS_BY_DEFAULT: i32 = eUserpref_SeqEditorFlags::USER_SEQ_ED_CONNECT_STRIPS_BY_DEFAULT as i32;
pub const USER_SEQ_ED_CLAMP_STRIPS_BY_DEFAULT: i32 = eUserpref_SeqEditorFlags::USER_SEQ_ED_CLAMP_STRIPS_BY_DEFAULT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_ShaderCompileMethod {
    USER_SHADER_COMPILE_THREAD = 0,
    USER_SHADER_COMPILE_SUBPROCESS = 1,
}

impl Default for eUserpref_ShaderCompileMethod {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_SHADER_COMPILE_THREAD: i32 = eUserpref_ShaderCompileMethod::USER_SHADER_COMPILE_THREAD as i32;
pub const USER_SHADER_COMPILE_SUBPROCESS: i32 = eUserpref_ShaderCompileMethod::USER_SHADER_COMPILE_SUBPROCESS as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserpref_Language {
    ULANGUAGE_AUTO = 0,
    ULANGUAGE_ENGLISH = 1,
}

impl Default for eUserpref_Language {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const ULANGUAGE_AUTO: i32 = eUserpref_Language::ULANGUAGE_AUTO as i32;
pub const ULANGUAGE_ENGLISH: i32 = eUserpref_Language::ULANGUAGE_ENGLISH as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ePathCompare_Flag {
    USER_PATHCMP_GLOB = (1 << 0),
}

impl Default for ePathCompare_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_PATHCMP_GLOB: i32 = ePathCompare_Flag::USER_PATHCMP_GLOB as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserMenu_Type {
    USER_MENU_TYPE_SEP = 1,
    USER_MENU_TYPE_OPERATOR = 2,
    USER_MENU_TYPE_MENU = 3,
    USER_MENU_TYPE_PROP = 4,
}

impl Default for eUserMenu_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_MENU_TYPE_SEP: i32 = eUserMenu_Type::USER_MENU_TYPE_SEP as i32;
pub const USER_MENU_TYPE_OPERATOR: i32 = eUserMenu_Type::USER_MENU_TYPE_OPERATOR as i32;
pub const USER_MENU_TYPE_MENU: i32 = eUserMenu_Type::USER_MENU_TYPE_MENU as i32;
pub const USER_MENU_TYPE_PROP: i32 = eUserMenu_Type::USER_MENU_TYPE_PROP as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserExtensionRepo_Flag {
    USER_EXTENSION_REPO_FLAG_NO_CACHE = 1 << 0,
    USER_EXTENSION_REPO_FLAG_DISABLED = 1 << 1,
    USER_EXTENSION_REPO_FLAG_USE_CUSTOM_DIRECTORY = 1 << 2,
    USER_EXTENSION_REPO_FLAG_USE_REMOTE_URL = 1 << 3,
    USER_EXTENSION_REPO_FLAG_SYNC_ON_STARTUP = 1 << 4,
    USER_EXTENSION_REPO_FLAG_USE_ACCESS_TOKEN = 1 << 5,
}

impl Default for eUserExtensionRepo_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_EXTENSION_REPO_FLAG_NO_CACHE: i32 = eUserExtensionRepo_Flag::USER_EXTENSION_REPO_FLAG_NO_CACHE as i32;
pub const USER_EXTENSION_REPO_FLAG_DISABLED: i32 = eUserExtensionRepo_Flag::USER_EXTENSION_REPO_FLAG_DISABLED as i32;
pub const USER_EXTENSION_REPO_FLAG_USE_CUSTOM_DIRECTORY: i32 = eUserExtensionRepo_Flag::USER_EXTENSION_REPO_FLAG_USE_CUSTOM_DIRECTORY as i32;
pub const USER_EXTENSION_REPO_FLAG_USE_REMOTE_URL: i32 = eUserExtensionRepo_Flag::USER_EXTENSION_REPO_FLAG_USE_REMOTE_URL as i32;
pub const USER_EXTENSION_REPO_FLAG_SYNC_ON_STARTUP: i32 = eUserExtensionRepo_Flag::USER_EXTENSION_REPO_FLAG_SYNC_ON_STARTUP as i32;
pub const USER_EXTENSION_REPO_FLAG_USE_ACCESS_TOKEN: i32 = eUserExtensionRepo_Flag::USER_EXTENSION_REPO_FLAG_USE_ACCESS_TOKEN as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserExtensionRepo_Source {
    USER_EXTENSION_REPO_SOURCE_USER = 0,
    USER_EXTENSION_REPO_SOURCE_SYSTEM = 1,
}

impl Default for eUserExtensionRepo_Source {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_EXTENSION_REPO_SOURCE_USER: i32 = eUserExtensionRepo_Source::USER_EXTENSION_REPO_SOURCE_USER as i32;
pub const USER_EXTENSION_REPO_SOURCE_SYSTEM: i32 = eUserExtensionRepo_Source::USER_EXTENSION_REPO_SOURCE_SYSTEM as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserPref_Section {
    USER_SECTION_INTERFACE = 0,
    USER_SECTION_EDITING = 1,
    USER_SECTION_SAVE_LOAD = 2,
    USER_SECTION_SYSTEM = 3,
    USER_SECTION_THEME = 4,
    USER_SECTION_INPUT = 5,
    USER_SECTION_ADDONS = 6,
    USER_SECTION_LIGHT = 7,
    USER_SECTION_KEYMAP = 8,
    USER_SECTION_WORKSPACE_CONFIG = 9,
    USER_SECTION_WORKSPACE_ADDONS = 10,
    USER_SECTION_WORKSPACE_KEYMAPS = 11,
    USER_SECTION_VIEWPORT = 12,
    USER_SECTION_ANIMATION = 13,
    USER_SECTION_NAVIGATION = 14,
    USER_SECTION_FILE_PATHS = 15,
    USER_SECTION_EXPERIMENTAL = 16,
    USER_SECTION_EXTENSIONS = 17,
    USER_SECTION_DEVELOPER_TOOLS = 18,
    USER_SECTION_ASSETS = 19,
}

impl Default for eUserPref_Section {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_SECTION_INTERFACE: i32 = eUserPref_Section::USER_SECTION_INTERFACE as i32;
pub const USER_SECTION_EDITING: i32 = eUserPref_Section::USER_SECTION_EDITING as i32;
pub const USER_SECTION_SAVE_LOAD: i32 = eUserPref_Section::USER_SECTION_SAVE_LOAD as i32;
pub const USER_SECTION_SYSTEM: i32 = eUserPref_Section::USER_SECTION_SYSTEM as i32;
pub const USER_SECTION_THEME: i32 = eUserPref_Section::USER_SECTION_THEME as i32;
pub const USER_SECTION_INPUT: i32 = eUserPref_Section::USER_SECTION_INPUT as i32;
pub const USER_SECTION_ADDONS: i32 = eUserPref_Section::USER_SECTION_ADDONS as i32;
pub const USER_SECTION_LIGHT: i32 = eUserPref_Section::USER_SECTION_LIGHT as i32;
pub const USER_SECTION_KEYMAP: i32 = eUserPref_Section::USER_SECTION_KEYMAP as i32;
pub const USER_SECTION_WORKSPACE_CONFIG: i32 = eUserPref_Section::USER_SECTION_WORKSPACE_CONFIG as i32;
pub const USER_SECTION_WORKSPACE_ADDONS: i32 = eUserPref_Section::USER_SECTION_WORKSPACE_ADDONS as i32;
pub const USER_SECTION_WORKSPACE_KEYMAPS: i32 = eUserPref_Section::USER_SECTION_WORKSPACE_KEYMAPS as i32;
pub const USER_SECTION_VIEWPORT: i32 = eUserPref_Section::USER_SECTION_VIEWPORT as i32;
pub const USER_SECTION_ANIMATION: i32 = eUserPref_Section::USER_SECTION_ANIMATION as i32;
pub const USER_SECTION_NAVIGATION: i32 = eUserPref_Section::USER_SECTION_NAVIGATION as i32;
pub const USER_SECTION_FILE_PATHS: i32 = eUserPref_Section::USER_SECTION_FILE_PATHS as i32;
pub const USER_SECTION_EXPERIMENTAL: i32 = eUserPref_Section::USER_SECTION_EXPERIMENTAL as i32;
pub const USER_SECTION_EXTENSIONS: i32 = eUserPref_Section::USER_SECTION_EXTENSIONS as i32;
pub const USER_SECTION_DEVELOPER_TOOLS: i32 = eUserPref_Section::USER_SECTION_DEVELOPER_TOOLS as i32;
pub const USER_SECTION_ASSETS: i32 = eUserPref_Section::USER_SECTION_ASSETS as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUserPref_SpaceData_Flag {
    USER_SPACEDATA_INPUT_HIDE_UI_KEYCONFIG = (1 << 0),
    USER_SPACEDATA_ADDONS_SHOW_ONLY_ENABLED = (1 << 1),
}

impl Default for eUserPref_SpaceData_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const USER_SPACEDATA_INPUT_HIDE_UI_KEYCONFIG: i32 = eUserPref_SpaceData_Flag::USER_SPACEDATA_INPUT_HIDE_UI_KEYCONFIG as i32;
pub const USER_SPACEDATA_ADDONS_SHOW_ONLY_ENABLED: i32 = eUserPref_SpaceData_Flag::USER_SPACEDATA_ADDONS_SHOW_ONLY_ENABLED as i32;
