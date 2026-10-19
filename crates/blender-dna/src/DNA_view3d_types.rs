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
pub struct eHandleDisplay(pub i32);

impl eHandleDisplay {
    pub const CURVE_HANDLE_SELECTED: Self = Self((0) as i32);
    pub const CURVE_HANDLE_ALL: Self = Self((1) as i32);
    pub const CURVE_HANDLE_NONE: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eView3D_StereoFlag(pub i16);

impl eView3D_StereoFlag {
    pub const V3D_S3D_DISPCAMERAS: Self = Self((1 << 0) as i16);
    pub const V3D_S3D_DISPPLANE: Self = Self((1 << 1) as i16);
    pub const V3D_S3D_DISPVOLUME: Self = Self((1 << 2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eView3D_Flag(pub i16);

impl eView3D_Flag {
    pub const V3D_LOCAL_COLLECTIONS: Self = Self((1 << 0) as i16);
    pub const V3D_FLAG_UNUSED_1: Self = Self((1 << 1) as i16);
    pub const V3D_HIDE_HELPLINES: Self = Self((1 << 2) as i16);
    pub const V3D_FLAG_UNUSED_2: Self = Self((1 << 3) as i16);
    pub const V3D_XR_SESSION_MIRROR: Self = Self((1 << 4) as i16);
    pub const V3D_XR_SESSION_SURFACE: Self = Self((1 << 5) as i16);
    pub const V3D_FLAG_UNUSED_10: Self = Self((1 << 10) as i16);
    pub const V3D_SELECT_OUTLINE: Self = Self((1 << 11) as i16);
    pub const V3D_FLAG_UNUSED_12: Self = Self((1 << 12) as i16);
    pub const V3D_GLOBAL_STATS: Self = Self((1 << 13) as i16);
    pub const V3D_DRAW_CENTERS: Self = Self(10 as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eView3D_Runtime_Flag(pub i32);

impl eView3D_Runtime_Flag {
    pub const V3D_RUNTIME_XR_SESSION_ROOT: Self = Self(((1 << 0)) as i32);
    pub const V3D_RUNTIME_DEPTHBUF_OVERRIDDEN: Self = Self(((1 << 1)) as i32);
    pub const V3D_RUNTIME_LOCAL_MAYBE_EMPTY: Self = Self(((1 << 2)) as i32);
    pub const V3D_RUNTIME_OFS_LAST_CENTER_IS_VALID: Self = Self(((1 << 3)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRegionView3D_Persp(pub i8);

impl eRegionView3D_Persp {
    pub const RV3D_ORTHO: Self = Self((0) as i8);
    pub const RV3D_PERSP: Self = Self((1) as i8);
    pub const RV3D_CAMOB: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRegionView3D_Flag(pub i16);

impl eRegionView3D_Flag {
    pub const RV3D_CLIPPING: Self = Self((1 << 2) as i16);
    pub const RV3D_NAVIGATING: Self = Self((1 << 3) as i16);
    pub const RV3D_GPULIGHT_UPDATE: Self = Self((1 << 4) as i16);
    pub const RV3D_PAINTING: Self = Self((1 << 5) as i16);
    pub const RV3D_ZOFFSET_DISABLED: Self = Self((1 << 6) as i16);
    pub const RV3D_WAS_CAMOB: Self = Self((1 << 7) as i16);
    pub const RV3D_FLIP_X: Self = Self((1 << 8) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRegionView3D_ViewLock(pub i8);

impl eRegionView3D_ViewLock {
    pub const RV3D_LOCK_ROTATION: Self = Self(((1 << 0)) as i8);
    pub const RV3D_BOXVIEW: Self = Self(((1 << 1)) as i8);
    pub const RV3D_BOXCLIP: Self = Self(((1 << 2)) as i8);
    pub const RV3D_LOCK_LOCATION: Self = Self(((1 << 3)) as i8);
    pub const RV3D_LOCK_ZOOM_AND_DOLLY: Self = Self(((1 << 4)) as i8);
    pub const RV3D_LOCK_ANY_TRANSFORM: Self = Self(5 as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRegionView3D_ViewLockQuad(pub i8);

impl eRegionView3D_ViewLockQuad {
    pub const RV3D_VIEWLOCK_INIT: Self = Self((1 << 7) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRegionView3D_View(pub i8);

impl eRegionView3D_View {
    pub const RV3D_VIEW_USER: Self = Self((0) as i8);
    pub const RV3D_VIEW_FRONT: Self = Self((1) as i8);
    pub const RV3D_VIEW_BACK: Self = Self((2) as i8);
    pub const RV3D_VIEW_LEFT: Self = Self((3) as i8);
    pub const RV3D_VIEW_RIGHT: Self = Self((4) as i8);
    pub const RV3D_VIEW_TOP: Self = Self((5) as i8);
    pub const RV3D_VIEW_BOTTOM: Self = Self((6) as i8);
    pub const RV3D_VIEW_CAMERA: Self = Self((8) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRegionView3D_ViewAxisRoll(pub i8);

impl eRegionView3D_ViewAxisRoll {
    pub const RV3D_VIEW_AXIS_ROLL_0: Self = Self((0) as i8);
    pub const RV3D_VIEW_AXIS_ROLL_90: Self = Self((1) as i8);
    pub const RV3D_VIEW_AXIS_ROLL_180: Self = Self((2) as i8);
    pub const RV3D_VIEW_AXIS_ROLL_270: Self = Self((3) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRegionView3D_NDOFFlag(pub i8);

impl eRegionView3D_NDOFFlag {
    pub const RV3D_NDOF_OFS_IS_VALID: Self = Self(((1 << 0)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eView3D_Flag2(pub i32);

impl eView3D_Flag2 {
    pub const V3D_HIDE_OVERLAYS: Self = Self((1 << 2) as i32);
    pub const V3D_SHOW_VIEWER: Self = Self((1 << 3) as i32);
    pub const V3D_SHOW_ANNOTATION: Self = Self((1 << 4) as i32);
    pub const V3D_LOCK_CAMERA: Self = Self((1 << 5) as i32);
    pub const V3D_FLAG2_UNUSED_6: Self = Self((1 << 6) as i32);
    pub const V3D_SHOW_RECONSTRUCTION: Self = Self((1 << 7) as i32);
    pub const V3D_SHOW_CAMERAPATH: Self = Self((1 << 8) as i32);
    pub const V3D_SHOW_BUNDLENAME: Self = Self((1 << 9) as i32);
    pub const V3D_FLAG2_UNUSED_10: Self = Self((1 << 10) as i32);
    pub const V3D_RENDER_BORDER: Self = Self((1 << 11) as i32);
    pub const V3D_FLAG2_UNUSED_12: Self = Self((1 << 12) as i32);
    pub const V3D_FLAG2_UNUSED_13: Self = Self((1 << 13) as i32);
    pub const V3D_FLAG2_UNUSED_14: Self = Self((1 << 14) as i32);
    pub const V3D_FLAG2_UNUSED_15: Self = Self((1 << 15) as i32);
    pub const V3D_XR_SHOW_CONTROLLERS: Self = Self((1 << 16) as i32);
    pub const V3D_XR_SHOW_CUSTOM_OVERLAYS: Self = Self((1 << 17) as i32);
    pub const V3D_SHOW_CAMERA_GUIDES: Self = Self(((1 << 18)) as i32);
    pub const V3D_SHOW_CAMERA_PASSEPARTOUT: Self = Self(((1 << 19)) as i32);
    pub const V3D_XR_SHOW_PASSTHROUGH: Self = Self((1 << 20) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eView3D_GPFlag(pub i16);

impl eView3D_GPFlag {
    pub const V3D_GP_FADE_OBJECTS: Self = Self((1 << 0) as i16);
    pub const V3D_GP_SHOW_GRID: Self = Self((1 << 1) as i16);
    pub const V3D_GP_SHOW_EDIT_LINES: Self = Self((1 << 2) as i16);
    pub const V3D_GP_SHOW_MULTIEDIT_LINES: Self = Self((1 << 3) as i16);
    pub const V3D_GP_SHOW_ONION_SKIN: Self = Self((1 << 4) as i16);
    pub const V3D_GP_FADE_NOACTIVE_LAYERS: Self = Self((1 << 5) as i16);
    pub const V3D_GP_FADE_NOACTIVE_GPENCIL: Self = Self((1 << 6) as i16);
    pub const V3D_GP_SHOW_STROKE_DIRECTION: Self = Self((1 << 7) as i16);
    pub const V3D_GP_SHOW_MATERIAL_NAME: Self = Self((1 << 8) as i16);
    pub const V3D_GP_SHOW_GRID_XRAY: Self = Self((1 << 9) as i16);
    pub const V3D_GP_FORCE_STROKE_ORDER_3D: Self = Self((1 << 10) as i16);
    pub const V3D_GP_ONION_SKIN_ACTIVE_OBJECT: Self = Self((1 << 11) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eView3DShading_Flag(pub i16);

impl eView3DShading_Flag {
    pub const V3D_SHADING_OBJECT_OUTLINE: Self = Self(((1 << 0)) as i16);
    pub const V3D_SHADING_XRAY: Self = Self(((1 << 1)) as i16);
    pub const V3D_SHADING_SHADOW: Self = Self(((1 << 2)) as i16);
    pub const V3D_SHADING_SCENE_LIGHTS: Self = Self(((1 << 3)) as i16);
    pub const V3D_SHADING_SPECULAR_HIGHLIGHT: Self = Self(((1 << 4)) as i16);
    pub const V3D_SHADING_CAVITY: Self = Self(((1 << 5)) as i16);
    pub const V3D_SHADING_MATCAP_FLIP_X: Self = Self(((1 << 6)) as i16);
    pub const V3D_SHADING_SCENE_WORLD: Self = Self(((1 << 7)) as i16);
    pub const V3D_SHADING_XRAY_WIREFRAME: Self = Self(((1 << 8)) as i16);
    pub const V3D_SHADING_WORLD_ORIENTATION: Self = Self(((1 << 9)) as i16);
    pub const V3D_SHADING_BACKFACE_CULLING: Self = Self(((1 << 10)) as i16);
    pub const V3D_SHADING_DEPTH_OF_FIELD: Self = Self(((1 << 11)) as i16);
    pub const V3D_SHADING_SCENE_LIGHTS_RENDER: Self = Self(((1 << 12)) as i16);
    pub const V3D_SHADING_SCENE_WORLD_RENDER: Self = Self(((1 << 13)) as i16);
    pub const V3D_SHADING_STUDIOLIGHT_VIEW_ROTATION: Self = Self(((1 << 14)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eView3D_DebugFlag(pub i16);

impl eView3D_DebugFlag {
    pub const V3D_DEBUG_FREEZE_CULLING: Self = Self(((1 << 0)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eView3DShading_CavityType(pub i8);

impl eView3DShading_CavityType {
    pub const V3D_SHADING_CAVITY_SSAO: Self = Self((0) as i8);
    pub const V3D_SHADING_CAVITY_CURVATURE: Self = Self((1) as i8);
    pub const V3D_SHADING_CAVITY_BOTH: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct View3DShadingUseCompositor(pub i8);

impl View3DShadingUseCompositor {
    pub const V3D_SHADING_USE_COMPOSITOR_DISABLED: Self = Self((0) as i8);
    pub const V3D_SHADING_USE_COMPOSITOR_CAMERA: Self = Self((1) as i8);
    pub const V3D_SHADING_USE_COMPOSITOR_ALWAYS: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eView3DOverlay_Flag(pub i32);

impl eView3DOverlay_Flag {
    pub const V3D_OVERLAY_FACE_ORIENTATION: Self = Self(((1 << 0)) as i32);
    pub const V3D_OVERLAY_HIDE_CURSOR: Self = Self(((1 << 1)) as i32);
    pub const V3D_OVERLAY_BONE_SELECT: Self = Self(((1 << 2)) as i32);
    pub const V3D_OVERLAY_LOOK_DEV: Self = Self(((1 << 3)) as i32);
    pub const V3D_OVERLAY_WIREFRAMES: Self = Self(((1 << 4)) as i32);
    pub const V3D_OVERLAY_HIDE_TEXT: Self = Self(((1 << 5)) as i32);
    pub const V3D_OVERLAY_HIDE_MOTION_PATHS: Self = Self(((1 << 6)) as i32);
    pub const V3D_OVERLAY_ONION_SKINS: Self = Self(((1 << 7)) as i32);
    pub const V3D_OVERLAY_HIDE_BONES: Self = Self(((1 << 8)) as i32);
    pub const V3D_OVERLAY_HIDE_OBJECT_XTRAS: Self = Self(((1 << 9)) as i32);
    pub const V3D_OVERLAY_HIDE_OBJECT_ORIGINS: Self = Self(((1 << 10)) as i32);
    pub const V3D_OVERLAY_STATS: Self = Self(((1 << 11)) as i32);
    pub const V3D_OVERLAY_FADE_INACTIVE: Self = Self(((1 << 12)) as i32);
    pub const V3D_OVERLAY_VIEWER_ATTRIBUTE: Self = Self(((1 << 13)) as i32);
    pub const V3D_OVERLAY_SCULPT_SHOW_MASK: Self = Self(((1 << 14)) as i32);
    pub const V3D_OVERLAY_SCULPT_SHOW_FACE_SETS: Self = Self(((1 << 15)) as i32);
    pub const V3D_OVERLAY_SCULPT_CURVES_CAGE: Self = Self(((1 << 16)) as i32);
    pub const V3D_OVERLAY_SHOW_LIGHT_COLORS: Self = Self(((1 << 17)) as i32);
    pub const V3D_OVERLAY_VIEWER_ATTRIBUTE_TEXT: Self = Self(((1 << 18)) as i32);
    pub const V3D_OVERLAY_PERFORMANCE: Self = Self(((1 << 19)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eView3DOverlay_EditFlag(pub i32);

impl eView3DOverlay_EditFlag {
    pub const V3D_OVERLAY_EDIT_VERT_NORMALS: Self = Self(((1 << 0)) as i32);
    pub const V3D_OVERLAY_EDIT_LOOP_NORMALS: Self = Self(((1 << 1)) as i32);
    pub const V3D_OVERLAY_EDIT_FACE_NORMALS: Self = Self(((1 << 2)) as i32);
    pub const V3D_OVERLAY_EDIT_RETOPOLOGY: Self = Self(((1 << 3)) as i32);
    pub const V3D_OVERLAY_EDIT_WEIGHT: Self = Self(((1 << 4)) as i32);
    pub const V3D_OVERLAY_EDIT_EDGES_DEPRECATED: Self = Self(((1 << 5)) as i32);
    pub const V3D_OVERLAY_EDIT_FACES: Self = Self(((1 << 6)) as i32);
    pub const V3D_OVERLAY_EDIT_FACE_DOT: Self = Self(((1 << 7)) as i32);
    pub const V3D_OVERLAY_EDIT_SEAMS: Self = Self(((1 << 8)) as i32);
    pub const V3D_OVERLAY_EDIT_SHARP: Self = Self(((1 << 9)) as i32);
    pub const V3D_OVERLAY_EDIT_CREASES: Self = Self(((1 << 10)) as i32);
    pub const V3D_OVERLAY_EDIT_BWEIGHTS: Self = Self(((1 << 11)) as i32);
    pub const V3D_OVERLAY_EDIT_FREESTYLE_EDGE: Self = Self(((1 << 12)) as i32);
    pub const V3D_OVERLAY_EDIT_FREESTYLE_FACE: Self = Self(((1 << 13)) as i32);
    pub const V3D_OVERLAY_EDIT_STATVIS: Self = Self(((1 << 14)) as i32);
    pub const V3D_OVERLAY_EDIT_EDGE_LEN: Self = Self(((1 << 15)) as i32);
    pub const V3D_OVERLAY_EDIT_EDGE_ANG: Self = Self(((1 << 16)) as i32);
    pub const V3D_OVERLAY_EDIT_FACE_ANG: Self = Self(((1 << 17)) as i32);
    pub const V3D_OVERLAY_EDIT_FACE_AREA: Self = Self(((1 << 18)) as i32);
    pub const V3D_OVERLAY_EDIT_INDICES: Self = Self(((1 << 19)) as i32);
    pub const V3D_OVERLAY_EDIT_CU_NORMALS: Self = Self(((1 << 21)) as i32);
    pub const V3D_OVERLAY_EDIT_CONSTANT_SCREEN_SIZE_NORMALS: Self = Self(((1 << 22)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eView3DOverlay_PaintFlag(pub i32);

impl eView3DOverlay_PaintFlag {
    pub const V3D_OVERLAY_PAINT_WIRE: Self = Self(((1 << 0)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eView3DOverlay_WPaintFlag(pub i32);

impl eView3DOverlay_WPaintFlag {
    pub const V3D_OVERLAY_WPAINT_CONTOURS: Self = Self(((1 << 0)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eView3D_Around(pub i32);

impl eView3D_Around {
    pub const V3D_AROUND_CENTER_BOUNDS: Self = Self((0) as i32);
    pub const V3D_AROUND_CENTER_MEDIAN: Self = Self((3) as i32);
    pub const V3D_AROUND_CURSOR: Self = Self((1) as i32);
    pub const V3D_AROUND_LOCAL_ORIGINS: Self = Self((2) as i32);
    pub const V3D_AROUND_ACTIVE: Self = Self((4) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eView3D_GridFlag(pub i8);

impl eView3D_GridFlag {
    pub const V3D_SHOW_FLOOR: Self = Self((1 << 0) as i8);
    pub const V3D_SHOW_X: Self = Self((1 << 1) as i8);
    pub const V3D_SHOW_Y: Self = Self((1 << 2) as i8);
    pub const V3D_SHOW_Z: Self = Self((1 << 3) as i8);
    pub const V3D_SHOW_ORTHO_GRID: Self = Self((1 << 4) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eView3D_Orientation(pub i32);

impl eView3D_Orientation {
    pub const V3D_ORIENT_GLOBAL: Self = Self((0) as i32);
    pub const V3D_ORIENT_LOCAL: Self = Self((1) as i32);
    pub const V3D_ORIENT_NORMAL: Self = Self((2) as i32);
    pub const V3D_ORIENT_VIEW: Self = Self((3) as i32);
    pub const V3D_ORIENT_GIMBAL: Self = Self((4) as i32);
    pub const V3D_ORIENT_CURSOR: Self = Self((5) as i32);
    pub const V3D_ORIENT_PARENT: Self = Self((6) as i32);
    pub const V3D_ORIENT_CUSTOM: Self = Self((1024) as i32);
    pub const V3D_ORIENT_CUSTOM_MATRIX: Self = Self(8 as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eView3D_GizmoFlag(pub i8);

impl eView3D_GizmoFlag {
    pub const V3D_GIZMO_HIDE: Self = Self(((1 << 0)) as i8);
    pub const V3D_GIZMO_HIDE_NAVIGATE: Self = Self(((1 << 1)) as i8);
    pub const V3D_GIZMO_HIDE_CONTEXT: Self = Self(((1 << 2)) as i8);
    pub const V3D_GIZMO_HIDE_TOOL: Self = Self(((1 << 3)) as i8);
    pub const V3D_GIZMO_HIDE_MODIFIER: Self = Self(((1 << 4)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eView3D_GizmoShowObject(pub i8);

impl eView3D_GizmoShowObject {
    pub const V3D_GIZMO_SHOW_OBJECT_TRANSLATE: Self = Self(((1 << 0)) as i8);
    pub const V3D_GIZMO_SHOW_OBJECT_ROTATE: Self = Self(((1 << 1)) as i8);
    pub const V3D_GIZMO_SHOW_OBJECT_SCALE: Self = Self(((1 << 2)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eView3D_GizmoShowArmature(pub i8);

impl eView3D_GizmoShowArmature {
    pub const V3D_GIZMO_SHOW_ARMATURE_BBONE: Self = Self(((1 << 0)) as i8);
    pub const V3D_GIZMO_SHOW_ARMATURE_ROLL: Self = Self(((1 << 1)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eView3D_GizmoShowEmpty(pub i8);

impl eView3D_GizmoShowEmpty {
    pub const V3D_GIZMO_SHOW_EMPTY_IMAGE: Self = Self(((1 << 0)) as i8);
    pub const V3D_GIZMO_SHOW_EMPTY_FORCE_FIELD: Self = Self(((1 << 1)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eView3D_GizmoShowLight(pub i8);

impl eView3D_GizmoShowLight {
    pub const V3D_GIZMO_SHOW_LIGHT_SIZE: Self = Self(((1 << 0)) as i8);
    pub const V3D_GIZMO_SHOW_LIGHT_LOOK_AT: Self = Self(((1 << 1)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eView3D_GizmoShowCamera(pub i8);

impl eView3D_GizmoShowCamera {
    pub const V3D_GIZMO_SHOW_CAMERA_LENS: Self = Self(((1 << 0)) as i8);
    pub const V3D_GIZMO_SHOW_CAMERA_DOF_DIST: Self = Self(((1 << 2)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eV3DPlaceDepth(pub i8);

impl eV3DPlaceDepth {
    pub const V3D_PLACE_DEPTH_SURFACE: Self = Self((0) as i8);
    pub const V3D_PLACE_DEPTH_CURSOR_PLANE: Self = Self((1) as i8);
    pub const V3D_PLACE_DEPTH_CURSOR_VIEW: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eV3DPlaceOrient(pub i8);

impl eV3DPlaceOrient {
    pub const V3D_PLACE_ORIENT_SURFACE: Self = Self((0) as i8);
    pub const V3D_PLACE_ORIENT_DEFAULT: Self = Self((1) as i8);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct RegionView3D {

}

impl Default for RegionView3D {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct View3DCursor {
    pub location: [f32; 3],
}

impl Default for View3DCursor {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct View3DShading {
    pub r#type: eDrawType,
    pub prev_type: eDrawType,
    pub prev_type_wire: eDrawType,
}

impl Default for View3DShading {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct View3DOverlay {
    pub flag: eView3DOverlay_Flag,
    pub edit_flag: eView3DOverlay_EditFlag,
    pub normals_length: f32,
    pub normals_constant_screen_size: f32,
    pub paint_flag: eView3DOverlay_PaintFlag,
}

impl Default for View3DOverlay {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct View3D_Runtime {
    pub properties_storage: *mut core::ffi::c_void,
    pub flag: i32,
    pub ofs_last_center: [f32; 3],
}

impl Default for View3D_Runtime {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct View3D {
    pub regionbase: ListBaseT<ARegion>,
    pub nullptr: ListBaseT<ARegion>,
}

impl Default for View3D {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

