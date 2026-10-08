//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eDupli_ID_Flags(pub i32);

impl eDupli_ID_Flags {
    pub const USER_DUP_MESH: Self = Self(((1 << 0)) as i32);
    pub const USER_DUP_CURVE: Self = Self(((1 << 1)) as i32);
    pub const USER_DUP_SURF: Self = Self(((1 << 2)) as i32);
    pub const USER_DUP_FONT: Self = Self(((1 << 3)) as i32);
    pub const USER_DUP_MBALL: Self = Self(((1 << 4)) as i32);
    pub const USER_DUP_LAMP: Self = Self(((1 << 5)) as i32);
    pub const USER_DUP_MAT: Self = Self(((1 << 7)) as i32);
    pub const USER_DUP_ARM: Self = Self(((1 << 9)) as i32);
    pub const USER_DUP_ACT: Self = Self(((1 << 10)) as i32);
    pub const USER_DUP_PSYS: Self = Self(((1 << 11)) as i32);
    pub const USER_DUP_LIGHTPROBE: Self = Self(((1 << 12)) as i32);
    pub const USER_DUP_GPENCIL: Self = Self(((1 << 13)) as i32);
    pub const USER_DUP_CURVES: Self = Self(((1 << 14)) as i32);
    pub const USER_DUP_POINTCLOUD: Self = Self(((1 << 15)) as i32);
    pub const USER_DUP_VOLUME: Self = Self(((1 << 16)) as i32);
    pub const USER_DUP_LATTICE: Self = Self(((1 << 17)) as i32);
    pub const USER_DUP_CAMERA: Self = Self(((1 << 18)) as i32);
    pub const USER_DUP_SPEAKER: Self = Self(((1 << 19)) as i32);
    pub const USER_DUP_NTREE: Self = Self(((1 << 20)) as i32);
    pub const USER_DUP_OBDATA: Self = Self(19 as i32);
    pub const USER_DUP_OBJECT: Self = Self(((1 << 24)) as i32);
    pub const USER_DUP_LINKED_ID: Self = Self(((1 << 30)) as i32);
}

