//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpace_Link_Flag(pub i8);

impl eSpace_Link_Flag {
    pub const SPACE_FLAG_TYPE_TEMPORARY: Self = Self(((1 << 0)) as i8);
    pub const SPACE_FLAG_TYPE_WAS_ACTIVE: Self = Self(((1 << 1)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceInfo_RptMask(pub i8);

impl eSpaceInfo_RptMask {
    pub const INFO_RPT_DEBUG: Self = Self(((1 << 0)) as i8);
    pub const INFO_RPT_INFO: Self = Self(((1 << 1)) as i8);
    pub const INFO_RPT_OP: Self = Self(((1 << 2)) as i8);
    pub const INFO_RPT_WARN: Self = Self(((1 << 3)) as i8);
    pub const INFO_RPT_ERR: Self = Self(((1 << 4)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceButtons_Context_Legacy(pub i16);

impl eSpaceButtons_Context_Legacy {
    pub const CONTEXT_SCENE: Self = Self((0) as i16);
    pub const CONTEXT_OBJECT: Self = Self((1) as i16);
    pub const CONTEXT_SHADING: Self = Self((3) as i16);
    pub const CONTEXT_EDITING: Self = Self((4) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceButtons_Legacy(pub i16);

impl eSpaceButtons_Legacy {
    pub const BUTS_LAMP: Self = Self((1) as i16);
    pub const BUTS_MAT: Self = Self((2) as i16);
    pub const BUTS_TEX: Self = Self((3) as i16);
    pub const BUTS_ANIM: Self = Self((4) as i16);
    pub const BUTS_WORLD: Self = Self((5) as i16);
    pub const BUTS_RENDER: Self = Self((6) as i16);
    pub const BUTS_EDIT: Self = Self((7) as i16);
    pub const BUTS_FPAINT: Self = Self((9) as i16);
    pub const BUTS_RADIO: Self = Self((10) as i16);
    pub const BUTS_SCRIPT: Self = Self((11) as i16);
    pub const BUTS_CONSTRAINT: Self = Self((13) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceButtons_Context(pub i16);

impl eSpaceButtons_Context {
    pub const BCONTEXT_SEPARATOR: Self = Self((-1) as i16);
    pub const BCONTEXT_RENDER: Self = Self((0) as i16);
    pub const BCONTEXT_SCENE: Self = Self((1) as i16);
    pub const BCONTEXT_WORLD: Self = Self((2) as i16);
    pub const BCONTEXT_OBJECT: Self = Self((3) as i16);
    pub const BCONTEXT_DATA: Self = Self((4) as i16);
    pub const BCONTEXT_MATERIAL: Self = Self((5) as i16);
    pub const BCONTEXT_TEXTURE: Self = Self((6) as i16);
    pub const BCONTEXT_PARTICLE: Self = Self((7) as i16);
    pub const BCONTEXT_PHYSICS: Self = Self((8) as i16);
    pub const BCONTEXT_BONE: Self = Self((9) as i16);
    pub const BCONTEXT_MODIFIER: Self = Self((10) as i16);
    pub const BCONTEXT_CONSTRAINT: Self = Self((11) as i16);
    pub const BCONTEXT_BONE_CONSTRAINT: Self = Self((12) as i16);
    pub const BCONTEXT_VIEW_LAYER: Self = Self((13) as i16);
    pub const BCONTEXT_TOOL: Self = Self((14) as i16);
    pub const BCONTEXT_SHADERFX: Self = Self((15) as i16);
    pub const BCONTEXT_OUTPUT: Self = Self((16) as i16);
    pub const BCONTEXT_COLLECTION: Self = Self((17) as i16);
    pub const BCONTEXT_STRIP: Self = Self((18) as i16);
    pub const BCONTEXT_STRIP_MODIFIER: Self = Self((19) as i16);
    pub const BCONTEXT_COMPOSITOR: Self = Self((20) as i16);
    pub const BCONTEXT_TOT: Self = Self(22 as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceButtons_Flag(pub i8);

impl eSpaceButtons_Flag {
    pub const SB_PIN_CONTEXT: Self = Self(((1 << 1)) as i8);
    pub const SB_FLAG_UNUSED_2: Self = Self(((1 << 2)) as i8);
    pub const SB_FLAG_UNUSED_3: Self = Self(((1 << 3)) as i8);
    pub const SB_TEX_USER_LIMITED: Self = Self(((1 << 3)) as i8);
    pub const SB_SHADING_CONTEXT: Self = Self(((1 << 4)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceButtons_OutlinerSync(pub i8);

impl eSpaceButtons_OutlinerSync {
    pub const PROPERTIES_SYNC_AUTO: Self = Self((0) as i8);
    pub const PROPERTIES_SYNC_NEVER: Self = Self((1) as i8);
    pub const PROPERTIES_SYNC_ALWAYS: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceOutliner_Flag(pub i16);

impl eSpaceOutliner_Flag {
    pub const SO_FLAG_UNUSED_1: Self = Self(((1 << 2)) as i16);
    pub const SO_FLAG_UNUSED_4: Self = Self(((1 << 4)) as i16);
    pub const SO_SYNC_SELECT: Self = Self(((1 << 5)) as i16);
    pub const SO_MODE_COLUMN: Self = Self(((1 << 6)) as i16);
    pub const SO_SCROLL_TO_ACTIVE: Self = Self(((1 << 7)) as i16);
    pub const SO_EXPAND_ON_FOCUS: Self = Self(((1 << 8)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceOutliner_SortMethod(pub i16);

impl eSpaceOutliner_SortMethod {
    pub const SO_SORT_NONE: Self = Self((0) as i16);
    pub const SO_SORT_ALPHA: Self = Self((1) as i16);
    pub const SO_SORT_CUSTOM: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceOutliner_Filter(pub i32);

impl eSpaceOutliner_Filter {
    pub const SO_FILTER_SEARCH: Self = Self(((1 << 0)) as i32);
    pub const SO_FILTER_CLEARED_1: Self = Self(((1 << 1)) as i32);
    pub const SO_FILTER_NO_OBJECT: Self = Self(((1 << 2)) as i32);
    pub const SO_FILTER_NO_OB_CONTENT: Self = Self(((1 << 3)) as i32);
    pub const SO_FILTER_NO_CHILDREN: Self = Self(((1 << 4)) as i32);
    pub const SO_FILTER_UNUSED_5: Self = Self(((1 << 5)) as i32);
    pub const SO_FILTER_SHOW_SYSTEM_OVERRIDES: Self = Self(6 as i32);
    pub const SO_FILTER_NO_OB_MESH: Self = Self(((1 << 6)) as i32);
    pub const SO_FILTER_NO_OB_ARMATURE: Self = Self(((1 << 7)) as i32);
    pub const SO_FILTER_NO_OB_EMPTY: Self = Self(((1 << 8)) as i32);
    pub const SO_FILTER_NO_OB_LAMP: Self = Self(((1 << 9)) as i32);
    pub const SO_FILTER_NO_OB_CAMERA: Self = Self(((1 << 10)) as i32);
    pub const SO_FILTER_NO_OB_OTHERS: Self = Self(((1 << 11)) as i32);
    pub const SO_FILTER_OB_STATE_SELECTABLE: Self = Self(((1 << 12)) as i32);
    pub const SO_FILTER_OB_STATE_VISIBLE: Self = Self(((1 << 13)) as i32);
    pub const SO_FILTER_OB_STATE_INVERSE: Self = Self(((1 << 14)) as i32);
    pub const SO_FILTER_OB_STATE_SELECTED: Self = Self(((1 << 15)) as i32);
    pub const SO_FILTER_OB_STATE_ACTIVE: Self = Self(((1 << 16)) as i32);
    pub const SO_FILTER_NO_COLLECTION: Self = Self(((1 << 17)) as i32);
    pub const SO_FILTER_NO_VIEW_LAYERS: Self = Self(((1 << 18)) as i32);
    pub const SO_FILTER_ID_TYPE: Self = Self(((1 << 19)) as i32);
    pub const SO_FILTER_NO_OB_GREASE_PENCIL: Self = Self(((1 << 20)) as i32);
    pub const SO_FILTER_NO_OB_DATA: Self = Self(((1 << 21)) as i32);
    pub const SO_FILTER_NO_OB_ANIMATION: Self = Self(((1 << 22)) as i32);
    pub const SO_FILTER_NO_OB_CONSTRAINTS: Self = Self(((1 << 23)) as i32);
    pub const SO_FILTER_NO_OB_SHAPE_KEYS: Self = Self(((1 << 24)) as i32);
    pub const SO_FILTER_NO_OB_MATERIAL: Self = Self(((1 << 25)) as i32);
    pub const SO_FILTER_NO_OB_DEFGROUP: Self = Self(((1 << 26)) as i32);
    pub const SO_FILTER_NO_OB_MODIFIERS: Self = Self(((1 << 27)) as i32);
    pub const SO_FILTER_NO_ARMATURE_BONE_COLLECTION: Self = Self(((1 << 28)) as i32);
    pub const SO_FILTER_NO_GREASE_PENCIL_EFFECTS: Self = Self(((1 << 29)) as i32);
    pub const SO_FILTER_NO_POSE_BONES: Self = Self(((1 << 30)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceOutliner_StateFilter(pub i8);

impl eSpaceOutliner_StateFilter {
    pub const SO_FILTER_OB_ALL: Self = Self((0) as i8);
    pub const SO_FILTER_OB_VISIBLE: Self = Self((1) as i8);
    pub const SO_FILTER_OB_HIDDEN: Self = Self((2) as i8);
    pub const SO_FILTER_OB_SELECTED: Self = Self((3) as i8);
    pub const SO_FILTER_OB_ACTIVE: Self = Self((4) as i8);
    pub const SO_FILTER_OB_SELECTABLE: Self = Self((5) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceOutliner_ShowRestrictFlag(pub i8);

impl eSpaceOutliner_ShowRestrictFlag {
    pub const SO_RESTRICT_ENABLE: Self = Self(((1 << 0)) as i8);
    pub const SO_RESTRICT_SELECT: Self = Self(((1 << 1)) as i8);
    pub const SO_RESTRICT_HIDE: Self = Self(((1 << 2)) as i8);
    pub const SO_RESTRICT_VIEWPORT: Self = Self(((1 << 3)) as i8);
    pub const SO_RESTRICT_RENDER: Self = Self(((1 << 4)) as i8);
    pub const SO_RESTRICT_HOLDOUT: Self = Self(((1 << 5)) as i8);
    pub const SO_RESTRICT_INDIRECT_ONLY: Self = Self(((1 << 6)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceOutliner_Mode(pub i16);

impl eSpaceOutliner_Mode {
    pub const SO_SCENES: Self = Self((0) as i16);
    pub const SO_LIBRARIES: Self = Self((7) as i16);
    pub const SO_SEQUENCE: Self = Self((10) as i16);
    pub const SO_DATA_API: Self = Self((11) as i16);
    pub const SO_ID_ORPHANS: Self = Self((14) as i16);
    pub const SO_VIEW_LAYER: Self = Self((15) as i16);
    pub const SO_OVERRIDES_LIBRARY: Self = Self((16) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceOutliner_LibOverrideViewMode(pub i16);

impl eSpaceOutliner_LibOverrideViewMode {
    pub const SO_LIB_OVERRIDE_VIEW_PROPERTIES: Self = Self((0) as i16);
    pub const SO_LIB_OVERRIDE_VIEW_HIERARCHIES: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceOutliner_StoreFlag(pub i16);

impl eSpaceOutliner_StoreFlag {
    pub const SO_TREESTORE_CLEANUP: Self = Self(((1 << 0)) as i16);
    pub const SO_TREESTORE_UNUSED_1: Self = Self(((1 << 1)) as i16);
    pub const SO_TREESTORE_REBUILD: Self = Self(((1 << 2)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceOutliner_Search_Flags(pub i8);

impl eSpaceOutliner_Search_Flags {
    pub const SO_FIND_CASE_SENSITIVE: Self = Self(((1 << 0)) as i8);
    pub const SO_FIND_COMPLETE: Self = Self(((1 << 1)) as i8);
    pub const SO_SEARCH_RECURSIVE: Self = Self(((1 << 2)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eGraphEdit_Flag(pub i32);

impl eGraphEdit_Flag {
    pub const SIPO_NOTRANSKEYCULL: Self = Self(((1 << 1)) as i32);
    pub const SIPO_NOHANDLES: Self = Self(((1 << 2)) as i32);
    pub const SIPO_AUTOLOCK_AXIS: Self = Self(((1 << 3)) as i32);
    pub const SIPO_DRAWTIME: Self = Self(((1 << 4)) as i32);
    pub const SIPO_SLIDERS: Self = Self(((1 << 7)) as i32);
    pub const SIPO_NODRAWCURSOR: Self = Self(((1 << 8)) as i32);
    pub const SIPO_SELVHANDLESONLY: Self = Self(((1 << 9)) as i32);
    pub const SIPO_NOREALTIMEUPDATES: Self = Self(((1 << 11)) as i32);
    pub const SIPO_NORMALIZE: Self = Self(((1 << 14)) as i32);
    pub const SIPO_NORMALIZE_FREEZE: Self = Self(((1 << 15)) as i32);
    pub const SIPO_SHOW_MARKERS: Self = Self(((1 << 16)) as i32);
    pub const SIPO_NO_DRAW_EXTRAPOLATION: Self = Self(((1 << 17)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eGraphEdit_Mode(pub i16);

impl eGraphEdit_Mode {
    pub const SIPO_MODE_ANIMATION: Self = Self((0) as i16);
    pub const SIPO_MODE_DRIVERS: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eGraphEdit_Runtime_Flag(pub i8);

impl eGraphEdit_Runtime_Flag {
    pub const SIPO_RUNTIME_FLAG_NEED_CHAN_SYNC: Self = Self(((1 << 0)) as i8);
    pub const SIPO_RUNTIME_FLAG_NEED_CHAN_SYNC_COLOR: Self = Self(((1 << 1)) as i8);
    pub const SIPO_RUNTIME_FLAG_TWEAK_HANDLES_LEFT: Self = Self(((1 << 2)) as i8);
    pub const SIPO_RUNTIME_FLAG_TWEAK_HANDLES_RIGHT: Self = Self(((1 << 3)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceNla_Flag(pub i16);

impl eSpaceNla_Flag {
    pub const SNLA_FLAG_UNUSED_0: Self = Self(((1 << 0)) as i16);
    pub const SNLA_FLAG_UNUSED_1: Self = Self(((1 << 1)) as i16);
    pub const SNLA_DRAWTIME: Self = Self(((1 << 2)) as i16);
    pub const SNLA_FLAG_UNUSED_3: Self = Self(((1 << 3)) as i16);
    pub const SNLA_NOSTRIPCURVES: Self = Self(((1 << 5)) as i16);
    pub const SNLA_NOREALTIMEUPDATES: Self = Self(((1 << 6)) as i16);
    pub const SNLA_NOLOCALMARKERS: Self = Self(((1 << 7)) as i16);
    pub const SNLA_SHOW_MARKERS: Self = Self(((1 << 8)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceSeq_SequencerPreviewOverlay_Flag(pub i32);

impl eSpaceSeq_SequencerPreviewOverlay_Flag {
    pub const SEQ_PREVIEW_SHOW_2D_CURSOR: Self = Self(((1 << 1)) as i32);
    pub const SEQ_PREVIEW_SHOW_OUTLINE_SELECTED: Self = Self(((1 << 2)) as i32);
    pub const SEQ_PREVIEW_SHOW_SAFE_MARGINS: Self = Self(((1 << 3)) as i32);
    pub const SEQ_PREVIEW_SHOW_GPENCIL: Self = Self(((1 << 4)) as i32);
    pub const SEQ_PREVIEW_SHOW_SAFE_CENTER: Self = Self(((1 << 9)) as i32);
    pub const SEQ_PREVIEW_SHOW_METADATA: Self = Self(((1 << 10)) as i32);
    pub const SEQ_PREVIEW_SHOW_COMPOSITION_GUIDES: Self = Self(((1 << 11)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceSeq_SequencerTimelineOverlay_Flag(pub i32);

impl eSpaceSeq_SequencerTimelineOverlay_Flag {
    pub const SEQ_TIMELINE_SHOW_STRIP_OFFSETS: Self = Self(((1 << 1)) as i32);
    pub const SEQ_TIMELINE_STRIP_END_THUMBNAILS: Self = Self(((1 << 2)) as i32);
    pub const SEQ_TIMELINE_SHOW_STRIP_COLOR_TAG: Self = Self(((1 << 3)) as i32);
    pub const SEQ_TIMELINE_SHOW_STRIP_RETIMING: Self = Self(((1 << 4)) as i32);
    pub const SEQ_TIMELINE_SHOW_FCURVES: Self = Self(((1 << 5)) as i32);
    pub const SEQ_TIMELINE_ALL_WAVEFORMS: Self = Self(((1 << 7)) as i32);
    pub const SEQ_TIMELINE_NO_WAVEFORMS: Self = Self(((1 << 8)) as i32);
    pub const SEQ_TIMELINE_WAVEFORMS_HALF: Self = Self(((1 << 9)) as i32);
    pub const SEQ_TIMELINE_SHOW_STRIP_NAME: Self = Self(((1 << 14)) as i32);
    pub const SEQ_TIMELINE_SHOW_STRIP_SOURCE: Self = Self(((1 << 15)) as i32);
    pub const SEQ_TIMELINE_SHOW_STRIP_DURATION: Self = Self(((1 << 16)) as i32);
    pub const SEQ_TIMELINE_SHOW_GRID: Self = Self(((1 << 18)) as i32);
    pub const SEQ_TIMELINE_CONTINUOUS_THUMBNAILS: Self = Self(((1 << 19)) as i32);
    pub const SEQ_TIMELINE_MIDDLE_THUMBNAILS: Self = Self(((1 << 20)) as i32);
    pub const SEQ_TIMELINE_SHOW_THUMBNAILS: Self = Self(((1 << 21)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceSeq_SequencerCacheOverlay_Flag(pub i32);

impl eSpaceSeq_SequencerCacheOverlay_Flag {
    pub const SEQ_CACHE_SHOW: Self = Self(((1 << 1)) as i32);
    pub const SEQ_CACHE_SHOW_RAW: Self = Self(((1 << 2)) as i32);
    pub const SEQ_CACHE_SHOW_FINAL_OUT: Self = Self(((1 << 5)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceSeq_RegionType(pub i16);

impl eSpaceSeq_RegionType {
    pub const SEQ_DRAW_IMG_IMBUF: Self = Self((1) as i16);
    pub const SEQ_DRAW_IMG_WAVEFORM: Self = Self((2) as i16);
    pub const SEQ_DRAW_IMG_VECTORSCOPE: Self = Self((3) as i16);
    pub const SEQ_DRAW_IMG_HISTOGRAM: Self = Self((4) as i16);
    pub const SEQ_DRAW_IMG_RGBPARADE: Self = Self((5) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceSeq_DrawFlag(pub i8);

impl eSpaceSeq_DrawFlag {
    pub const SEQ_DRAW_UNUSED_0: Self = Self(((1 << 0)) as i8);
    pub const SEQ_DRAW_UNUSED_1: Self = Self(((1 << 1)) as i8);
    pub const SEQ_DRAW_TRANSFORM_PREVIEW: Self = Self(((1 << 2)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceSeq_Flag(pub i32);

impl eSpaceSeq_Flag {
    pub const SEQ_DRAWFRAMES: Self = Self(((1 << 0)) as i32);
    pub const SEQ_MARKER_TRANS: Self = Self(((1 << 1)) as i32);
    pub const SEQ_DRAW_COLOR_SEPARATED_UNUSED_2: Self = Self(((1 << 2)) as i32);
    pub const SEQ_CLAMP_VIEW: Self = Self(((1 << 3)) as i32);
    pub const SPACE_SEQ_DESELECT_STRIP_HANDLE: Self = Self(((1 << 4)) as i32);
    pub const SPACE_SEQ_FLAG_UNUSED_5: Self = Self(((1 << 5)) as i32);
    pub const SEQ_USE_ALPHA: Self = Self(((1 << 6)) as i32);
    pub const SPACE_SEQ_FLAG_UNUSED_10: Self = Self(((1 << 10)) as i32);
    pub const SEQ_SHOW_MARKERS: Self = Self(((1 << 11)) as i32);
    pub const SEQ_ZOOM_TO_FIT: Self = Self(((1 << 12)) as i32);
    pub const SEQ_SHOW_OVERLAY: Self = Self(((1 << 13)) as i32);
    pub const SPACE_SEQ_FLAG_UNUSED_14: Self = Self(((1 << 14)) as i32);
    pub const SPACE_SEQ_FLAG_UNUSED_15: Self = Self(((1 << 15)) as i32);
    pub const SPACE_SEQ_FLAG_UNUSED_16: Self = Self(((1 << 16)) as i32);
    pub const SEQ_USE_PROXIES: Self = Self(((1 << 17)) as i32);
    pub const SEQ_SHOW_GRID: Self = Self(((1 << 18)) as i32);
    pub const SEQ_SHOW_SCRUBBING_REGION: Self = Self(((1 << 19)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceSeq_Displays(pub i8);

impl eSpaceSeq_Displays {
    pub const SEQ_VIEW_SEQUENCE: Self = Self((1) as i8);
    pub const SEQ_VIEW_PREVIEW: Self = Self((2) as i8);
    pub const SEQ_VIEW_SEQUENCE_PREVIEW: Self = Self((3) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceSeq_Proxy_RenderSize(pub i16);

impl eSpaceSeq_Proxy_RenderSize {
    pub const SEQ_RENDER_SIZE_NONE: Self = Self((-1) as i16);
    pub const SEQ_RENDER_SIZE_SCENE: Self = Self((0) as i16);
    pub const SEQ_RENDER_SIZE_PROXY_25: Self = Self((25) as i16);
    pub const SEQ_RENDER_SIZE_PROXY_50: Self = Self((50) as i16);
    pub const SEQ_RENDER_SIZE_PROXY_75: Self = Self((75) as i16);
    pub const SEQ_RENDER_SIZE_PROXY_100: Self = Self((99) as i16);
    pub const SEQ_RENDER_SIZE_FULL_DEPRECATED: Self = Self((100) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceSeq_Gizmo_Flag(pub i8);

impl eSpaceSeq_Gizmo_Flag {
    pub const SEQ_GIZMO_HIDE: Self = Self(((1 << 0)) as i8);
    pub const SEQ_GIZMO_HIDE_NAVIGATE: Self = Self(((1 << 1)) as i8);
    pub const SEQ_GIZMO_HIDE_CONTEXT: Self = Self(((1 << 2)) as i8);
    pub const SEQ_GIZMO_HIDE_TOOL: Self = Self(((1 << 3)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceSeq_OverlayFrameType(pub i8);

impl eSpaceSeq_OverlayFrameType {
    pub const SEQ_OVERLAY_FRAME_TYPE_RECT: Self = Self((0) as i8);
    pub const SEQ_OVERLAY_FRAME_TYPE_REFERENCE: Self = Self((1) as i8);
    pub const SEQ_OVERLAY_FRAME_TYPE_CURRENT: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eFileAssetImportMethod(pub i16);

impl eFileAssetImportMethod {
    pub const FILE_ASSET_IMPORT_LINK: Self = Self((0) as i16);
    pub const FILE_ASSET_IMPORT_APPEND: Self = Self((1) as i16);
    pub const FILE_ASSET_IMPORT_APPEND_REUSE: Self = Self((2) as i16);
    pub const FILE_ASSET_IMPORT_FOLLOW_PREFS: Self = Self((3) as i16);
    pub const FILE_ASSET_IMPORT_PACK: Self = Self((4) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eFileAssetImportFlags(pub i16);

impl eFileAssetImportFlags {
    pub const FILE_ASSET_IMPORT_INSTANCE_COLLECTIONS_ON_LINK: Self = Self(((1 << 0)) as i16);
    pub const FILE_ASSET_IMPORT_INSTANCE_COLLECTIONS_ON_APPEND: Self = Self(((1 << 1)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eFileSel_AssetParams_Flag(pub i32);

impl eFileSel_AssetParams_Flag {

}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eFileBrowse_Mode(pub i8);

impl eFileBrowse_Mode {
    pub const FILE_BROWSE_MODE_FILES: Self = Self((0) as i8);
    pub const FILE_BROWSE_MODE_ASSETS: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eFileDisplayType(pub i16);

impl eFileDisplayType {
    pub const FILE_DEFAULTDISPLAY: Self = Self((0) as i16);
    pub const FILE_VERTICALDISPLAY: Self = Self((1) as i16);
    pub const FILE_HORIZONTALDISPLAY: Self = Self((2) as i16);
    pub const FILE_IMGDISPLAY: Self = Self((3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eFileSortType(pub i16);

impl eFileSortType {
    pub const FILE_SORT_DEFAULT: Self = Self((0) as i16);
    pub const FILE_SORT_ALPHA: Self = Self((1) as i16);
    pub const FILE_SORT_EXTENSION: Self = Self((2) as i16);
    pub const FILE_SORT_TIME: Self = Self((3) as i16);
    pub const FILE_SORT_SIZE: Self = Self((4) as i16);
    pub const FILE_SORT_ASSET_CATALOG: Self = Self((5) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eFileTags(pub i16);

impl eFileTags {
    pub const FILE_TAG_REBUILD_MAIN_FILES: Self = Self(((1 << 0)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eFileDetails(pub i8);

impl eFileDetails {
    pub const FILE_DETAILS_SIZE: Self = Self(((1 << 0)) as i8);
    pub const FILE_DETAILS_DATETIME: Self = Self(((1 << 1)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eFileSelectType(pub i16);

impl eFileSelectType {
    pub const FILE_LOADLIB: Self = Self((1) as i16);
    pub const FILE_MAIN_ASSET: Self = Self((3) as i16);
    pub const FILE_ASSET_LIBRARY: Self = Self((4) as i16);
    pub const FILE_ASSET_LIBRARY_ALL: Self = Self((5) as i16);
    pub const FILE_ASSET_LIBRARY_REMOTE: Self = Self((6) as i16);
    pub const FILE_ASSET_LIBRARY_ESSENTIALS: Self = Self((7) as i16);
    pub const FILE_UNIX: Self = Self((8) as i16);
    pub const FILE_BLENDER: Self = Self((8) as i16);
    pub const FILE_SPECIAL: Self = Self((9) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eFileSel_Params_Flag(pub i16);

impl eFileSel_Params_Flag {
    pub const FILE_PARAMS_FLAG_UNUSED_1: Self = Self(((1 << 0)) as i16);
    pub const FILE_RELPATH: Self = Self(((1 << 1)) as i16);
    pub const FILE_LINK: Self = Self(((1 << 2)) as i16);
    pub const FILE_HIDE_DOT: Self = Self(((1 << 3)) as i16);
    pub const FILE_AUTOSELECT: Self = Self(((1 << 4)) as i16);
    pub const FILE_ACTIVE_COLLECTION: Self = Self(((1 << 5)) as i16);
    pub const FILE_PARAMS_FLAG_UNUSED_2: Self = Self(((1 << 6)) as i16);
    pub const FILE_DIRSEL_ONLY: Self = Self(((1 << 7)) as i16);
    pub const FILE_FILTER: Self = Self(((1 << 8)) as i16);
    pub const FILE_PARAMS_FLAG_UNUSED_3: Self = Self(((1 << 9)) as i16);
    pub const FILE_PATH_TOKENS_ALLOW: Self = Self(((1 << 10)) as i16);
    pub const FILE_SORT_INVERT: Self = Self(((1 << 11)) as i16);
    pub const FILE_HIDE_TOOL_PROPS: Self = Self(((1 << 12)) as i16);
    pub const FILE_CHECK_EXISTING: Self = Self(((1 << 13)) as i16);
    pub const FILE_ASSETS_ONLY: Self = Self(((1 << 14)) as i16);
    pub const FILE_FILTER_ASSET_CATALOG: Self = Self(15 as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eFileSel_Params_AssetCatalogVisibility(pub i16);

impl eFileSel_Params_AssetCatalogVisibility {
    pub const FILE_SHOW_ASSETS_ALL_CATALOGS: Self = Self(0 as i16);
    pub const FILE_SHOW_ASSETS_FROM_CATALOG: Self = Self(1 as i16);
    pub const FILE_SHOW_ASSETS_WITHOUT_CATALOG: Self = Self(2 as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eFileSel_Params_RenameFlag(pub i16);

impl eFileSel_Params_RenameFlag {
    pub const FILE_PARAMS_RENAME_PENDING: Self = Self((1 << 0) as i16);
    pub const FILE_PARAMS_RENAME_ACTIVE: Self = Self((1 << 1) as i16);
    pub const FILE_PARAMS_RENAME_POSTSCROLL_PENDING: Self = Self((1 << 2) as i16);
    pub const FILE_PARAMS_RENAME_POSTSCROLL_ACTIVE: Self = Self((1 << 3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eFileSel_File_Types(pub i32);

impl eFileSel_File_Types {
    pub const FILE_TYPE_BLENDER: Self = Self(((1 << 2)) as i32);
    pub const FILE_TYPE_BLENDER_BACKUP: Self = Self(((1 << 3)) as i32);
    pub const FILE_TYPE_IMAGE: Self = Self(((1 << 4)) as i32);
    pub const FILE_TYPE_MOVIE: Self = Self(((1 << 5)) as i32);
    pub const FILE_TYPE_PYSCRIPT: Self = Self(((1 << 6)) as i32);
    pub const FILE_TYPE_FTFONT: Self = Self(((1 << 7)) as i32);
    pub const FILE_TYPE_SOUND: Self = Self(((1 << 8)) as i32);
    pub const FILE_TYPE_TEXT: Self = Self(((1 << 9)) as i32);
    pub const FILE_TYPE_ARCHIVE: Self = Self(((1 << 10)) as i32);
    pub const FILE_TYPE_FOLDER: Self = Self(((1 << 11)) as i32);
    pub const FILE_TYPE_BTX: Self = Self(((1 << 12)) as i32);
    pub const FILE_TYPE_UNUSED_13: Self = Self(((1 << 13)) as i32);
    pub const FILE_TYPE_OPERATOR: Self = Self(((1 << 14)) as i32);
    pub const FILE_TYPE_BUNDLE: Self = Self(((1 << 15)) as i32);
    pub const FILE_TYPE_ALEMBIC: Self = Self(((1 << 16)) as i32);
    pub const FILE_TYPE_OBJECT_IO: Self = Self(((1 << 17)) as i32);
    pub const FILE_TYPE_USD: Self = Self(((1 << 18)) as i32);
    pub const FILE_TYPE_VOLUME: Self = Self(((1 << 19)) as i32);
    pub const FILE_TYPE_ASSET: Self = Self(((1 << 28)) as i32);
    pub const FILE_TYPE_ASSET_ONLINE: Self = Self(((1 << 29)) as i32);
    pub const FILE_TYPE_DIR: Self = Self(((1 << 30)) as i32);
    pub const FILE_TYPE_BLENDERLIB: Self = Self(21 as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eDirEntry_SelectFlag(pub i8);

impl eDirEntry_SelectFlag {
    pub const FILE_SEL_HIGHLIGHTED: Self = Self(((1 << 2)) as i8);
    pub const FILE_SEL_SELECTED: Self = Self(((1 << 3)) as i8);
    pub const FILE_SEL_EDITING: Self = Self(((1 << 4)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eFileEntry_Flag(pub i16);

impl eFileEntry_Flag {
    pub const FILE_ENTRY_INVALID_PREVIEW: Self = Self((1 << 0) as i16);
    pub const FILE_ENTRY_NAME_FREE: Self = Self((1 << 1) as i16);
    pub const FILE_ENTRY_PREVIEW_LOADING: Self = Self((1 << 2) as i16);
    pub const FILE_ENTRY_BLENDERLIB_NO_PREVIEW: Self = Self((1 << 3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceImage_GridShapeSource(pub i8);

impl eSpaceImage_GridShapeSource {
    pub const SI_GRID_SHAPE_DYNAMIC: Self = Self((0) as i8);
    pub const SI_GRID_SHAPE_FIXED: Self = Self((1) as i8);
    pub const SI_GRID_SHAPE_PIXEL: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceImage_UVDT(pub i8);

impl eSpaceImage_UVDT {
    pub const SI_UVDT_OUTLINE: Self = Self((0) as i8);
    pub const SI_UVDT_DASH: Self = Self((1) as i8);
    pub const SI_UVDT_BLACK: Self = Self((2) as i8);
    pub const SI_UVDT_WHITE: Self = Self((3) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceImage_UVDT_Stretch(pub i8);

impl eSpaceImage_UVDT_Stretch {
    pub const SI_UVDT_STRETCH_ANGLE: Self = Self((0) as i8);
    pub const SI_UVDT_STRETCH_AREA: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceImage_PixelRoundMode(pub i8);

impl eSpaceImage_PixelRoundMode {
    pub const SI_PIXEL_ROUND_DISABLED: Self = Self((0) as i8);
    pub const SI_PIXEL_ROUND_CENTER: Self = Self((1) as i8);
    pub const SI_PIXEL_ROUND_CORNER: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceImage_Mode(pub i8);

impl eSpaceImage_Mode {
    pub const SI_MODE_VIEW: Self = Self((0) as i8);
    pub const SI_MODE_PAINT: Self = Self((1) as i8);
    pub const SI_MODE_MASK: Self = Self((2) as i8);
    pub const SI_MODE_UV: Self = Self((3) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceImage_Flag(pub i32);

impl eSpaceImage_Flag {
    pub const SI_FLAG_UNUSED_0: Self = Self(((1 << 0)) as i32);
    pub const SI_FLAG_UNUSED_1: Self = Self(((1 << 1)) as i32);
    pub const SI_CLIP_UV: Self = Self(((1 << 2)) as i32);
    pub const SI_FLAG_UNUSED_3: Self = Self(((1 << 3)) as i32);
    pub const SI_NO_DRAWFACES: Self = Self(((1 << 4)) as i32);
    pub const SI_DRAWSHADOW: Self = Self(((1 << 5)) as i32);
    pub const SI_FLAG_UNUSED_6: Self = Self(((1 << 6)) as i32);
    pub const SI_FLAG_UNUSED_7: Self = Self(((1 << 7)) as i32);
    pub const SI_FLAG_UNUSED_8: Self = Self(((1 << 8)) as i32);
    pub const SI_COORDFLOATS: Self = Self(((1 << 9)) as i32);
    pub const SI_FLAG_UNUSED_10: Self = Self(((1 << 10)) as i32);
    pub const SI_LIVE_UNWRAP: Self = Self(((1 << 11)) as i32);
    pub const SI_USE_ALPHA: Self = Self(((1 << 12)) as i32);
    pub const SI_SHOW_ALPHA: Self = Self(((1 << 13)) as i32);
    pub const SI_SHOW_ZBUF: Self = Self(((1 << 14)) as i32);
    pub const SI_PREVSPACE: Self = Self(((1 << 15)) as i32);
    pub const SI_FULLWINDOW: Self = Self(((1 << 16)) as i32);
    pub const SI_FLAG_UNUSED_17: Self = Self(((1 << 17)) as i32);
    pub const SI_FLAG_UNUSED_18: Self = Self(((1 << 18)) as i32);
    pub const SI_DRAW_TILE: Self = Self(((1 << 19)) as i32);
    pub const SI_FLAG_UNUSED_20: Self = Self(((1 << 20)) as i32);
    pub const SI_DRAW_STRETCH: Self = Self(((1 << 21)) as i32);
    pub const SI_SHOW_GPENCIL: Self = Self(((1 << 22)) as i32);
    pub const SI_FLAG_UNUSED_23: Self = Self(((1 << 23)) as i32);
    pub const SI_FLAG_UNUSED_24: Self = Self(((1 << 24)) as i32);
    pub const SI_NO_DRAW_TEXPAINT: Self = Self(((1 << 25)) as i32);
    pub const SI_DRAW_METADATA: Self = Self(((1 << 26)) as i32);
    pub const SI_SHOW_R: Self = Self(((1 << 27)) as i32);
    pub const SI_SHOW_G: Self = Self(((1 << 28)) as i32);
    pub const SI_SHOW_B: Self = Self(((1 << 29)) as i32);
    pub const SI_GRID_OVER_IMAGE: Self = Self(((1 << 30)) as i32);
    pub const SI_NO_DRAW_UV_GUIDE: Self = Self(((1 << 31)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceImageOverlay_Flag(pub i32);

impl eSpaceImageOverlay_Flag {
    pub const SI_OVERLAY_SHOW_OVERLAYS: Self = Self(((1 << 0)) as i32);
    pub const SI_OVERLAY_SHOW_GRID_BACKGROUND: Self = Self(((1 << 1)) as i32);
    pub const SI_OVERLAY_DRAW_RENDER_REGION: Self = Self(((1 << 2)) as i32);
    pub const SI_OVERLAY_DRAW_TEXT_INFO: Self = Self(((1 << 3)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceImage_Gizmo_Flag(pub i8);

impl eSpaceImage_Gizmo_Flag {
    pub const SI_GIZMO_HIDE: Self = Self(((1 << 0)) as i8);
    pub const SI_GIZMO_HIDE_NAVIGATE: Self = Self(((1 << 1)) as i8);
    pub const SI_GIZMO_HIDE_ACTIVE_NODE: Self = Self(((1 << 2)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceText_Flags(pub i16);

impl eSpaceText_Flags {
    pub const ST_SCROLL_SELECT: Self = Self(((1 << 0)) as i16);
    pub const ST_FLAG_UNUSED_4: Self = Self(((1 << 4)) as i16);
    pub const ST_FIND_WRAP: Self = Self(((1 << 5)) as i16);
    pub const ST_FIND_ALL: Self = Self(((1 << 6)) as i16);
    pub const ST_SHOW_MARGIN: Self = Self(((1 << 7)) as i16);
    pub const ST_MATCH_CASE: Self = Self(((1 << 8)) as i16);
    pub const ST_FLAG_UNUSED_9: Self = Self(((1 << 9)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceNodeOverlay_Flag(pub i32);

impl eSpaceNodeOverlay_Flag {
    pub const SN_OVERLAY_SHOW_OVERLAYS: Self = Self(((1 << 1)) as i32);
    pub const SN_OVERLAY_SHOW_WIRE_COLORS: Self = Self(((1 << 2)) as i32);
    pub const SN_OVERLAY_SHOW_TIMINGS: Self = Self(((1 << 3)) as i32);
    pub const SN_OVERLAY_SHOW_PATH: Self = Self(((1 << 4)) as i32);
    pub const SN_OVERLAY_SHOW_NAMED_ATTRIBUTES: Self = Self(((1 << 5)) as i32);
    pub const SN_OVERLAY_SHOW_PREVIEWS: Self = Self(((1 << 6)) as i32);
    pub const SN_OVERLAY_SHOW_REROUTE_AUTO_LABELS: Self = Self(((1 << 7)) as i32);
    pub const SN_OVERLAY_SHOW_RENDER_REGION: Self = Self(((1 << 8)) as i32);
    pub const SN_OVERLAY_SHOW_TEXT_INFO: Self = Self(((1 << 9)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceNodeOverlay_preview_shape(pub i32);

impl eSpaceNodeOverlay_preview_shape {
    pub const SN_OVERLAY_PREVIEW_FLAT: Self = Self((0) as i32);
    pub const SN_OVERLAY_PREVIEW_3D: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceNode_Flag(pub i16);

impl eSpaceNode_Flag {
    pub const SNODE_BACKDRAW: Self = Self(((1 << 1)) as i16);
    pub const SNODE_SHOW_GPENCIL: Self = Self(((1 << 2)) as i16);
    pub const SNODE_USE_ALPHA: Self = Self(((1 << 3)) as i16);
    pub const SNODE_SHOW_ALPHA: Self = Self(((1 << 4)) as i16);
    pub const SNODE_SHOW_R: Self = Self(((1 << 7)) as i16);
    pub const SNODE_SHOW_G: Self = Self(((1 << 8)) as i16);
    pub const SNODE_SHOW_B: Self = Self(((1 << 9)) as i16);
    pub const SNODE_FLAG_UNUSED_5: Self = Self(((1 << 5)) as i16);
    pub const SNODE_FLAG_UNUSED_6: Self = Self(((1 << 6)) as i16);
    pub const SNODE_FLAG_UNUSED_10: Self = Self(((1 << 10)) as i16);
    pub const SNODE_FLAG_UNUSED_11: Self = Self(((1 << 11)) as i16);
    pub const SNODE_PIN: Self = Self(((1 << 12)) as i16);
    pub const SNODE_FLAG_UNUSED_12: Self = Self(((1 << 13)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceNode_Gizmo_Flag(pub i8);

impl eSpaceNode_Gizmo_Flag {
    pub const SNODE_GIZMO_HIDE: Self = Self(((1 << 0)) as i8);
    pub const SNODE_GIZMO_HIDE_ACTIVE_NODE: Self = Self(((1 << 1)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceNode_TexFrom(pub i16);

impl eSpaceNode_TexFrom {
    pub const SNODE_TEX_WORLD: Self = Self((1) as i16);
    pub const SNODE_TEX_BRUSH: Self = Self((2) as i16);
    pub const SNODE_TEX_LINESTYLE: Self = Self((3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceNode_ShaderFrom(pub i8);

impl eSpaceNode_ShaderFrom {
    pub const SNODE_SHADER_OBJECT: Self = Self((0) as i8);
    pub const SNODE_SHADER_WORLD: Self = Self((1) as i8);
    pub const SNODE_SHADER_LINESTYLE: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SpaceNodeGeometryNodesType(pub i8);

impl SpaceNodeGeometryNodesType {
    pub const SNODE_GEOMETRY_MODIFIER: Self = Self((0) as i8);
    pub const SNODE_GEOMETRY_TOOL: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SpaceNodeCompositorNodesType(pub i8);

impl SpaceNodeCompositorNodesType {
    pub const SNODE_COMPOSITOR_SCENE: Self = Self((0) as i8);
    pub const SNODE_COMPOSITOR_SEQUENCER: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceNode_InsertOffsetDir(pub i8);

impl eSpaceNode_InsertOffsetDir {
    pub const SNODE_INSERTOFS_DIR_RIGHT: Self = Self((0) as i8);
    pub const SNODE_INSERTOFS_DIR_LEFT: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eConsoleLine_Type(pub i32);

impl eConsoleLine_Type {
    pub const CONSOLE_LINE_OUTPUT: Self = Self((0) as i32);
    pub const CONSOLE_LINE_INPUT: Self = Self((1) as i32);
    pub const CONSOLE_LINE_INFO: Self = Self((2) as i32);
    pub const CONSOLE_LINE_ERROR: Self = Self((3) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceClipOverlay_Flag(pub i32);

impl eSpaceClipOverlay_Flag {
    pub const SC_SHOW_OVERLAYS: Self = Self(((1 << 0)) as i32);
    pub const SC_SHOW_CURSOR: Self = Self(((1 << 1)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceClip_Flag(pub i32);

impl eSpaceClip_Flag {
    pub const SC_SHOW_MARKER_PATTERN: Self = Self(((1 << 0)) as i32);
    pub const SC_SHOW_MARKER_SEARCH: Self = Self(((1 << 1)) as i32);
    pub const SC_LOCK_SELECTION: Self = Self(((1 << 2)) as i32);
    pub const SC_SHOW_TINY_MARKER: Self = Self(((1 << 3)) as i32);
    pub const SC_SHOW_TRACK_PATH: Self = Self(((1 << 4)) as i32);
    pub const SC_SHOW_BUNDLES: Self = Self(((1 << 5)) as i32);
    pub const SC_MUTE_FOOTAGE: Self = Self(((1 << 6)) as i32);
    pub const SC_HIDE_DISABLED: Self = Self(((1 << 7)) as i32);
    pub const SC_SHOW_NAMES: Self = Self(((1 << 8)) as i32);
    pub const SC_SHOW_GRID: Self = Self(((1 << 9)) as i32);
    pub const SC_SHOW_STABLE: Self = Self(((1 << 10)) as i32);
    pub const SC_MANUAL_CALIBRATION: Self = Self(((1 << 11)) as i32);
    pub const SC_SHOW_ANNOTATION: Self = Self(((1 << 12)) as i32);
    pub const SC_SHOW_FILTERS: Self = Self(((1 << 13)) as i32);
    pub const SC_SHOW_GRAPH_FRAMES: Self = Self(((1 << 14)) as i32);
    pub const SC_SHOW_GRAPH_TRACKS_MOTION: Self = Self(((1 << 15)) as i32);
    pub const SC_LOCK_TIMECURSOR: Self = Self(((1 << 17)) as i32);
    pub const SC_SHOW_SECONDS: Self = Self(((1 << 18)) as i32);
    pub const SC_SHOW_GRAPH_SEL_ONLY: Self = Self(((1 << 19)) as i32);
    pub const SC_SHOW_GRAPH_HIDDEN: Self = Self(((1 << 20)) as i32);
    pub const SC_SHOW_GRAPH_TRACKS_ERROR: Self = Self(((1 << 21)) as i32);
    pub const SC_SHOW_METADATA: Self = Self(((1 << 22)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceClip_Mode(pub i16);

impl eSpaceClip_Mode {
    pub const SC_MODE_TRACKING: Self = Self((0) as i16);
    pub const SC_MODE_MASKEDIT: Self = Self((3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceClip_View(pub i16);

impl eSpaceClip_View {
    pub const SC_VIEW_CLIP: Self = Self((0) as i16);
    pub const SC_VIEW_GRAPH: Self = Self((1) as i16);
    pub const SC_VIEW_DOPESHEET: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceClip_GPencil_Source(pub i16);

impl eSpaceClip_GPencil_Source {
    pub const SC_GPENCIL_SRC_CLIP: Self = Self((0) as i16);
    pub const SC_GPENCIL_SRC_TRACK: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceClip_Gizmo_Flag(pub i8);

impl eSpaceClip_Gizmo_Flag {
    pub const SCLIP_GIZMO_HIDE: Self = Self(((1 << 0)) as i8);
    pub const SCLIP_GIZMO_HIDE_NAVIGATE: Self = Self(((1 << 1)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceSpreadsheet_Flag(pub i32);

impl eSpaceSpreadsheet_Flag {
    pub const SPREADSHEET_FLAG_PINNED: Self = Self(((1 << 0)) as i32);
    pub const SPREADSHEET_FLAG_CONTEXT_PATH_COLLAPSED_LEGACY: Self = Self(((1 << 1)) as i32);
    pub const SPREADSHEET_FLAG_SHOW_INTERNAL_ATTRIBUTES: Self = Self(((1 << 2)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceSpreadsheet_FilterFlag(pub u8);

impl eSpaceSpreadsheet_FilterFlag {
    pub const SPREADSHEET_FILTER_SELECTED_ONLY: Self = Self(((1 << 0)) as u8);
    pub const SPREADSHEET_FILTER_ENABLE: Self = Self(((1 << 1)) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceSpreadsheet_RowFilterFlag(pub u8);

impl eSpaceSpreadsheet_RowFilterFlag {
    pub const SPREADSHEET_ROW_FILTER_UI_EXPAND: Self = Self(((1 << 0)) as u8);
    pub const SPREADSHEET_ROW_FILTER_BOOL_VALUE: Self = Self(((1 << 1)) as u8);
    pub const SPREADSHEET_ROW_FILTER_ENABLED: Self = Self(((1 << 2)) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpreadsheetFilterOperation(pub u8);

impl eSpreadsheetFilterOperation {
    pub const SPREADSHEET_ROW_FILTER_EQUAL: Self = Self((0) as u8);
    pub const SPREADSHEET_ROW_FILTER_GREATER: Self = Self((1) as u8);
    pub const SPREADSHEET_ROW_FILTER_LESS: Self = Self((2) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceSpreadsheet_ObjectEvalState(pub u8);

impl eSpaceSpreadsheet_ObjectEvalState {
    pub const SPREADSHEET_OBJECT_EVAL_STATE_EVALUATED: Self = Self((0) as u8);
    pub const SPREADSHEET_OBJECT_EVAL_STATE_ORIGINAL: Self = Self((1) as u8);
    pub const SPREADSHEET_OBJECT_EVAL_STATE_VIEWER_NODE: Self = Self((2) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpaceSpreadsheet_ContextType(pub i32);

impl eSpaceSpreadsheet_ContextType {
    pub const SPREADSHEET_CONTEXT_OBJECT: Self = Self((0) as i32);
    pub const SPREADSHEET_CONTEXT_MODIFIER: Self = Self((1) as i32);
    pub const SPREADSHEET_CONTEXT_NODE: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpreadsheetColumnValueType(pub i32);

impl eSpreadsheetColumnValueType {
    pub const SPREADSHEET_VALUE_TYPE_UNKNOWN: Self = Self((-1) as i32);
    pub const SPREADSHEET_VALUE_TYPE_BOOL: Self = Self((0) as i32);
    pub const SPREADSHEET_VALUE_TYPE_INT32: Self = Self((1) as i32);
    pub const SPREADSHEET_VALUE_TYPE_FLOAT: Self = Self((2) as i32);
    pub const SPREADSHEET_VALUE_TYPE_FLOAT2: Self = Self((3) as i32);
    pub const SPREADSHEET_VALUE_TYPE_FLOAT3: Self = Self((4) as i32);
    pub const SPREADSHEET_VALUE_TYPE_COLOR: Self = Self((5) as i32);
    pub const SPREADSHEET_VALUE_TYPE_INSTANCES: Self = Self((6) as i32);
    pub const SPREADSHEET_VALUE_TYPE_STRING: Self = Self((7) as i32);
    pub const SPREADSHEET_VALUE_TYPE_BYTE_COLOR: Self = Self((8) as i32);
    pub const SPREADSHEET_VALUE_TYPE_INT8: Self = Self((9) as i32);
    pub const SPREADSHEET_VALUE_TYPE_INT32_2D: Self = Self((10) as i32);
    pub const SPREADSHEET_VALUE_TYPE_QUATERNION: Self = Self((11) as i32);
    pub const SPREADSHEET_VALUE_TYPE_FLOAT4X4: Self = Self((12) as i32);
    pub const SPREADSHEET_VALUE_TYPE_BUNDLE_ITEM: Self = Self((13) as i32);
    pub const SPREADSHEET_VALUE_TYPE_INT64: Self = Self((14) as i32);
    pub const SPREADSHEET_VALUE_TYPE_INT32_3D: Self = Self((15) as i32);
    pub const SPREADSHEET_VALUE_TYPE_FLOAT4: Self = Self((16) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpreadsheetColumnFlag(pub i32);

impl eSpreadsheetColumnFlag {
    pub const SPREADSHEET_COLUMN_FLAG_UNAVAILABLE: Self = Self(((1 << 0)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpreadsheetTableIDType(pub i32);

impl eSpreadsheetTableIDType {
    pub const SPREADSHEET_TABLE_ID_TYPE_GEOMETRY: Self = Self((0) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpreadsheetTableFlag(pub i32);

impl eSpreadsheetTableFlag {
    pub const SPREADSHEET_TABLE_FLAG_MANUALLY_EDITED: Self = Self(((1 << 0)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSpace_Type(pub i8);

impl eSpace_Type {
    pub const SPACE_EMPTY: Self = Self((0) as i8);
    pub const SPACE_VIEW3D: Self = Self((1) as i8);
    pub const SPACE_GRAPH: Self = Self((2) as i8);
    pub const SPACE_OUTLINER: Self = Self((3) as i8);
    pub const SPACE_PROPERTIES: Self = Self((4) as i8);
    pub const SPACE_FILE: Self = Self((5) as i8);
    pub const SPACE_IMAGE: Self = Self((6) as i8);
    pub const SPACE_INFO: Self = Self((7) as i8);
    pub const SPACE_SEQ: Self = Self((8) as i8);
    pub const SPACE_TEXT: Self = Self((9) as i8);
    pub const SPACE_IMASEL: Self = Self((10) as i8);
    pub const SPACE_SOUND: Self = Self((11) as i8);
    pub const SPACE_ACTION: Self = Self((12) as i8);
    pub const SPACE_NLA: Self = Self((13) as i8);
    pub const SPACE_SCRIPT: Self = Self((14) as i8);
    pub const SPACE_TIME: Self = Self((15) as i8);
    pub const SPACE_NODE: Self = Self((16) as i8);
    pub const SPACE_LOGIC: Self = Self((17) as i8);
    pub const SPACE_CONSOLE: Self = Self((18) as i8);
    pub const SPACE_USERPREF: Self = Self((19) as i8);
    pub const SPACE_CLIP: Self = Self((20) as i8);
    pub const SPACE_TOPBAR: Self = Self((21) as i8);
    pub const SPACE_STATUSBAR: Self = Self((22) as i8);
    pub const SPACE_SPREADSHEET: Self = Self((23) as i8);
    pub const SPACE_PROJECT: Self = Self((24) as i8);
}

