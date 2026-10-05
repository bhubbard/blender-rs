//! Auto-transpiled C/C++ header module: DNA_dynamicpaint_types

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct DynamicPaintSurface {
    pub next: *mut DynamicPaintSurface,
    pub canvas: *mut DynamicPaintCanvasSettings,
    pub data: *mut PaintSurfaceData,
    pub brush_group: *mut Collection,
    pub effector_weights: *mut EffectorWeights,
    pub pointcache: *mut PointCache,
    pub ptcaches: ListBaseT<PointCache>,
    pub current_frame: i32,
    pub name: [i8; 64],
    pub format: eDynamicPaint_SurfaceFormat,
    pub r#type: eDynamicPaint_SurfaceType,
    pub disp_type: eDynamicPaint_DispType,
    pub image_fileformat: eDynamicPaint_ImageFormat,
    pub effect_ui: i16,
    pub init_color_type: eDynamicPaint_InitColorType,
    pub flags: eDynamicPaint_SurfaceFlags,
    pub effect: eDynamicPaint_EffectFlags,
    pub image_resolution: i32,
    pub start_frame: i32,
    pub init_color: [f32; 4],
    pub init_texture: *mut Tex,
    pub init_layername: [i8; 68],
    pub dry_speed: i32,
    pub color_dry_threshold: f32,
    pub depth_clamp: f32,
    pub spread_speed: f32,
    pub drip_vel: f32,
    pub influence_scale: f32,
    pub wave_damping: f32,
    pub _pad2: [i8; 4],
    pub uvlayer_name: [i8; 68],
    pub image_output_path: [i8; 1024],
    pub output_name: [i8; 68],
    pub output_name2: [i8; 68],
}

impl Default for DynamicPaintSurface {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct DynamicPaintCanvasSettings {
    pub pmd: *mut DynamicPaintModifierData,
    pub surfaces: ListBaseT<DynamicPaintSurface>,
    pub active_sur: i16,
    pub flags: eDynamicPaint_CanvasFlags,
    pub _pad: [i8; 4],
    pub error: [i8; 64],
}

impl Default for DynamicPaintCanvasSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DynamicPaintBrushSettings {
    pub pmd: *mut DynamicPaintModifierData,
    pub psys: *mut ParticleSystem,
    pub flags: eDynamicPaint_BrushFlags,
    pub collision: eDynamicPaint_CollisionType,
    pub r: f32,
    pub wetness: f32,
    pub particle_radius: f32,
    pub paint_distance: f32,
    pub paint_ramp: *mut ColorBand,
    pub vel_ramp: *mut ColorBand,
    pub proximity_falloff: eDynamicPaint_ProximityFalloff,
    pub wave_type: eDynamicPaint_WaveBrushType,
    pub ray_dir: eDynamicPaint_RayDir,
    pub _pad: [i8; 2],
    pub wave_factor: f32,
    pub max_velocity: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PaintSurfaceData {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Collection {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EffectorWeights {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PointCache {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Tex {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DynamicPaintModifierData {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ParticleSystem {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ColorBand {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDynamicPaint_SurfaceType {
    MOD_DPAINT_SURFACE_T_PAINT = 0,
    MOD_DPAINT_SURFACE_T_DISPLACE = 1,
    MOD_DPAINT_SURFACE_T_WEIGHT = 2,
    MOD_DPAINT_SURFACE_T_WAVE = 3,
}

impl Default for eDynamicPaint_SurfaceType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MOD_DPAINT_SURFACE_T_PAINT: i32 = eDynamicPaint_SurfaceType::MOD_DPAINT_SURFACE_T_PAINT as i32;
pub const MOD_DPAINT_SURFACE_T_DISPLACE: i32 = eDynamicPaint_SurfaceType::MOD_DPAINT_SURFACE_T_DISPLACE as i32;
pub const MOD_DPAINT_SURFACE_T_WEIGHT: i32 = eDynamicPaint_SurfaceType::MOD_DPAINT_SURFACE_T_WEIGHT as i32;
pub const MOD_DPAINT_SURFACE_T_WAVE: i32 = eDynamicPaint_SurfaceType::MOD_DPAINT_SURFACE_T_WAVE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDynamicPaint_SurfaceFlags {
    MOD_DPAINT_ACTIVE = 1 << 0,
    MOD_DPAINT_ANTIALIAS = 1 << 1,
    MOD_DPAINT_DISSOLVE = 1 << 2,
    MOD_DPAINT_MULALPHA = 1 << 3,
    MOD_DPAINT_DISSOLVE_LOG = 1 << 4,
    MOD_DPAINT_DRY_LOG = 1 << 5,
    MOD_DPAINT_WAVE_OPEN_BORDERS = 1 << 7,
    MOD_DPAINT_DISP_INCREMENTAL = 1 << 8,
    MOD_DPAINT_USE_DRYING = 1 << 9,
    MOD_DPAINT_OUT1 = 1 << 10,
    MOD_DPAINT_OUT2 = 1 << 11,
}

impl Default for eDynamicPaint_SurfaceFlags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MOD_DPAINT_ACTIVE: i32 = eDynamicPaint_SurfaceFlags::MOD_DPAINT_ACTIVE as i32;
pub const MOD_DPAINT_ANTIALIAS: i32 = eDynamicPaint_SurfaceFlags::MOD_DPAINT_ANTIALIAS as i32;
pub const MOD_DPAINT_DISSOLVE: i32 = eDynamicPaint_SurfaceFlags::MOD_DPAINT_DISSOLVE as i32;
pub const MOD_DPAINT_MULALPHA: i32 = eDynamicPaint_SurfaceFlags::MOD_DPAINT_MULALPHA as i32;
pub const MOD_DPAINT_DISSOLVE_LOG: i32 = eDynamicPaint_SurfaceFlags::MOD_DPAINT_DISSOLVE_LOG as i32;
pub const MOD_DPAINT_DRY_LOG: i32 = eDynamicPaint_SurfaceFlags::MOD_DPAINT_DRY_LOG as i32;
pub const MOD_DPAINT_WAVE_OPEN_BORDERS: i32 = eDynamicPaint_SurfaceFlags::MOD_DPAINT_WAVE_OPEN_BORDERS as i32;
pub const MOD_DPAINT_DISP_INCREMENTAL: i32 = eDynamicPaint_SurfaceFlags::MOD_DPAINT_DISP_INCREMENTAL as i32;
pub const MOD_DPAINT_USE_DRYING: i32 = eDynamicPaint_SurfaceFlags::MOD_DPAINT_USE_DRYING as i32;
pub const MOD_DPAINT_OUT1: i32 = eDynamicPaint_SurfaceFlags::MOD_DPAINT_OUT1 as i32;
pub const MOD_DPAINT_OUT2: i32 = eDynamicPaint_SurfaceFlags::MOD_DPAINT_OUT2 as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDynamicPaint_ImageFormat {
    MOD_DPAINT_IMGFORMAT_PNG = 0,
    MOD_DPAINT_IMGFORMAT_OPENEXR = 1,
}

impl Default for eDynamicPaint_ImageFormat {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MOD_DPAINT_IMGFORMAT_PNG: i32 = eDynamicPaint_ImageFormat::MOD_DPAINT_IMGFORMAT_PNG as i32;
pub const MOD_DPAINT_IMGFORMAT_OPENEXR: i32 = eDynamicPaint_ImageFormat::MOD_DPAINT_IMGFORMAT_OPENEXR as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDynamicPaint_DispType {
    MOD_DPAINT_DISP_DISPLACE = 0,
    MOD_DPAINT_DISP_DEPTH = 1,
}

impl Default for eDynamicPaint_DispType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MOD_DPAINT_DISP_DISPLACE: i32 = eDynamicPaint_DispType::MOD_DPAINT_DISP_DISPLACE as i32;
pub const MOD_DPAINT_DISP_DEPTH: i32 = eDynamicPaint_DispType::MOD_DPAINT_DISP_DEPTH as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDynamicPaint_EffectFlags {
    MOD_DPAINT_EFFECT_DO_SPREAD = 1 << 0,
    MOD_DPAINT_EFFECT_DO_DRIP = 1 << 1,
    MOD_DPAINT_EFFECT_DO_SHRINK = 1 << 2,
}

impl Default for eDynamicPaint_EffectFlags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MOD_DPAINT_EFFECT_DO_SPREAD: i32 = eDynamicPaint_EffectFlags::MOD_DPAINT_EFFECT_DO_SPREAD as i32;
pub const MOD_DPAINT_EFFECT_DO_DRIP: i32 = eDynamicPaint_EffectFlags::MOD_DPAINT_EFFECT_DO_DRIP as i32;
pub const MOD_DPAINT_EFFECT_DO_SHRINK: i32 = eDynamicPaint_EffectFlags::MOD_DPAINT_EFFECT_DO_SHRINK as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDynamicPaint_InitColorType {
    MOD_DPAINT_INITIAL_NONE = 0,
    MOD_DPAINT_INITIAL_COLOR = 1,
    MOD_DPAINT_INITIAL_TEXTURE = 2,
    MOD_DPAINT_INITIAL_VERTEXCOLOR = 3,
}

impl Default for eDynamicPaint_InitColorType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MOD_DPAINT_INITIAL_NONE: i32 = eDynamicPaint_InitColorType::MOD_DPAINT_INITIAL_NONE as i32;
pub const MOD_DPAINT_INITIAL_COLOR: i32 = eDynamicPaint_InitColorType::MOD_DPAINT_INITIAL_COLOR as i32;
pub const MOD_DPAINT_INITIAL_TEXTURE: i32 = eDynamicPaint_InitColorType::MOD_DPAINT_INITIAL_TEXTURE as i32;
pub const MOD_DPAINT_INITIAL_VERTEXCOLOR: i32 = eDynamicPaint_InitColorType::MOD_DPAINT_INITIAL_VERTEXCOLOR as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDynamicPaint_CanvasFlags {
    MOD_DPAINT_BAKING = 1 << 1,
}

impl Default for eDynamicPaint_CanvasFlags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MOD_DPAINT_BAKING: i32 = eDynamicPaint_CanvasFlags::MOD_DPAINT_BAKING as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDynamicPaint_BrushFlags {
    MOD_DPAINT_PART_RAD = 1 << 0,
    MOD_DPAINT_ABS_ALPHA = 1 << 2,
    MOD_DPAINT_ERASE = 1 << 3,
    MOD_DPAINT_RAMP_ALPHA = 1 << 4,
    MOD_DPAINT_PROX_PROJECT = 1 << 5,
    MOD_DPAINT_INVERSE_PROX = 1 << 6,
    MOD_DPAINT_NEGATE_VOLUME = 1 << 7,
    MOD_DPAINT_DO_SMUDGE = 1 << 8,
    MOD_DPAINT_VELOCITY_ALPHA = 1 << 9,
    MOD_DPAINT_VELOCITY_COLOR = 1 << 10,
    MOD_DPAINT_VELOCITY_DEPTH = 1 << 11,
    MOD_DPAINT_USES_VELOCITY = (MOD_DPAINT_DO_SMUDGE | MOD_DPAINT_VELOCITY_ALPHA | MOD_DPAINT_VELOCITY_COLOR | MOD_DPAINT_VELOCITY_DEPTH),
}

impl Default for eDynamicPaint_BrushFlags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MOD_DPAINT_PART_RAD: i32 = eDynamicPaint_BrushFlags::MOD_DPAINT_PART_RAD as i32;
pub const MOD_DPAINT_ABS_ALPHA: i32 = eDynamicPaint_BrushFlags::MOD_DPAINT_ABS_ALPHA as i32;
pub const MOD_DPAINT_ERASE: i32 = eDynamicPaint_BrushFlags::MOD_DPAINT_ERASE as i32;
pub const MOD_DPAINT_RAMP_ALPHA: i32 = eDynamicPaint_BrushFlags::MOD_DPAINT_RAMP_ALPHA as i32;
pub const MOD_DPAINT_PROX_PROJECT: i32 = eDynamicPaint_BrushFlags::MOD_DPAINT_PROX_PROJECT as i32;
pub const MOD_DPAINT_INVERSE_PROX: i32 = eDynamicPaint_BrushFlags::MOD_DPAINT_INVERSE_PROX as i32;
pub const MOD_DPAINT_NEGATE_VOLUME: i32 = eDynamicPaint_BrushFlags::MOD_DPAINT_NEGATE_VOLUME as i32;
pub const MOD_DPAINT_DO_SMUDGE: i32 = eDynamicPaint_BrushFlags::MOD_DPAINT_DO_SMUDGE as i32;
pub const MOD_DPAINT_VELOCITY_ALPHA: i32 = eDynamicPaint_BrushFlags::MOD_DPAINT_VELOCITY_ALPHA as i32;
pub const MOD_DPAINT_VELOCITY_COLOR: i32 = eDynamicPaint_BrushFlags::MOD_DPAINT_VELOCITY_COLOR as i32;
pub const MOD_DPAINT_VELOCITY_DEPTH: i32 = eDynamicPaint_BrushFlags::MOD_DPAINT_VELOCITY_DEPTH as i32;
pub const MOD_DPAINT_USES_VELOCITY: i32 = eDynamicPaint_BrushFlags::MOD_DPAINT_USES_VELOCITY as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDynamicPaint_CollisionType {
    MOD_DPAINT_COL_VOLUME = 0,
    MOD_DPAINT_COL_DIST = 1,
    MOD_DPAINT_COL_VOLDIST = 2,
    MOD_DPAINT_COL_PSYS = 3,
    MOD_DPAINT_COL_POINT = 4,
}

impl Default for eDynamicPaint_CollisionType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MOD_DPAINT_COL_VOLUME: i32 = eDynamicPaint_CollisionType::MOD_DPAINT_COL_VOLUME as i32;
pub const MOD_DPAINT_COL_DIST: i32 = eDynamicPaint_CollisionType::MOD_DPAINT_COL_DIST as i32;
pub const MOD_DPAINT_COL_VOLDIST: i32 = eDynamicPaint_CollisionType::MOD_DPAINT_COL_VOLDIST as i32;
pub const MOD_DPAINT_COL_PSYS: i32 = eDynamicPaint_CollisionType::MOD_DPAINT_COL_PSYS as i32;
pub const MOD_DPAINT_COL_POINT: i32 = eDynamicPaint_CollisionType::MOD_DPAINT_COL_POINT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDynamicPaint_ProximityFalloff {
    MOD_DPAINT_PRFALL_CONSTANT = 0,
    MOD_DPAINT_PRFALL_SMOOTH = 1,
    MOD_DPAINT_PRFALL_RAMP = 2,
}

impl Default for eDynamicPaint_ProximityFalloff {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MOD_DPAINT_PRFALL_CONSTANT: i32 = eDynamicPaint_ProximityFalloff::MOD_DPAINT_PRFALL_CONSTANT as i32;
pub const MOD_DPAINT_PRFALL_SMOOTH: i32 = eDynamicPaint_ProximityFalloff::MOD_DPAINT_PRFALL_SMOOTH as i32;
pub const MOD_DPAINT_PRFALL_RAMP: i32 = eDynamicPaint_ProximityFalloff::MOD_DPAINT_PRFALL_RAMP as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDynamicPaint_WaveBrushType {
    MOD_DPAINT_WAVEB_DEPTH = 0,
    MOD_DPAINT_WAVEB_FORCE = 1,
    MOD_DPAINT_WAVEB_REFLECT = 2,
    MOD_DPAINT_WAVEB_CHANGE = 3,
}

impl Default for eDynamicPaint_WaveBrushType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MOD_DPAINT_WAVEB_DEPTH: i32 = eDynamicPaint_WaveBrushType::MOD_DPAINT_WAVEB_DEPTH as i32;
pub const MOD_DPAINT_WAVEB_FORCE: i32 = eDynamicPaint_WaveBrushType::MOD_DPAINT_WAVEB_FORCE as i32;
pub const MOD_DPAINT_WAVEB_REFLECT: i32 = eDynamicPaint_WaveBrushType::MOD_DPAINT_WAVEB_REFLECT as i32;
pub const MOD_DPAINT_WAVEB_CHANGE: i32 = eDynamicPaint_WaveBrushType::MOD_DPAINT_WAVEB_CHANGE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDynamicPaint_RayDir {
    MOD_DPAINT_RAY_CANVAS = 0,
    MOD_DPAINT_RAY_BRUSH_AVG = 1,
    MOD_DPAINT_RAY_ZPLUS = 2,
}

impl Default for eDynamicPaint_RayDir {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MOD_DPAINT_RAY_CANVAS: i32 = eDynamicPaint_RayDir::MOD_DPAINT_RAY_CANVAS as i32;
pub const MOD_DPAINT_RAY_BRUSH_AVG: i32 = eDynamicPaint_RayDir::MOD_DPAINT_RAY_BRUSH_AVG as i32;
pub const MOD_DPAINT_RAY_ZPLUS: i32 = eDynamicPaint_RayDir::MOD_DPAINT_RAY_ZPLUS as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDynamicPaint_SurfaceFormat {
    MOD_DPAINT_SURFACE_F_PTEX = 0,
    MOD_DPAINT_SURFACE_F_VERTEX = 1,
    MOD_DPAINT_SURFACE_F_IMAGESEQ = 2,
}

impl Default for eDynamicPaint_SurfaceFormat {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const MOD_DPAINT_SURFACE_F_PTEX: i32 = eDynamicPaint_SurfaceFormat::MOD_DPAINT_SURFACE_F_PTEX as i32;
pub const MOD_DPAINT_SURFACE_F_VERTEX: i32 = eDynamicPaint_SurfaceFormat::MOD_DPAINT_SURFACE_F_VERTEX as i32;
pub const MOD_DPAINT_SURFACE_F_IMAGESEQ: i32 = eDynamicPaint_SurfaceFormat::MOD_DPAINT_SURFACE_F_IMAGESEQ as i32;

