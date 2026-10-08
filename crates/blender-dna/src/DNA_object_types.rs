//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ebDeformGroup_Flag(pub i8);

impl ebDeformGroup_Flag {
    pub const DG_LOCK_WEIGHT: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eObject_Flag(pub i16);

impl eObject_Flag {
    pub const OB_SELECT: Self = Self(((1 << 0)) as i16);
    pub const BA_WAS_SEL: Self = Self(((1 << 1)) as i16);
    pub const BA_SNAP_FIX_DEPS_FIASCO: Self = Self(((1 << 2)) as i16);
    pub const BA_TEMP_TAG: Self = Self((1 << 5) as i16);
    pub const BA_TRANSFORM_LOCKED_IN_PLACE: Self = Self((1 << 7) as i16);
    pub const BA_TRANSFORM_CHILD: Self = Self((1 << 8) as i16);
    pub const BA_TRANSFORM_PARENT: Self = Self((1 << 13) as i16);
    pub const OB_FROMDUPLI: Self = Self((1 << 9) as i16);
    pub const OB_DONE: Self = Self((1 << 10) as i16);
    pub const OB_FLAG_USE_SIMULATION_CACHE: Self = Self((1 << 11) as i16);
    pub const OB_FLAG_ACTIVE_CLIPBOARD: Self = Self((1 << 12) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ObjectType(pub i16);

impl ObjectType {
    pub const OB_EMPTY: Self = Self((0) as i16);
    pub const OB_MESH: Self = Self((1) as i16);
    pub const OB_CURVES_LEGACY: Self = Self((2) as i16);
    pub const OB_SURF: Self = Self((3) as i16);
    pub const OB_FONT: Self = Self((4) as i16);
    pub const OB_MBALL: Self = Self((5) as i16);
    pub const OB_LAMP: Self = Self((10) as i16);
    pub const OB_CAMERA: Self = Self((11) as i16);
    pub const OB_SPEAKER: Self = Self((12) as i16);
    pub const OB_LIGHTPROBE: Self = Self((13) as i16);
    pub const OB_LATTICE: Self = Self((22) as i16);
    pub const OB_ARMATURE: Self = Self((25) as i16);
    pub const OB_GPENCIL_LEGACY: Self = Self((26) as i16);
    pub const OB_CURVES: Self = Self((27) as i16);
    pub const OB_POINTCLOUD: Self = Self((28) as i16);
    pub const OB_VOLUME: Self = Self((29) as i16);
    pub const OB_GREASE_PENCIL: Self = Self((30) as i16);
    pub const OB_TYPE_MAX: Self = Self(17 as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eObject_Partype(pub i16);

impl eObject_Partype {
    pub const PAROBJECT: Self = Self((0) as i16);
    pub const PARSKEL: Self = Self((4) as i16);
    pub const PARVERT1: Self = Self((5) as i16);
    pub const PARVERT3: Self = Self((6) as i16);
    pub const PARBONE: Self = Self((7) as i16);
    pub const PARTYPE: Self = Self(((1 << 4) - 1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eObject_TransFlag(pub i16);

impl eObject_TransFlag {
    pub const OB_TRANSFORM_ADJUST_ROOT_PARENT_FOR_VIEW_LOCK: Self = Self((1 << 0) as i16);
    pub const OB_TRANSFLAG_UNUSED_1: Self = Self((1 << 1) as i16);
    pub const OB_NEG_SCALE: Self = Self((1 << 2) as i16);
    pub const OB_TRANSFLAG_UNUSED_3: Self = Self((1 << 3) as i16);
    pub const OB_DUPLIVERTS: Self = Self((1 << 4) as i16);
    pub const OB_DUPLIROT: Self = Self((1 << 5) as i16);
    pub const OB_TRANSFLAG_UNUSED_6: Self = Self((1 << 6) as i16);
    pub const OB_TRANSFLAG_UNUSED_7: Self = Self((1 << 7) as i16);
    pub const OB_DUPLICOLLECTION: Self = Self((1 << 8) as i16);
    pub const OB_DUPLIFACES: Self = Self((1 << 9) as i16);
    pub const OB_DUPLIFACES_SCALE: Self = Self((1 << 10) as i16);
    pub const OB_DUPLIPARTS: Self = Self((1 << 11) as i16);
    pub const OB_TRANSFLAG_UNUSED_12: Self = Self((1 << 12) as i16);
    pub const OB_NO_CONSTRAINTS: Self = Self((1 << 13) as i16);
    pub const OB_PARENT_USE_FINAL_INDICES: Self = Self((1 << 14) as i16);
    pub const OB_DUPLI: Self = Self(15 as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eObject_Axis(pub i16);

impl eObject_Axis {
    pub const OB_POSX: Self = Self((0) as i16);
    pub const OB_POSY: Self = Self((1) as i16);
    pub const OB_POSZ: Self = Self((2) as i16);
    pub const OB_NEGX: Self = Self((3) as i16);
    pub const OB_NEGY: Self = Self((4) as i16);
    pub const OB_NEGZ: Self = Self((5) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eObject_DrawExtraFlag(pub i16);

impl eObject_DrawExtraFlag {
    pub const OB_DRAWBOUNDOX: Self = Self((1 << 0) as i16);
    pub const OB_AXIS: Self = Self((1 << 1) as i16);
    pub const OB_TEXSPACE: Self = Self((1 << 2) as i16);
    pub const OB_DRAWNAME: Self = Self((1 << 3) as i16);
    pub const OB_DRAWWIRE: Self = Self((1 << 5) as i16);
    pub const OB_DRAW_IN_FRONT: Self = Self((1 << 6) as i16);
    pub const OB_DRAWTRANSP: Self = Self((1 << 7) as i16);
    pub const OB_DRAW_ALL_EDGES: Self = Self((1 << 8) as i16);
    pub const OB_DRAW_NO_SHADOW_CAST: Self = Self((1 << 9) as i16);
    pub const OB_USE_GPENCIL_LIGHTS: Self = Self((1 << 10) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eObject_EmptyDrawType(pub i8);

impl eObject_EmptyDrawType {
    pub const OB_ARROWS: Self = Self((1) as i8);
    pub const OB_PLAINAXES: Self = Self((2) as i8);
    pub const OB_CIRCLE: Self = Self((3) as i8);
    pub const OB_SINGLE_ARROW: Self = Self((4) as i8);
    pub const OB_CUBE: Self = Self((5) as i8);
    pub const OB_EMPTY_SPHERE: Self = Self((6) as i8);
    pub const OB_EMPTY_CONE: Self = Self((7) as i8);
    pub const OB_EMPTY_IMAGE: Self = Self((8) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eGpencil_AddType(pub i32);

impl eGpencil_AddType {
    pub const GP_EMPTY: Self = Self((0) as i32);
    pub const GP_STROKE: Self = Self((1) as i32);
    pub const GP_MONKEY: Self = Self((2) as i32);
    pub const GREASE_PENCIL_LINEART_SCENE: Self = Self((3) as i32);
    pub const GREASE_PENCIL_LINEART_OBJECT: Self = Self((4) as i32);
    pub const GREASE_PENCIL_LINEART_COLLECTION: Self = Self((5) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eObject_BoundType(pub i8);

impl eObject_BoundType {
    pub const OB_BOUND_BOX: Self = Self((0) as i8);
    pub const OB_BOUND_SPHERE: Self = Self((1) as i8);
    pub const OB_BOUND_CYLINDER: Self = Self((2) as i8);
    pub const OB_BOUND_CONE: Self = Self((3) as i8);
    pub const OB_BOUND_CAPSULE: Self = Self((7) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eObject_VisibilityFlag(pub i32);

impl eObject_VisibilityFlag {
    pub const OB_HIDE_VIEWPORT: Self = Self((1 << 0) as i32);
    pub const OB_HIDE_SELECT: Self = Self((1 << 1) as i32);
    pub const OB_HIDE_RENDER: Self = Self((1 << 2) as i32);
    pub const OB_HIDE_CAMERA: Self = Self((1 << 3) as i32);
    pub const OB_HIDE_DIFFUSE: Self = Self((1 << 4) as i32);
    pub const OB_HIDE_GLOSSY: Self = Self((1 << 5) as i32);
    pub const OB_HIDE_TRANSMISSION: Self = Self((1 << 6) as i32);
    pub const OB_HIDE_VOLUME_SCATTER: Self = Self((1 << 7) as i32);
    pub const OB_HIDE_SHADOW: Self = Self((1 << 8) as i32);
    pub const OB_HOLDOUT: Self = Self((1 << 9) as i32);
    pub const OB_SHADOW_CATCHER: Self = Self((1 << 10) as i32);
    pub const OB_HIDE_PROBE_VOLUME: Self = Self((1 << 11) as i32);
    pub const OB_HIDE_PROBE_CUBEMAP: Self = Self((1 << 12) as i32);
    pub const OB_HIDE_PROBE_PLANAR: Self = Self((1 << 13) as i32);
    pub const OB_HIDE_SURFACE_PICK: Self = Self((1 << 14) as i32);
    pub const OB_HIDE_RAYCAST: Self = Self((1 << 15) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eObject_ShapeFlag(pub i8);

impl eObject_ShapeFlag {
    pub const OB_SHAPE_LOCK: Self = Self((1 << 0) as i8);
    pub const OB_SHAPE_FLAG_UNUSED_1: Self = Self((1 << 1) as i8);
    pub const OB_SHAPE_EDIT_MODE: Self = Self((1 << 2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eObject_NlaFlag(pub i16);

impl eObject_NlaFlag {
    pub const OB_ADS_UNUSED_1: Self = Self((1 << 0) as i16);
    pub const OB_ADS_UNUSED_2: Self = Self((1 << 1) as i16);
    pub const OB_ADS_COLLAPSED: Self = Self((1 << 10) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eObject_ProtectFlag(pub i16);

impl eObject_ProtectFlag {
    pub const OB_LOCK_LOCX: Self = Self((1 << 0) as i16);
    pub const OB_LOCK_LOCY: Self = Self((1 << 1) as i16);
    pub const OB_LOCK_LOCZ: Self = Self((1 << 2) as i16);
    pub const OB_LOCK_LOC: Self = Self(3 as i16);
    pub const OB_LOCK_ROTX: Self = Self((1 << 3) as i16);
    pub const OB_LOCK_ROTY: Self = Self((1 << 4) as i16);
    pub const OB_LOCK_ROTZ: Self = Self((1 << 5) as i16);
    pub const OB_LOCK_ROT: Self = Self(7 as i16);
    pub const OB_LOCK_SCALEX: Self = Self((1 << 6) as i16);
    pub const OB_LOCK_SCALEY: Self = Self((1 << 7) as i16);
    pub const OB_LOCK_SCALEZ: Self = Self((1 << 8) as i16);
    pub const OB_LOCK_SCALE: Self = Self(11 as i16);
    pub const OB_LOCK_ROTW: Self = Self((1 << 9) as i16);
    pub const OB_LOCK_ROT4D: Self = Self((1 << 10) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eObject_DuplicatorVisibilityFlag(pub i8);

impl eObject_DuplicatorVisibilityFlag {
    pub const OB_DUPLI_FLAG_VIEWPORT: Self = Self((1 << 0) as i8);
    pub const OB_DUPLI_FLAG_RENDER: Self = Self((1 << 1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eObject_EmptyImageDepth(pub i8);

impl eObject_EmptyImageDepth {
    pub const OB_EMPTY_IMAGE_DEPTH_DEFAULT: Self = Self((0) as i8);
    pub const OB_EMPTY_IMAGE_DEPTH_FRONT: Self = Self((1) as i8);
    pub const OB_EMPTY_IMAGE_DEPTH_BACK: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eObject_EmptyImageVisibilityFlag(pub i8);

impl eObject_EmptyImageVisibilityFlag {
    pub const OB_EMPTY_IMAGE_HIDE_PERSPECTIVE: Self = Self((1 << 0) as i8);
    pub const OB_EMPTY_IMAGE_HIDE_ORTHOGRAPHIC: Self = Self((1 << 1) as i8);
    pub const OB_EMPTY_IMAGE_HIDE_BACK: Self = Self((1 << 2) as i8);
    pub const OB_EMPTY_IMAGE_HIDE_FRONT: Self = Self((1 << 3) as i8);
    pub const OB_EMPTY_IMAGE_HIDE_NON_AXIS_ALIGNED: Self = Self((1 << 4) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eObject_EmptyImageFlag(pub i8);

impl eObject_EmptyImageFlag {
    pub const OB_EMPTY_IMAGE_USE_ALPHA_BLEND: Self = Self((1 << 0) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ObjectModifierFlag(pub u8);

impl ObjectModifierFlag {
    pub const OB_MODIFIER_FLAG_ADD_REST_POSITION: Self = Self((1 << 0) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eObjectLineArt_Usage(pub i16);

impl eObjectLineArt_Usage {
    pub const OBJECT_LRT_INHERIT: Self = Self((0) as i16);
    pub const OBJECT_LRT_INCLUDE: Self = Self(((1 << 0)) as i16);
    pub const OBJECT_LRT_OCCLUSION_ONLY: Self = Self(((1 << 1)) as i16);
    pub const OBJECT_LRT_EXCLUDE: Self = Self(((1 << 2)) as i16);
    pub const OBJECT_LRT_INTERSECTION_ONLY: Self = Self(((1 << 3)) as i16);
    pub const OBJECT_LRT_NO_INTERSECTION: Self = Self(((1 << 4)) as i16);
    pub const OBJECT_LRT_FORCE_INTERSECTION: Self = Self(((1 << 5)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eObjectLineArt_Flags(pub i16);

impl eObjectLineArt_Flags {
    pub const OBJECT_LRT_OWN_CREASE: Self = Self(((1 << 0)) as i16);
    pub const OBJECT_LRT_OWN_INTERSECTION_PRIORITY: Self = Self(((1 << 1)) as i16);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bDeformGroup {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub name: [u8; 64],
    pub flag: ebDeformGroup_Flag,
}

impl Default for bDeformGroup {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bFaceMap {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub name: [u8; 64],
    pub flag: i8,
    pub _pad0: [u8; 7],
}

impl Default for bFaceMap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BoundBox {
    pub vec: [[f32; 3]; 8],
}

impl Default for BoundBox {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ObjectLineArt {
    pub usage: eObjectLineArt_Usage,
    pub flags: eObjectLineArt_Flags,
}

impl Default for ObjectLineArt {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct LightLinkingRuntime {
    pub light_set_membership: usize,
    pub shadow_set_membership: usize,
    pub receiver_light_set: u8,
    pub blocker_shadow_set: u8,
    pub _pad: [u8; 6],
}

impl Default for LightLinkingRuntime {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct LightLinking {
    pub receiver_collection: *mut core::ffi::c_void,
    pub blocker_collection: *mut core::ffi::c_void,
    pub runtime: LightLinkingRuntime,
}

impl Default for LightLinking {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Object {
    pub adt: *mut core::ffi::c_void,
    pub r#type: ObjectType,
    pub partype: eObject_Partype,
    pub par1: i32,
    pub par2: i32,
    pub par3: i32,
    pub parsubstr: [u8; 64],
    pub parent: *mut core::ffi::c_void,
    pub track: *mut core::ffi::c_void,
    pub parent_bone_head_tail_factor: f32,
    pub _pad4: [u8; 4],
}

impl Default for Object {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ObHook {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub parent: *mut core::ffi::c_void,
    pub parentinv: [[f32; 4]; 4],
}

impl Default for ObHook {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

