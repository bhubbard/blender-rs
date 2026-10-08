//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i8)]
pub enum GreasePencilStrokeCapType {
    #[default]
    GP_STROKE_CAP_TYPE_ROUND = 0,
    GP_STROKE_CAP_TYPE_FLAT = 1,
    GP_STROKE_CAP_TYPE_MAX,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i8)]
pub enum GreasePencilDrawingType {
    #[default]
    GP_DRAWING = 0,
    GP_DRAWING_REFERENCE = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u32)]
pub enum GreasePencilDrawingBaseFlag {
    #[default]
    GreasePencilDrawingBaseFlag_TODO,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u32)]
pub enum GreasePencilFrameFlag {
    #[default]
    GP_FRAME_SELECTED = (1 << 0),
    GP_FRAME_IMPLICIT_HOLD = (1 << 1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum GreasePencilLayerFramesMapStorageFlag {
    #[default]
    GP_LAYER_FRAMES_STORAGE_DIRTY = (1 << 0),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u16)]
pub enum GreasePencilLayerMaskFlag {
    #[default]
    GP_LAYER_MASK_HIDE = (1 << 0),
    GP_LAYER_MASK_INVERT = (1 << 1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i8)]
pub enum GreasePencilLayerBlendMode {
    #[default]
    GP_LAYER_BLEND_NONE = 0,
    GP_LAYER_BLEND_HARDLIGHT = 1,
    GP_LAYER_BLEND_ADD = 2,
    GP_LAYER_BLEND_SUBTRACT = 3,
    GP_LAYER_BLEND_MULTIPLY = 4,
    GP_LAYER_BLEND_DIVIDE = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i8)]
pub enum GreasePencilLayerTreeNodeType {
    #[default]
    GP_LAYER_TREE_LEAF = 0,
    GP_LAYER_TREE_GROUP = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u32)]
pub enum GreasePencilLayerTreeNodeFlag {
    #[default]
    GP_LAYER_TREE_NODE_HIDE = (1 << 0),
    GP_LAYER_TREE_NODE_LOCKED = (1 << 1),
    GP_LAYER_TREE_NODE_SELECT = (1 << 2),
    GP_LAYER_TREE_NODE_MUTE = (1 << 3),
    GP_LAYER_TREE_NODE_USE_LIGHTS = (1 << 4),
    GP_LAYER_TREE_NODE_HIDE_ONION_SKINNING = (1 << 5),
    GP_LAYER_TREE_NODE_EXPANDED = (1 << 6),
    GP_LAYER_TREE_NODE_HIDE_MASKS = (1 << 7),
    GP_LAYER_TREE_NODE_DISABLE_MASKS_IN_VIEWLAYER = (1 << 8),
    GP_LAYER_TREE_NODE_IGNORE_LOCKED_MATERIALS = (1 << 9),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i8)]
pub enum GroupColorTag {
    #[default]
    LAYERGROUP_COLOR_NONE = -1,
    LAYERGROUP_COLOR_01,
    LAYERGROUP_COLOR_02,
    LAYERGROUP_COLOR_03,
    LAYERGROUP_COLOR_04,
    LAYERGROUP_COLOR_05,
    LAYERGROUP_COLOR_06,
    LAYERGROUP_COLOR_07,
    LAYERGROUP_COLOR_08,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u32)]
pub enum GreasePencilFlag {
    #[default]
    GREASE_PENCIL_ANIM_CHANNEL_EXPANDED = (1 << 0),
    GREASE_PENCIL_AUTOLOCK_LAYERS = (1 << 1),
    GREASE_PENCIL_STROKE_ORDER_3D = (1 << 2),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i8)]
pub enum GreasePencilOnionSkinningMode {
    #[default]
    GP_ONION_SKINNING_MODE_ABSOLUTE = 0,
    GP_ONION_SKINNING_MODE_RELATIVE = 1,
    GP_ONION_SKINNING_MODE_SELECTED = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum GreasePencilOnionSkinningFlag {
    #[default]
    GP_ONION_SKINNING_USE_CUSTOM_COLORS = (1 << 0),
    GP_ONION_SKINNING_USE_FADE = (1 << 1),
    GP_ONION_SKINNING_SHOW_LOOP = (1 << 2),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum GreasePencilOnionSkinningFilter {
    #[default]
    GP_ONION_SKINNING_FILTER_KEYTYPE_KEYFRAME = (1 << 0),
    GP_ONION_SKINNING_FILTER_KEYTYPE_EXTREME = (1 << 1),
    GP_ONION_SKINNING_FILTER_KEYTYPE_BREAKDOWN = (1 << 2),
    GP_ONION_SKINNING_FILTER_KEYTYPE_JITTER = (1 << 3),
    GP_ONION_SKINNING_FILTER_KEYTYPE_MOVEHOLD = (1 << 4),
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct GreasePencilDrawingBase {
    pub r#type: GreasePencilDrawingType,
    pub _pad: [u8; 3],
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct GreasePencilDrawing {
    pub base: GreasePencilDrawingBase,
    pub geometry: CurvesGeometry,
    pub runtime: *mut core::ffi::c_void,
    pub r#const: bke::greasepencil::Drawing &wrap(),
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct GreasePencilDrawingReference {
    pub base: GreasePencilDrawingBase,
    pub id_reference: *mut core::ffi::c_void,
    pub r#const: bke::greasepencil::DrawingReference &wrap(),
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct GreasePencilFrame {
    pub drawing_index: i32,
    pub flag: GreasePencilFrameFlag,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct GreasePencilLayerFramesMapStorage {
    pub keys: *mut core::ffi::c_void,
    pub values: *mut core::ffi::c_void,
    pub num: i32,
    pub flag: GreasePencilLayerFramesMapStorageFlag,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct GreasePencilLayerMask {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub layer_name: *mut core::ffi::c_void,
    pub flag: GreasePencilLayerMaskFlag,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct GreasePencilLayerTreeNode {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub parent: *mut core::ffi::c_void,
    pub name: *mut core::ffi::c_void,
    pub r#type: GreasePencilLayerTreeNodeType,
    pub _pad: [u8; 7],
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct GreasePencilLayer {
    pub base: GreasePencilLayerTreeNode,
    pub frames_storage: GreasePencilLayerFramesMapStorage,
    pub blend_mode: GreasePencilLayerBlendMode,
    pub _pad: [u8; 3],
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct GreasePencilLayerTreeGroup {
    pub base: GreasePencilLayerTreeNode,
    pub children: ListBaseT<GreasePencilLayerTreeNode>,
    pub nullptr: ListBaseT<GreasePencilLayerTreeNode>,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct GreasePencilOnionSkinningSettings {
    pub opacity: f32,
    pub mode: GreasePencilOnionSkinningMode,
    pub flag: GreasePencilOnionSkinningFlag,
    pub filter: GreasePencilOnionSkinningFilter,
    pub _pad: [u8; 1],
}

#[derive(Debug, Clone, PartialEq, Default)]
#[repr(C)]
pub struct GreasePencil {
    pub adt: *mut core::ffi::c_void,
    pub drawing_array: *mut core::ffi::c_void,
    pub drawing_array_num: i32,
    pub _pad: [u8; 4],
}

