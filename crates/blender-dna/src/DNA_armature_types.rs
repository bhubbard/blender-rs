//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[allow(non_camel_case_types)]
pub type int32_t = i32;
#[allow(non_camel_case_types)]
pub type uint32_t = u32;
#[allow(non_camel_case_types)]
pub type int16_t = i16;
#[allow(non_camel_case_types)]
pub type uint16_t = u16;
#[allow(non_camel_case_types)]
pub type int64_t = i64;
#[allow(non_camel_case_types)]
pub type uint64_t = u64;
#[allow(non_camel_case_types)]
pub type int8_t = i8;
#[allow(non_camel_case_types)]
pub type uint8_t = u8;
#[allow(non_camel_case_types)]
pub type uchar = u8;
#[allow(non_camel_case_types)]
pub type ushort = u16;
#[allow(non_camel_case_types)]
pub type uint = u32;
#[allow(non_camel_case_types)]
pub type ulong = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub struct bUUID { pub data: [u8; 16] }

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eArmature_Flag(pub i32);

impl eArmature_Flag {
    pub const ARM_RESTPOS: Self = Self(((1 << 0)) as i32);
    pub const ARM_FLAG_UNUSED_1: Self = Self(((1 << 1)) as i32);
    pub const ARM_DRAWAXES: Self = Self(((1 << 2)) as i32);
    pub const ARM_DRAWNAMES: Self = Self(((1 << 3)) as i32);
    pub const ARM_DRAW_RELATION_FROM_HEAD: Self = Self(((1 << 5)) as i32);
    pub const ARM_BCOLL_SOLO_ACTIVE: Self = Self(((1 << 6)) as i32);
    pub const ARM_FLAG_UNUSED_7: Self = Self(((1 << 7)) as i32);
    pub const ARM_MIRROR_EDIT: Self = Self(((1 << 8)) as i32);
    pub const ARM_FLAG_UNUSED_9: Self = Self(((1 << 9)) as i32);
    pub const ARM_NO_CUSTOM: Self = Self(((1 << 10)) as i32);
    pub const ARM_COL_CUSTOM: Self = Self(((1 << 11)) as i32);
    pub const ARM_FLAG_UNUSED_12: Self = Self(((1 << 12)) as i32);
    pub const ARM_DS_EXPAND: Self = Self(((1 << 13)) as i32);
    pub const ARM_HAS_VIZ_DEPS: Self = Self(((1 << 14)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eArmature_Drawtype(pub i32);

impl eArmature_Drawtype {
    pub const ARM_DRAW_TYPE_ARMATURE_DEFINED: Self = Self((-1) as i32);
    pub const ARM_DRAW_TYPE_OCTA: Self = Self((0) as i32);
    pub const ARM_DRAW_TYPE_STICK: Self = Self((1) as i32);
    pub const ARM_DRAW_TYPE_B_BONE: Self = Self((2) as i32);
    pub const ARM_DRAW_TYPE_ENVELOPE: Self = Self((3) as i32);
    pub const ARM_DRAW_TYPE_WIRE: Self = Self((4) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eArmature_DeformFlag(pub i16);

impl eArmature_DeformFlag {
    pub const ARM_DEF_VGROUP: Self = Self(((1 << 0)) as i16);
    pub const ARM_DEF_ENVELOPE: Self = Self(((1 << 1)) as i16);
    pub const ARM_DEF_QUATERNION: Self = Self(((1 << 2)) as i16);
    pub const ARM_DEF_B_BONE_REST: Self = Self(((1 << 3)) as i16);
    pub const ARM_DEF_INVERT_VGROUP: Self = Self(((1 << 4)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eArmature_PathFlag(pub i16);

impl eArmature_PathFlag {
    pub const ARM_PATH_FNUMS: Self = Self(((1 << 0)) as i16);
    pub const ARM_PATH_KFRAS: Self = Self(((1 << 1)) as i16);
    pub const ARM_PATH_HEADS: Self = Self(((1 << 2)) as i16);
    pub const ARM_PATH_ACFRA: Self = Self(((1 << 3)) as i16);
    pub const ARM_PATH_KFNOS: Self = Self(((1 << 4)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBone_Flag(pub i32);

impl eBone_Flag {
    pub const BONE_SELECTED: Self = Self(((1 << 0)) as i32);
    pub const BONE_ROOTSEL: Self = Self(((1 << 1)) as i32);
    pub const BONE_TIPSEL: Self = Self(((1 << 2)) as i32);
    pub const BONE_TRANSFORM: Self = Self(((1 << 3)) as i32);
    pub const BONE_CONNECTED: Self = Self(((1 << 4)) as i32);
    pub const BONE_HIDDEN_P: Self = Self(((1 << 6)) as i32);
    pub const BONE_DONE: Self = Self(((1 << 7)) as i32);
    pub const BONE_DRAW_ACTIVE: Self = Self(((1 << 8)) as i32);
    pub const BONE_HINGE: Self = Self(((1 << 9)) as i32);
    pub const BONE_HIDDEN_A: Self = Self(((1 << 10)) as i32);
    pub const BONE_MULT_VG_ENV: Self = Self(((1 << 11)) as i32);
    pub const BONE_NO_DEFORM: Self = Self(((1 << 12)) as i32);
    pub const BONE_UNKEYED: Self = Self(((1 << 13)) as i32);
    pub const BONE_HINGE_CHILD_TRANSFORM: Self = Self(((1 << 14)) as i32);
    pub const BONE_NO_SCALE: Self = Self(((1 << 15)) as i32);
    pub const BONE_DRAWWIRE: Self = Self(((1 << 17)) as i32);
    pub const BONE_NO_CYCLICOFFSET: Self = Self(((1 << 18)) as i32);
    pub const BONE_EDITMODE_LOCKED: Self = Self(((1 << 19)) as i32);
    pub const BONE_TRANSFORM_CHILD: Self = Self(((1 << 20)) as i32);
    pub const BONE_UNSELECTABLE: Self = Self(((1 << 21)) as i32);
    pub const BONE_NO_LOCAL_LOCATION: Self = Self(((1 << 22)) as i32);
    pub const BONE_RELATIVE_PARENTING: Self = Self(((1 << 23)) as i32);
    pub const BONE_ADD_PARENT_END_ROLL: Self = Self(((1 << 24)) as i32);
    pub const BONE_TRANSFORM_MIRROR: Self = Self(((1 << 25)) as i32);
    pub const BONE_DRAW_LOCKED_WEIGHT: Self = Self(((1 << 26)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBone_InheritScaleMode(pub i8);

impl eBone_InheritScaleMode {
    pub const BONE_INHERIT_SCALE_FULL: Self = Self((0) as i8);
    pub const BONE_INHERIT_SCALE_FIX_SHEAR: Self = Self((1) as i8);
    pub const BONE_INHERIT_SCALE_AVERAGE: Self = Self((2) as i8);
    pub const BONE_INHERIT_SCALE_NONE: Self = Self((3) as i8);
    pub const BONE_INHERIT_SCALE_NONE_LEGACY: Self = Self((4) as i8);
    pub const BONE_INHERIT_SCALE_ALIGNED: Self = Self((5) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBone_BBoneHandleType(pub i8);

impl eBone_BBoneHandleType {
    pub const BBONE_HANDLE_AUTO: Self = Self((0) as i8);
    pub const BBONE_HANDLE_ABSOLUTE: Self = Self((1) as i8);
    pub const BBONE_HANDLE_RELATIVE: Self = Self((2) as i8);
    pub const BBONE_HANDLE_TANGENT: Self = Self((3) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBone_BBoneMappingMode(pub i8);

impl eBone_BBoneMappingMode {
    pub const BBONE_MAPPING_STRAIGHT: Self = Self((0) as i8);
    pub const BBONE_MAPPING_CURVED: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBone_BBoneFlag(pub i32);

impl eBone_BBoneFlag {
    pub const BBONE_ADD_PARENT_END_ROLL: Self = Self(((1 << 0)) as i32);
    pub const BBONE_SCALE_EASING: Self = Self(((1 << 1)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBone_BBoneHandleFlag(pub i16);

impl eBone_BBoneHandleFlag {
    pub const BBONE_HANDLE_SCALE_X: Self = Self(((1 << 0)) as i16);
    pub const BBONE_HANDLE_SCALE_Y: Self = Self(((1 << 1)) as i16);
    pub const BBONE_HANDLE_SCALE_Z: Self = Self(((1 << 2)) as i16);
    pub const BBONE_HANDLE_SCALE_EASE: Self = Self(((1 << 3)) as i16);
    pub const BBONE_HANDLE_SCALE_ANY: Self = Self(4 as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBoneCollection_Flag(pub u8);

impl eBoneCollection_Flag {
    pub const BONE_COLLECTION_VISIBLE: Self = Self(((1 << 0)) as u8);
    pub const BONE_COLLECTION_SELECTABLE: Self = Self(((1 << 1)) as u8);
    pub const BONE_COLLECTION_OVERRIDE_LIBRARY_LOCAL: Self = Self(((1 << 2)) as u8);
    pub const BONE_COLLECTION_ANCESTORS_VISIBLE: Self = Self(((1 << 3)) as u8);
    pub const BONE_COLLECTION_SOLO: Self = Self(((1 << 4)) as u8);
    pub const BONE_COLLECTION_EXPANDED: Self = Self(((1 << 5)) as u8);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BoneColor {
    pub palette_index: i8,
    pub _pad0: [u8; 7],
}

impl Default for BoneColor {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Bone_Runtime {
    pub collections: ListBaseT<BoneCollectionReference>,
    pub nullptr: ListBaseT<BoneCollectionReference>,
}

impl Default for Bone_Runtime {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Bone {
    pub prop: *mut core::ffi::c_void,
    pub system_properties: *mut core::ffi::c_void,
    pub _pad0: *mut core::ffi::c_void,
    pub parent: *mut core::ffi::c_void,
    pub childbase: ListBaseT<Bone>,
    pub nullptr: ListBaseT<Bone>,
}

impl Default for Bone {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bArmature {
    pub adt: *mut core::ffi::c_void,
    pub bonebase: ListBaseT<Bone>,
    pub nullptr: ListBaseT<Bone>,
}

impl Default for bArmature {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BoneCollection {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub name: [u8; 64],
    pub bones: ListBaseT<BoneCollectionMember>,
    pub nullptr: ListBaseT<BoneCollectionMember>,
}

impl Default for BoneCollection {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BoneCollectionMember {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub bone: *mut core::ffi::c_void,
}

impl Default for BoneCollectionMember {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BoneCollectionReference {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub bcoll: *mut core::ffi::c_void,
}

impl Default for BoneCollectionReference {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

