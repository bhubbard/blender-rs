//! Auto-transpiled C/C++ header module: DNA_lattice_types

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EditLatt {
    pub latt: *mut Lattice,
    pub shapenr: i32,
    pub needs_flush_to_id: i8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Lattice {
    pub id: ID,
    pub adt: *mut AnimData,
    pub pntsu: i16,
    pub flag: eLattice_Flag,
    pub opntsu: i16,
    pub _pad2: [i8; 3],
    pub typeu: i8,
    pub actbp: i32,
    pub fu: f32,
    pub def: *mut BPoint,
    pub key: *mut Key,
    pub dvert: *mut MDeformVert,
    pub vgroup: [i8; 64],
    pub vertex_group_names: ListBaseT<bDeformGroup>,
    pub vertex_group_active_index: i32,
    pub _pad0: [i8; 4],
    pub editlatt: *mut EditLatt,
}

impl Default for Lattice {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnimData {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BPoint {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Key {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MDeformVert {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LatticeBatchCache {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bDeformGroup {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eLattice_Flag {
    LT_GRID = 1 << 0,
    LT_OUTSIDE = 1 << 1,
    LT_DS_EXPAND = 1 << 2,
}

impl Default for eLattice_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LT_GRID: i32 = eLattice_Flag::LT_GRID as i32;
pub const LT_OUTSIDE: i32 = eLattice_Flag::LT_OUTSIDE as i32;
pub const LT_DS_EXPAND: i32 = eLattice_Flag::LT_DS_EXPAND as i32;

