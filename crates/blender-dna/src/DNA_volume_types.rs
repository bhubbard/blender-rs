//! Auto-transpiled C/C++ header module: DNA_volume_types

use crate::*;

pub const VOLUME_MATERIAL_NR: i32 = 1;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VolumeDisplay {
    pub density: f32,
    pub wireframe_type: VolumeWireframeType,
    pub wireframe_detail: VolumeWireframeDetail,
    pub interpolation_method: VolumeDisplayInterpMethod,
    pub axis_slice_method: AxisAlignedSlicingMethod,
    pub slice_axis: SliceAxis,
    pub slice_depth: f32,
    pub _pad: [i32; 1],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VolumeRender {
    pub precision: VolumeRenderPrecision,
    pub space: VolumeRenderSpace,
    pub step_size: f32,
    pub clipping: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Volume {
    pub id: ID,
    pub adt: *mut AnimData,
    pub filepath: [i8; 1024],
    pub packedfile: *mut PackedFile,
    pub is_sequence: i8,
    pub sequence_mode: VolumeSequenceMode,
    pub _pad1: [i8; 2],
    pub frame_start: i32,
    pub frame_duration: i32,
    pub frame_offset: i32,
    pub flag: eVolume_Flag,
    pub active_grid: i32,
    pub mat: *mut *mut Material,
    pub totcol: i16,
    pub _pad2: [i16; 3],
    pub render: VolumeRender,
    pub display: VolumeDisplay,
    pub velocity_grid: [i8; 64],
    pub _pad3: [i8; 3],
    pub velocity_unit: i8,
    pub velocity_scale: f32,
}

impl Default for Volume {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PackedFile {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VolumeBatchCache {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VolumeRuntime {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnimData {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Material {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eVolume_Flag {
    VO_DS_EXPAND = (1 << 0),
}

impl Default for eVolume_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const VO_DS_EXPAND: i32 = eVolume_Flag::VO_DS_EXPAND as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolumeSequenceMode {
    VOLUME_SEQUENCE_CLIP = 0,
    VOLUME_SEQUENCE_EXTEND = 1,
    VOLUME_SEQUENCE_REPEAT = 2,
    VOLUME_SEQUENCE_PING_PONG = 3,
}

impl Default for VolumeSequenceMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const VOLUME_SEQUENCE_CLIP: i32 = VolumeSequenceMode::VOLUME_SEQUENCE_CLIP as i32;
pub const VOLUME_SEQUENCE_EXTEND: i32 = VolumeSequenceMode::VOLUME_SEQUENCE_EXTEND as i32;
pub const VOLUME_SEQUENCE_REPEAT: i32 = VolumeSequenceMode::VOLUME_SEQUENCE_REPEAT as i32;
pub const VOLUME_SEQUENCE_PING_PONG: i32 = VolumeSequenceMode::VOLUME_SEQUENCE_PING_PONG as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolumeWireframeType {
    VOLUME_WIREFRAME_NONE = 0,
    VOLUME_WIREFRAME_BOUNDS = 1,
    VOLUME_WIREFRAME_BOXES = 2,
    VOLUME_WIREFRAME_POINTS = 3,
}

impl Default for VolumeWireframeType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const VOLUME_WIREFRAME_NONE: i32 = VolumeWireframeType::VOLUME_WIREFRAME_NONE as i32;
pub const VOLUME_WIREFRAME_BOUNDS: i32 = VolumeWireframeType::VOLUME_WIREFRAME_BOUNDS as i32;
pub const VOLUME_WIREFRAME_BOXES: i32 = VolumeWireframeType::VOLUME_WIREFRAME_BOXES as i32;
pub const VOLUME_WIREFRAME_POINTS: i32 = VolumeWireframeType::VOLUME_WIREFRAME_POINTS as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolumeWireframeDetail {
    VOLUME_WIREFRAME_COARSE = 0,
    VOLUME_WIREFRAME_FINE = 1,
}

impl Default for VolumeWireframeDetail {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const VOLUME_WIREFRAME_COARSE: i32 = VolumeWireframeDetail::VOLUME_WIREFRAME_COARSE as i32;
pub const VOLUME_WIREFRAME_FINE: i32 = VolumeWireframeDetail::VOLUME_WIREFRAME_FINE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolumeRenderPrecision {
    VOLUME_PRECISION_HALF = 0,
    VOLUME_PRECISION_FULL = 1,
    VOLUME_PRECISION_VARIABLE = 2,
}

impl Default for VolumeRenderPrecision {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const VOLUME_PRECISION_HALF: i32 = VolumeRenderPrecision::VOLUME_PRECISION_HALF as i32;
pub const VOLUME_PRECISION_FULL: i32 = VolumeRenderPrecision::VOLUME_PRECISION_FULL as i32;
pub const VOLUME_PRECISION_VARIABLE: i32 = VolumeRenderPrecision::VOLUME_PRECISION_VARIABLE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolumeRenderSpace {
    VOLUME_SPACE_OBJECT = 0,
    VOLUME_SPACE_WORLD = 1,
}

impl Default for VolumeRenderSpace {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const VOLUME_SPACE_OBJECT: i32 = VolumeRenderSpace::VOLUME_SPACE_OBJECT as i32;
pub const VOLUME_SPACE_WORLD: i32 = VolumeRenderSpace::VOLUME_SPACE_WORLD as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolumeDisplayInterpMethod {
    VOLUME_DISPLAY_INTERP_LINEAR = 0,
    VOLUME_DISPLAY_INTERP_CUBIC = 1,
    VOLUME_DISPLAY_INTERP_CLOSEST = 2,
}

impl Default for VolumeDisplayInterpMethod {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const VOLUME_DISPLAY_INTERP_LINEAR: i32 = VolumeDisplayInterpMethod::VOLUME_DISPLAY_INTERP_LINEAR as i32;
pub const VOLUME_DISPLAY_INTERP_CUBIC: i32 = VolumeDisplayInterpMethod::VOLUME_DISPLAY_INTERP_CUBIC as i32;
pub const VOLUME_DISPLAY_INTERP_CLOSEST: i32 = VolumeDisplayInterpMethod::VOLUME_DISPLAY_INTERP_CLOSEST as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisAlignedSlicingMethod {
    VOLUME_AXIS_SLICE_FULL = 0,
    VOLUME_AXIS_SLICE_SINGLE = 1,
}

impl Default for AxisAlignedSlicingMethod {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const VOLUME_AXIS_SLICE_FULL: i32 = AxisAlignedSlicingMethod::VOLUME_AXIS_SLICE_FULL as i32;
pub const VOLUME_AXIS_SLICE_SINGLE: i32 = AxisAlignedSlicingMethod::VOLUME_AXIS_SLICE_SINGLE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliceAxis {
    VOLUME_SLICE_AXIS_AUTO = 0,
    VOLUME_SLICE_AXIS_X = 1,
    VOLUME_SLICE_AXIS_Y = 2,
    VOLUME_SLICE_AXIS_Z = 3,
}

impl Default for SliceAxis {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const VOLUME_SLICE_AXIS_AUTO: i32 = SliceAxis::VOLUME_SLICE_AXIS_AUTO as i32;
pub const VOLUME_SLICE_AXIS_X: i32 = SliceAxis::VOLUME_SLICE_AXIS_X as i32;
pub const VOLUME_SLICE_AXIS_Y: i32 = SliceAxis::VOLUME_SLICE_AXIS_Y as i32;
pub const VOLUME_SLICE_AXIS_Z: i32 = SliceAxis::VOLUME_SLICE_AXIS_Z as i32;

