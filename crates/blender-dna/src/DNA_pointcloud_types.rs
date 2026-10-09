//! Auto-transpiled C/C++ header module: DNA_pointcloud_types

use crate::*;

pub const POINTCLOUD_MATERIAL_NR: i32 = 1;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PointCloud {
    pub id: ID,
    pub adt: *mut AnimData,
    pub r#type: PointCloudType,
    pub _pad1: [i16; 3],
    pub flag: ePointCloud_Flag,
    pub totpoint: i32,
    pub attribute_storage: AttributeStorage,
    pub pdata_legacy: CustomData,
    pub attributes_active_index: i32,
    pub _pad4: i32,
    pub mat: *mut *mut Material,
    pub totcol: i16,
    pub _pad3: [i16; 3],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PointCloudRuntime {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PointCloudBatchCache {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GSplatBatchCache {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AttributeAccessor {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MutableAttributeAccessor {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Tree {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnimData {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AttributeStorage {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CustomData {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Material {
    pub _opaque: [u8; 0],
}


#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PointCloudType {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ePointCloud_Flag {
    PT_DS_EXPAND = (1 << 0),
}

impl Default for ePointCloud_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const PT_DS_EXPAND: i32 = ePointCloud_Flag::PT_DS_EXPAND as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointCloudType_2 {
    Points = 0,
    GSplat = 1,
}

impl Default for PointCloudType_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const Points: i32 = PointCloudType_2::Points as i32;
pub const GSplat: i32 = PointCloudType_2::GSplat as i32;

