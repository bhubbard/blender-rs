//! Auto-transpiled C/C++ header module: DNA_mask_types

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Mask_Runtime {
    pub last_update: u64,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Mask {
    pub id: ID,
    pub adt: *mut AnimData,
    pub masklayers: ListBaseT<MaskLayer>,
    pub masklay_act: i32,
    pub masklay_tot: i32,
    pub sfra: i32,
    pub flag: MaskAnimFlag,
    pub _pad: [i8; 4],
    pub runtime: Mask_Runtime,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct MaskParent {
    pub id_type: i32,
    pub r#type: MaskParentType,
    pub id: *mut ID,
    pub parent: [i8; 64],
    pub sub_parent: [i8; 64],
    pub parent_orig: [f32; 2],
    pub parent_corners_orig: [[f32; 2]; 4],
}

impl Default for MaskParent {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MaskSplinePointUW {
    pub u: f32,
    pub flag: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MaskSplinePoint {
    pub bezt: BezTriple,
    pub _pad: [i8; 4],
    pub tot_uw: i32,
    pub uw: *mut MaskSplinePointUW,
    pub parent: MaskParent,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MaskSpline {
    pub next: *mut MaskSpline,
    pub flag: MaskSplineFlag,
    pub offset_mode: MaskSplineOffset,
    pub weight_interp: MaskSplineInterp,
    pub tot_point: i32,
    pub points: *mut MaskSplinePoint,
    pub parent: MaskParent,
    pub runtime: *mut MaskSplineRuntime,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MaskLayerShape {
    pub next: *mut MaskLayerShape,
    pub data: *mut f32,
    pub tot_vert: i32,
    pub frame: i32,
    pub flag: MaskLayerShapeFlag,
    pub _pad: [i8; 7],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct MaskLayer {
    pub next: *mut MaskLayer,
    pub name: [i8; 64],
    pub splines: ListBaseT<MaskSpline>,
    pub splines_shapes: ListBaseT<MaskLayerShape>,
    pub act_spline: *mut MaskSpline,
    pub act_point: *mut MaskSplinePoint,
    pub alpha: f32,
    pub blend: MaskLayerBlend,
    pub blend_flag: MaskLayerBlendFlag,
    pub falloff: i8,
    pub fill_solver: MaskLayerFillSolverType,
    pub _pad: [i8; 6],
    pub flag: MaskLayerFlag,
    pub visibility_flag: MaskLayerVisibility,
}

impl Default for MaskLayer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MaskSplineRuntime {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MaskLayerShapeElem {
    pub _opaque: [u8; 0],
}


#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskParentType {
    MASK_PARENT_POINT_TRACK = 0,
    MASK_PARENT_PLANE_TRACK = 1,
}

impl Default for MaskParentType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MASK_PARENT_POINT_TRACK: i32 = MaskParentType::MASK_PARENT_POINT_TRACK as i32;
pub const MASK_PARENT_PLANE_TRACK: i32 = MaskParentType::MASK_PARENT_PLANE_TRACK as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskSplineFlag {
    MASK_SPLINE_SELECT = (1 << 0),
    MASK_SPLINE_CYCLIC = (1 << 1),
    MASK_SPLINE_NOFILL = (1 << 2),
    MASK_SPLINE_NOINTERSECT = (1 << 3),
}

impl Default for MaskSplineFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MASK_SPLINE_SELECT: i32 = MaskSplineFlag::MASK_SPLINE_SELECT as i32;
pub const MASK_SPLINE_CYCLIC: i32 = MaskSplineFlag::MASK_SPLINE_CYCLIC as i32;
pub const MASK_SPLINE_NOFILL: i32 = MaskSplineFlag::MASK_SPLINE_NOFILL as i32;
pub const MASK_SPLINE_NOINTERSECT: i32 = MaskSplineFlag::MASK_SPLINE_NOINTERSECT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskSplineInterp {
    MASK_SPLINE_INTERP_LINEAR = 1,
    MASK_SPLINE_INTERP_EASE = 2,
}

impl Default for MaskSplineInterp {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MASK_SPLINE_INTERP_LINEAR: i32 = MaskSplineInterp::MASK_SPLINE_INTERP_LINEAR as i32;
pub const MASK_SPLINE_INTERP_EASE: i32 = MaskSplineInterp::MASK_SPLINE_INTERP_EASE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskSplineOffset {
    MASK_SPLINE_OFFSET_EVEN = 0,
    MASK_SPLINE_OFFSET_SMOOTH = 1,
}

impl Default for MaskSplineOffset {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MASK_SPLINE_OFFSET_EVEN: i32 = MaskSplineOffset::MASK_SPLINE_OFFSET_EVEN as i32;
pub const MASK_SPLINE_OFFSET_SMOOTH: i32 = MaskSplineOffset::MASK_SPLINE_OFFSET_SMOOTH as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskLayerVisibility {
    MASK_HIDE_VIEW = 1 << 0,
    MASK_HIDE_SELECT = 1 << 1,
    MASK_HIDE_RENDER = 1 << 2,
}

impl Default for MaskLayerVisibility {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MASK_HIDE_VIEW: i32 = MaskLayerVisibility::MASK_HIDE_VIEW as i32;
pub const MASK_HIDE_SELECT: i32 = MaskLayerVisibility::MASK_HIDE_SELECT as i32;
pub const MASK_HIDE_RENDER: i32 = MaskLayerVisibility::MASK_HIDE_RENDER as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskDrawFlag {
    MASK_DRAWFLAG_SMOOTH_DEPRECATED = 1 << 0,
    MASK_DRAWFLAG_OVERLAY = 1 << 1,
    MASK_DRAWFLAG_SPLINE = 1 << 2,
}

impl Default for MaskDrawFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MASK_DRAWFLAG_SMOOTH_DEPRECATED: i32 = MaskDrawFlag::MASK_DRAWFLAG_SMOOTH_DEPRECATED as i32;
pub const MASK_DRAWFLAG_OVERLAY: i32 = MaskDrawFlag::MASK_DRAWFLAG_OVERLAY as i32;
pub const MASK_DRAWFLAG_SPLINE: i32 = MaskDrawFlag::MASK_DRAWFLAG_SPLINE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskDrawType {
    MASK_DT_OUTLINE = 0,
    MASK_DT_DASH = 1,
    MASK_DT_BLACK = 2,
    MASK_DT_WHITE = 3,
}

impl Default for MaskDrawType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MASK_DT_OUTLINE: i32 = MaskDrawType::MASK_DT_OUTLINE as i32;
pub const MASK_DT_DASH: i32 = MaskDrawType::MASK_DT_DASH as i32;
pub const MASK_DT_BLACK: i32 = MaskDrawType::MASK_DT_BLACK as i32;
pub const MASK_DT_WHITE: i32 = MaskDrawType::MASK_DT_WHITE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskOverlayMode {
    MASK_OVERLAY_ALPHACHANNEL = 0,
    MASK_OVERLAY_COMBINED = 1,
}

impl Default for MaskOverlayMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MASK_OVERLAY_ALPHACHANNEL: i32 = MaskOverlayMode::MASK_OVERLAY_ALPHACHANNEL as i32;
pub const MASK_OVERLAY_COMBINED: i32 = MaskOverlayMode::MASK_OVERLAY_COMBINED as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskLayerBlend {
    MASK_BLEND_ADD = 0,
    MASK_BLEND_SUBTRACT = 1,
    MASK_BLEND_LIGHTEN = 2,
    MASK_BLEND_DARKEN = 3,
    MASK_BLEND_MUL = 4,
    MASK_BLEND_REPLACE = 5,
    MASK_BLEND_DIFFERENCE = 6,
    MASK_BLEND_MERGE_ADD = 7,
    MASK_BLEND_MERGE_SUBTRACT = 8,
}

impl Default for MaskLayerBlend {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MASK_BLEND_ADD: i32 = MaskLayerBlend::MASK_BLEND_ADD as i32;
pub const MASK_BLEND_SUBTRACT: i32 = MaskLayerBlend::MASK_BLEND_SUBTRACT as i32;
pub const MASK_BLEND_LIGHTEN: i32 = MaskLayerBlend::MASK_BLEND_LIGHTEN as i32;
pub const MASK_BLEND_DARKEN: i32 = MaskLayerBlend::MASK_BLEND_DARKEN as i32;
pub const MASK_BLEND_MUL: i32 = MaskLayerBlend::MASK_BLEND_MUL as i32;
pub const MASK_BLEND_REPLACE: i32 = MaskLayerBlend::MASK_BLEND_REPLACE as i32;
pub const MASK_BLEND_DIFFERENCE: i32 = MaskLayerBlend::MASK_BLEND_DIFFERENCE as i32;
pub const MASK_BLEND_MERGE_ADD: i32 = MaskLayerBlend::MASK_BLEND_MERGE_ADD as i32;
pub const MASK_BLEND_MERGE_SUBTRACT: i32 = MaskLayerBlend::MASK_BLEND_MERGE_SUBTRACT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskLayerBlendFlag {
    MASK_BLENDFLAG_INVERT = (1 << 0),
}

impl Default for MaskLayerBlendFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MASK_BLENDFLAG_INVERT: i32 = MaskLayerBlendFlag::MASK_BLENDFLAG_INVERT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskLayerFlag {
    MASK_LAYERFLAG_LOCKED = (1 << 4),
    MASK_LAYERFLAG_SELECT = (1 << 5),
    MASK_LAYERFLAG_FILL_DISCRETE = (1 << 6),
    MASK_LAYERFLAG_FILL_OVERLAP = 1 << 7,
}

impl Default for MaskLayerFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MASK_LAYERFLAG_LOCKED: i32 = MaskLayerFlag::MASK_LAYERFLAG_LOCKED as i32;
pub const MASK_LAYERFLAG_SELECT: i32 = MaskLayerFlag::MASK_LAYERFLAG_SELECT as i32;
pub const MASK_LAYERFLAG_FILL_DISCRETE: i32 = MaskLayerFlag::MASK_LAYERFLAG_FILL_DISCRETE as i32;
pub const MASK_LAYERFLAG_FILL_OVERLAP: i32 = MaskLayerFlag::MASK_LAYERFLAG_FILL_OVERLAP as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskLayerShapeFlag {
    MASK_SHAPE_SELECT = (1 << 0),
}

impl Default for MaskLayerShapeFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MASK_SHAPE_SELECT: i32 = MaskLayerShapeFlag::MASK_SHAPE_SELECT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskAnimFlag {
    MASK_ANIMF_EXPAND = (1 << 4),
}

impl Default for MaskAnimFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MASK_ANIMF_EXPAND: i32 = MaskAnimFlag::MASK_ANIMF_EXPAND as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskLayerFillSolverType {
    MASK_FILL_SOLVER_SWEEP_LINE = 0,
    MASK_FILL_SOLVER_CDT = 1,
}

impl Default for MaskLayerFillSolverType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MASK_FILL_SOLVER_SWEEP_LINE: i32 = MaskLayerFillSolverType::MASK_FILL_SOLVER_SWEEP_LINE as i32;
pub const MASK_FILL_SOLVER_CDT: i32 = MaskLayerFillSolverType::MASK_FILL_SOLVER_CDT as i32;
