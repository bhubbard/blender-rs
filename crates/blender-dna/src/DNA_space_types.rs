//! Auto-transpiled C/C++ header module: DNA_space_types

use core::ffi::c_void;
use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceLink {
    pub next: *mut SpaceLink,
    pub regionbase: ListBaseT<ARegion>,
    pub spacetype: i8,
    pub link_flag: eSpace_Link_Flag,
    pub _pad0: [i8; 6],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceInfo {
    pub next: *mut SpaceLink,
    pub regionbase: ListBaseT<ARegion>,
    pub spacetype: i8,
    pub link_flag: eSpace_Link_Flag,
    pub _pad0: [i8; 6],
    pub rpt_mask: i8,
    pub _pad: [i8; 7],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceProperties {
    pub next: *mut SpaceLink,
    pub regionbase: ListBaseT<ARegion>,
    pub spacetype: i8,
    pub link_flag: eSpace_Link_Flag,
    pub _pad0: [i8; 6],
    pub v2d: View2D,
    pub space_subtype: i16,
    pub visible_tabs: u32,
    pub preview: i16,
    pub flag: eSpaceButtons_Flag,
    pub outliner_sync: eSpaceButtons_OutlinerSync,
    pub path: *mut core::ffi::c_void,
    pub pathflag: i32,
    pub pinid: *mut ID,
    pub texuser: *mut core::ffi::c_void,
    pub runtime: *mut SpaceProperties_Runtime,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct SpaceOutliner {
    pub next: *mut SpaceLink,
    pub regionbase: ListBaseT<ARegion>,
    pub spacetype: i8,
    pub link_flag: eSpace_Link_Flag,
    pub _pad0: [i8; 6],
    pub v2d: View2D,
    pub treestore: *mut BLI_mempool,
    pub search_string: [i8; 64],
    pub flag: eSpaceOutliner_Flag,
    pub outlinevis: eSpaceOutliner_Mode,
    pub lib_override_view_mode: eSpaceOutliner_LibOverrideViewMode,
    pub storeflag: eSpaceOutliner_StoreFlag,
    pub search_flags: eSpaceOutliner_Search_Flags,
    pub _pad: [i8; 6],
    pub sync_select_dirty: i8,
    pub filter: eSpaceOutliner_Filter,
    pub filter_state: eSpaceOutliner_StateFilter,
    pub show_restrict_flags: eSpaceOutliner_ShowRestrictFlag,
    pub filter_id_type: i16,
    pub sort_method: eSpaceOutliner_SortMethod,
    pub _pad1: [i8; 6],
}

impl Default for SpaceOutliner {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceGraph_Runtime {
    pub flag: i8,
    pub _pad: [i8; 7],
    pub ghost_curves: ListBaseT<FCurve>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceGraph {
    pub next: *mut SpaceLink,
    pub regionbase: ListBaseT<ARegion>,
    pub spacetype: i8,
    pub link_flag: eSpace_Link_Flag,
    pub _pad0: [i8; 6],
    pub v2d: View2D,
    pub ads: *mut bDopeSheet,
    pub mode: eGraphEdit_Mode,
    pub autosnap: i16,
    pub flag: eGraphEdit_Flag,
    pub cursorTime: f32,
    pub cursorVal: f32,
    pub around: i32,
    pub local_view_bit: u16,
    pub _pad1: [i8; 2],
    pub local_view_visible_region_before: rctf,
    pub runtime: SpaceGraph_Runtime,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceNla {
    pub next: *mut SpaceLink,
    pub regionbase: ListBaseT<ARegion>,
    pub spacetype: i8,
    pub link_flag: eSpace_Link_Flag,
    pub _pad0: [i8; 6],
    pub autosnap: i16,
    pub flag: eSpaceNla_Flag,
    pub _pad: [i8; 4],
    pub ads: *mut bDopeSheet,
    pub v2d: View2D,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SequencerPreviewOverlay {
    pub flag: eSpaceSeq_SequencerPreviewOverlay_Flag,
    pub composition_guide_flags: eCompositionGuideFlags,
    pub _pad: [i8; 2],
    pub composition_guide_color: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SequencerTimelineOverlay {
    pub flag: eSpaceSeq_SequencerTimelineOverlay_Flag,
    pub _pad0: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SequencerCacheOverlay {
    pub flag: eSpaceSeq_SequencerCacheOverlay_Flag,
    pub _pad0: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceSeq {
    pub next: *mut SpaceLink,
    pub regionbase: ListBaseT<ARegion>,
    pub spacetype: i8,
    pub link_flag: eSpace_Link_Flag,
    pub _pad0: [i8; 6],
    pub v2d: View2D,
    pub xof: f32,
    pub yof: f32,
    pub mainb: eSpaceSeq_RegionType,
    pub render_size: eSpaceSeq_Proxy_RenderSize,
    pub chanshown: i16,
    pub zebra: i16,
    pub flag: eSpaceSeq_Flag,
    pub zoom: f32,
    pub view: eSpaceSeq_Displays,
    pub overlay_frame_type: eSpaceSeq_OverlayFrameType,
    pub draw_flag: eSpaceSeq_DrawFlag,
    pub gizmo_flag: eSpaceSeq_Gizmo_Flag,
    pub _pad: [i8; 4],
    pub cursor: [f32; 2],
    pub gpd: *mut bGPdata,
    pub preview_overlay: SequencerPreviewOverlay,
    pub timeline_overlay: SequencerTimelineOverlay,
    pub cache_overlay: SequencerCacheOverlay,
    pub multiview_eye: i8,
    pub _pad2: [i8; 7],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MaskSpaceInfo {
    pub mask: *mut Mask,
    pub draw_flag: MaskDrawFlag,
    pub draw_type: MaskDrawType,
    pub overlay_mode: MaskOverlayMode,
    pub _pad3: [i8; 1],
    pub blend_factor: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct FileSelectParams {
    pub title: [i8; 96],
    pub dir: [i8; 1282],
    pub file: [i8; 256],
    pub renamefile: [i8; 256],
    pub rename_flag: eFileSel_Params_RenameFlag,
    pub _pad: [i8; 4],
    pub rename_id: *mut ID,
    pub _pad3: *mut core::ffi::c_void,
    pub filter_glob: [i8; 256],
    pub filter_search: [i8; 64],
    pub filter_id: u64,
    pub active_file: i32,
    pub highlight_file: i32,
    pub sel_first: i32,
    pub sel_last: i32,
    pub thumbnail_size: u16,
    pub list_thumbnail_size: u16,
    pub list_column_size: u16,
    pub r#type: eFileSelectType,
    pub flag: eFileSel_Params_Flag,
    pub sort: eFileSortType,
    pub display: eFileDisplayType,
    pub details_flags: eFileDetails,
    pub _pad1: i8,
    pub filter: i32,
    pub recursion_level: i16,
    pub _pad2: [i8; 2],
}

impl Default for FileSelectParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FileAssetSelectParams {
    pub base_params: FileSelectParams,
    pub asset_library_ref: AssetLibraryReference,
    pub _pad: [i8; 5],
    pub asset_access: AssetAccess,
    pub asset_catalog_visibility: eFileSel_Params_AssetCatalogVisibility,
    pub catalog_id: bUUID,
    pub import_method: eFileAssetImportMethod,
    pub import_flags: eFileAssetImportFlags,
    pub asset_flags: eFileSel_AssetParams_Flag,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FileFolderHistory {
    pub next: *mut FileFolderLists,
    pub browse_mode: eFileBrowse_Mode,
    pub _pad: [i8; 7],
    pub folders_prev: ListBaseT<FolderList>,
    pub folders_next: ListBaseT<FolderList>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceFile {
    pub next: *mut SpaceLink,
    pub regionbase: ListBaseT<ARegion>,
    pub spacetype: i8,
    pub link_flag: eSpace_Link_Flag,
    pub _pad0: [i8; 6],
    pub browse_mode: eFileBrowse_Mode,
    pub _pad1: [i8; 1],
    pub tags: eFileTags,
    pub scroll_offset: i32,
    pub params: *mut FileSelectParams,
    pub asset_params: *mut FileAssetSelectParams,
    pub _pad2: *mut core::ffi::c_void,
    pub files: *mut FileList,
    pub folders_prev: *mut ListBaseT<FolderList>,
    pub folders_next: *mut ListBaseT<FolderList>,
    pub folder_histories: ListBaseT<FileFolderHistory>,
    pub op: *mut wmOperator,
    pub smoothscroll_timer: *mut wmTimer,
    pub previews_timer: *mut wmTimer,
    pub layout: *mut FileLayout,
    pub recentnr: i16,
    pub systemnr: i16,
    pub runtime: *mut SpaceFile_Runtime,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct FileDirEntry {
    pub next: *mut FileDirEntry,
    pub uid: u32,
    pub name: *mut i8,
    pub size: u64,
    pub time: i64,
    pub size_str: [i8; 16],
    pub datetime_str: [i8; 40],
    pub typeflag: i32,
    pub blentype: i32,
    pub relpath: *mut i8,
    pub redirection_path: *mut i8,
    pub id: *mut ID,
    pub preview_icon_id: i32,
    pub flags: eFileEntry_Flag,
    pub attributes: i32,
}

impl Default for FileDirEntry {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct FileDirEntryArr {
    pub entries: ListBaseT<FileListInternEntry>,
    pub entries_num: i32,
    pub entries_filtered_num: i32,
    pub root: [i8; 1024],
}

impl Default for FileDirEntryArr {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceImageOverlay {
    pub flag: eSpaceImageOverlay_Flag,
    pub passepartout_alpha: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceImage {
    pub next: *mut SpaceLink,
    pub regionbase: ListBaseT<ARegion>,
    pub spacetype: i8,
    pub link_flag: eSpace_Link_Flag,
    pub _pad0: [i8; 6],
    pub image: *mut Image,
    pub iuser: ImageUser,
    pub scopes: Scopes,
    pub sample_line_hist: Histogram,
    pub gpd: *mut bGPdata,
    pub cursor: [f32; 2],
    pub xof: f32,
    pub zoom: f32,
    pub centx: f32,
    pub mode: eSpaceImage_Mode,
    pub mode_prev: eSpaceImage_Mode,
    pub pin: i8,
    pub pixel_round_mode: eSpaceImage_PixelRoundMode,
    pub lock: i8,
    pub dt_uv: eSpaceImage_UVDT,
    pub dt_uvstretch: eSpaceImage_UVDT_Stretch,
    pub around: i8,
    pub gizmo_flag: eSpaceImage_Gizmo_Flag,
    pub grid_shape_source: eSpaceImage_GridShapeSource,
    pub _pad1: [i8; 6],
    pub flag: eSpaceImage_Flag,
    pub uv_opacity: f32,
    pub uv_face_opacity: f32,
    pub uv_edge_opacity: f32,
    pub stretch_opacity: f32,
    pub tile_grid_shape: [i32; 2],
    pub custom_grid_subdiv: [i32; 2],
    pub mask_info: MaskSpaceInfo,
    pub overlay: SpaceImageOverlay,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct SpaceText {
    pub next: *mut SpaceLink,
    pub regionbase: ListBaseT<ARegion>,
    pub spacetype: i8,
    pub link_flag: eSpace_Link_Flag,
    pub _pad0: [i8; 6],
    pub text: *mut Text,
    pub top: i32,
    pub left: i32,
    pub _pad1: [i8; 4],
    pub flags: eSpaceText_Flags,
    pub line_height: i16,
    pub tabnumber: i32,
    pub wordwrap: i8,
    pub doplugins: i8,
    pub showlinenrs: i8,
    pub showsyntax: i8,
    pub line_hlight: i8,
    pub overwrite: i8,
    pub live_edit: i8,
    pub _pad2: [i8; 1],
    pub findstr: [i8; 256],
    pub replacestr: [i8; 256],
    pub margin_column: i16,
    pub _pad3: [i8; 2],
}

impl Default for SpaceText {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Script {
    pub id: ID,
    pub py_draw: *mut core::ffi::c_void,
    pub py_event: *mut core::ffi::c_void,
    pub py_button: *mut core::ffi::c_void,
    pub py_browsercallback: *mut core::ffi::c_void,
    pub py_globaldict: *mut core::ffi::c_void,
    pub flags: i32,
    pub scriptname: [i8; 1024],
    pub scriptarg: [i8; 256],
}

impl Default for Script {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceScript {
    pub next: *mut SpaceLink,
    pub regionbase: ListBaseT<ARegion>,
    pub spacetype: i8,
    pub link_flag: i8,
    pub _pad0: [i8; 6],
    pub script: *mut Script,
    pub flags: i16,
    pub _pad1: [i8; 4],
    pub but_refs: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bNodeTreePath {
    pub next: *mut bNodeTreePath,
    pub nodetree: *mut bNodeTree,
    pub parent_key: bNodeInstanceKey,
    pub view_center: [f32; 2],
    pub view_width: f32,
    pub node_name: [i8; 64],
    pub display_name: [i8; 64],
}

impl Default for bNodeTreePath {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceNodeOverlay {
    pub flag: eSpaceNodeOverlay_Flag,
    pub preview_shape: eSpaceNodeOverlay_preview_shape,
    pub passepartout_alpha: f32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct SpaceNode {
    pub next: *mut SpaceLink,
    pub regionbase: ListBaseT<ARegion>,
    pub spacetype: i8,
    pub link_flag: eSpace_Link_Flag,
    pub _pad0: [i8; 6],
    pub v2d: View2D,
    pub id: *mut ID,
    pub flag: eSpaceNode_Flag,
    pub insert_ofs_dir: eSpaceNode_InsertOffsetDir,
    pub _pad1: i8,
    pub xof: f32,
    pub zoom: f32,
    pub treepath: ListBaseT<bNodeTreePath>,
    pub edittree: *mut bNodeTree,
    pub nodetree: *mut bNodeTree,
    pub tree_idname: [i8; 64],
    pub treetype: i32,
    pub texfrom: eSpaceNode_TexFrom,
    pub shaderfrom: eSpaceNode_ShaderFrom,
    pub node_tree_sub_type: i8,
    pub selected_node_group: *mut bNodeTree,
    pub gpd: *mut bGPdata,
    pub gizmo_flag: eSpaceNode_Gizmo_Flag,
    pub _pad2: [i8; 7],
    pub overlay: SpaceNodeOverlay,
}

impl Default for SpaceNode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ConsoleLine {
    pub next: *mut ConsoleLine,
    pub len_alloc: i32,
    pub len: i32,
    pub line: *mut i8,
    pub cursor: i32,
    pub r#type: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct SpaceConsole {
    pub next: *mut SpaceLink,
    pub regionbase: ListBaseT<ARegion>,
    pub spacetype: i8,
    pub link_flag: i8,
    pub _pad0: [i8; 6],
    pub scrollback: ListBaseT<ConsoleLine>,
    pub history: ListBaseT<ConsoleLine>,
    pub prompt: [i8; 256],
    pub language: [i8; 32],
    pub line_height: i32,
    pub history_index: i32,
    pub sel_start: i32,
    pub sel_end: i32,
    pub runtime: *mut SpaceConsole_Runtime,
}

impl Default for SpaceConsole {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct SpaceUserPref {
    pub next: *mut SpaceLink,
    pub regionbase: ListBaseT<ARegion>,
    pub spacetype: i8,
    pub link_flag: i8,
    pub _pad0: [i8; 6],
    pub _pad1: [i8; 7],
    pub filter_type: i8,
    pub filter: [i8; 64],
    pub runtime: *mut SpaceUserPref_Runtime,
}

impl Default for SpaceUserPref {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceClipOverlay {
    pub flag: i32,
    pub _pad0: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceClip {
    pub next: *mut SpaceLink,
    pub regionbase: ListBaseT<ARegion>,
    pub spacetype: i8,
    pub link_flag: eSpace_Link_Flag,
    pub _pad0: [i8; 6],
    pub gizmo_flag: eSpaceClip_Gizmo_Flag,
    pub _pad1: [i8; 3],
    pub xof: f32,
    pub xlockof: f32,
    pub zoom: f32,
    pub user: MovieClipUser,
    pub clip: *mut MovieClip,
    pub scopes: MovieClipScopes,
    pub flag: eSpaceClip_Flag,
    pub mode: eSpaceClip_Mode,
    pub view: eSpaceClip_View,
    pub path_length: i32,
    pub loc: [f32; 2],
    pub _pad: [i8; 4],
    pub stabmat: [[f32; 4]; 4],
    pub unistabmat: [[f32; 4]; 4],
    pub postproc_flag: i32,
    pub gpencil_src: eSpaceClip_GPencil_Source,
    pub _pad2: [i8; 2],
    pub around: eView3D_Around,
    pub _pad4: [i8; 4],
    pub cursor: [f32; 2],
    pub mask_info: MaskSpaceInfo,
    pub overlay: SpaceClipOverlay,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceTopBar {
    pub next: *mut SpaceLink,
    pub regionbase: ListBaseT<ARegion>,
    pub spacetype: i8,
    pub link_flag: i8,
    pub _pad0: [i8; 6],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceStatusBar {
    pub next: *mut SpaceLink,
    pub regionbase: ListBaseT<ARegion>,
    pub spacetype: i8,
    pub link_flag: i8,
    pub _pad0: [i8; 6],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpreadsheetColumnID {
    pub name: *mut i8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpreadsheetColumn {
    pub id: *mut SpreadsheetColumnID,
    pub data_type: u8,
    pub _pad0: [i8; 3],
    pub flag: u32,
    pub width: f32,
    pub last_used: u32,
    pub display_name: *mut i8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpreadsheetInstanceID {
    pub reference_index: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpreadsheetTableID {
    pub r#type: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpreadsheetBundlePathElem {
    pub identifier: *mut i8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpreadsheetBundleTreeViewPath {
    pub bundle_path: *mut SpreadsheetBundlePathElem,
    pub bundle_path_num: i32,
    pub closure_input_output: SpreadsheetClosureInputOutput,
    pub _pad: [i8; 3],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpreadsheetTableIDGeometry {
    pub base: SpreadsheetTableID,
    pub _pad0: [i8; 4],
    pub viewer_path: ViewerPath,
    pub viewer_item_identifier: i32,
    pub _pad3: [i8; 4],
    pub viewer_item_bundle_path: SpreadsheetBundleTreeViewPath,
    pub instance_ids: *mut SpreadsheetInstanceID,
    pub instance_ids_num: i32,
    pub geometry_component_type: u8,
    pub attribute_domain: u8,
    pub object_eval_state: u8,
    pub geometry_item_type: SpreadsheetGeometryItemType,
    pub geometry_bundle_path: SpreadsheetBundleTreeViewPath,
    pub _pad1: [i8; 4],
    pub layer_index: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpreadsheetTable {
    pub id: *mut SpreadsheetTableID,
    pub columns: *mut *mut SpreadsheetColumn,
    pub num_columns: i32,
    pub flag: u32,
    pub last_used: u32,
    pub column_use_clock: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceSpreadsheet {
    pub next: *mut SpaceLink,
    pub regionbase: ListBaseT<ARegion>,
    pub spacetype: i8,
    pub link_flag: eSpace_Link_Flag,
    pub _pad0: [i8; 6],
    pub tables: *mut *mut SpreadsheetTable,
    pub num_tables: i32,
    pub _pad1: [i8; 3],
    pub filter_flag: u8,
    pub row_filters: ListBaseT<SpreadsheetRowFilter>,
    pub geometry_id: SpreadsheetTableIDGeometry,
    pub flag: u32,
    pub table_use_clock: u32,
    pub active_viewer_path_index: i32,
    pub _pad2: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct SpreadsheetRowFilter {
    pub next: *mut SpreadsheetRowFilter,
    pub column_name: [i8; 64],
    pub operation: u8,
    pub flag: u8,
    pub _pad0: [i8; 6],
    pub value_int: i32,
    pub value_int2: [i32; 2],
    pub value_int3: [i32; 3],
    pub value_string: *mut i8,
    pub value_float: f32,
    pub threshold: f32,
    pub value_float2: [f32; 2],
    pub value_float3: [f32; 3],
    pub value_float4: [f32; 4],
    pub value_color: [f32; 4],
    pub _pad1: [i8; 4],
}

impl Default for SpreadsheetRowFilter {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceProject {
    pub next: *mut SpaceLink,
    pub regionbase: ListBaseT<ARegion>,
    pub spacetype: i8,
    pub link_flag: i8,
    pub _pad0: [i8; 6],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BLI_mempool {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FileLayout {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FileList {
    pub _opaque: [u8; 0],
}



#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct wmOperator {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct wmTimer {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceUserPref_Runtime {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceProperties_Runtime {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceConsole_Runtime {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceNode_Runtime {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceOutliner_Runtime {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceSeq_Runtime {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceText_Runtime {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceSpreadsheet_Runtime {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpreadsheetColumnRuntime {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TreeElement {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpaceFile_Runtime {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AssetRepresentation {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FileFolderLists {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FolderList {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FileListInternEntry {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpreadsheetClosureInputOutput {
    SPREADSHEET_CLOSURE_NONE = 0,
    SPREADSHEET_CLOSURE_INPUT = 1,
    SPREADSHEET_CLOSURE_OUTPUT = 2,
}

impl Default for SpreadsheetClosureInputOutput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const SPREADSHEET_CLOSURE_NONE: i32 = SpreadsheetClosureInputOutput::SPREADSHEET_CLOSURE_NONE as i32;
pub const SPREADSHEET_CLOSURE_INPUT: i32 = SpreadsheetClosureInputOutput::SPREADSHEET_CLOSURE_INPUT as i32;
pub const SPREADSHEET_CLOSURE_OUTPUT: i32 = SpreadsheetClosureInputOutput::SPREADSHEET_CLOSURE_OUTPUT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpreadsheetGeometryItemType {
    SPREADSHEET_GEOMETRY_ITEM_TYPE_DOMAIN = 0,
    SPREADSHEET_GEOMETRY_ITEM_TYPE_BUNDLE = 1,
}

impl Default for SpreadsheetGeometryItemType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const SPREADSHEET_GEOMETRY_ITEM_TYPE_DOMAIN: i32 = SpreadsheetGeometryItemType::SPREADSHEET_GEOMETRY_ITEM_TYPE_DOMAIN as i32;
pub const SPREADSHEET_GEOMETRY_ITEM_TYPE_BUNDLE: i32 = SpreadsheetGeometryItemType::SPREADSHEET_GEOMETRY_ITEM_TYPE_BUNDLE as i32;
