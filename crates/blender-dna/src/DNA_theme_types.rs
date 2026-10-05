//! Auto-transpiled C/C++ header module: DNA_theme_types

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct uiFont {
    pub next: *mut uiFont,
    pub prev: *mut uiFont,
    pub filepath: [i8; 1024],
    pub blf_id: i16,
    pub uifont_id: i16,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct uiFontStyle {
    pub uifont_id: i16,
    pub _pad1: [i8; 2],
    pub points: f32,
    pub italic: i16,
    pub bold: i16,
    pub shadow: i16,
    pub shadx: i16,
    pub shady: i16,
    pub _pad0: [i8; 2],
    pub shadowalpha: f32,
    pub shadowcolor: f32,
    pub character_weight: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct uiStyle {
    pub next: *mut uiStyle,
    pub prev: *mut uiStyle,
    pub name: [i8; 64],
    pub paneltitle: uiFontStyle,
    pub grouplabel: uiFontStyle,
    pub widget: uiFontStyle,
    pub tooltip: uiFontStyle,
    pub panelzoom: f32,
    pub minlabelchars: i16,
    pub minwidgetchars: i16,
    pub columnspace: i16,
    pub templatespace: i16,
    pub boxspace: i16,
    pub buttonspacex: i16,
    pub buttonspacey: i16,
    pub panelspace: i16,
    pub panelouter: i16,
    pub _pad0: [i8; 2],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ThemeRegionsAssetShelf {
    pub back: [u8; 4],
    pub header_back: [u8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ThemeRegionsChannels {
    pub back: [u8; 4],
    pub text: [u8; 4],
    pub text_selected: [u8; 4],
    pub _pad0: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ThemeRegionsScrubbing {
    pub back: [u8; 4],
    pub text: [u8; 4],
    pub time_marker: [u8; 4],
    pub time_marker_selected: [u8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ThemeRegionsSidebars {
    pub back: [u8; 4],
    pub tab_back: [u8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ThemeRegions {
    pub asset_shelf: ThemeRegionsAssetShelf,
    pub channels: ThemeRegionsChannels,
    pub scrubbing: ThemeRegionsScrubbing,
    pub sidebars: ThemeRegionsSidebars,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ThemeCommonAnim {
    pub playhead: [u8; 4],
    pub preview_range: [u8; 4],
    pub channels: [u8; 4],
    pub channels_sub: [u8; 4],
    pub channel_group: [u8; 4],
    pub channel_group_active: [u8; 4],
    pub channel: [u8; 4],
    pub channel_selected: [u8; 4],
    pub long_key: [u8; 4],
    pub long_key_selected: [u8; 4],
    pub scene_strip_range: [u8; 4],
    pub _pad0: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ThemeCommonCurves {
    pub handle_vertex: [u8; 4],
    pub handle_vertex_select: [u8; 4],
    pub handle_vertex_size: u8,
    pub _pad0: [i8; 3],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ThemeCommon {
    pub anim: ThemeCommonAnim,
    pub curves: ThemeCommonCurves,
    pub _pad0: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct uiWidgetColors {
    pub outline: [u8; 4],
    pub outline_sel: [u8; 4],
    pub inner: [u8; 4],
    pub inner_sel: [u8; 4],
    pub item: [u8; 4],
    pub text: [u8; 4],
    pub text_sel: [u8; 4],
    pub shaded: u8,
    pub _pad0: [i8; 3],
    pub shadetop: i16,
    pub shadedown: i16,
    pub roundness: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct uiWidgetStateColors {
    pub error: [u8; 4],
    pub warning: [u8; 4],
    pub info: [u8; 4],
    pub success: [u8; 4],
    pub inner_anim: [u8; 4],
    pub inner_anim_sel: [u8; 4],
    pub inner_key: [u8; 4],
    pub inner_key_sel: [u8; 4],
    pub inner_driven: [u8; 4],
    pub inner_driven_sel: [u8; 4],
    pub inner_overridden: [u8; 4],
    pub inner_overridden_sel: [u8; 4],
    pub inner_changed: [u8; 4],
    pub inner_changed_sel: [u8; 4],
    pub blend: f32,
    pub _pad0: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ThemeUI {
    pub wcol_regular: uiWidgetColors,
    pub wcol_tool: uiWidgetColors,
    pub wcol_toolbar_item: uiWidgetColors,
    pub wcol_text: uiWidgetColors,
    pub wcol_radio: uiWidgetColors,
    pub wcol_option: uiWidgetColors,
    pub wcol_toggle: uiWidgetColors,
    pub wcol_num: uiWidgetColors,
    pub wcol_numslider: uiWidgetColors,
    pub wcol_tab: uiWidgetColors,
    pub wcol_curve: uiWidgetColors,
    pub wcol_menu: uiWidgetColors,
    pub wcol_pulldown: uiWidgetColors,
    pub wcol_menu_back: uiWidgetColors,
    pub wcol_menu_item: uiWidgetColors,
    pub wcol_tooltip: uiWidgetColors,
    pub wcol_box: uiWidgetColors,
    pub wcol_scroll: uiWidgetColors,
    pub wcol_progress: uiWidgetColors,
    pub wcol_list_item: uiWidgetColors,
    pub wcol_pie_menu: uiWidgetColors,
    pub wcol_state: uiWidgetStateColors,
    pub widget_emboss: [u8; 4],
    pub menu_shadow_fac: f32,
    pub menu_shadow_width: i16,
    pub editor_border: [u8; 4],
    pub editor_outline: [u8; 4],
    pub editor_outline_active: [u8; 4],
    pub transparent_checker_primary: [u8; 4],
    pub transparent_checker_secondary: [u8; 4],
    pub transparent_checker_size: u8,
    pub link: [u8; 4],
    pub _pad1: [i8; 1],
    pub icon_alpha: f32,
    pub icon_saturation: f32,
    pub widget_text_cursor: [u8; 4],
    pub xaxis: [u8; 4],
    pub yaxis: [u8; 4],
    pub zaxis: [u8; 4],
    pub waxis: [u8; 4],
    pub gizmo_hi: [u8; 4],
    pub gizmo_primary: [u8; 4],
    pub gizmo_secondary: [u8; 4],
    pub gizmo_view_align: [u8; 4],
    pub gizmo_a: [u8; 4],
    pub gizmo_b: [u8; 4],
    pub icon_scene: [u8; 4],
    pub icon_collection: [u8; 4],
    pub icon_object: [u8; 4],
    pub icon_object_data: [u8; 4],
    pub icon_modifier: [u8; 4],
    pub icon_shading: [u8; 4],
    pub icon_folder: [u8; 4],
    pub icon_autokey: [u8; 4],
    pub _pad3: [i8; 4],
    pub icon_border_intensity: f32,
    pub panel_roundness: f32,
    pub panel_header: [u8; 4],
    pub panel_back: [u8; 4],
    pub panel_sub_back: [u8; 4],
    pub panel_outline: [u8; 4],
    pub panel_title: [u8; 4],
    pub panel_text: [u8; 4],
    pub panel_active: [u8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ThemeSpace {
    pub back: [u8; 4],
    pub back_grad: [u8; 4],
    pub background_type: i8,
    pub _pad0: [i8; 3],
    pub text: [u8; 4],
    pub text_hi: [u8; 4],
    pub header: [u8; 4],
    pub shade1: [u8; 4],
    pub shade2: [u8; 4],
    pub hilite: [u8; 4],
    pub grid: [u8; 4],
    pub grid_major: [u8; 4],
    pub grid_axis_brightness: f32,
    pub view_overlay: [u8; 4],
    pub wire: [u8; 4],
    pub wire_edit: [u8; 4],
    pub select: [u8; 4],
    pub lamp: [u8; 4],
    pub speaker: [u8; 4],
    pub empty: [u8; 4],
    pub camera: [u8; 4],
    pub active: [u8; 4],
    pub transform: [u8; 4],
    pub vertex: [u8; 4],
    pub vertex_select: [u8; 4],
    pub vertex_active: [u8; 4],
    pub vertex_unreferenced: [u8; 4],
    pub edge: [u8; 4],
    pub edge_select: [u8; 4],
    pub edge_mode_select: [u8; 4],
    pub face: [u8; 4],
    pub face_select: [u8; 4],
    pub face_mode_select: [u8; 4],
    pub face_retopology: [u8; 4],
    pub face_back: [u8; 4],
    pub face_front: [u8; 4],
    pub extra_edge_len: [u8; 4],
    pub extra_edge_angle: [u8; 4],
    pub extra_face_angle: [u8; 4],
    pub extra_face_area: [u8; 4],
    pub normal: [u8; 4],
    pub vertex_normal: [u8; 4],
    pub loop_normal: [u8; 4],
    pub bone_solid: [u8; 4],
    pub bone_pose: [u8; 4],
    pub bone_pose_active: [u8; 4],
    pub bone_locked_weight: [u8; 4],
    pub strip: [u8; 4],
    pub strip_select: [u8; 4],
    pub before_current_frame: [u8; 4],
    pub after_current_frame: [u8; 4],
    pub time_gp_keyframe: [u8; 4],
    pub bevel: [u8; 4],
    pub seam: [u8; 4],
    pub sharp: [u8; 4],
    pub crease: [u8; 4],
    pub freestyle: [u8; 4],
    pub nurb_uline: [u8; 4],
    pub nurb_vline: [u8; 4],
    pub nurb_sel_uline: [u8; 4],
    pub nurb_sel_vline: [u8; 4],
    pub keyborder: [u8; 4],
    pub keyborder_select: [u8; 4],
    pub _pad4: [i8; 3],
    pub console_output: [u8; 4],
    pub console_input: [u8; 4],
    pub console_info: [u8; 4],
    pub console_error: [u8; 4],
    pub console_cursor: [u8; 4],
    pub console_select: [u8; 4],
    pub vertex_size: u8,
    pub edge_width: u8,
    pub outline_width: u8,
    pub obcenter_dia: u8,
    pub facedot_size: u8,
    pub noodle_curving: u8,
    pub grid_levels: u8,
    pub _pad2: [i8; 2],
    pub dash_alpha: f32,
    pub syntaxl: [u8; 4],
    pub syntaxs: [u8; 4],
    pub syntaxb: [u8; 4],
    pub syntaxn: [u8; 4],
    pub syntaxv: [u8; 4],
    pub syntaxc: [u8; 4],
    pub syntaxd: [u8; 4],
    pub syntaxr: [u8; 4],
    pub line_numbers: [u8; 4],
    pub node_outline: [u8; 4],
    pub nodeclass_output: [u8; 4],
    pub nodeclass_filter: [u8; 4],
    pub nodeclass_vector: [u8; 4],
    pub nodeclass_texture: [u8; 4],
    pub nodeclass_shader: [u8; 4],
    pub nodeclass_script: [u8; 4],
    pub nodeclass_geometry: [u8; 4],
    pub nodeclass_attribute: [u8; 4],
    pub node_zone_simulation: [u8; 4],
    pub node_zone_repeat: [u8; 4],
    pub node_zone_foreach_geometry_element: [u8; 4],
    pub node_zone_closure: [u8; 4],
    pub simulated_frames: [u8; 4],
    pub movie: [u8; 4],
    pub movieclip: [u8; 4],
    pub mask: [u8; 4],
    pub image: [u8; 4],
    pub scene: [u8; 4],
    pub audio: [u8; 4],
    pub effect: [u8; 4],
    pub transition: [u8; 4],
    pub meta: [u8; 4],
    pub text_strip: [u8; 4],
    pub color_strip: [u8; 4],
    pub active_strip: [u8; 4],
    pub selected_strip: [u8; 4],
    pub text_strip_cursor: [u8; 4],
    pub selected_text: [u8; 4],
    pub keyframe_scale_fac: f32,
    pub editmesh_active: [u8; 4],
    pub _pad3: [i8; 1],
    pub clipping_border_3d: [u8; 4],
    pub bundle_solid: [u8; 4],
    pub path_before: [u8; 4],
    pub path_after: [u8; 4],
    pub path_keyframe_before: [u8; 4],
    pub path_keyframe_after: [u8; 4],
    pub camera_path: [u8; 4],
    pub camera_passepartout: [u8; 4],
    pub _pad1: [u8; 2],
    pub gp_wire_edit: [u8; 4],
    pub gp_vertex_size: u8,
    pub gp_vertex: [u8; 4],
    pub gp_vertex_select: [u8; 4],
    pub _pad11: [i8; 12],
    pub preview_back: [u8; 4],
    pub preview_stitch_face: [u8; 4],
    pub preview_stitch_edge: [u8; 4],
    pub preview_stitch_vert: [u8; 4],
    pub preview_stitch_stitchable: [u8; 4],
    pub preview_stitch_unstitchable: [u8; 4],
    pub preview_stitch_active: [u8; 4],
    pub uv_shadow: [u8; 4],
    pub r#match: [u8; 4],
    pub selected_highlight: [u8; 4],
    pub selected_object: [u8; 4],
    pub active_object: [u8; 4],
    pub edited_object: [u8; 4],
    pub row_alternate: [u8; 4],
    pub skin_root: [u8; 4],
    pub anim_active: [u8; 4],
    pub anim_non_active: [u8; 4],
    pub nla_tweaking: [u8; 4],
    pub nla_tweakdupli: [u8; 4],
    pub nla_transition: [u8; 4],
    pub nla_transition_sel: [u8; 4],
    pub nla_meta: [u8; 4],
    pub nla_meta_sel: [u8; 4],
    pub nla_sound: [u8; 4],
    pub nla_sound_sel: [u8; 4],
    pub info_selected: [u8; 4],
    pub info_selected_text: [u8; 4],
    pub info_error_text: [u8; 4],
    pub info_warning_text: [u8; 4],
    pub info_info_text: [u8; 4],
    pub info_debug: [u8; 4],
    pub info_debug_text: [u8; 4],
    pub info_property: [u8; 4],
    pub info_property_text: [u8; 4],
    pub info_operator: [u8; 4],
    pub info_operator_text: [u8; 4],
    pub metadatabg: [u8; 4],
    pub metadatatext: [u8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ThemeWireColor {
    pub solid: [u8; 4],
    pub select: [u8; 4],
    pub active: [u8; 4],
    pub flag: i16,
    pub _pad0: [i8; 2],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ThemeCollectionColor {
    pub color: [u8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ThemeStripColor {
    pub color: [u8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bTheme {
    pub next: *mut bTheme,
    pub prev: *mut bTheme,
    pub name: [i8; 64],
    pub filepath: [i8; 1024],
    pub tui: ThemeUI,
    pub regions: ThemeRegions,
    pub common: ThemeCommon,
    pub space_properties: ThemeSpace,
    pub space_view3d: ThemeSpace,
    pub space_file: ThemeSpace,
    pub space_graph: ThemeSpace,
    pub space_info: ThemeSpace,
    pub space_action: ThemeSpace,
    pub space_nla: ThemeSpace,
    pub space_sequencer: ThemeSpace,
    pub space_image: ThemeSpace,
    pub space_text: ThemeSpace,
    pub space_outliner: ThemeSpace,
    pub space_node: ThemeSpace,
    pub space_preferences: ThemeSpace,
    pub space_console: ThemeSpace,
    pub space_clip: ThemeSpace,
    pub space_topbar: ThemeSpace,
    pub space_statusbar: ThemeSpace,
    pub space_spreadsheet: ThemeSpace,
    pub tarm: [ThemeWireColor; 20],
    pub collection_color: [ThemeCollectionColor; 8],
    pub strip_color: [ThemeStripColor; 9],
    pub active_theme_area: i32,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUIFont_ID {
    UIFONT_DEFAULT = 0,
    UIFONT_CUSTOM1 = 2,
}

impl Default for eUIFont_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const UIFONT_DEFAULT: i32 = eUIFont_ID::UIFONT_DEFAULT as i32;
pub const UIFONT_CUSTOM1: i32 = eUIFont_ID::UIFONT_CUSTOM1 as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eBackgroundGradientTypes {
    TH_BACKGROUND_SINGLE_COLOR = 0,
    TH_BACKGROUND_GRADIENT_LINEAR = 1,
    TH_BACKGROUND_GRADIENT_RADIAL = 2,
}

impl Default for eBackgroundGradientTypes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TH_BACKGROUND_SINGLE_COLOR: i32 = eBackgroundGradientTypes::TH_BACKGROUND_SINGLE_COLOR as i32;
pub const TH_BACKGROUND_GRADIENT_LINEAR: i32 = eBackgroundGradientTypes::TH_BACKGROUND_GRADIENT_LINEAR as i32;
pub const TH_BACKGROUND_GRADIENT_RADIAL: i32 = eBackgroundGradientTypes::TH_BACKGROUND_GRADIENT_RADIAL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eWireColor_Flags {
    TH_WIRECOLOR_CONSTCOLS = (1 << 0),
}

impl Default for eWireColor_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const TH_WIRECOLOR_CONSTCOLS: i32 = eWireColor_Flags::TH_WIRECOLOR_CONSTCOLS as i32;

