//! Auto-transpiled C/C++ header module: depsgraph_type
//! Upstream: source/blender/depsgraph/intern/depsgraph_type.hh

use crate::*;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eUpdateSource {
    DEG_UPDATE_SOURCE_TIME = 1 << 0,
    DEG_UPDATE_SOURCE_USER_EDIT = 1 << 1,
    DEG_UPDATE_SOURCE_RELATIONS = 1 << 2,
    DEG_UPDATE_SOURCE_VISIBILITY = 1 << 3,
    DEG_UPDATE_SOURCE_SIDE_EFFECT_REQUEST = 1 << 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DEGCustomDataMeshMasks {
    pub vert_mask: u64,
    pub edge_mask: u64,
    pub face_mask: u64,
    pub loop_mask: u64,
    pub poly_mask: u64,
}

impl DEGCustomDataMeshMasks {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mask_vert(vert_mask: u64) -> Self {
        Self {
            vert_mask,
            ..Default::default()
        }
    }

    pub fn mask_edge(edge_mask: u64) -> Self {
        Self {
            edge_mask,
            ..Default::default()
        }
    }

    pub fn mask_face(face_mask: u64) -> Self {
        Self {
            face_mask,
            ..Default::default()
        }
    }
}

impl std::ops::BitOr for DEGCustomDataMeshMasks {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            vert_mask: self.vert_mask | rhs.vert_mask,
            edge_mask: self.edge_mask | rhs.edge_mask,
            face_mask: self.face_mask | rhs.face_mask,
            loop_mask: self.loop_mask | rhs.loop_mask,
            poly_mask: self.poly_mask | rhs.poly_mask,
        }
    }
}

impl std::ops::BitOrAssign for DEGCustomDataMeshMasks {
    fn bitor_assign(&mut self, rhs: Self) {
        self.vert_mask |= rhs.vert_mask;
        self.edge_mask |= rhs.edge_mask;
        self.face_mask |= rhs.face_mask;
        self.loop_mask |= rhs.loop_mask;
        self.poly_mask |= rhs.poly_mask;
    }
}
