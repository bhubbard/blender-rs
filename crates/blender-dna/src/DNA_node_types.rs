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
pub struct eNodeStack_Datatype(pub i16);

impl eNodeStack_Datatype {
    pub const NS_OSA_VECTORS: Self = Self((1) as i16);
    pub const NS_OSA_VALUES: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeSocket_ConversionRule(pub i16);

impl eNodeSocket_ConversionRule {
    pub const NS_CR_CENTER: Self = Self((0) as i16);
    pub const NS_CR_NONE: Self = Self((1) as i16);
    pub const NS_CR_FIT_WIDTH: Self = Self((2) as i16);
    pub const NS_CR_FIT_HEIGHT: Self = Self((3) as i16);
    pub const NS_CR_FIT: Self = Self((4) as i16);
    pub const NS_CR_STRETCH: Self = Self((5) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeSocketDatatype(pub i16);

impl eNodeSocketDatatype {
    pub const SOCK_CUSTOM: Self = Self((-1) as i16);
    pub const SOCK_FLOAT: Self = Self((0) as i16);
    pub const SOCK_VECTOR: Self = Self((1) as i16);
    pub const SOCK_RGBA: Self = Self((2) as i16);
    pub const SOCK_SHADER: Self = Self((3) as i16);
    pub const SOCK_BOOLEAN: Self = Self((4) as i16);
    pub const SOCK_INT: Self = Self((6) as i16);
    pub const SOCK_STRING: Self = Self((7) as i16);
    pub const SOCK_OBJECT: Self = Self((8) as i16);
    pub const SOCK_IMAGE: Self = Self((9) as i16);
    pub const SOCK_GEOMETRY: Self = Self((10) as i16);
    pub const SOCK_COLLECTION: Self = Self((11) as i16);
    pub const SOCK_TEXTURE: Self = Self((12) as i16);
    pub const SOCK_MATERIAL: Self = Self((13) as i16);
    pub const SOCK_ROTATION: Self = Self((14) as i16);
    pub const SOCK_MENU: Self = Self((15) as i16);
    pub const SOCK_MATRIX: Self = Self((16) as i16);
    pub const SOCK_BUNDLE: Self = Self((17) as i16);
    pub const SOCK_CLOSURE: Self = Self((18) as i16);
    pub const SOCK_FONT: Self = Self((19) as i16);
    pub const SOCK_SCENE: Self = Self((20) as i16);
    pub const SOCK_TEXT_ID: Self = Self((21) as i16);
    pub const SOCK_MASK: Self = Self((22) as i16);
    pub const SOCK_SOUND: Self = Self((23) as i16);
    pub const SOCK_INT_VECTOR: Self = Self((24) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeSocketDisplayShape(pub i8);

impl eNodeSocketDisplayShape {
    pub const SOCK_DISPLAY_SHAPE_CIRCLE: Self = Self((0) as i8);
    pub const SOCK_DISPLAY_SHAPE_SQUARE: Self = Self((1) as i8);
    pub const SOCK_DISPLAY_SHAPE_DIAMOND: Self = Self((2) as i8);
    pub const SOCK_DISPLAY_SHAPE_CIRCLE_DOT: Self = Self((3) as i8);
    pub const SOCK_DISPLAY_SHAPE_SQUARE_DOT: Self = Self((4) as i8);
    pub const SOCK_DISPLAY_SHAPE_DIAMOND_DOT: Self = Self((5) as i8);
    pub const SOCK_DISPLAY_SHAPE_LINE: Self = Self((6) as i8);
    pub const SOCK_DISPLAY_SHAPE_VOLUME_GRID: Self = Self((7) as i8);
    pub const SOCK_DISPLAY_SHAPE_LIST: Self = Self((8) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeSocketInOut(pub i16);

impl eNodeSocketInOut {
    pub const SOCK_IN: Self = Self((1 << 0) as i16);
    pub const SOCK_OUT: Self = Self((1 << 1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeSocketFlag(pub i16);

impl eNodeSocketFlag {
    pub const SOCK_SELECT: Self = Self(((1 << 0)) as i16);
    pub const SOCK_HIDDEN: Self = Self(((1 << 1)) as i16);
    pub const SOCK_IS_LINKED: Self = Self(((1 << 2)) as i16);
    pub const SOCK_UNAVAIL: Self = Self(((1 << 3)) as i16);
    pub const SOCK_GIZMO_PIN: Self = Self(((1 << 4)) as i16);
    pub const SOCK_COLLAPSED: Self = Self(((1 << 6)) as i16);
    pub const SOCK_HIDE_VALUE: Self = Self(((1 << 7)) as i16);
    pub const SOCK_AUTO_HIDDEN__DEPRECATED: Self = Self(((1 << 8)) as i16);
    pub const SOCK_NO_INTERNAL_LINK_LEGACY: Self = Self(((1 << 9)) as i16);
    pub const SOCK_COMPACT_LEGACY: Self = Self(((1 << 10)) as i16);
    pub const SOCK_MULTI_INPUT: Self = Self(((1 << 11)) as i16);
    pub const SOCK_HIDE_LABEL_LEGACY: Self = Self(((1 << 12)) as i16);
    pub const SOCK_HIDE_IN_MODIFIER: Self = Self(((1 << 13)) as i16);
    pub const SOCK_PANEL_COLLAPSED: Self = Self(((1 << 14)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodePanelFlag(pub i8);

impl eNodePanelFlag {
    pub const NODE_PANEL_COLLAPSED: Self = Self(((1 << 0)) as i8);
    pub const NODE_PANEL_PARENT_COLLAPSED: Self = Self(((1 << 1)) as i8);
    pub const NODE_PANEL_CONTENT_VISIBLE: Self = Self(((1 << 2)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eViewerNodeShortcut(pub i32);

impl eViewerNodeShortcut {
    pub const NODE_VIEWER_SHORTCUT_NONE: Self = Self((0) as i32);
    pub const NODE_VIEWER_SHORCTUT_SLOT_1: Self = Self((1) as i32);
    pub const NODE_VIEWER_SHORCTUT_SLOT_2: Self = Self((2) as i32);
    pub const NODE_VIEWER_SHORCTUT_SLOT_3: Self = Self((3) as i32);
    pub const NODE_VIEWER_SHORCTUT_SLOT_4: Self = Self((4) as i32);
    pub const NODE_VIEWER_SHORCTUT_SLOT_5: Self = Self((5) as i32);
    pub const NODE_VIEWER_SHORCTUT_SLOT_6: Self = Self((6) as i32);
    pub const NODE_VIEWER_SHORCTUT_SLOT_7: Self = Self((7) as i32);
    pub const NODE_VIEWER_SHORCTUT_SLOT_8: Self = Self((8) as i32);
    pub const NODE_VIEWER_SHORCTUT_SLOT_9: Self = Self((9) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeWarningPropagation(pub i8);

impl NodeWarningPropagation {
    pub const NODE_WARNING_PROPAGATION_ALL: Self = Self((0) as i8);
    pub const NODE_WARNING_PROPAGATION_NONE: Self = Self((1) as i8);
    pub const NODE_WARNING_PROPAGATION_ONLY_ERRORS: Self = Self((2) as i8);
    pub const NODE_WARNING_PROPAGATION_ONLY_ERRORS_AND_WARNINGS: Self = Self((3) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNode_Flag(pub i32);

impl eNode_Flag {
    pub const NODE_SELECT: Self = Self((1 << 0) as i32);
    pub const NODE_OPTIONS: Self = Self((1 << 1) as i32);
    pub const NODE_PREVIEW: Self = Self((1 << 2) as i32);
    pub const NODE_COLLAPSED: Self = Self((1 << 3) as i32);
    pub const NODE_ACTIVE: Self = Self((1 << 4) as i32);
    pub const NODE_DO_OUTPUT: Self = Self((1 << 6) as i32);
    pub const NODE_TEST: Self = Self((1 << 8) as i32);
    pub const NODE_MUTED: Self = Self((1 << 9) as i32);
    pub const NODE_ACTIVE_TEXTURE: Self = Self((1 << 14) as i32);
    pub const NODE_CUSTOM_COLOR: Self = Self((1 << 15) as i32);
    pub const NODE_INIT: Self = Self((1 << 16) as i32);
    pub const NODE_ACTIVE_PAINT_CANVAS: Self = Self((1 << 19) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNode_Update(pub i16);

impl eNode_Update {
    pub const NODE_UPDATE_ID: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeLink_Flag(pub i32);

impl eNodeLink_Flag {
    pub const NODE_LINK_INSERT_TARGET: Self = Self((1 << 0) as i32);
    pub const NODE_LINK_VALID: Self = Self((1 << 1) as i32);
    pub const NODE_LINK_TEST: Self = Self((1 << 2) as i32);
    pub const NODE_LINK_TEMP_HIGHLIGHT: Self = Self((1 << 3) as i32);
    pub const NODE_LINK_MUTED: Self = Self((1 << 4) as i32);
    pub const NODE_LINK_INSERT_TARGET_INVALID: Self = Self((1 << 5) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeTree_Type(pub i32);

impl eNodeTree_Type {
    pub const NTREE_UNDEFINED: Self = Self((-2) as i32);
    pub const NTREE_CUSTOM: Self = Self((-1) as i32);
    pub const NTREE_SHADER: Self = Self((0) as i32);
    pub const NTREE_COMPOSIT: Self = Self((1) as i32);
    pub const NTREE_TEXTURE: Self = Self((2) as i32);
    pub const NTREE_GEOMETRY: Self = Self((3) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeTree_Flag(pub i32);

impl eNodeTree_Flag {
    pub const NTREE_DS_EXPAND: Self = Self((1 << 0) as i32);
    pub const NTREE_UNUSED_2: Self = Self((1 << 2) as i32);
    pub const NTREE_VIEWER_BORDER: Self = Self((1 << 4) as i32);
    pub const NTREE_IS_GPU_SHADER_INTERNAL: Self = Self((1 << 6) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeTreeRuntimeFlag(pub i32);

impl eNodeTreeRuntimeFlag {
    pub const NTREE_RUNTIME_FLAG_HAS_IMAGE_ANIMATION: Self = Self((1 << 0) as i32);
    pub const NTREE_RUNTIME_FLAG_HAS_MATERIAL_OUTPUT: Self = Self((1 << 1) as i32);
    pub const NTREE_RUNTIME_FLAG_HAS_SIMULATION_ZONE: Self = Self((1 << 2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeAssetTraitFlag(pub i32);

impl GeometryNodeAssetTraitFlag {
    pub const GEO_NODE_ASSET_TOOL: Self = Self(((1 << 0)) as i32);
    pub const GEO_NODE_ASSET_EDIT: Self = Self(((1 << 1)) as i32);
    pub const GEO_NODE_ASSET_SCULPT: Self = Self(((1 << 2)) as i32);
    pub const GEO_NODE_ASSET_MESH: Self = Self(((1 << 3)) as i32);
    pub const GEO_NODE_ASSET_CURVE: Self = Self(((1 << 4)) as i32);
    pub const GEO_NODE_ASSET_POINTCLOUD: Self = Self(((1 << 5)) as i32);
    pub const GEO_NODE_ASSET_MODIFIER: Self = Self(((1 << 6)) as i32);
    pub const GEO_NODE_ASSET_OBJECT: Self = Self(((1 << 7)) as i32);
    pub const GEO_NODE_ASSET_WAIT_FOR_CURSOR: Self = Self(((1 << 8)) as i32);
    pub const GEO_NODE_ASSET_GREASE_PENCIL: Self = Self(((1 << 9)) as i32);
    pub const GEO_NODE_ASSET_PAINT: Self = Self(((1 << 10)) as i32);
    pub const GEO_NODE_ASSET_HIDE_MODIFIER_MANAGE_PANEL: Self = Self(((1 << 11)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CompositorNodeAssetTraitFlag(pub i32);

impl CompositorNodeAssetTraitFlag {
    pub const COMPOSIT_NODE_ASSET_STRIP_MODIFIER: Self = Self(((1 << 0)) as i32);
    pub const COMPOSIT_NODE_ASSET_SCENE_EFFECT: Self = Self(((1 << 1)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeMaskType(pub i32);

impl CMPNodeMaskType {
    pub const CMP_NODE_MASKTYPE_ADD: Self = Self((0) as i32);
    pub const CMP_NODE_MASKTYPE_SUBTRACT: Self = Self((1) as i32);
    pub const CMP_NODE_MASKTYPE_MULTIPLY: Self = Self((2) as i32);
    pub const CMP_NODE_MASKTYPE_NOT: Self = Self((3) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeDilateErodeMethod(pub i32);

impl CMPNodeDilateErodeMethod {
    pub const CMP_NODE_DILATE_ERODE_STEP: Self = Self((0) as i32);
    pub const CMP_NODE_DILATE_ERODE_DISTANCE_THRESHOLD: Self = Self((1) as i32);
    pub const CMP_NODE_DILATE_ERODE_DISTANCE: Self = Self((2) as i32);
    pub const CMP_NODE_DILATE_ERODE_DISTANCE_FEATHER: Self = Self((3) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeInpaint_Type(pub i16);

impl eNodeInpaint_Type {
    pub const CMP_NODE_INPAINT_SIMPLE: Self = Self((0) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeMaskFlags(pub i32);

impl CMPNodeMaskFlags {
    pub const CMP_NODE_MASK_FLAG_NO_FEATHER: Self = Self(((1 << 1)) as i32);
    pub const CMP_NODE_MASK_FLAG_MOTION_BLUR: Self = Self(((1 << 2)) as i32);
    pub const CMP_NODE_MASK_FLAG_SIZE_FIXED: Self = Self(((1 << 8)) as i32);
    pub const CMP_NODE_MASK_FLAG_SIZE_FIXED_SCENE: Self = Self(((1 << 9)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeGlareQuality(pub i32);

impl CMPNodeGlareQuality {
    pub const CMP_NODE_GLARE_QUALITY_HIGH: Self = Self((0) as i32);
    pub const CMP_NODE_GLARE_QUALITY_MEDIUM: Self = Self((1) as i32);
    pub const CMP_NODE_GLARE_QUALITY_LOW: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeGeometryViewerItemFlag(pub i32);

impl NodeGeometryViewerItemFlag {
    pub const NODE_GEO_VIEWER_ITEM_FLAG_AUTO_REMOVE: Self = Self(((1 << 0)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeClosureFlag(pub u8);

impl NodeClosureFlag {
    pub const NODE_CLOSURE_FLAG_DEFINE_SIGNATURE: Self = Self(((1 << 0)) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeEvaluateClosureFlag(pub u8);

impl NodeEvaluateClosureFlag {
    pub const NODE_EVALUATE_CLOSURE_FLAG_DEFINE_SIGNATURE: Self = Self(((1 << 0)) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeGeometryTransformGizmoFlag(pub u32);

impl NodeGeometryTransformGizmoFlag {
    pub const GEO_NODE_TRANSFORM_GIZMO_USE_TRANSLATION_X: Self = Self((1 << 0) as u32);
    pub const GEO_NODE_TRANSFORM_GIZMO_USE_TRANSLATION_Y: Self = Self((1 << 1) as u32);
    pub const GEO_NODE_TRANSFORM_GIZMO_USE_TRANSLATION_Z: Self = Self((1 << 2) as u32);
    pub const GEO_NODE_TRANSFORM_GIZMO_USE_ROTATION_X: Self = Self((1 << 3) as u32);
    pub const GEO_NODE_TRANSFORM_GIZMO_USE_ROTATION_Y: Self = Self((1 << 4) as u32);
    pub const GEO_NODE_TRANSFORM_GIZMO_USE_ROTATION_Z: Self = Self((1 << 5) as u32);
    pub const GEO_NODE_TRANSFORM_GIZMO_USE_SCALE_X: Self = Self((1 << 6) as u32);
    pub const GEO_NODE_TRANSFORM_GIZMO_USE_SCALE_Y: Self = Self((1 << 7) as u32);
    pub const GEO_NODE_TRANSFORM_GIZMO_USE_SCALE_Z: Self = Self((1 << 8) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeGeometryBakeItemFlag(pub i32);

impl NodeGeometryBakeItemFlag {
    pub const GEO_NODE_BAKE_ITEM_IS_ATTRIBUTE: Self = Self(((1 << 0)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeCombineBundleFlag(pub u8);

impl NodeCombineBundleFlag {
    pub const NODE_COMBINE_BUNDLE_FLAG_DEFINE_SIGNATURE: Self = Self(((1 << 0)) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeSeparateBundleFlag(pub u8);

impl NodeSeparateBundleFlag {
    pub const NODE_SEPARATE_BUNDLE_FLAG_DEFINE_SIGNATURE: Self = Self(((1 << 0)) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeScript_Mode(pub i8);

impl eNodeScript_Mode {
    pub const NODE_SCRIPT_INTERNAL: Self = Self((0) as i8);
    pub const NODE_SCRIPT_EXTERNAL: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeScript_Flag(pub i8);

impl eNodeScript_Flag {
    pub const NODE_SCRIPT_AUTO_UPDATE: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeIES_Mode(pub i8);

impl eNodeIES_Mode {
    pub const NODE_IES_INTERNAL: Self = Self((0) as i8);
    pub const NODE_IES_EXTERNAL: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeFrame_Flag(pub i8);

impl eNodeFrame_Flag {
    pub const NODE_FRAME_SHRINK: Self = Self((1) as i8);
    pub const NODE_FRAME_RESIZEABLE: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeProxy_Flag(pub i8);

impl eNodeProxy_Flag {
    pub const NODE_PROXY_AUTOTYPE: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeConducetiveFresnel_Type(pub i16);

impl eNodeConducetiveFresnel_Type {
    pub const SHD_PHYSICAL_CONDUCTOR: Self = Self((0) as i16);
    pub const SHD_CONDUCTOR_F82: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeGlossy_Dist(pub i16);

impl eNodeGlossy_Dist {
    pub const SHD_GLOSSY_BECKMANN: Self = Self((0) as i16);
    pub const SHD_GLOSSY_SHARP_DEPRECATED: Self = Self((1) as i16);
    pub const SHD_GLOSSY_GGX: Self = Self((2) as i16);
    pub const SHD_GLOSSY_ASHIKHMIN_SHIRLEY: Self = Self((3) as i16);
    pub const SHD_GLOSSY_MULTI_GGX: Self = Self((4) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeLightEval_Mode(pub i16);

impl eNodeLightEval_Mode {
    pub const SHD_LIGHT_EVAL_DIFFUSE: Self = Self((0) as i16);
    pub const SHD_LIGHT_EVAL_GLOSSY: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeVectorTransform_Type(pub i16);

impl eNodeVectorTransform_Type {
    pub const SHD_VECT_TRANSFORM_TYPE_VECTOR: Self = Self((0) as i16);
    pub const SHD_VECT_TRANSFORM_TYPE_POINT: Self = Self((1) as i16);
    pub const SHD_VECT_TRANSFORM_TYPE_NORMAL: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeVectorTransform_Space(pub i16);

impl eNodeVectorTransform_Space {
    pub const SHD_VECT_TRANSFORM_SPACE_WORLD: Self = Self((0) as i16);
    pub const SHD_VECT_TRANSFORM_SPACE_OBJECT: Self = Self((1) as i16);
    pub const SHD_VECT_TRANSFORM_SPACE_CAMERA: Self = Self((2) as i16);
    pub const SHD_VECT_TRANSFORM_SPACE_LIGHT: Self = Self((3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeShader_AttributeType(pub i16);

impl eNodeShader_AttributeType {
    pub const SHD_ATTRIBUTE_GEOMETRY: Self = Self((0) as i16);
    pub const SHD_ATTRIBUTE_OBJECT: Self = Self((1) as i16);
    pub const SHD_ATTRIBUTE_INSTANCER: Self = Self((2) as i16);
    pub const SHD_ATTRIBUTE_VIEW_LAYER: Self = Self((3) as i16);
    pub const SHD_ATTRIBUTE_LIGHT: Self = Self((4) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeToon_Mode(pub i16);

impl eNodeToon_Mode {
    pub const SHD_TOON_DIFFUSE: Self = Self((0) as i16);
    pub const SHD_TOON_GLOSSY: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeHair_Component(pub i16);

impl eNodeHair_Component {
    pub const SHD_HAIR_REFLECTION: Self = Self((0) as i16);
    pub const SHD_HAIR_TRANSMISSION: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodePrincipledHair_Model(pub i16);

impl eNodePrincipledHair_Model {
    pub const SHD_PRINCIPLED_HAIR_CHIANG: Self = Self((0) as i16);
    pub const SHD_PRINCIPLED_HAIR_HUANG: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodePrincipledHair_Param(pub i16);

impl eNodePrincipledHair_Param {
    pub const SHD_PRINCIPLED_HAIR_REFLECTANCE: Self = Self((0) as i16);
    pub const SHD_PRINCIPLED_HAIR_PIGMENT_CONCENTRATION: Self = Self((1) as i16);
    pub const SHD_PRINCIPLED_HAIR_DIRECT_ABSORPTION: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeBlend_Type(pub i16);

impl eNodeBlend_Type {
    pub const SHD_BLEND_LINEAR: Self = Self((0) as i16);
    pub const SHD_BLEND_QUADRATIC: Self = Self((1) as i16);
    pub const SHD_BLEND_EASING: Self = Self((2) as i16);
    pub const SHD_BLEND_DIAGONAL: Self = Self((3) as i16);
    pub const SHD_BLEND_RADIAL: Self = Self((4) as i16);
    pub const SHD_BLEND_QUADRATIC_SPHERE: Self = Self((5) as i16);
    pub const SHD_BLEND_SPHERICAL: Self = Self((6) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeNoise_Basis(pub i16);

impl eNodeNoise_Basis {
    pub const SHD_NOISE_PERLIN: Self = Self((0) as i16);
    pub const SHD_NOISE_VORONOI_F1: Self = Self((1) as i16);
    pub const SHD_NOISE_VORONOI_F2: Self = Self((2) as i16);
    pub const SHD_NOISE_VORONOI_F3: Self = Self((3) as i16);
    pub const SHD_NOISE_VORONOI_F4: Self = Self((4) as i16);
    pub const SHD_NOISE_VORONOI_F2_F1: Self = Self((5) as i16);
    pub const SHD_NOISE_VORONOI_CRACKLE: Self = Self((6) as i16);
    pub const SHD_NOISE_CELL_NOISE: Self = Self((7) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeNoise_Type(pub i16);

impl eNodeNoise_Type {
    pub const SHD_NOISE_SOFT: Self = Self((0) as i16);
    pub const SHD_NOISE_HARD: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeVoronoi_Dist(pub i16);

impl eNodeVoronoi_Dist {
    pub const SHD_VORONOI_EUCLIDEAN: Self = Self((0) as i16);
    pub const SHD_VORONOI_MANHATTAN: Self = Self((1) as i16);
    pub const SHD_VORONOI_CHEBYCHEV: Self = Self((2) as i16);
    pub const SHD_VORONOI_MINKOWSKI: Self = Self((3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeVoronoi_Type(pub i16);

impl eNodeVoronoi_Type {
    pub const SHD_VORONOI_F1: Self = Self((0) as i16);
    pub const SHD_VORONOI_F2: Self = Self((1) as i16);
    pub const SHD_VORONOI_SMOOTH_F1: Self = Self((2) as i16);
    pub const SHD_VORONOI_DISTANCE_TO_EDGE: Self = Self((3) as i16);
    pub const SHD_VORONOI_N_SPHERE_RADIUS: Self = Self((4) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeMusgrave_Type(pub i16);

impl eNodeMusgrave_Type {
    pub const SHD_MUSGRAVE_MULTIFRACTAL: Self = Self((0) as i16);
    pub const SHD_MUSGRAVE_FBM: Self = Self((1) as i16);
    pub const SHD_MUSGRAVE_HYBRID_MULTIFRACTAL: Self = Self((2) as i16);
    pub const SHD_MUSGRAVE_RIDGED_MULTIFRACTAL: Self = Self((3) as i16);
    pub const SHD_MUSGRAVE_HETERO_TERRAIN: Self = Self((4) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeNoiseTexture_Type(pub i16);

impl eNodeNoiseTexture_Type {
    pub const SHD_NOISE_MULTIFRACTAL: Self = Self((0) as i16);
    pub const SHD_NOISE_FBM: Self = Self((1) as i16);
    pub const SHD_NOISE_HYBRID_MULTIFRACTAL: Self = Self((2) as i16);
    pub const SHD_NOISE_RIDGED_MULTIFRACTAL: Self = Self((3) as i16);
    pub const SHD_NOISE_HETERO_TERRAIN: Self = Self((4) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeWave_Type(pub i16);

impl eNodeWave_Type {
    pub const SHD_WAVE_BANDS: Self = Self((0) as i16);
    pub const SHD_WAVE_RINGS: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeWave_BandsDir(pub i16);

impl eNodeWave_BandsDir {
    pub const SHD_WAVE_BANDS_DIRECTION_X: Self = Self((0) as i16);
    pub const SHD_WAVE_BANDS_DIRECTION_Y: Self = Self((1) as i16);
    pub const SHD_WAVE_BANDS_DIRECTION_Z: Self = Self((2) as i16);
    pub const SHD_WAVE_BANDS_DIRECTION_DIAGONAL: Self = Self((3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeWave_RingsDir(pub i16);

impl eNodeWave_RingsDir {
    pub const SHD_WAVE_RINGS_DIRECTION_X: Self = Self((0) as i16);
    pub const SHD_WAVE_RINGS_DIRECTION_Y: Self = Self((1) as i16);
    pub const SHD_WAVE_RINGS_DIRECTION_Z: Self = Self((2) as i16);
    pub const SHD_WAVE_RINGS_DIRECTION_SPHERICAL: Self = Self((3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeWave_Profile(pub i16);

impl eNodeWave_Profile {
    pub const SHD_WAVE_PROFILE_SIN: Self = Self((0) as i16);
    pub const SHD_WAVE_PROFILE_SAW: Self = Self((1) as i16);
    pub const SHD_WAVE_PROFILE_TRI: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeSky_Type(pub i16);

impl eNodeSky_Type {
    pub const SHD_SKY_PREETHAM: Self = Self((0) as i16);
    pub const SHD_SKY_HOSEK: Self = Self((1) as i16);
    pub const SHD_SKY_SINGLE_SCATTERING: Self = Self((2) as i16);
    pub const SHD_SKY_MULTIPLE_SCATTERING: Self = Self((3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeProj_Env(pub i16);

impl eNodeProj_Env {
    pub const SHD_PROJ_EQUIRECTANGULAR: Self = Self((0) as i16);
    pub const SHD_PROJ_MIRROR_BALL: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeGaborType(pub i8);

impl NodeGaborType {
    pub const SHD_GABOR_TYPE_2D: Self = Self((0) as i8);
    pub const SHD_GABOR_TYPE_3D: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeImage_Extension(pub i16);

impl eNodeImage_Extension {
    pub const SHD_IMAGE_EXTENSION_REPEAT: Self = Self((0) as i16);
    pub const SHD_IMAGE_EXTENSION_EXTEND: Self = Self((1) as i16);
    pub const SHD_IMAGE_EXTENSION_CLIP: Self = Self((2) as i16);
    pub const SHD_IMAGE_EXTENSION_MIRROR: Self = Self((3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeProj_Image(pub i16);

impl eNodeProj_Image {
    pub const SHD_PROJ_FLAT: Self = Self((0) as i16);
    pub const SHD_PROJ_BOX: Self = Self((1) as i16);
    pub const SHD_PROJ_SPHERE: Self = Self((2) as i16);
    pub const SHD_PROJ_TUBE: Self = Self((3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeInterp_Image(pub i16);

impl eNodeInterp_Image {
    pub const SHD_INTERP_LINEAR: Self = Self((0) as i16);
    pub const SHD_INTERP_CLOSEST: Self = Self((1) as i16);
    pub const SHD_INTERP_CUBIC: Self = Self((2) as i16);
    pub const SHD_INTERP_SMART: Self = Self((3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeTangent_Type(pub i16);

impl eNodeTangent_Type {
    pub const SHD_TANGENT_RADIAL: Self = Self((0) as i16);
    pub const SHD_TANGENT_UVMAP: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeTangent_Axis(pub i16);

impl eNodeTangent_Axis {
    pub const SHD_TANGENT_AXIS_X: Self = Self((0) as i16);
    pub const SHD_TANGENT_AXIS_Y: Self = Self((1) as i16);
    pub const SHD_TANGENT_AXIS_Z: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeSpace_Normal(pub i16);

impl eNodeSpace_Normal {
    pub const SHD_SPACE_TANGENT: Self = Self((0) as i16);
    pub const SHD_SPACE_OBJECT: Self = Self((1) as i16);
    pub const SHD_SPACE_WORLD: Self = Self((2) as i16);
    pub const SHD_SPACE_BLENDER_OBJECT: Self = Self((3) as i16);
    pub const SHD_SPACE_BLENDER_WORLD: Self = Self((4) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeNormalMap_Convention(pub i16);

impl eNodeNormalMap_Convention {
    pub const SHD_NORMAL_MAP_CONVENTION_OPENGL: Self = Self((0) as i16);
    pub const SHD_NORMAL_MAP_CONVENTION_DIRECTX: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeNormalMap_Base(pub i16);

impl eNodeNormalMap_Base {
    pub const SHD_NORMAL_MAP_BASE_ORIGINAL: Self = Self((0) as i16);
    pub const SHD_NORMAL_MAP_BASE_DISPLACED: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeAO_Flag(pub i16);

impl eNodeAO_Flag {
    pub const SHD_AO_INSIDE: Self = Self((1) as i16);
    pub const SHD_AO_LOCAL: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeMapping_VectorType(pub i16);

impl eNodeMapping_VectorType {
    pub const NODE_MAPPING_TYPE_POINT: Self = Self((0) as i16);
    pub const NODE_MAPPING_TYPE_TEXTURE: Self = Self((1) as i16);
    pub const NODE_MAPPING_TYPE_VECTOR: Self = Self((2) as i16);
    pub const NODE_MAPPING_TYPE_NORMAL: Self = Self((3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeVectorRotate_Type(pub i16);

impl eNodeVectorRotate_Type {
    pub const NODE_VECTOR_ROTATE_TYPE_AXIS: Self = Self((0) as i16);
    pub const NODE_VECTOR_ROTATE_TYPE_AXIS_X: Self = Self((1) as i16);
    pub const NODE_VECTOR_ROTATE_TYPE_AXIS_Y: Self = Self((2) as i16);
    pub const NODE_VECTOR_ROTATE_TYPE_AXIS_Z: Self = Self((3) as i16);
    pub const NODE_VECTOR_ROTATE_TYPE_EULER_XYZ: Self = Self((4) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeShader_MathClamp(pub i8);

impl eNodeShader_MathClamp {
    pub const SHD_MATH_CLAMP: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeMathOperation(pub i32);

impl NodeMathOperation {
    pub const NODE_MATH_ADD: Self = Self((0) as i32);
    pub const NODE_MATH_SUBTRACT: Self = Self((1) as i32);
    pub const NODE_MATH_MULTIPLY: Self = Self((2) as i32);
    pub const NODE_MATH_DIVIDE: Self = Self((3) as i32);
    pub const NODE_MATH_SINE: Self = Self((4) as i32);
    pub const NODE_MATH_COSINE: Self = Self((5) as i32);
    pub const NODE_MATH_TANGENT: Self = Self((6) as i32);
    pub const NODE_MATH_ARCSINE: Self = Self((7) as i32);
    pub const NODE_MATH_ARCCOSINE: Self = Self((8) as i32);
    pub const NODE_MATH_ARCTANGENT: Self = Self((9) as i32);
    pub const NODE_MATH_POWER: Self = Self((10) as i32);
    pub const NODE_MATH_LOGARITHM: Self = Self((11) as i32);
    pub const NODE_MATH_MINIMUM: Self = Self((12) as i32);
    pub const NODE_MATH_MAXIMUM: Self = Self((13) as i32);
    pub const NODE_MATH_ROUND: Self = Self((14) as i32);
    pub const NODE_MATH_LESS_THAN: Self = Self((15) as i32);
    pub const NODE_MATH_GREATER_THAN: Self = Self((16) as i32);
    pub const NODE_MATH_MODULO: Self = Self((17) as i32);
    pub const NODE_MATH_ABSOLUTE: Self = Self((18) as i32);
    pub const NODE_MATH_ARCTAN2: Self = Self((19) as i32);
    pub const NODE_MATH_FLOOR: Self = Self((20) as i32);
    pub const NODE_MATH_CEIL: Self = Self((21) as i32);
    pub const NODE_MATH_FRACTION: Self = Self((22) as i32);
    pub const NODE_MATH_SQRT: Self = Self((23) as i32);
    pub const NODE_MATH_INV_SQRT: Self = Self((24) as i32);
    pub const NODE_MATH_SIGN: Self = Self((25) as i32);
    pub const NODE_MATH_EXPONENT: Self = Self((26) as i32);
    pub const NODE_MATH_RADIANS: Self = Self((27) as i32);
    pub const NODE_MATH_DEGREES: Self = Self((28) as i32);
    pub const NODE_MATH_SINH: Self = Self((29) as i32);
    pub const NODE_MATH_COSH: Self = Self((30) as i32);
    pub const NODE_MATH_TANH: Self = Self((31) as i32);
    pub const NODE_MATH_TRUNC: Self = Self((32) as i32);
    pub const NODE_MATH_SNAP: Self = Self((33) as i32);
    pub const NODE_MATH_WRAP: Self = Self((34) as i32);
    pub const NODE_MATH_COMPARE: Self = Self((35) as i32);
    pub const NODE_MATH_MULTIPLY_ADD: Self = Self((36) as i32);
    pub const NODE_MATH_PINGPONG: Self = Self((37) as i32);
    pub const NODE_MATH_SMOOTH_MIN: Self = Self((38) as i32);
    pub const NODE_MATH_SMOOTH_MAX: Self = Self((39) as i32);
    pub const NODE_MATH_FLOORED_MODULO: Self = Self((40) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeVectorMathOperation(pub i32);

impl NodeVectorMathOperation {
    pub const NODE_VECTOR_MATH_ADD: Self = Self((0) as i32);
    pub const NODE_VECTOR_MATH_SUBTRACT: Self = Self((1) as i32);
    pub const NODE_VECTOR_MATH_MULTIPLY: Self = Self((2) as i32);
    pub const NODE_VECTOR_MATH_DIVIDE: Self = Self((3) as i32);
    pub const NODE_VECTOR_MATH_CROSS_PRODUCT: Self = Self((4) as i32);
    pub const NODE_VECTOR_MATH_PROJECT: Self = Self((5) as i32);
    pub const NODE_VECTOR_MATH_REFLECT: Self = Self((6) as i32);
    pub const NODE_VECTOR_MATH_DOT_PRODUCT: Self = Self((7) as i32);
    pub const NODE_VECTOR_MATH_DISTANCE: Self = Self((8) as i32);
    pub const NODE_VECTOR_MATH_LENGTH: Self = Self((9) as i32);
    pub const NODE_VECTOR_MATH_SCALE: Self = Self((10) as i32);
    pub const NODE_VECTOR_MATH_NORMALIZE: Self = Self((11) as i32);
    pub const NODE_VECTOR_MATH_SNAP: Self = Self((12) as i32);
    pub const NODE_VECTOR_MATH_FLOOR: Self = Self((13) as i32);
    pub const NODE_VECTOR_MATH_CEIL: Self = Self((14) as i32);
    pub const NODE_VECTOR_MATH_MODULO: Self = Self((15) as i32);
    pub const NODE_VECTOR_MATH_FRACTION: Self = Self((16) as i32);
    pub const NODE_VECTOR_MATH_ABSOLUTE: Self = Self((17) as i32);
    pub const NODE_VECTOR_MATH_MINIMUM: Self = Self((18) as i32);
    pub const NODE_VECTOR_MATH_MAXIMUM: Self = Self((19) as i32);
    pub const NODE_VECTOR_MATH_WRAP: Self = Self((20) as i32);
    pub const NODE_VECTOR_MATH_SINE: Self = Self((21) as i32);
    pub const NODE_VECTOR_MATH_COSINE: Self = Self((22) as i32);
    pub const NODE_VECTOR_MATH_TANGENT: Self = Self((23) as i32);
    pub const NODE_VECTOR_MATH_REFRACT: Self = Self((24) as i32);
    pub const NODE_VECTOR_MATH_FACEFORWARD: Self = Self((25) as i32);
    pub const NODE_VECTOR_MATH_MULTIPLY_ADD: Self = Self((26) as i32);
    pub const NODE_VECTOR_MATH_POWER: Self = Self((27) as i32);
    pub const NODE_VECTOR_MATH_SIGN: Self = Self((28) as i32);
    pub const NODE_VECTOR_MATH_ROUND: Self = Self((29) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeBooleanMathOperation(pub i32);

impl NodeBooleanMathOperation {
    pub const NODE_BOOLEAN_MATH_AND: Self = Self((0) as i32);
    pub const NODE_BOOLEAN_MATH_OR: Self = Self((1) as i32);
    pub const NODE_BOOLEAN_MATH_NOT: Self = Self((2) as i32);
    pub const NODE_BOOLEAN_MATH_NAND: Self = Self((3) as i32);
    pub const NODE_BOOLEAN_MATH_NOR: Self = Self((4) as i32);
    pub const NODE_BOOLEAN_MATH_XNOR: Self = Self((5) as i32);
    pub const NODE_BOOLEAN_MATH_XOR: Self = Self((6) as i32);
    pub const NODE_BOOLEAN_MATH_IMPLY: Self = Self((7) as i32);
    pub const NODE_BOOLEAN_MATH_NIMPLY: Self = Self((8) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeShaderMixMode(pub i8);

impl NodeShaderMixMode {
    pub const NODE_MIX_MODE_UNIFORM: Self = Self((0) as i8);
    pub const NODE_MIX_MODE_NON_UNIFORM: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeCompareMode(pub i8);

impl NodeCompareMode {
    pub const NODE_COMPARE_MODE_ELEMENT: Self = Self((0) as i8);
    pub const NODE_COMPARE_MODE_LENGTH: Self = Self((1) as i8);
    pub const NODE_COMPARE_MODE_AVERAGE: Self = Self((2) as i8);
    pub const NODE_COMPARE_MODE_DOT_PRODUCT: Self = Self((3) as i8);
    pub const NODE_COMPARE_MODE_DIRECTION: Self = Self((4) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeCompareOperation(pub i8);

impl NodeCompareOperation {
    pub const NODE_COMPARE_LESS_THAN: Self = Self((0) as i8);
    pub const NODE_COMPARE_LESS_EQUAL: Self = Self((1) as i8);
    pub const NODE_COMPARE_GREATER_THAN: Self = Self((2) as i8);
    pub const NODE_COMPARE_GREATER_EQUAL: Self = Self((3) as i8);
    pub const NODE_COMPARE_EQUAL: Self = Self((4) as i8);
    pub const NODE_COMPARE_NOT_EQUAL: Self = Self((5) as i8);
    pub const NODE_COMPARE_COLOR_BRIGHTER: Self = Self((6) as i8);
    pub const NODE_COMPARE_COLOR_DARKER: Self = Self((7) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeIntegerMathOperation(pub i32);

impl NodeIntegerMathOperation {
    pub const NODE_INTEGER_MATH_ADD: Self = Self((0) as i32);
    pub const NODE_INTEGER_MATH_SUBTRACT: Self = Self((1) as i32);
    pub const NODE_INTEGER_MATH_MULTIPLY: Self = Self((2) as i32);
    pub const NODE_INTEGER_MATH_DIVIDE: Self = Self((3) as i32);
    pub const NODE_INTEGER_MATH_MULTIPLY_ADD: Self = Self((4) as i32);
    pub const NODE_INTEGER_MATH_POWER: Self = Self((5) as i32);
    pub const NODE_INTEGER_MATH_FLOORED_MODULO: Self = Self((6) as i32);
    pub const NODE_INTEGER_MATH_ABSOLUTE: Self = Self((7) as i32);
    pub const NODE_INTEGER_MATH_MINIMUM: Self = Self((8) as i32);
    pub const NODE_INTEGER_MATH_MAXIMUM: Self = Self((9) as i32);
    pub const NODE_INTEGER_MATH_GCD: Self = Self((10) as i32);
    pub const NODE_INTEGER_MATH_LCM: Self = Self((11) as i32);
    pub const NODE_INTEGER_MATH_NEGATE: Self = Self((12) as i32);
    pub const NODE_INTEGER_MATH_SIGN: Self = Self((13) as i32);
    pub const NODE_INTEGER_MATH_DIVIDE_FLOOR: Self = Self((14) as i32);
    pub const NODE_INTEGER_MATH_DIVIDE_CEIL: Self = Self((15) as i32);
    pub const NODE_INTEGER_MATH_DIVIDE_ROUND: Self = Self((16) as i32);
    pub const NODE_INTEGER_MATH_MODULO: Self = Self((17) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FloatToIntRoundingMode(pub i32);

impl FloatToIntRoundingMode {
    pub const FN_NODE_FLOAT_TO_INT_ROUND: Self = Self((0) as i32);
    pub const FN_NODE_FLOAT_TO_INT_FLOOR: Self = Self((1) as i32);
    pub const FN_NODE_FLOAT_TO_INT_CEIL: Self = Self((2) as i32);
    pub const FN_NODE_FLOAT_TO_INT_TRUNCATE: Self = Self((3) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeClamp_Type(pub i8);

impl eNodeClamp_Type {
    pub const NODE_CLAMP_MINMAX: Self = Self((0) as i8);
    pub const NODE_CLAMP_RANGE: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeMapRange_Type(pub i8);

impl eNodeMapRange_Type {
    pub const NODE_MAP_RANGE_LINEAR: Self = Self((0) as i8);
    pub const NODE_MAP_RANGE_STEPPED: Self = Self((1) as i8);
    pub const NODE_MAP_RANGE_SMOOTHSTEP: Self = Self((2) as i8);
    pub const NODE_MAP_RANGE_SMOOTHERSTEP: Self = Self((3) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeShader_MixRgbFlag(pub i8);

impl eNodeShader_MixRgbFlag {
    pub const SHD_MIXRGB_USE_ALPHA: Self = Self((1) as i8);
    pub const SHD_MIXRGB_CLAMP: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeSubsurface_Type(pub i16);

impl eNodeSubsurface_Type {
    pub const SHD_SUBSURFACE_COMPATIBLE: Self = Self((0) as i16);
    pub const SHD_SUBSURFACE_CUBIC: Self = Self((1) as i16);
    pub const SHD_SUBSURFACE_GAUSSIAN: Self = Self((2) as i16);
    pub const SHD_SUBSURFACE_BURLEY: Self = Self((3) as i16);
    pub const SHD_SUBSURFACE_RANDOM_WALK_LEGACY: Self = Self((4) as i16);
    pub const SHD_SUBSURFACE_RANDOM_WALK_SKIN: Self = Self((5) as i16);
    pub const SHD_SUBSURFACE_RANDOM_WALK: Self = Self((6) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeBlur_Aspect(pub i16);

impl eNodeBlur_Aspect {
    pub const CMP_NODE_BLUR_ASPECT_NONE: Self = Self((0) as i16);
    pub const CMP_NODE_BLUR_ASPECT_Y: Self = Self((1) as i16);
    pub const CMP_NODE_BLUR_ASPECT_X: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeTranslateRepeatAxis(pub i32);

impl CMPNodeTranslateRepeatAxis {
    pub const CMP_NODE_TRANSLATE_REPEAT_AXIS_NONE: Self = Self((0) as i32);
    pub const CMP_NODE_TRANSLATE_REPEAT_AXIS_X: Self = Self((1) as i32);
    pub const CMP_NODE_TRANSLATE_REPEAT_AXIS_Y: Self = Self((2) as i32);
    pub const CMP_NODE_TRANSLATE_REPEAT_AXIS_XY: Self = Self((3) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPExtensionMode(pub i32);

impl CMPExtensionMode {
    pub const CMP_NODE_EXTENSION_MODE_CLIP: Self = Self((0) as i32);
    pub const CMP_NODE_EXTENSION_MODE_EXTEND: Self = Self((1) as i32);
    pub const CMP_NODE_EXTENSION_MODE_REPEAT: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeCompositor_OutputFlag(pub i8);

impl eNodeCompositor_OutputFlag {
    pub const CMP_NODE_OUTPUT_IGNORE_ALPHA: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeColorBalanceMethod(pub i32);

impl CMPNodeColorBalanceMethod {
    pub const CMP_NODE_COLOR_BALANCE_LGG: Self = Self((0) as i32);
    pub const CMP_NODE_COLOR_BALANCE_ASC_CDL: Self = Self((1) as i32);
    pub const CMP_NODE_COLOR_BALANCE_WHITEPOINT: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeAlphaConvertMode(pub i32);

impl CMPNodeAlphaConvertMode {
    pub const CMP_NODE_ALPHA_CONVERT_PREMULTIPLY: Self = Self((0) as i32);
    pub const CMP_NODE_ALPHA_CONVERT_UNPREMULTIPLY: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeDistanceMatteColorSpace(pub i32);

impl CMPNodeDistanceMatteColorSpace {
    pub const CMP_NODE_DISTANCE_MATTE_COLOR_SPACE_RGBA: Self = Self((0) as i32);
    pub const CMP_NODE_DISTANCE_MATTE_COLOR_SPACE_YCCA: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeColorSpillLimitAlgorithm(pub i32);

impl CMPNodeColorSpillLimitAlgorithm {
    pub const CMP_NODE_COLOR_SPILL_LIMIT_ALGORITHM_SINGLE: Self = Self((0) as i32);
    pub const CMP_NODE_COLOR_SPILL_LIMIT_ALGORITHM_AVERAGE: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeChannelMatteLimitAlgorithm(pub i32);

impl CMPNodeChannelMatteLimitAlgorithm {
    pub const CMP_NODE_CHANNEL_MATTE_LIMIT_ALGORITHM_SINGLE: Self = Self((0) as i32);
    pub const CMP_NODE_CHANNEL_MATTE_LIMIT_ALGORITHM_MAX: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeFlipMode(pub i32);

impl CMPNodeFlipMode {
    pub const CMP_NODE_FLIP_X: Self = Self((0) as i32);
    pub const CMP_NODE_FLIP_Y: Self = Self((1) as i32);
    pub const CMP_NODE_FLIP_X_Y: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeScaleMethod(pub i32);

impl CMPNodeScaleMethod {
    pub const CMP_NODE_SCALE_RELATIVE: Self = Self((0) as i32);
    pub const CMP_NODE_SCALE_ABSOLUTE: Self = Self((1) as i32);
    pub const CMP_NODE_SCALE_RENDER_PERCENT: Self = Self((2) as i32);
    pub const CMP_NODE_SCALE_RENDER_SIZE: Self = Self((3) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeScaleRenderSizeMethod(pub i32);

impl CMPNodeScaleRenderSizeMethod {
    pub const CMP_NODE_SCALE_RENDER_SIZE_STRETCH: Self = Self((0) as i32);
    pub const CMP_NODE_SCALE_RENDER_SIZE_FIT: Self = Self((1) as i32);
    pub const CMP_NODE_SCALE_RENDER_SIZE_CROP: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeFilterMethod(pub i32);

impl CMPNodeFilterMethod {
    pub const CMP_NODE_FILTER_SOFT: Self = Self((0) as i32);
    pub const CMP_NODE_FILTER_SHARP_BOX: Self = Self((1) as i32);
    pub const CMP_NODE_FILTER_LAPLACE: Self = Self((2) as i32);
    pub const CMP_NODE_FILTER_SOBEL: Self = Self((3) as i32);
    pub const CMP_NODE_FILTER_PREWITT: Self = Self((4) as i32);
    pub const CMP_NODE_FILTER_KIRSCH: Self = Self((5) as i32);
    pub const CMP_NODE_FILTER_SHADOW: Self = Self((6) as i32);
    pub const CMP_NODE_FILTER_SHARP_DIAMOND: Self = Self((7) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeLevelsChannel(pub i32);

impl CMPNodeLevelsChannel {
    pub const CMP_NODE_LEVLES_LUMINANCE: Self = Self((1) as i32);
    pub const CMP_NODE_LEVLES_RED: Self = Self((2) as i32);
    pub const CMP_NODE_LEVLES_GREEN: Self = Self((3) as i32);
    pub const CMP_NODE_LEVLES_BLUE: Self = Self((4) as i32);
    pub const CMP_NODE_LEVLES_LUMINANCE_BT709: Self = Self((5) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeToneMapType(pub i32);

impl CMPNodeToneMapType {
    pub const CMP_NODE_TONE_MAP_SIMPLE: Self = Self((0) as i32);
    pub const CMP_NODE_TONE_MAP_PHOTORECEPTOR: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeTrackPositionMode(pub i32);

impl CMPNodeTrackPositionMode {
    pub const CMP_NODE_TRACK_POSITION_ABSOLUTE: Self = Self((0) as i32);
    pub const CMP_NODE_TRACK_POSITION_RELATIVE_START: Self = Self((1) as i32);
    pub const CMP_NODE_TRACK_POSITION_RELATIVE_FRAME: Self = Self((2) as i32);
    pub const CMP_NODE_TRACK_POSITION_ABSOLUTE_FRAME: Self = Self((3) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeGlareType(pub i32);

impl CMPNodeGlareType {
    pub const CMP_NODE_GLARE_SIMPLE_STAR: Self = Self((0) as i32);
    pub const CMP_NODE_GLARE_FOG_GLOW: Self = Self((1) as i32);
    pub const CMP_NODE_GLARE_STREAKS: Self = Self((2) as i32);
    pub const CMP_NODE_GLARE_GHOST: Self = Self((3) as i32);
    pub const CMP_NODE_GLARE_BLOOM: Self = Self((4) as i32);
    pub const CMP_NODE_GLARE_SUN_BEAMS: Self = Self((5) as i32);
    pub const CMP_NODE_GLARE_KERNEL: Self = Self((6) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeKuwahara(pub i32);

impl CMPNodeKuwahara {
    pub const CMP_NODE_KUWAHARA_CLASSIC: Self = Self((0) as i32);
    pub const CMP_NODE_KUWAHARA_ANISOTROPIC: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeInterpolation(pub i32);

impl CMPNodeInterpolation {
    pub const CMP_NODE_INTERPOLATION_NEAREST: Self = Self((0) as i32);
    pub const CMP_NODE_INTERPOLATION_BILINEAR: Self = Self((1) as i32);
    pub const CMP_NODE_INTERPOLATION_BICUBIC: Self = Self((2) as i32);
    pub const CMP_NODE_INTERPOLATION_ANISOTROPIC: Self = Self((3) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeSetAlphaMode(pub i32);

impl CMPNodeSetAlphaMode {
    pub const CMP_NODE_SETALPHA_MODE_APPLY: Self = Self((0) as i32);
    pub const CMP_NODE_SETALPHA_MODE_REPLACE_ALPHA: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeBlurType(pub i32);

impl CMPNodeBlurType {
    pub const CMP_NODE_BLUR_TYPE_BOX: Self = Self((0) as i32);
    pub const CMP_NODE_BLUR_TYPE_TENT: Self = Self((1) as i32);
    pub const CMP_NODE_BLUR_TYPE_QUAD: Self = Self((2) as i32);
    pub const CMP_NODE_BLUR_TYPE_CUBIC: Self = Self((3) as i32);
    pub const CMP_NODE_BLUR_TYPE_CATROM: Self = Self((4) as i32);
    pub const CMP_NODE_BLUR_TYPE_GAUSS: Self = Self((5) as i32);
    pub const CMP_NODE_BLUR_TYPE_MITCH: Self = Self((6) as i32);
    pub const CMP_NODE_BLUR_TYPE_FAST_GAUSS: Self = Self((7) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeDenoisePrefilter(pub i32);

impl CMPNodeDenoisePrefilter {
    pub const CMP_NODE_DENOISE_PREFILTER_FAST: Self = Self((0) as i32);
    pub const CMP_NODE_DENOISE_PREFILTER_NONE: Self = Self((1) as i32);
    pub const CMP_NODE_DENOISE_PREFILTER_ACCURATE: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeDenoiseQuality(pub i32);

impl CMPNodeDenoiseQuality {
    pub const CMP_NODE_DENOISE_QUALITY_SCENE: Self = Self((0) as i32);
    pub const CMP_NODE_DENOISE_QUALITY_HIGH: Self = Self((1) as i32);
    pub const CMP_NODE_DENOISE_QUALITY_BALANCED: Self = Self((2) as i32);
    pub const CMP_NODE_DENOISE_QUALITY_FAST: Self = Self((3) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeCombSepColorMode(pub u8);

impl CMPNodeCombSepColorMode {
    pub const CMP_NODE_COMBSEP_COLOR_RGB: Self = Self((0) as u8);
    pub const CMP_NODE_COMBSEP_COLOR_HSV: Self = Self((1) as u8);
    pub const CMP_NODE_COMBSEP_COLOR_HSL: Self = Self((2) as u8);
    pub const CMP_NODE_COMBSEP_COLOR_YCC: Self = Self((3) as u8);
    pub const CMP_NODE_COMBSEP_COLOR_YUV: Self = Self((4) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeCryptomatteSource(pub i32);

impl CMPNodeCryptomatteSource {
    pub const CMP_NODE_CRYPTOMATTE_SOURCE_RENDER: Self = Self((0) as i32);
    pub const CMP_NODE_CRYPTOMATTE_SOURCE_IMAGE: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeChannelMatteColorSpace(pub i32);

impl CMPNodeChannelMatteColorSpace {
    pub const CMP_NODE_CHANNEL_MATTE_CS_RGB: Self = Self((0) as i32);
    pub const CMP_NODE_CHANNEL_MATTE_CS_HSV: Self = Self((1) as i32);
    pub const CMP_NODE_CHANNEL_MATTE_CS_YUV: Self = Self((2) as i32);
    pub const CMP_NODE_CHANNEL_MATTE_CS_YCC: Self = Self((3) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeLensDistortionType(pub i32);

impl CMPNodeLensDistortionType {
    pub const CMP_NODE_LENS_DISTORTION_RADIAL: Self = Self((0) as i32);
    pub const CMP_NODE_LENS_DISTORTION_HORIZONTAL: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeAlphaOverOperationType(pub i32);

impl CMPNodeAlphaOverOperationType {
    pub const CMP_NODE_ALPHA_OVER_OPERATION_TYPE_OVER: Self = Self((0) as i32);
    pub const CMP_NODE_ALPHA_OVER_OPERATION_TYPE_DISJOINT_OVER: Self = Self((1) as i32);
    pub const CMP_NODE_ALPHA_OVER_OPERATION_TYPE_CONJOINT_OVER: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeRelativeToPixelDataType(pub i32);

impl CMPNodeRelativeToPixelDataType {
    pub const CMP_NODE_RELATIVE_TO_PIXEL_DATA_TYPE_FLOAT: Self = Self((0) as i32);
    pub const CMP_NODE_RELATIVE_TO_PIXEL_DATA_TYPE_VECTOR: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeRelativeToPixelReferenceDimension(pub i32);

impl CMPNodeRelativeToPixelReferenceDimension {
    pub const CMP_NODE_RELATIVE_TO_PIXEL_REFERENCE_DIMENSION_PER_DIMENSION: Self = Self((0) as i32);
    pub const CMP_NODE_RELATIVE_TO_PIXEL_REFERENCE_DIMENSION_X: Self = Self((1) as i32);
    pub const CMP_NODE_RELATIVE_TO_PIXEL_REFERENCE_DIMENSION_Y: Self = Self((2) as i32);
    pub const CMP_NODE_RELATIVE_TO_PIXEL_REFERENCE_DIMENSION_GREATER: Self = Self((3) as i32);
    pub const CMP_NODE_RELATIVE_TO_PIXEL_REFERENCE_DIMENSION_SMALLER: Self = Self((4) as i32);
    pub const CMP_NODE_RELATIVE_TO_PIXEL_REFERENCE_DIMENSION_DIAGONAL: Self = Self((5) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeStringToImageHorizontalAlignment(pub i32);

impl CMPNodeStringToImageHorizontalAlignment {
    pub const CMP_NODE_STRING_TO_IMAGE_HORIZONTAL_ALIGNMENT_LEFT: Self = Self((0) as i32);
    pub const CMP_NODE_STRING_TO_IMAGE_HORIZONTAL_ALIGNMENT_CENTER: Self = Self((1) as i32);
    pub const CMP_NODE_STRING_TO_IMAGE_HORIZONTAL_ALIGNMENT_RIGHT: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CMPNodeStringToImageVerticalAlignment(pub i32);

impl CMPNodeStringToImageVerticalAlignment {
    pub const CMP_NODE_STRING_TO_IMAGE_VERTICAL_ALIGNMENT_TOP: Self = Self((0) as i32);
    pub const CMP_NODE_STRING_TO_IMAGE_VERTICAL_ALIGNMENT_TOP_BASELINE: Self = Self((1) as i32);
    pub const CMP_NODE_STRING_TO_IMAGE_VERTICAL_ALIGNMENT_MIDDLE: Self = Self((2) as i32);
    pub const CMP_NODE_STRING_TO_IMAGE_VERTICAL_ALIGNMENT_BOTTOM_BASELINE: Self = Self((3) as i32);
    pub const CMP_NODE_STRING_TO_IMAGE_VERTICAL_ALIGNMENT_BOTTOM: Self = Self((4) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eNodeScattering_PhaseFunc(pub i16);

impl eNodeScattering_PhaseFunc {
    pub const SHD_PHASE_HENYEY_GREENSTEIN: Self = Self((0) as i16);
    pub const SHD_PHASE_FOURNIER_FORAND: Self = Self((1) as i16);
    pub const SHD_PHASE_DRAINE: Self = Self((2) as i16);
    pub const SHD_PHASE_RAYLEIGH: Self = Self((3) as i16);
    pub const SHD_PHASE_MIE: Self = Self((4) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeShaderOutputTarget(pub i32);

impl NodeShaderOutputTarget {
    pub const SHD_OUTPUT_ALL: Self = Self((0) as i32);
    pub const SHD_OUTPUT_EEVEE: Self = Self((1) as i32);
    pub const SHD_OUTPUT_CYCLES: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeProximityTargetType(pub u8);

impl GeometryNodeProximityTargetType {
    pub const GEO_NODE_PROX_TARGET_POINTS: Self = Self((0) as u8);
    pub const GEO_NODE_PROX_TARGET_EDGES: Self = Self((1) as u8);
    pub const GEO_NODE_PROX_TARGET_FACES: Self = Self((2) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeCurvePrimitiveCircleMode(pub i32);

impl GeometryNodeCurvePrimitiveCircleMode {
    pub const GEO_NODE_CURVE_PRIMITIVE_CIRCLE_TYPE_POINTS: Self = Self((0) as i32);
    pub const GEO_NODE_CURVE_PRIMITIVE_CIRCLE_TYPE_RADIUS: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeCurveHandleType(pub u8);

impl GeometryNodeCurveHandleType {
    pub const GEO_NODE_CURVE_HANDLE_FREE: Self = Self((0) as u8);
    pub const GEO_NODE_CURVE_HANDLE_AUTO: Self = Self((1) as u8);
    pub const GEO_NODE_CURVE_HANDLE_VECTOR: Self = Self((2) as u8);
    pub const GEO_NODE_CURVE_HANDLE_ALIGN: Self = Self((3) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeCurveHandleMode(pub u8);

impl GeometryNodeCurveHandleMode {
    pub const GEO_NODE_CURVE_HANDLE_LEFT: Self = Self(((1 << 0)) as u8);
    pub const GEO_NODE_CURVE_HANDLE_RIGHT: Self = Self(((1 << 1)) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeDistributePointsInVolumeMode(pub i32);

impl GeometryNodeDistributePointsInVolumeMode {
    pub const GEO_NODE_DISTRIBUTE_POINTS_IN_VOLUME_DENSITY_RANDOM: Self = Self((0) as i32);
    pub const GEO_NODE_DISTRIBUTE_POINTS_IN_VOLUME_DENSITY_GRID: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeDistributePointsOnFacesMode(pub i32);

impl GeometryNodeDistributePointsOnFacesMode {
    pub const GEO_NODE_POINT_DISTRIBUTE_POINTS_ON_FACES_RANDOM: Self = Self((0) as i32);
    pub const GEO_NODE_POINT_DISTRIBUTE_POINTS_ON_FACES_POISSON: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeExtrudeMeshMode(pub u8);

impl GeometryNodeExtrudeMeshMode {
    pub const GEO_NODE_EXTRUDE_MESH_VERTICES: Self = Self((0) as u8);
    pub const GEO_NODE_EXTRUDE_MESH_EDGES: Self = Self((1) as u8);
    pub const GEO_NODE_EXTRUDE_MESH_FACES: Self = Self((2) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FunctionNodeRotateEulerType(pub i32);

impl FunctionNodeRotateEulerType {
    pub const FN_NODE_ROTATE_EULER_TYPE_EULER: Self = Self((0) as i32);
    pub const FN_NODE_ROTATE_EULER_TYPE_AXIS_ANGLE: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FunctionNodeRotateEulerSpace(pub i32);

impl FunctionNodeRotateEulerSpace {
    pub const FN_NODE_ROTATE_EULER_SPACE_OBJECT: Self = Self((0) as i32);
    pub const FN_NODE_ROTATE_EULER_SPACE_LOCAL: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeAlignEulerToVectorAxis(pub i32);

impl NodeAlignEulerToVectorAxis {
    pub const FN_NODE_ALIGN_EULER_TO_VECTOR_AXIS_X: Self = Self((0) as i32);
    pub const FN_NODE_ALIGN_EULER_TO_VECTOR_AXIS_Y: Self = Self((1) as i32);
    pub const FN_NODE_ALIGN_EULER_TO_VECTOR_AXIS_Z: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeAlignEulerToVectorPivotAxis(pub i32);

impl NodeAlignEulerToVectorPivotAxis {
    pub const FN_NODE_ALIGN_EULER_TO_VECTOR_PIVOT_AXIS_AUTO: Self = Self((0) as i32);
    pub const FN_NODE_ALIGN_EULER_TO_VECTOR_PIVOT_AXIS_X: Self = Self((1) as i32);
    pub const FN_NODE_ALIGN_EULER_TO_VECTOR_PIVOT_AXIS_Y: Self = Self((2) as i32);
    pub const FN_NODE_ALIGN_EULER_TO_VECTOR_PIVOT_AXIS_Z: Self = Self((3) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeTransformSpace(pub u8);

impl GeometryNodeTransformSpace {
    pub const GEO_NODE_TRANSFORM_SPACE_ORIGINAL: Self = Self((0) as u8);
    pub const GEO_NODE_TRANSFORM_SPACE_RELATIVE: Self = Self((1) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodePointsToVolumeResolutionMode(pub u8);

impl GeometryNodePointsToVolumeResolutionMode {
    pub const GEO_NODE_POINTS_TO_VOLUME_RESOLUTION_MODE_AMOUNT: Self = Self((0) as u8);
    pub const GEO_NODE_POINTS_TO_VOLUME_RESOLUTION_MODE_SIZE: Self = Self((1) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeMeshCircleFillType(pub u8);

impl GeometryNodeMeshCircleFillType {
    pub const GEO_NODE_MESH_CIRCLE_FILL_NONE: Self = Self((0) as u8);
    pub const GEO_NODE_MESH_CIRCLE_FILL_NGON: Self = Self((1) as u8);
    pub const GEO_NODE_MESH_CIRCLE_FILL_TRIANGLE_FAN: Self = Self((2) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeMergeByDistanceMode(pub u8);

impl GeometryNodeMergeByDistanceMode {
    pub const GEO_NODE_MERGE_BY_DISTANCE_MODE_ALL: Self = Self((0) as u8);
    pub const GEO_NODE_MERGE_BY_DISTANCE_MODE_CONNECTED: Self = Self((1) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeUVUnwrapMethod(pub u8);

impl GeometryNodeUVUnwrapMethod {
    pub const GEO_NODE_UV_UNWRAP_METHOD_ANGLE_BASED: Self = Self((0) as u8);
    pub const GEO_NODE_UV_UNWRAP_METHOD_CONFORMAL: Self = Self((1) as u8);
    pub const GEO_NODE_UV_UNWRAP_METHOD_MINIMUM_STRETCH: Self = Self((2) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeRealizeInstanceFlag(pub i32);

impl GeometryNodeRealizeInstanceFlag {
    pub const GEO_NODE_REALIZE_TO_POINT_DOMAIN: Self = Self(((1 << 0)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeMeshLineMode(pub u8);

impl GeometryNodeMeshLineMode {
    pub const GEO_NODE_MESH_LINE_MODE_END_POINTS: Self = Self((0) as u8);
    pub const GEO_NODE_MESH_LINE_MODE_OFFSET: Self = Self((1) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeMeshLineCountMode(pub u8);

impl GeometryNodeMeshLineCountMode {
    pub const GEO_NODE_MESH_LINE_COUNT_TOTAL: Self = Self((0) as u8);
    pub const GEO_NODE_MESH_LINE_COUNT_RESOLUTION: Self = Self((1) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeCurvePrimitiveArcMode(pub u8);

impl GeometryNodeCurvePrimitiveArcMode {
    pub const GEO_NODE_CURVE_PRIMITIVE_ARC_TYPE_POINTS: Self = Self((0) as u8);
    pub const GEO_NODE_CURVE_PRIMITIVE_ARC_TYPE_RADIUS: Self = Self((1) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeCurvePrimitiveLineMode(pub u8);

impl GeometryNodeCurvePrimitiveLineMode {
    pub const GEO_NODE_CURVE_PRIMITIVE_LINE_MODE_POINTS: Self = Self((0) as u8);
    pub const GEO_NODE_CURVE_PRIMITIVE_LINE_MODE_DIRECTION: Self = Self((1) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeCurvePrimitiveQuadMode(pub u8);

impl GeometryNodeCurvePrimitiveQuadMode {
    pub const GEO_NODE_CURVE_PRIMITIVE_QUAD_MODE_RECTANGLE: Self = Self((0) as u8);
    pub const GEO_NODE_CURVE_PRIMITIVE_QUAD_MODE_PARALLELOGRAM: Self = Self((1) as u8);
    pub const GEO_NODE_CURVE_PRIMITIVE_QUAD_MODE_TRAPEZOID: Self = Self((2) as u8);
    pub const GEO_NODE_CURVE_PRIMITIVE_QUAD_MODE_KITE: Self = Self((3) as u8);
    pub const GEO_NODE_CURVE_PRIMITIVE_QUAD_MODE_POINTS: Self = Self((4) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeCurvePrimitiveBezierSegmentMode(pub u8);

impl GeometryNodeCurvePrimitiveBezierSegmentMode {
    pub const GEO_NODE_CURVE_PRIMITIVE_BEZIER_SEGMENT_POSITION: Self = Self((0) as u8);
    pub const GEO_NODE_CURVE_PRIMITIVE_BEZIER_SEGMENT_OFFSET: Self = Self((1) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeCurveResampleMode(pub u8);

impl GeometryNodeCurveResampleMode {
    pub const GEO_NODE_CURVE_RESAMPLE_COUNT: Self = Self((0) as u8);
    pub const GEO_NODE_CURVE_RESAMPLE_LENGTH: Self = Self((1) as u8);
    pub const GEO_NODE_CURVE_RESAMPLE_EVALUATED: Self = Self((2) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeCurveSampleMode(pub u8);

impl GeometryNodeCurveSampleMode {
    pub const GEO_NODE_CURVE_SAMPLE_FACTOR: Self = Self((0) as u8);
    pub const GEO_NODE_CURVE_SAMPLE_LENGTH: Self = Self((1) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeCurveFilletMode(pub u8);

impl GeometryNodeCurveFilletMode {
    pub const GEO_NODE_CURVE_FILLET_BEZIER: Self = Self((0) as u8);
    pub const GEO_NODE_CURVE_FILLET_POLY: Self = Self((1) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeAttributeTransferMode(pub u8);

impl GeometryNodeAttributeTransferMode {
    pub const GEO_NODE_ATTRIBUTE_TRANSFER_NEAREST_FACE_INTERPOLATED: Self = Self((0) as u8);
    pub const GEO_NODE_ATTRIBUTE_TRANSFER_NEAREST: Self = Self((1) as u8);
    pub const GEO_NODE_ATTRIBUTE_TRANSFER_INDEX: Self = Self((2) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeRaycastMapMode(pub u8);

impl GeometryNodeRaycastMapMode {
    pub const GEO_NODE_RAYCAST_INTERPOLATED: Self = Self((0) as u8);
    pub const GEO_NODE_RAYCAST_NEAREST: Self = Self((1) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeCurveFillMode(pub u8);

impl GeometryNodeCurveFillMode {
    pub const GEO_NODE_CURVE_FILL_MODE_TRIANGULATED: Self = Self((0) as u8);
    pub const GEO_NODE_CURVE_FILL_MODE_NGONS: Self = Self((1) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeCurveFillRule(pub u8);

impl GeometryNodeCurveFillRule {
    pub const GEO_NODE_CURVE_FILL_RULE_EVEN_ODD: Self = Self((0) as u8);
    pub const GEO_NODE_CURVE_FILL_RULE_NON_ZERO: Self = Self((1) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeMeshToPointsMode(pub u8);

impl GeometryNodeMeshToPointsMode {
    pub const GEO_NODE_MESH_TO_POINTS_VERTICES: Self = Self((0) as u8);
    pub const GEO_NODE_MESH_TO_POINTS_EDGES: Self = Self((1) as u8);
    pub const GEO_NODE_MESH_TO_POINTS_FACES: Self = Self((2) as u8);
    pub const GEO_NODE_MESH_TO_POINTS_CORNERS: Self = Self((3) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeStringToCurvesOverflowMode(pub u8);

impl GeometryNodeStringToCurvesOverflowMode {
    pub const GEO_NODE_STRING_TO_CURVES_MODE_OVERFLOW: Self = Self((0) as u8);
    pub const GEO_NODE_STRING_TO_CURVES_MODE_SCALE_TO_FIT: Self = Self((1) as u8);
    pub const GEO_NODE_STRING_TO_CURVES_MODE_TRUNCATE: Self = Self((2) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeStringToCurvesAlignXMode(pub u8);

impl GeometryNodeStringToCurvesAlignXMode {
    pub const GEO_NODE_STRING_TO_CURVES_ALIGN_X_LEFT: Self = Self((0) as u8);
    pub const GEO_NODE_STRING_TO_CURVES_ALIGN_X_CENTER: Self = Self((1) as u8);
    pub const GEO_NODE_STRING_TO_CURVES_ALIGN_X_RIGHT: Self = Self((2) as u8);
    pub const GEO_NODE_STRING_TO_CURVES_ALIGN_X_JUSTIFY: Self = Self((3) as u8);
    pub const GEO_NODE_STRING_TO_CURVES_ALIGN_X_FLUSH: Self = Self((4) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeStringToCurvesAlignYMode(pub u8);

impl GeometryNodeStringToCurvesAlignYMode {
    pub const GEO_NODE_STRING_TO_CURVES_ALIGN_Y_TOP_BASELINE: Self = Self((0) as u8);
    pub const GEO_NODE_STRING_TO_CURVES_ALIGN_Y_TOP: Self = Self((1) as u8);
    pub const GEO_NODE_STRING_TO_CURVES_ALIGN_Y_MIDDLE: Self = Self((2) as u8);
    pub const GEO_NODE_STRING_TO_CURVES_ALIGN_Y_BOTTOM_BASELINE: Self = Self((3) as u8);
    pub const GEO_NODE_STRING_TO_CURVES_ALIGN_Y_BOTTOM: Self = Self((4) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeStringToCurvesPivotMode(pub u8);

impl GeometryNodeStringToCurvesPivotMode {
    pub const GEO_NODE_STRING_TO_CURVES_PIVOT_MODE_MIDPOINT: Self = Self((0) as u8);
    pub const GEO_NODE_STRING_TO_CURVES_PIVOT_MODE_TOP_LEFT: Self = Self((1) as u8);
    pub const GEO_NODE_STRING_TO_CURVES_PIVOT_MODE_TOP_CENTER: Self = Self((2) as u8);
    pub const GEO_NODE_STRING_TO_CURVES_PIVOT_MODE_TOP_RIGHT: Self = Self((3) as u8);
    pub const GEO_NODE_STRING_TO_CURVES_PIVOT_MODE_BOTTOM_LEFT: Self = Self((4) as u8);
    pub const GEO_NODE_STRING_TO_CURVES_PIVOT_MODE_BOTTOM_CENTER: Self = Self((5) as u8);
    pub const GEO_NODE_STRING_TO_CURVES_PIVOT_MODE_BOTTOM_RIGHT: Self = Self((6) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeDeleteGeometryMode(pub i8);

impl GeometryNodeDeleteGeometryMode {
    pub const GEO_NODE_DELETE_GEOMETRY_MODE_ALL: Self = Self((0) as i8);
    pub const GEO_NODE_DELETE_GEOMETRY_MODE_EDGE_FACE: Self = Self((1) as i8);
    pub const GEO_NODE_DELETE_GEOMETRY_MODE_ONLY_FACE: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeScaleElementsMode(pub i32);

impl GeometryNodeScaleElementsMode {
    pub const GEO_NODE_SCALE_ELEMENTS_UNIFORM: Self = Self((0) as i32);
    pub const GEO_NODE_SCALE_ELEMENTS_SINGLE_AXIS: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeCombSepColorMode(pub i8);

impl NodeCombSepColorMode {
    pub const NODE_COMBSEP_COLOR_RGB: Self = Self((0) as i8);
    pub const NODE_COMBSEP_COLOR_HSV: Self = Self((1) as i8);
    pub const NODE_COMBSEP_COLOR_HSL: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeGizmoColor(pub i32);

impl GeometryNodeGizmoColor {
    pub const GEO_NODE_GIZMO_COLOR_PRIMARY: Self = Self((0) as i32);
    pub const GEO_NODE_GIZMO_COLOR_SECONDARY: Self = Self((1) as i32);
    pub const GEO_NODE_GIZMO_COLOR_X: Self = Self((2) as i32);
    pub const GEO_NODE_GIZMO_COLOR_Y: Self = Self((3) as i32);
    pub const GEO_NODE_GIZMO_COLOR_Z: Self = Self((4) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodeLinearGizmoDrawStyle(pub i32);

impl GeometryNodeLinearGizmoDrawStyle {
    pub const GEO_NODE_LINEAR_GIZMO_DRAW_STYLE_ARROW: Self = Self((0) as i32);
    pub const GEO_NODE_LINEAR_GIZMO_DRAW_STYLE_CROSS: Self = Self((1) as i32);
    pub const GEO_NODE_LINEAR_GIZMO_DRAW_STYLE_BOX: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeGeometryTransformMode(pub i32);

impl NodeGeometryTransformMode {
    pub const GEO_NODE_TRANSFORM_MODE_COMPONENTS: Self = Self((0) as i32);
    pub const GEO_NODE_TRANSFORM_MODE_MATRIX: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeGeometryMergeLayersMode(pub i32);

impl NodeGeometryMergeLayersMode {
    pub const GEO_NODE_MERGE_LAYERS_BY_NAME: Self = Self((0) as i32);
    pub const GEO_NODE_MERGE_LAYERS_BY_ID: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeGeometryGreasePencilStrokeType(pub i8);

impl NodeGeometryGreasePencilStrokeType {
    pub const GEO_NODE_GREASE_PENCIL_STROKE: Self = Self((0) as i8);
    pub const GEO_NODE_GREASE_PENCIL_FILL: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeGeometryRasterizePointsItemType(pub i32);

impl NodeGeometryRasterizePointsItemType {
    pub const GEO_NODE_RASTERIZE_POINTS_ITEM_TYPE_SCALAR: Self = Self((0) as i32);
    pub const GEO_NODE_RASTERIZE_POINTS_ITEM_TYPE_VECTOR: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeCommentFlag(pub u8);

impl NodeCommentFlag {
    pub const Edit: Self = Self(((1 << 0)) as u8);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeStack {
    pub vec: [f32; 4],
}

impl Default for bNodeStack {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeSocket {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub prop: *mut core::ffi::c_void,
    pub identifier: [u8; 64],
    pub name: [u8; 64],
    pub storage: *mut core::ffi::c_void,
    pub r#type: eNodeSocketDatatype,
}

impl Default for bNodeSocket {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodePanelState {
    pub identifier: i32,
    pub flag: eNodePanelFlag,
}

impl Default for bNodePanelState {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNode {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub inputs: ListBaseT<bNodeSocket>,
    pub outputs: ListBaseT<bNodeSocket>,
    pub name: [u8; 64],
    pub identifier: i32,
    pub flag: eNode_Flag,
}

impl Default for bNode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeLink {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub fromnode: *mut core::ffi::c_void,
    pub tonode: *mut core::ffi::c_void,
    pub fromsock: *mut core::ffi::c_void,
    pub tosock: *mut core::ffi::c_void,
    pub flag: eNodeLink_Flag,
}

impl Default for bNodeLink {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNestedNodePath {
    pub node_id: i32,
    pub id_in_node: i32,
}

impl Default for bNestedNodePath {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNestedNodeRef {
    pub id: i32,
    pub _pad: [u8; 4],
}

impl Default for bNestedNodeRef {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeTree {
    pub adt: *mut core::ffi::c_void,
    pub owner_id: *mut core::ffi::c_void,
    pub typeinfo: *mut core::ffi::c_void,
    pub idname: [u8; 64],
    pub description: *mut core::ffi::c_void,
    pub gpd: *mut core::ffi::c_void,
    pub view_center: [f32; 2],
}

impl Default for bNodeTree {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeSocketValueInt {
    pub subtype: i32,
    pub value: i32,
    pub min: i32,
    pub max: i32,
}

impl Default for bNodeSocketValueInt {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeSocketValueFloat {
    pub subtype: i32,
    pub value: f32,
    pub min: f32,
    pub max: f32,
}

impl Default for bNodeSocketValueFloat {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeSocketValueBoolean {
    pub value: i8,
}

impl Default for bNodeSocketValueBoolean {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeSocketValueVector {
    pub subtype: i32,
    pub value: [f32; 4],
}

impl Default for bNodeSocketValueVector {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeSocketValueIntVector {
    pub subtype: i32,
    pub value: [i32; 3],
}

impl Default for bNodeSocketValueIntVector {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeSocketValueRotation {
    pub value_euler: [f32; 3],
}

impl Default for bNodeSocketValueRotation {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeSocketValueRGBA {
    pub value: [f32; 4],
}

impl Default for bNodeSocketValueRGBA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeSocketValueString {
    pub subtype: i32,
    pub _pad: [u8; 4],
}

impl Default for bNodeSocketValueString {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeSocketValueObject {
    pub value: *mut core::ffi::c_void,
}

impl Default for bNodeSocketValueObject {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeSocketValueImage {
    pub value: *mut core::ffi::c_void,
}

impl Default for bNodeSocketValueImage {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeSocketValueCollection {
    pub value: *mut core::ffi::c_void,
}

impl Default for bNodeSocketValueCollection {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeSocketValueTexture {
    pub value: *mut core::ffi::c_void,
}

impl Default for bNodeSocketValueTexture {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeSocketValueMaterial {
    pub value: *mut core::ffi::c_void,
}

impl Default for bNodeSocketValueMaterial {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeSocketValueFont {
    pub value: *mut core::ffi::c_void,
}

impl Default for bNodeSocketValueFont {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeSocketValueScene {
    pub value: *mut core::ffi::c_void,
}

impl Default for bNodeSocketValueScene {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeSocketValueText {
    pub value: *mut core::ffi::c_void,
}

impl Default for bNodeSocketValueText {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeSocketValueMask {
    pub value: *mut core::ffi::c_void,
}

impl Default for bNodeSocketValueMask {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeSocketValueSound {
    pub value: *mut core::ffi::c_void,
}

impl Default for bNodeSocketValueSound {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeSocketValueMenu {
    pub value: i32,
    pub runtime_flag: i32,
    pub enum_items: *mut core::ffi::c_void,
}

impl Default for bNodeSocketValueMenu {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GeometryNodeAssetTraits {
    pub flag: i32,
    pub _pad: [u8; 4],
}

impl Default for GeometryNodeAssetTraits {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CompositorNodeAssetTraits {
    pub flag: i32,
    pub _pad: [u8; 4],
}

impl Default for CompositorNodeAssetTraits {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeFrame {
    pub label_size: i16,
}

impl Default for NodeFrame {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeReroute {

}

impl Default for NodeReroute {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeImplicitConversion {

}

impl Default for NodeImplicitConversion {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeImageAnim {
    pub _pad: [u8; 2],
}

impl Default for NodeImageAnim {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ColorCorrectionData {
    pub _pad: [u8; 4],
}

impl Default for ColorCorrectionData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeColorCorrection {

}

impl Default for NodeColorCorrection {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeBokehImage {

}

impl Default for NodeBokehImage {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeBoxMask {
    pub _pad: [u8; 4],
}

impl Default for NodeBoxMask {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeEllipseMask {
    pub _pad: [u8; 4],
}

impl Default for NodeEllipseMask {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeImageLayer {
    pub pass_name: [u8; 64],
}

impl Default for NodeImageLayer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeBlurData {

}

impl Default for NodeBlurData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeDBlurData {
    pub _pad: [u8; 2],
}

impl Default for NodeDBlurData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeBilateralBlurData {
    pub _pad: [u8; 2],
}

impl Default for NodeBilateralBlurData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeKuwaharaData {
    pub _pad: [u8; 3],
}

impl Default for NodeKuwaharaData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeAntiAliasingData {

}

impl Default for NodeAntiAliasingData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeHueSat {

}

impl Default for NodeHueSat {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeImageFile {
    pub im_format: ImageFormatData,
    pub sfra: i32,
    pub efra: i32,
}

impl Default for NodeImageFile {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeCompositorFileOutputItem {
    pub identifier: i32,
    pub socket_type: eNodeSocketDatatype,
}

impl Default for NodeCompositorFileOutputItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeCompositorFileOutput {
    pub file_name: *mut core::ffi::c_void,
    pub format: ImageFormatData,
    pub items: *mut core::ffi::c_void,
    pub items_count: i32,
    pub active_item_index: i32,
    pub save_as_render: i8,
    pub use_file_extension: i8,
    pub _pad: [u8; 6],
}

impl Default for NodeCompositorFileOutput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeImageMultiFileSocket {
    pub _pad1: [u8; 3],
}

impl Default for NodeImageMultiFileSocket {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeChroma {

}

impl Default for NodeChroma {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeTwoXYs {

}

impl Default for NodeTwoXYs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeTwoFloats {

}

impl Default for NodeTwoFloats {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeVertexCol {

}

impl Default for NodeVertexCol {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeCMPCombSepColor {
    pub ycc_mode: u8,
}

impl Default for NodeCMPCombSepColor {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeDefocus {
    pub no_zbuf: i8,
    pub _pad0: i8,
}

impl Default for NodeDefocus {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeScriptDict {
    pub node: *mut core::ffi::c_void,
}

impl Default for NodeScriptDict {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGlare {
    pub _pad0: i8,
}

impl Default for NodeGlare {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeTonemap {

}

impl Default for NodeTonemap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeLensDist {
    pub _pad: [u8; 2],
}

impl Default for NodeLensDist {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeColorBalance {

}

impl Default for NodeColorBalance {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeColorspill {

}

impl Default for NodeColorspill {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeConvertColorSpace {
    pub from_interop_id: [u8; 64],
    pub to_color_space: [u8; 64],
    pub to_interop_id: [u8; 64],
}

impl Default for NodeConvertColorSpace {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeConvertToDisplay {
    pub view_settings: ColorManagedViewSettings,
}

impl Default for NodeConvertToDisplay {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeDilateErode {

}

impl Default for NodeDilateErode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeMask {

}

impl Default for NodeMask {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeSetAlpha {

}

impl Default for NodeSetAlpha {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeTexBase {
    pub tex_mapping: TexMapping,
    pub color_mapping: ColorMapping,
}

impl Default for NodeTexBase {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeTexSky {
    pub sky_model: i32,
    pub sun_direction: [f32; 3],
    pub _0: f32,
    pub _1: f32,
}

impl Default for NodeTexSky {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeTexImage {
    pub iuser: ImageUser,
    pub projection: i32,
    pub projection_blend: f32,
    pub interpolation: i32,
    pub extension: i32,
    pub _pad: [u8; 4],
}

impl Default for NodeTexImage {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeTexChecker {

}

impl Default for NodeTexChecker {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeTexBrick {
    pub offset_freq: i32,
    pub squash_freq: i32,
    pub offset: f32,
    pub squash: f32,
}

impl Default for NodeTexBrick {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeTexEnvironment {
    pub iuser: ImageUser,
    pub projection: i32,
    pub interpolation: i32,
    pub _pad: [u8; 4],
}

impl Default for NodeTexEnvironment {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeTexGabor {
    pub r#type: NodeGaborType,
    pub _pad: [u8; 7],
}

impl Default for NodeTexGabor {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeTexGradient {
    pub gradient_type: i32,
    pub _pad: [u8; 4],
}

impl Default for NodeTexGradient {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeTexNoise {
    pub dimensions: i32,
    pub r#type: u8,
    pub normalize: u8,
    pub _pad: [u8; 2],
}

impl Default for NodeTexNoise {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeTexVoronoi {
    pub dimensions: i32,
    pub feature: i32,
    pub distance: i32,
    pub normalize: i32,
    pub _pad: [u8; 4],
}

impl Default for NodeTexVoronoi {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeTexMusgrave {

}

impl Default for NodeTexMusgrave {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeTexWave {
    pub wave_type: i32,
    pub bands_direction: i32,
    pub rings_direction: i32,
    pub wave_profile: i32,
}

impl Default for NodeTexWave {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeTexMagic {
    pub depth: i32,
    pub _pad: [u8; 4],
}

impl Default for NodeTexMagic {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeShaderAttribute {
    pub r#type: i32,
    pub _pad: [u8; 4],
}

impl Default for NodeShaderAttribute {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeShaderVectTransform {
    pub convert_from: i32,
    pub convert_to: i32,
    pub _pad: [u8; 4],
}

impl Default for NodeShaderVectTransform {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeShaderPrincipled {
    pub _pad: [u8; 3],
}

impl Default for NodeShaderPrincipled {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeShaderHairPrincipled {
    pub parametrization: i16,
    pub _pad: [u8; 4],
}

impl Default for NodeShaderHairPrincipled {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct TexNodeOutput {

}

impl Default for TexNodeOutput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeKeyingScreenData {

}

impl Default for NodeKeyingScreenData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeKeyingData {

}

impl Default for NodeKeyingData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeTrackPosData {
    pub track_name: [u8; 64],
}

impl Default for NodeTrackPosData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeTransformData {

}

impl Default for NodeTransformData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeTranslateData {

}

impl Default for NodeTranslateData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeRotateData {

}

impl Default for NodeRotateData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeScaleData {

}

impl Default for NodeScaleData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeCornerPinData {

}

impl Default for NodeCornerPinData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeDisplaceData {

}

impl Default for NodeDisplaceData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeMapUVData {

}

impl Default for NodeMapUVData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodePlaneTrackDeformData {
    pub plane_track_name: [u8; 64],
    pub _pad: [u8; 2],
}

impl Default for NodePlaneTrackDeformData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeShaderScript {
    pub flag: i32,
    pub filepath: [u8; 1024],
    pub bytecode_hash: [u8; 64],
    pub bytecode: *mut core::ffi::c_void,
}

impl Default for NodeShaderScript {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeShaderTangent {
    pub axis: i32,
    pub uv_map: [u8; 64],
}

impl Default for NodeShaderTangent {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeShaderNormalMap {
    pub uv_map: [u8; 64],
    pub convention: i8,
    pub base: i8,
    pub _pad: [u8; 6],
}

impl Default for NodeShaderNormalMap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeRadialTiling {
    pub _pad: [u8; 7],
}

impl Default for NodeRadialTiling {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeShaderUVMap {

}

impl Default for NodeShaderUVMap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeShaderVertexColor {

}

impl Default for NodeShaderVertexColor {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeShaderTexIES {
    pub filepath: [u8; 1024],
}

impl Default for NodeShaderTexIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeShaderOutputAOV {

}

impl Default for NodeShaderOutputAOV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeSunBeams {

}

impl Default for NodeSunBeams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CryptomatteEntry {
    pub encoded_hash: f32,
    pub name: [u8; 64],
    pub _pad: [u8; 4],
}

impl Default for CryptomatteEntry {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CryptomatteLayer {
    pub name: [u8; 64],
}

impl Default for CryptomatteLayer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeCryptomatte_Runtime {

}

impl Default for NodeCryptomatte_Runtime {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeCryptomatte {
    pub entries: ListBaseT<CryptomatteEntry>,
    pub nullptr: ListBaseT<CryptomatteEntry>,
}

impl Default for NodeCryptomatte {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeDenoise {
    pub _pad: [u8; 1],
}

impl Default for NodeDenoise {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeMapRange {
    pub interpolation_type: u8,
    pub clamp: u8,
    pub _pad: [u8; 5],
}

impl Default for NodeMapRange {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeRandomValue {

}

impl Default for NodeRandomValue {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeAccumulateField {
    pub domain: u8,
}

impl Default for NodeAccumulateField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeInputBool {

}

impl Default for NodeInputBool {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeInputInt {

}

impl Default for NodeInputInt {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeInputMenu {

}

impl Default for NodeInputMenu {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeInputRotation {

}

impl Default for NodeInputRotation {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeInputVector {

}

impl Default for NodeInputVector {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeInputIntVector {

}

impl Default for NodeInputIntVector {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeInputColor {

}

impl Default for NodeInputColor {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeInputString {
    pub textbox_state: TextboxState,
}

impl Default for NodeInputString {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryExtrudeMesh {

}

impl Default for NodeGeometryExtrudeMesh {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryObjectInfo {

}

impl Default for NodeGeometryObjectInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryPointsToVolume {

}

impl Default for NodeGeometryPointsToVolume {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryCollectionInfo {

}

impl Default for NodeGeometryCollectionInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryProximity {

}

impl Default for NodeGeometryProximity {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryVolumeToMesh {

}

impl Default for NodeGeometryVolumeToMesh {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryMeshToVolume {

}

impl Default for NodeGeometryMeshToVolume {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometrySubdivisionSurface {
    pub boundary_smooth: u8,
}

impl Default for NodeGeometrySubdivisionSurface {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryMeshCircle {

}

impl Default for NodeGeometryMeshCircle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryMeshCylinder {

}

impl Default for NodeGeometryMeshCylinder {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryMeshCone {

}

impl Default for NodeGeometryMeshCone {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryMergeByDistance {

}

impl Default for NodeGeometryMergeByDistance {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryMeshLine {
    pub count_mode: GeometryNodeMeshLineCountMode,
}

impl Default for NodeGeometryMeshLine {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeSwitch {

}

impl Default for NodeSwitch {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeEnumItem {
    pub name: *mut core::ffi::c_void,
    pub description: *mut core::ffi::c_void,
    pub identifier: i32,
    pub _pad: [u8; 4],
}

impl Default for NodeEnumItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeEnumDefinition {
    pub items_num: i32,
    pub active_index: i32,
    pub next_identifier: u32,
    pub _pad: [u8; 4],
}

impl Default for NodeEnumDefinition {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeMenuSwitch {
    pub data_type: eNodeSocketDatatype,
}

impl Default for NodeMenuSwitch {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryCurveSplineType {

}

impl Default for NodeGeometryCurveSplineType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometrySetCurveHandlePositions {

}

impl Default for NodeGeometrySetCurveHandlePositions {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryCurveSetHandles {
    pub mode: GeometryNodeCurveHandleMode,
}

impl Default for NodeGeometryCurveSetHandles {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryCurveSelectHandles {
    pub mode: GeometryNodeCurveHandleMode,
}

impl Default for NodeGeometryCurveSelectHandles {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryCurvePrimitiveArc {

}

impl Default for NodeGeometryCurvePrimitiveArc {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryCurvePrimitiveLine {

}

impl Default for NodeGeometryCurvePrimitiveLine {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryCurvePrimitiveBezierSegment {

}

impl Default for NodeGeometryCurvePrimitiveBezierSegment {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryCurvePrimitiveCircle {

}

impl Default for NodeGeometryCurvePrimitiveCircle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryCurvePrimitiveQuad {

}

impl Default for NodeGeometryCurvePrimitiveQuad {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryCurveResample {
    pub keep_last_segment: u8,
}

impl Default for NodeGeometryCurveResample {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryCurveFillet {

}

impl Default for NodeGeometryCurveFillet {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryCurveTrim {

}

impl Default for NodeGeometryCurveTrim {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryCurveToPoints {

}

impl Default for NodeGeometryCurveToPoints {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryCurveSample {
    pub use_all_curves: i8,
    pub data_type: i8,
    pub _pad: [u8; 1],
}

impl Default for NodeGeometryCurveSample {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryTransferAttribute {
    pub domain: i8,
    pub mode: GeometryNodeAttributeTransferMode,
    pub _pad: [u8; 1],
}

impl Default for NodeGeometryTransferAttribute {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometrySampleIndex {
    pub domain: i8,
    pub clamp: i8,
    pub _pad: [u8; 1],
}

impl Default for NodeGeometrySampleIndex {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeRaycastSampleAttributeItem {
    pub data_type: i8,
    pub _pad: [u8; 3],
}

impl Default for NodeRaycastSampleAttributeItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryRaycast {
    pub data_type: i8,
}

impl Default for NodeGeometryRaycast {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeShaderRaycast {
    pub next_identifier: i32,
    pub sample_attribute_items: *mut core::ffi::c_void,
    pub sample_attribute_items_num: i32,
    pub active_index: i32,
}

impl Default for NodeShaderRaycast {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryCurveFill {
    pub fill_rule: GeometryNodeCurveFillRule,
}

impl Default for NodeGeometryCurveFill {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryMeshToPoints {

}

impl Default for NodeGeometryMeshToPoints {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryAttributeCaptureItem {
    pub data_type: i8,
    pub _pad: [u8; 3],
}

impl Default for NodeGeometryAttributeCaptureItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryAttributeCapture {
    pub domain: i8,
    pub _pad: [u8; 2],
}

impl Default for NodeGeometryAttributeCapture {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryStoreNamedAttribute {
    pub domain: i8,
}

impl Default for NodeGeometryStoreNamedAttribute {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryInputNamedAttribute {

}

impl Default for NodeGeometryInputNamedAttribute {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryStringToCurves {
    pub align_x: GeometryNodeStringToCurvesAlignXMode,
    pub align_y: GeometryNodeStringToCurvesAlignYMode,
    pub pivot_mode: GeometryNodeStringToCurvesPivotMode,
}

impl Default for NodeGeometryStringToCurves {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryDeleteGeometry {
    pub mode: GeometryNodeDeleteGeometryMode,
}

impl Default for NodeGeometryDeleteGeometry {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryDuplicateElements {

}

impl Default for NodeGeometryDuplicateElements {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryMergeLayers {

}

impl Default for NodeGeometryMergeLayers {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometrySeparateGeometry {

}

impl Default for NodeGeometrySeparateGeometry {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryImageTexture {
    pub extension: i8,
}

impl Default for NodeGeometryImageTexture {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryViewerItem {
    pub name: *mut core::ffi::c_void,
    pub socket_type: eNodeSocketDatatype,
}

impl Default for NodeGeometryViewerItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryViewer {
    pub items_num: i32,
    pub active_index: i32,
    pub next_identifier: i32,
    pub data_type_legacy: i8,
    pub domain: i8,
    pub _pad: [u8; 2],
}

impl Default for NodeGeometryViewer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryUVUnwrap {

}

impl Default for NodeGeometryUVUnwrap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeSimulationItem {
    pub name: *mut core::ffi::c_void,
    pub socket_type: eNodeSocketDatatype,
}

impl Default for NodeSimulationItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometrySimulationInput {

}

impl Default for NodeGeometrySimulationInput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometrySimulationOutput {
    pub items_num: i32,
    pub active_index: i32,
    pub next_identifier: i32,
    pub _pad: i32,
}

impl Default for NodeGeometrySimulationOutput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeRepeatItem {
    pub name: *mut core::ffi::c_void,
    pub socket_type: eNodeSocketDatatype,
}

impl Default for NodeRepeatItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryRepeatInput {

}

impl Default for NodeGeometryRepeatInput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryRepeatOutput {
    pub items_num: i32,
    pub active_index: i32,
    pub next_identifier: i32,
    pub inspection_index: i32,
}

impl Default for NodeGeometryRepeatOutput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryForeachGeometryElementInput {

}

impl Default for NodeGeometryForeachGeometryElementInput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeForeachGeometryElementInputItem {
    pub name: *mut core::ffi::c_void,
    pub socket_type: eNodeSocketDatatype,
}

impl Default for NodeForeachGeometryElementInputItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeForeachGeometryElementMainItem {
    pub name: *mut core::ffi::c_void,
    pub socket_type: eNodeSocketDatatype,
}

impl Default for NodeForeachGeometryElementMainItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeForeachGeometryElementGenerationItem {
    pub name: *mut core::ffi::c_void,
    pub socket_type: eNodeSocketDatatype,
}

impl Default for NodeForeachGeometryElementGenerationItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeForeachGeometryElementInputItems {
    pub items_num: i32,
    pub active_index: i32,
    pub next_identifier: i32,
    pub _pad: [u8; 4],
}

impl Default for NodeForeachGeometryElementInputItems {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeForeachGeometryElementMainItems {
    pub items_num: i32,
    pub active_index: i32,
    pub next_identifier: i32,
    pub _pad: [u8; 4],
}

impl Default for NodeForeachGeometryElementMainItems {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeForeachGeometryElementGenerationItems {
    pub items_num: i32,
    pub active_index: i32,
    pub next_identifier: i32,
    pub _pad: [u8; 4],
}

impl Default for NodeForeachGeometryElementGenerationItems {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryForeachGeometryElementOutput {
    pub main_items: NodeForeachGeometryElementMainItems,
    pub generation_items: NodeForeachGeometryElementGenerationItems,
    pub inspection_index: i32,
    pub domain: u8,
    pub _pad: [u8; 3],
}

impl Default for NodeGeometryForeachGeometryElementOutput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeClosureInput {

}

impl Default for NodeClosureInput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeClosureInputItem {
    pub name: *mut core::ffi::c_void,
    pub socket_type: eNodeSocketDatatype,
}

impl Default for NodeClosureInputItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeClosureOutputItem {
    pub name: *mut core::ffi::c_void,
    pub socket_type: eNodeSocketDatatype,
}

impl Default for NodeClosureOutputItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeClosureInputItems {
    pub items_num: i32,
    pub active_index: i32,
    pub next_identifier: i32,
    pub _pad: [u8; 4],
}

impl Default for NodeClosureInputItems {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeClosureOutputItems {
    pub items_num: i32,
    pub active_index: i32,
    pub next_identifier: i32,
    pub _pad: [u8; 4],
}

impl Default for NodeClosureOutputItems {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeClosureOutput {
    pub output_items: NodeClosureOutputItems,
    pub flag: NodeClosureFlag,
}

impl Default for NodeClosureOutput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeEvaluateClosureInputItem {
    pub name: *mut core::ffi::c_void,
    pub socket_type: eNodeSocketDatatype,
}

impl Default for NodeEvaluateClosureInputItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeEvaluateClosureOutputItem {
    pub name: *mut core::ffi::c_void,
    pub socket_type: eNodeSocketDatatype,
}

impl Default for NodeEvaluateClosureOutputItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeEvaluateClosureInputItems {
    pub items_num: i32,
    pub active_index: i32,
    pub next_identifier: i32,
    pub _pad: [u8; 4],
}

impl Default for NodeEvaluateClosureInputItems {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeEvaluateClosureOutputItems {
    pub items_num: i32,
    pub active_index: i32,
    pub next_identifier: i32,
    pub _pad: [u8; 4],
}

impl Default for NodeEvaluateClosureOutputItems {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeEvaluateClosure {
    pub output_items: NodeEvaluateClosureOutputItems,
    pub flag: NodeEvaluateClosureFlag,
}

impl Default for NodeEvaluateClosure {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct IndexSwitchItem {
    pub identifier: i32,
}

impl Default for IndexSwitchItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeIndexSwitch {
    pub items_num: i32,
    pub next_identifier: i32,
    pub data_type: eNodeSocketDatatype,
}

impl Default for NodeIndexSwitch {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CombineListItem {
    pub identifier: i32,
}

impl Default for CombineListItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeCombineList {
    pub items_num: i32,
    pub next_identifier: i32,
    pub data_type: eNodeSocketDatatype,
}

impl Default for NodeCombineList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GeometryNodeFieldToGridItem {
    pub data_type: eNodeSocketDatatype,
}

impl Default for GeometryNodeFieldToGridItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GeometryNodeFieldToGrid {

}

impl Default for GeometryNodeFieldToGrid {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GeometryNodeFieldToListItem {
    pub socket_type: eNodeSocketDatatype,
    pub _pad: [u8; 2],
}

impl Default for GeometryNodeFieldToListItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GeometryNodeFieldToList {
    pub _pad: [u8; 4],
}

impl Default for GeometryNodeFieldToList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GeometryNodeClosureToListItem {
    pub socket_type: eNodeSocketDatatype,
    pub structure_type: NodeSocketInterfaceStructureType,
    pub _pad: [u8; 1],
}

impl Default for GeometryNodeClosureToListItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GeometryNodeClosureToList {
    pub _pad: [u8; 4],
}

impl Default for GeometryNodeClosureToList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryDistributePointsInVolume {

}

impl Default for NodeGeometryDistributePointsInVolume {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryRasterizePointsItem {
    pub name: *mut core::ffi::c_void,
    pub r#type: i16,
    pub _pad1: [u8; 2],
    pub identifier: i32,
}

impl Default for NodeGeometryRasterizePointsItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryRasterizePoints {
    pub items_num: i32,
    pub active_index: i32,
    pub next_identifier: i32,
    pub _pad: [u8; 4],
}

impl Default for NodeGeometryRasterizePoints {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeFunctionCompare {
    pub mode: NodeCompareMode,
    pub data_type: eNodeSocketDatatype,
}

impl Default for NodeFunctionCompare {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeCombSepColor {

}

impl Default for NodeCombSepColor {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeShaderMix {

}

impl Default for NodeShaderMix {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryLinearGizmo {
    pub draw_style: GeometryNodeLinearGizmoDrawStyle,
}

impl Default for NodeGeometryLinearGizmo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryDialGizmo {

}

impl Default for NodeGeometryDialGizmo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryTransformGizmo {

}

impl Default for NodeGeometryTransformGizmo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryBakeItem {
    pub name: *mut core::ffi::c_void,
    pub socket_type: eNodeSocketDatatype,
}

impl Default for NodeGeometryBakeItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryBake {
    pub items_num: i32,
    pub next_identifier: i32,
    pub active_index: i32,
    pub _pad: [u8; 4],
}

impl Default for NodeGeometryBake {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeCombineBundleItem {
    pub name: *mut core::ffi::c_void,
    pub identifier: i32,
    pub socket_type: eNodeSocketDatatype,
}

impl Default for NodeCombineBundleItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeCombineBundle {
    pub items_num: i32,
    pub next_identifier: i32,
    pub active_index: i32,
    pub flag: NodeCombineBundleFlag,
}

impl Default for NodeCombineBundle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeSeparateBundleItem {
    pub name: *mut core::ffi::c_void,
    pub identifier: i32,
    pub socket_type: eNodeSocketDatatype,
}

impl Default for NodeSeparateBundleItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeSeparateBundle {
    pub items_num: i32,
    pub next_identifier: i32,
    pub active_index: i32,
    pub flag: NodeSeparateBundleFlag,
}

impl Default for NodeSeparateBundle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeFunctionFormatStringItem {
    pub name: *mut core::ffi::c_void,
    pub identifier: i32,
    pub socket_type: eNodeSocketDatatype,
}

impl Default for NodeFunctionFormatStringItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeFunctionFormatString {
    pub items_num: i32,
    pub next_identifier: i32,
    pub active_index: i32,
    pub _pad: [u8; 4],
}

impl Default for NodeFunctionFormatString {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGeometryListGetItem {
    pub socket_type: eNodeSocketDatatype,
    pub structure_type: NodeSocketInterfaceStructureType,
    pub _pad: i8,
}

impl Default for NodeGeometryListGetItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeGetBundleItem {
    pub socket_type: eNodeSocketDatatype,
}

impl Default for NodeGetBundleItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeStoreBundleItem {
    pub socket_type: eNodeSocketDatatype,
}

impl Default for NodeStoreBundleItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodeComment {
    pub text: *mut core::ffi::c_void,
    pub textbox_state_node: TextboxState,
    pub textbox_state_panel: TextboxState,
    pub flag: NodeCommentFlag,
}

impl Default for NodeComment {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

