//! Auto-transpiled C/C++ header module: DNA_meshdata_types

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MSelect {
    pub index: i32,
    pub r#type: eMSelect_Type,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MFloatProperty {
    pub f: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MIntProperty {
    pub i: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct MStringProperty {
    pub s: [i8; 255],
    pub s_len: i8,
}

impl Default for MStringProperty {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MBoolProperty {
    pub b: u8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MInt8Property {
    pub i: i8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MDeformWeight {
    pub def_nr: u32,
    pub weight: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MDeformVert {
    pub dw: *mut MDeformWeight,
    pub totweight: i32,
    pub flag: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MVertSkin {
    pub radius: [f32; 3],
    pub flag: eMVertSkinFlag,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MLoopCol {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MPropCol {
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MDisps {
    pub totdisp: i32,
    pub _pad: i32,
    pub hidden: *mut u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GridPaintMask {
    pub data: *mut f32,
    pub level: u32,
    pub _pad: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct OrigSpaceFace {
    pub uv: [[f32; 2]; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct OrigSpaceLoop {
    pub uv: [f32; 2],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FreestyleEdge {
    pub flag: eFreestyleEdge_Flag,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FreestyleFace {
    pub flag: eFreestyleFace_Flag,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MEdge {
    pub v1: u32,
    pub v2: u32,
    pub crease_legacy: i8,
    pub bweight_legacy: i8,
    pub flag_legacy: eMEdge_Flag,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MPoly {
    pub loopstart: i32,
    pub totloop: i32,
    pub mat_nr_legacy: i16,
    pub flag_legacy: eMPoly_Flag,
    pub _pad: i8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MLoopUV {
    pub uv: [f32; 2],
    pub flag: eMLoopUV_Flag,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MVert {
    pub co_legacy: [f32; 3],
    pub flag_legacy: eMVert_Flag,
    pub bweight_legacy: i8,
    pub _pad: [i8; 2],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MLoop {
    pub v: u32,
    pub e: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MFace {
    pub v1: u32,
    pub v2: u32,
    pub v3: u32,
    pub v4: u32,
    pub mat_nr: i16,
    pub edcode: eMFace_EdgeCode,
    pub flag: i8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MTFace {
    pub uv: [[f32; 2]; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MCol {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MRecast {
    pub i: i32,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMSelect_Type {
    ME_VSEL = 0,
    ME_ESEL = 1,
    ME_FSEL = 2,
}

impl Default for eMSelect_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const ME_VSEL: i32 = eMSelect_Type::ME_VSEL as i32;
pub const ME_ESEL: i32 = eMSelect_Type::ME_ESEL as i32;
pub const ME_FSEL: i32 = eMSelect_Type::ME_FSEL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMVertSkinFlag {
    MVERT_SKIN_ROOT = 1,
    MVERT_SKIN_LOOSE = 2,
}

impl Default for eMVertSkinFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MVERT_SKIN_ROOT: i32 = eMVertSkinFlag::MVERT_SKIN_ROOT as i32;
pub const MVERT_SKIN_LOOSE: i32 = eMVertSkinFlag::MVERT_SKIN_LOOSE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMFace_EdgeCode {
    ME_V1V2 = (1 << 0),
    ME_V2V3 = (1 << 1),
    ME_V3V1 = (1 << 2),
    ME_V4V1 = (1 << 3),
}

impl Default for eMFace_EdgeCode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const ME_V1V2: i32 = eMFace_EdgeCode::ME_V1V2 as i32;
pub const ME_V2V3: i32 = eMFace_EdgeCode::ME_V2V3 as i32;
pub const ME_V3V1: i32 = eMFace_EdgeCode::ME_V3V1 as i32;
pub const ME_V3V4: i32 = ME_V3V1;
pub const ME_V4V1: i32 = eMFace_EdgeCode::ME_V4V1 as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFreestyleEdge_Flag {
    FREESTYLE_EDGE_MARK = 1,
}

impl Default for eFreestyleEdge_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FREESTYLE_EDGE_MARK: i32 = eFreestyleEdge_Flag::FREESTYLE_EDGE_MARK as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFreestyleFace_Flag {
    FREESTYLE_FACE_MARK = 1,
}

impl Default for eFreestyleFace_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FREESTYLE_FACE_MARK: i32 = eFreestyleFace_Flag::FREESTYLE_FACE_MARK as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMEdge_Flag {
    ME_SEAM = (1 << 2),
    ME_LOOSEEDGE = (1 << 7),
    ME_SHARP = (1 << 9),
}

impl Default for eMEdge_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const ME_SEAM: i32 = eMEdge_Flag::ME_SEAM as i32;
pub const ME_LOOSEEDGE: i32 = eMEdge_Flag::ME_LOOSEEDGE as i32;
pub const ME_SHARP: i32 = eMEdge_Flag::ME_SHARP as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMPoly_Flag {
    ME_SMOOTH = (1 << 0),
    ME_FACE_SEL = (1 << 1),
}

impl Default for eMPoly_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const ME_SMOOTH: i32 = eMPoly_Flag::ME_SMOOTH as i32;
pub const ME_FACE_SEL: i32 = eMPoly_Flag::ME_FACE_SEL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMLoopUV_Flag {
    MLOOPUV_EDGESEL = (1 << 0),
    MLOOPUV_VERTSEL = (1 << 1),
    MLOOPUV_PINNED = (1 << 2),
}

impl Default for eMLoopUV_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MLOOPUV_EDGESEL: i32 = eMLoopUV_Flag::MLOOPUV_EDGESEL as i32;
pub const MLOOPUV_VERTSEL: i32 = eMLoopUV_Flag::MLOOPUV_VERTSEL as i32;
pub const MLOOPUV_PINNED: i32 = eMLoopUV_Flag::MLOOPUV_PINNED as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eMVert_Flag {
    ME_HIDE = (1 << 4),
}

impl Default for eMVert_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const ME_HIDE: i32 = eMVert_Flag::ME_HIDE as i32;

