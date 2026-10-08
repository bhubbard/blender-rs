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
pub struct GreasePencilStrokeCapType(pub i8);

impl GreasePencilStrokeCapType {
    pub const GP_STROKE_CAP_TYPE_ROUND: Self = Self((0) as i8);
    pub const GP_STROKE_CAP_TYPE_FLAT: Self = Self((1) as i8);
    pub const GP_STROKE_CAP_TYPE_MAX: Self = Self(2 as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilDrawingType(pub i8);

impl GreasePencilDrawingType {
    pub const GP_DRAWING: Self = Self((0) as i8);
    pub const GP_DRAWING_REFERENCE: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilDrawingBaseFlag(pub u32);

impl GreasePencilDrawingBaseFlag {
    pub const GreasePencilDrawingBaseFlag_TODO: Self = Self(0 as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilFrameFlag(pub u32);

impl GreasePencilFrameFlag {
    pub const GP_FRAME_SELECTED: Self = Self(((1 << 0)) as u32);
    pub const GP_FRAME_IMPLICIT_HOLD: Self = Self(((1 << 1)) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilLayerFramesMapStorageFlag(pub i32);

impl GreasePencilLayerFramesMapStorageFlag {
    pub const GP_LAYER_FRAMES_STORAGE_DIRTY: Self = Self(((1 << 0)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilLayerMaskFlag(pub u16);

impl GreasePencilLayerMaskFlag {
    pub const GP_LAYER_MASK_HIDE: Self = Self(((1 << 0)) as u16);
    pub const GP_LAYER_MASK_INVERT: Self = Self(((1 << 1)) as u16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilLayerBlendMode(pub i8);

impl GreasePencilLayerBlendMode {
    pub const GP_LAYER_BLEND_NONE: Self = Self((0) as i8);
    pub const GP_LAYER_BLEND_HARDLIGHT: Self = Self((1) as i8);
    pub const GP_LAYER_BLEND_ADD: Self = Self((2) as i8);
    pub const GP_LAYER_BLEND_SUBTRACT: Self = Self((3) as i8);
    pub const GP_LAYER_BLEND_MULTIPLY: Self = Self((4) as i8);
    pub const GP_LAYER_BLEND_DIVIDE: Self = Self((5) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilLayerTreeNodeType(pub i8);

impl GreasePencilLayerTreeNodeType {
    pub const GP_LAYER_TREE_LEAF: Self = Self((0) as i8);
    pub const GP_LAYER_TREE_GROUP: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilLayerTreeNodeFlag(pub u32);

impl GreasePencilLayerTreeNodeFlag {
    pub const GP_LAYER_TREE_NODE_HIDE: Self = Self(((1 << 0)) as u32);
    pub const GP_LAYER_TREE_NODE_LOCKED: Self = Self(((1 << 1)) as u32);
    pub const GP_LAYER_TREE_NODE_SELECT: Self = Self(((1 << 2)) as u32);
    pub const GP_LAYER_TREE_NODE_MUTE: Self = Self(((1 << 3)) as u32);
    pub const GP_LAYER_TREE_NODE_USE_LIGHTS: Self = Self(((1 << 4)) as u32);
    pub const GP_LAYER_TREE_NODE_HIDE_ONION_SKINNING: Self = Self(((1 << 5)) as u32);
    pub const GP_LAYER_TREE_NODE_EXPANDED: Self = Self(((1 << 6)) as u32);
    pub const GP_LAYER_TREE_NODE_HIDE_MASKS: Self = Self(((1 << 7)) as u32);
    pub const GP_LAYER_TREE_NODE_DISABLE_MASKS_IN_VIEWLAYER: Self = Self(((1 << 8)) as u32);
    pub const GP_LAYER_TREE_NODE_IGNORE_LOCKED_MATERIALS: Self = Self(((1 << 9)) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GroupColorTag(pub i8);

impl GroupColorTag {
    pub const LAYERGROUP_COLOR_NONE: Self = Self((-1) as i8);
    pub const LAYERGROUP_COLOR_01: Self = Self(1 as i8);
    pub const LAYERGROUP_COLOR_02: Self = Self(2 as i8);
    pub const LAYERGROUP_COLOR_03: Self = Self(3 as i8);
    pub const LAYERGROUP_COLOR_04: Self = Self(4 as i8);
    pub const LAYERGROUP_COLOR_05: Self = Self(5 as i8);
    pub const LAYERGROUP_COLOR_06: Self = Self(6 as i8);
    pub const LAYERGROUP_COLOR_07: Self = Self(7 as i8);
    pub const LAYERGROUP_COLOR_08: Self = Self(8 as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilFlag(pub u32);

impl GreasePencilFlag {
    pub const GREASE_PENCIL_ANIM_CHANNEL_EXPANDED: Self = Self(((1 << 0)) as u32);
    pub const GREASE_PENCIL_AUTOLOCK_LAYERS: Self = Self(((1 << 1)) as u32);
    pub const GREASE_PENCIL_STROKE_ORDER_3D: Self = Self(((1 << 2)) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilOnionSkinningMode(pub i8);

impl GreasePencilOnionSkinningMode {
    pub const GP_ONION_SKINNING_MODE_ABSOLUTE: Self = Self((0) as i8);
    pub const GP_ONION_SKINNING_MODE_RELATIVE: Self = Self((1) as i8);
    pub const GP_ONION_SKINNING_MODE_SELECTED: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilOnionSkinningFlag(pub u8);

impl GreasePencilOnionSkinningFlag {
    pub const GP_ONION_SKINNING_USE_CUSTOM_COLORS: Self = Self(((1 << 0)) as u8);
    pub const GP_ONION_SKINNING_USE_FADE: Self = Self(((1 << 1)) as u8);
    pub const GP_ONION_SKINNING_SHOW_LOOP: Self = Self(((1 << 2)) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilOnionSkinningFilter(pub u8);

impl GreasePencilOnionSkinningFilter {
    pub const GP_ONION_SKINNING_FILTER_KEYTYPE_KEYFRAME: Self = Self(((1 << 0)) as u8);
    pub const GP_ONION_SKINNING_FILTER_KEYTYPE_EXTREME: Self = Self(((1 << 1)) as u8);
    pub const GP_ONION_SKINNING_FILTER_KEYTYPE_BREAKDOWN: Self = Self(((1 << 2)) as u8);
    pub const GP_ONION_SKINNING_FILTER_KEYTYPE_JITTER: Self = Self(((1 << 3)) as u8);
    pub const GP_ONION_SKINNING_FILTER_KEYTYPE_MOVEHOLD: Self = Self(((1 << 4)) as u8);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilDrawingBase {
    pub r#type: GreasePencilDrawingType,
    pub _pad: [u8; 3],
}

impl Default for GreasePencilDrawingBase {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilDrawing {
    pub base: GreasePencilDrawingBase,
    pub geometry: CurvesGeometry,
    pub runtime: *mut core::ffi::c_void,
}

impl Default for GreasePencilDrawing {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilDrawingReference {
    pub base: GreasePencilDrawingBase,
    pub id_reference: *mut core::ffi::c_void,
}

impl Default for GreasePencilDrawingReference {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilFrame {
    pub drawing_index: i32,
    pub flag: GreasePencilFrameFlag,
}

impl Default for GreasePencilFrame {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilLayerFramesMapStorage {
    pub keys: *mut core::ffi::c_void,
    pub values: *mut core::ffi::c_void,
    pub num: i32,
    pub flag: GreasePencilLayerFramesMapStorageFlag,
}

impl Default for GreasePencilLayerFramesMapStorage {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilLayerMask {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub layer_name: *mut core::ffi::c_void,
    pub flag: GreasePencilLayerMaskFlag,
}

impl Default for GreasePencilLayerMask {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilLayerTreeNode {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub parent: *mut core::ffi::c_void,
    pub name: *mut core::ffi::c_void,
    pub r#type: GreasePencilLayerTreeNodeType,
    pub _pad: [u8; 7],
}

impl Default for GreasePencilLayerTreeNode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilLayer {
    pub base: GreasePencilLayerTreeNode,
    pub frames_storage: GreasePencilLayerFramesMapStorage,
    pub blend_mode: GreasePencilLayerBlendMode,
    pub _pad: [u8; 3],
}

impl Default for GreasePencilLayer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilLayerTreeGroup {
    pub base: GreasePencilLayerTreeNode,
    pub children: ListBaseT<GreasePencilLayerTreeNode>,
    pub nullptr: ListBaseT<GreasePencilLayerTreeNode>,
}

impl Default for GreasePencilLayerTreeGroup {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilOnionSkinningSettings {
    pub opacity: f32,
    pub mode: GreasePencilOnionSkinningMode,
    pub flag: GreasePencilOnionSkinningFlag,
    pub filter: GreasePencilOnionSkinningFilter,
    pub _pad: [u8; 1],
}

impl Default for GreasePencilOnionSkinningSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencil {
    pub adt: *mut core::ffi::c_void,
    pub drawing_array: *mut core::ffi::c_void,
    pub drawing_array_num: i32,
    pub _pad: [u8; 4],
}

impl Default for GreasePencil {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

