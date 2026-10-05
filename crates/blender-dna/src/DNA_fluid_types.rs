//! Auto-transpiled C/C++ header module: DNA_fluid_types

use core::ffi::c_void;
use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct FluidDomainSettings {
    pub fmd: *mut FluidModifierData,
    pub fluid: *mut MANTA,
    pub fluid_old: *mut MANTA,
    pub fluid_mutex: *mut core::ffi::c_void,
    pub fluid_group: *mut Collection,
    pub force_group: *mut Collection,
    pub effector_group: *mut Collection,
    pub guide_parent: *mut Object,
    pub effector_weights: *mut EffectorWeights,
    pub p1: [f32; 3],
    pub dp0: [f32; 3],
    pub cell_size: [f32; 3],
    pub global_size: [f32; 3],
    pub prev_loc: [f32; 3],
    pub shift: [i32; 3],
    pub shift_f: [f32; 3],
    pub imat: [[f32; 4]; 4],
    pub obmat: [[f32; 4]; 4],
    pub fluidmat: [[f32; 4]; 4],
    pub fluidmat_wt: [[f32; 4]; 4],
    pub base_res: [i32; 3],
    pub res_min: [i32; 3],
    pub res_max: [i32; 3],
    pub res: [i32; 3],
    pub total_cells: i32,
    pub dx: f32,
    pub scale: f32,
    pub boundary_width: i32,
    pub adapt_margin: i32,
    pub adapt_res: i32,
    pub adapt_threshold: f32,
    pub maxres: i32,
    pub solver_res: i32,
    pub border_collisions: eFluidDomain_BorderFlags,
    pub gravity: [f32; 3],
    pub active_fields: eFluidDomain_ActiveFields,
    pub r#type: eFluidDomain_Type,
    pub _pad2: [i8; 6],
    pub alpha: f32,
    pub beta: f32,
    pub diss_speed: i32,
    pub vorticity: f32,
    pub active_color: [f32; 3],
    pub highres_sampling: eFluidDomain_HighresSampling,
    pub burning_rate: f32,
    pub flame_ignition: f32,
    pub flame_smoke_color: [f32; 3],
    pub noise_strength: f32,
    pub noise_pos_scale: f32,
    pub noise_time_anim: f32,
    pub res_noise: [i32; 3],
    pub noise_scale: i32,
    pub _pad3: [i8; 4],
    pub particle_randomness: f32,
    pub particle_number: i32,
    pub particle_minimum: i32,
    pub particle_maximum: i32,
    pub particle_radius: f32,
    pub particle_band_width: f32,
    pub fractions_threshold: f32,
    pub fractions_distance: f32,
    pub flip_ratio: f32,
    pub sys_particle_maximum: i32,
    pub simulation_method: eFluidDomain_SimMethod,
    pub _pad4: [i8; 6],
    pub viscosity_value: f32,
    pub _pad5: [i8; 4],
    pub surface_tension: f32,
    pub viscosity_base: f32,
    pub viscosity_exponent: i32,
    pub mesh_concave_upper: f32,
    pub mesh_concave_lower: f32,
    pub mesh_particle_radius: f32,
    pub mesh_smoothen_pos: i32,
    pub mesh_smoothen_neg: i32,
    pub mesh_scale: i32,
    pub mesh_generator: eFluidDomain_MeshGenerator,
    pub _pad6: [i8; 2],
    pub particle_type: eFluidDomain_ParticleTypes,
    pub particle_scale: i32,
    pub sndparticle_tau_min_wc: f32,
    pub sndparticle_tau_max_wc: f32,
    pub sndparticle_tau_min_ta: f32,
    pub sndparticle_tau_max_ta: f32,
    pub sndparticle_tau_min_k: f32,
    pub sndparticle_tau_max_k: f32,
    pub sndparticle_k_wc: i32,
    pub sndparticle_k_ta: i32,
    pub sndparticle_k_b: f32,
    pub sndparticle_k_d: f32,
    pub sndparticle_l_min: f32,
    pub sndparticle_l_max: f32,
    pub sndparticle_potential_radius: i32,
    pub sndparticle_update_radius: i32,
    pub sndparticle_boundary: eFluidDomain_SndParticleBoundary,
    pub _pad7: [i8; 6],
    pub guide_alpha: f32,
    pub guide_beta: i32,
    pub guide_vel_factor: f32,
    pub guide_res: [i32; 3],
    pub guide_source: eFluidDomain_GuideSource,
    pub _pad8: [i8; 2],
    pub cache_frame_start: i32,
    pub cache_frame_end: i32,
    pub cache_frame_pause_data: i32,
    pub cache_frame_pause_noise: i32,
    pub cache_frame_pause_mesh: i32,
    pub cache_frame_pause_particles: i32,
    pub cache_frame_pause_guide: i32,
    pub cache_frame_offset: i32,
    pub cache_flag: eFluidDomain_CacheFlag,
    pub cache_mesh_format: eFluidDomain_FileFormat,
    pub cache_data_format: eFluidDomain_FileFormat,
    pub cache_particle_format: eFluidDomain_FileFormat,
    pub cache_noise_format: eFluidDomain_FileFormat,
    pub cache_directory: [i8; 1024],
    pub error: [i8; 64],
    pub cache_type: eFluidDomain_CacheType,
    pub cache_id: [i8; 4],
    pub _pad9: [i8; 2],
    pub dt: f32,
    pub time_total: f32,
    pub time_per_frame: f32,
    pub frame_length: f32,
    pub time_scale: f32,
    pub cfl_condition: f32,
    pub timesteps_minimum: i32,
    pub timesteps_maximum: i32,
    pub slice_per_voxel: f32,
    pub slice_depth: f32,
    pub display_thickness: f32,
    pub grid_scale: f32,
    pub coba: *mut ColorBand,
    pub vector_scale: f32,
    pub gridlines_lower_bound: f32,
    pub gridlines_upper_bound: f32,
    pub gridlines_range_color: [f32; 4],
    pub axis_slice_method: eFluidDomain_AxisSliceMethod,
    pub slice_axis: eFluidDomain_SliceAxis,
    pub show_gridlines: i8,
    pub draw_velocity: i8,
    pub vector_draw_type: eFluidDomain_VectorDrawType,
    pub vector_scale_with_magnitude: i8,
    pub use_coba: i8,
    pub interp_method: FLUID_DisplayInterpolationMethod,
    pub gridlines_cell_filter: eFluidDomain_GridlineCellFilter,
    pub _pad10: [i8; 3],
    pub velocity_scale: f32,
    pub openvdb_compression: eFluidDomain_OpenVDBCompression,
    pub clipping: f32,
    pub openvdb_data_depth: eFluidDomain_OpenVDBDepth,
    pub _pad11: [i8; 7],
    pub viewsettings: i32,
    pub _pad12: [i8; 4],
    pub point_cache: [*mut PointCache; 2],
    pub cache_comp: eFluidDomain_CacheComp,
    pub cache_high_comp: eFluidDomain_CacheComp,
    pub cache_file_format: eFluidDomain_FileFormat,
    pub _pad13: [i8; 7],
}

impl Default for FluidDomainSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct FluidFlowSettings {
    pub fmd: *mut FluidModifierData,
    pub mesh: *mut Mesh,
    pub psys: *mut ParticleSystem,
    pub noise_texture: *mut Tex,
    pub verts_old: *mut f32,
    pub numverts: i32,
    pub vel_multi: f32,
    pub vel_normal: f32,
    pub vel_random: f32,
    pub vel_coord: [f32; 3],
    pub _pad1: [i8; 4],
    pub density: f32,
    pub color: [f32; 3],
    pub fuel_amount: f32,
    pub temperature: f32,
    pub volume_density: f32,
    pub surface_distance: f32,
    pub particle_size: f32,
    pub subframes: i32,
    pub texture_size: f32,
    pub texture_offset: f32,
    pub _pad2: [i8; 4],
    pub uvlayer_name: [i8; 68],
    pub _pad3: [i8; 4],
    pub vgroup_density: i16,
    pub r#type: eFluidFlow_Type,
    pub behavior: eFluidFlow_Behavior,
    pub source: eFluidFlow_Source,
    pub texture_type: eFluidFlow_TextureType,
    pub _pad4: [i16; 3],
}

impl Default for FluidFlowSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FluidEffectorSettings {
    pub fmd: *mut FluidModifierData,
    pub mesh: *mut Mesh,
    pub verts_old: *mut f32,
    pub numverts: i32,
    pub surface_distance: f32,
    pub flags: eFluidEffector_Flags,
    pub subframes: i32,
    pub r#type: eFluidEffector_Type,
    pub _pad1: [i8; 6],
    pub vel_multi: f32,
    pub guide_mode: eFluidEffector_GuideMode,
    pub _pad2: [i8; 2],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Texture {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FluidModifierData {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MANTA {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Collection {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Object {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EffectorWeights {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ColorBand {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PointCache {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ListBaseT {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Mesh {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ParticleSystem {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Tex {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_Flags {
    FLUID_DOMAIN_USE_NOISE = (1 << 1),
    FLUID_DOMAIN_USE_DISSOLVE = (1 << 2),
    FLUID_DOMAIN_USE_DISSOLVE_LOG = (1 << 3),
    FLUID_DOMAIN_USE_HIGH_SMOOTH = (1 << 5),
    FLUID_DOMAIN_FILE_LOAD = (1 << 6),
    FLUID_DOMAIN_USE_ADAPTIVE_DOMAIN = (1 << 7),
    FLUID_DOMAIN_USE_ADAPTIVE_TIME = (1 << 8),
    FLUID_DOMAIN_USE_MESH = (1 << 9),
    FLUID_DOMAIN_USE_GUIDE = (1 << 10),
    FLUID_DOMAIN_USE_SPEED_VECTORS = (1 << 11),
    FLUID_DOMAIN_EXPORT_MANTA_SCRIPT = (1 << 12),
    FLUID_DOMAIN_USE_FRACTIONS = (1 << 13),
    FLUID_DOMAIN_DELETE_IN_OBSTACLE = (1 << 14),
    FLUID_DOMAIN_USE_DIFFUSION = (1 << 15),
    FLUID_DOMAIN_USE_RESUMABLE_CACHE = (1 << 16),
    FLUID_DOMAIN_USE_VISCOSITY = (1 << 17),
}

impl Default for eFluidDomain_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_DOMAIN_USE_NOISE: i32 = eFluidDomain_Flags::FLUID_DOMAIN_USE_NOISE as i32;
pub const FLUID_DOMAIN_USE_DISSOLVE: i32 = eFluidDomain_Flags::FLUID_DOMAIN_USE_DISSOLVE as i32;
pub const FLUID_DOMAIN_USE_DISSOLVE_LOG: i32 = eFluidDomain_Flags::FLUID_DOMAIN_USE_DISSOLVE_LOG as i32;
pub const FLUID_DOMAIN_USE_HIGH_SMOOTH: i32 = eFluidDomain_Flags::FLUID_DOMAIN_USE_HIGH_SMOOTH as i32;
pub const FLUID_DOMAIN_FILE_LOAD: i32 = eFluidDomain_Flags::FLUID_DOMAIN_FILE_LOAD as i32;
pub const FLUID_DOMAIN_USE_ADAPTIVE_DOMAIN: i32 = eFluidDomain_Flags::FLUID_DOMAIN_USE_ADAPTIVE_DOMAIN as i32;
pub const FLUID_DOMAIN_USE_ADAPTIVE_TIME: i32 = eFluidDomain_Flags::FLUID_DOMAIN_USE_ADAPTIVE_TIME as i32;
pub const FLUID_DOMAIN_USE_MESH: i32 = eFluidDomain_Flags::FLUID_DOMAIN_USE_MESH as i32;
pub const FLUID_DOMAIN_USE_GUIDE: i32 = eFluidDomain_Flags::FLUID_DOMAIN_USE_GUIDE as i32;
pub const FLUID_DOMAIN_USE_SPEED_VECTORS: i32 = eFluidDomain_Flags::FLUID_DOMAIN_USE_SPEED_VECTORS as i32;
pub const FLUID_DOMAIN_EXPORT_MANTA_SCRIPT: i32 = eFluidDomain_Flags::FLUID_DOMAIN_EXPORT_MANTA_SCRIPT as i32;
pub const FLUID_DOMAIN_USE_FRACTIONS: i32 = eFluidDomain_Flags::FLUID_DOMAIN_USE_FRACTIONS as i32;
pub const FLUID_DOMAIN_DELETE_IN_OBSTACLE: i32 = eFluidDomain_Flags::FLUID_DOMAIN_DELETE_IN_OBSTACLE as i32;
pub const FLUID_DOMAIN_USE_DIFFUSION: i32 = eFluidDomain_Flags::FLUID_DOMAIN_USE_DIFFUSION as i32;
pub const FLUID_DOMAIN_USE_RESUMABLE_CACHE: i32 = eFluidDomain_Flags::FLUID_DOMAIN_USE_RESUMABLE_CACHE as i32;
pub const FLUID_DOMAIN_USE_VISCOSITY: i32 = eFluidDomain_Flags::FLUID_DOMAIN_USE_VISCOSITY as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_BorderFlags {
    FLUID_DOMAIN_BORDER_FRONT = (1 << 1),
    FLUID_DOMAIN_BORDER_BACK = (1 << 2),
    FLUID_DOMAIN_BORDER_RIGHT = (1 << 3),
    FLUID_DOMAIN_BORDER_LEFT = (1 << 4),
    FLUID_DOMAIN_BORDER_TOP = (1 << 5),
    FLUID_DOMAIN_BORDER_BOTTOM = (1 << 6),
}

impl Default for eFluidDomain_BorderFlags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_DOMAIN_BORDER_FRONT: i32 = eFluidDomain_BorderFlags::FLUID_DOMAIN_BORDER_FRONT as i32;
pub const FLUID_DOMAIN_BORDER_BACK: i32 = eFluidDomain_BorderFlags::FLUID_DOMAIN_BORDER_BACK as i32;
pub const FLUID_DOMAIN_BORDER_RIGHT: i32 = eFluidDomain_BorderFlags::FLUID_DOMAIN_BORDER_RIGHT as i32;
pub const FLUID_DOMAIN_BORDER_LEFT: i32 = eFluidDomain_BorderFlags::FLUID_DOMAIN_BORDER_LEFT as i32;
pub const FLUID_DOMAIN_BORDER_TOP: i32 = eFluidDomain_BorderFlags::FLUID_DOMAIN_BORDER_TOP as i32;
pub const FLUID_DOMAIN_BORDER_BOTTOM: i32 = eFluidDomain_BorderFlags::FLUID_DOMAIN_BORDER_BOTTOM as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_FileFormat {
    FLUID_DOMAIN_FILE_UNI = (1 << 0),
    FLUID_DOMAIN_FILE_OPENVDB = (1 << 1),
    FLUID_DOMAIN_FILE_RAW = (1 << 2),
    FLUID_DOMAIN_FILE_OBJECT = (1 << 3),
    FLUID_DOMAIN_FILE_BIN_OBJECT = (1 << 4),
}

impl Default for eFluidDomain_FileFormat {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_DOMAIN_FILE_UNI: i32 = eFluidDomain_FileFormat::FLUID_DOMAIN_FILE_UNI as i32;
pub const FLUID_DOMAIN_FILE_OPENVDB: i32 = eFluidDomain_FileFormat::FLUID_DOMAIN_FILE_OPENVDB as i32;
pub const FLUID_DOMAIN_FILE_RAW: i32 = eFluidDomain_FileFormat::FLUID_DOMAIN_FILE_RAW as i32;
pub const FLUID_DOMAIN_FILE_OBJECT: i32 = eFluidDomain_FileFormat::FLUID_DOMAIN_FILE_OBJECT as i32;
pub const FLUID_DOMAIN_FILE_BIN_OBJECT: i32 = eFluidDomain_FileFormat::FLUID_DOMAIN_FILE_BIN_OBJECT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_AxisSliceMethod {
    AXIS_SLICE_FULL = 0,
    AXIS_SLICE_SINGLE = 1,
}

impl Default for eFluidDomain_AxisSliceMethod {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const AXIS_SLICE_FULL: i32 = eFluidDomain_AxisSliceMethod::AXIS_SLICE_FULL as i32;
pub const AXIS_SLICE_SINGLE: i32 = eFluidDomain_AxisSliceMethod::AXIS_SLICE_SINGLE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_SliceAxis {
    SLICE_AXIS_AUTO = 0,
    SLICE_AXIS_X = 1,
    SLICE_AXIS_Y = 2,
    SLICE_AXIS_Z = 3,
}

impl Default for eFluidDomain_SliceAxis {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const SLICE_AXIS_AUTO: i32 = eFluidDomain_SliceAxis::SLICE_AXIS_AUTO as i32;
pub const SLICE_AXIS_X: i32 = eFluidDomain_SliceAxis::SLICE_AXIS_X as i32;
pub const SLICE_AXIS_Y: i32 = eFluidDomain_SliceAxis::SLICE_AXIS_Y as i32;
pub const SLICE_AXIS_Z: i32 = eFluidDomain_SliceAxis::SLICE_AXIS_Z as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FLUID_DisplayInterpolationMethod {
    FLUID_DISPLAY_INTERP_LINEAR = 0,
    FLUID_DISPLAY_INTERP_CUBIC = 1,
    FLUID_DISPLAY_INTERP_CLOSEST = 2,
}

impl Default for FLUID_DisplayInterpolationMethod {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_DISPLAY_INTERP_LINEAR: i32 = FLUID_DisplayInterpolationMethod::FLUID_DISPLAY_INTERP_LINEAR as i32;
pub const FLUID_DISPLAY_INTERP_CUBIC: i32 = FLUID_DisplayInterpolationMethod::FLUID_DISPLAY_INTERP_CUBIC as i32;
pub const FLUID_DISPLAY_INTERP_CLOSEST: i32 = FLUID_DisplayInterpolationMethod::FLUID_DISPLAY_INTERP_CLOSEST as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_VectorDrawType {
    VECTOR_DRAW_NEEDLE = 0,
    VECTOR_DRAW_STREAMLINE = 1,
    VECTOR_DRAW_MAC = 2,
}

impl Default for eFluidDomain_VectorDrawType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const VECTOR_DRAW_NEEDLE: i32 = eFluidDomain_VectorDrawType::VECTOR_DRAW_NEEDLE as i32;
pub const VECTOR_DRAW_STREAMLINE: i32 = eFluidDomain_VectorDrawType::VECTOR_DRAW_STREAMLINE as i32;
pub const VECTOR_DRAW_MAC: i32 = eFluidDomain_VectorDrawType::VECTOR_DRAW_MAC as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_VectorDrawMAC {
    VECTOR_DRAW_MAC_X = (1 << 0),
    VECTOR_DRAW_MAC_Y = (1 << 1),
    VECTOR_DRAW_MAC_Z = (1 << 2),
}

impl Default for eFluidDomain_VectorDrawMAC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const VECTOR_DRAW_MAC_X: i32 = eFluidDomain_VectorDrawMAC::VECTOR_DRAW_MAC_X as i32;
pub const VECTOR_DRAW_MAC_Y: i32 = eFluidDomain_VectorDrawMAC::VECTOR_DRAW_MAC_Y as i32;
pub const VECTOR_DRAW_MAC_Z: i32 = eFluidDomain_VectorDrawMAC::VECTOR_DRAW_MAC_Z as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FLUID_DisplayVectorField {
    FLUID_DOMAIN_VECTOR_FIELD_VELOCITY = 0,
    FLUID_DOMAIN_VECTOR_FIELD_GUIDE_VELOCITY = 1,
    FLUID_DOMAIN_VECTOR_FIELD_FORCE = 2,
}

impl Default for FLUID_DisplayVectorField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_DOMAIN_VECTOR_FIELD_VELOCITY: i32 = FLUID_DisplayVectorField::FLUID_DOMAIN_VECTOR_FIELD_VELOCITY as i32;
pub const FLUID_DOMAIN_VECTOR_FIELD_GUIDE_VELOCITY: i32 = FLUID_DisplayVectorField::FLUID_DOMAIN_VECTOR_FIELD_GUIDE_VELOCITY as i32;
pub const FLUID_DOMAIN_VECTOR_FIELD_FORCE: i32 = FLUID_DisplayVectorField::FLUID_DOMAIN_VECTOR_FIELD_FORCE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_SndParticleBoundary {
    SNDPARTICLE_BOUNDARY_DELETE = 0,
    SNDPARTICLE_BOUNDARY_PUSHOUT = 1,
}

impl Default for eFluidDomain_SndParticleBoundary {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const SNDPARTICLE_BOUNDARY_DELETE: i32 = eFluidDomain_SndParticleBoundary::SNDPARTICLE_BOUNDARY_DELETE as i32;
pub const SNDPARTICLE_BOUNDARY_PUSHOUT: i32 = eFluidDomain_SndParticleBoundary::SNDPARTICLE_BOUNDARY_PUSHOUT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_SndParticleCombinedExport {
    SNDPARTICLE_COMBINED_EXPORT_OFF = 0,
    SNDPARTICLE_COMBINED_EXPORT_SPRAY_FOAM = 1,
    SNDPARTICLE_COMBINED_EXPORT_SPRAY_BUBBLE = 2,
    SNDPARTICLE_COMBINED_EXPORT_FOAM_BUBBLE = 3,
    SNDPARTICLE_COMBINED_EXPORT_SPRAY_FOAM_BUBBLE = 4,
}

impl Default for eFluidDomain_SndParticleCombinedExport {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const SNDPARTICLE_COMBINED_EXPORT_OFF: i32 = eFluidDomain_SndParticleCombinedExport::SNDPARTICLE_COMBINED_EXPORT_OFF as i32;
pub const SNDPARTICLE_COMBINED_EXPORT_SPRAY_FOAM: i32 = eFluidDomain_SndParticleCombinedExport::SNDPARTICLE_COMBINED_EXPORT_SPRAY_FOAM as i32;
pub const SNDPARTICLE_COMBINED_EXPORT_SPRAY_BUBBLE: i32 = eFluidDomain_SndParticleCombinedExport::SNDPARTICLE_COMBINED_EXPORT_SPRAY_BUBBLE as i32;
pub const SNDPARTICLE_COMBINED_EXPORT_FOAM_BUBBLE: i32 = eFluidDomain_SndParticleCombinedExport::SNDPARTICLE_COMBINED_EXPORT_FOAM_BUBBLE as i32;
pub const SNDPARTICLE_COMBINED_EXPORT_SPRAY_FOAM_BUBBLE: i32 = eFluidDomain_SndParticleCombinedExport::SNDPARTICLE_COMBINED_EXPORT_SPRAY_FOAM_BUBBLE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_CobaField {
    FLUID_DOMAIN_FIELD_DENSITY = 0,
    FLUID_DOMAIN_FIELD_HEAT = 1,
    FLUID_DOMAIN_FIELD_FUEL = 2,
    FLUID_DOMAIN_FIELD_REACT = 3,
    FLUID_DOMAIN_FIELD_FLAME = 4,
    FLUID_DOMAIN_FIELD_VELOCITY_X = 5,
    FLUID_DOMAIN_FIELD_VELOCITY_Y = 6,
    FLUID_DOMAIN_FIELD_VELOCITY_Z = 7,
    FLUID_DOMAIN_FIELD_COLOR_R = 8,
    FLUID_DOMAIN_FIELD_COLOR_G = 9,
    FLUID_DOMAIN_FIELD_COLOR_B = 10,
    FLUID_DOMAIN_FIELD_FORCE_X = 11,
    FLUID_DOMAIN_FIELD_FORCE_Y = 12,
    FLUID_DOMAIN_FIELD_FORCE_Z = 13,
    FLUID_DOMAIN_FIELD_PHI = 14,
    FLUID_DOMAIN_FIELD_PHI_IN = 15,
    FLUID_DOMAIN_FIELD_PHI_OUT = 16,
    FLUID_DOMAIN_FIELD_PHI_OBSTACLE = 17,
    FLUID_DOMAIN_FIELD_FLAGS = 18,
    FLUID_DOMAIN_FIELD_PRESSURE = 19,
}

impl Default for eFluidDomain_CobaField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_DOMAIN_FIELD_DENSITY: i32 = eFluidDomain_CobaField::FLUID_DOMAIN_FIELD_DENSITY as i32;
pub const FLUID_DOMAIN_FIELD_HEAT: i32 = eFluidDomain_CobaField::FLUID_DOMAIN_FIELD_HEAT as i32;
pub const FLUID_DOMAIN_FIELD_FUEL: i32 = eFluidDomain_CobaField::FLUID_DOMAIN_FIELD_FUEL as i32;
pub const FLUID_DOMAIN_FIELD_REACT: i32 = eFluidDomain_CobaField::FLUID_DOMAIN_FIELD_REACT as i32;
pub const FLUID_DOMAIN_FIELD_FLAME: i32 = eFluidDomain_CobaField::FLUID_DOMAIN_FIELD_FLAME as i32;
pub const FLUID_DOMAIN_FIELD_VELOCITY_X: i32 = eFluidDomain_CobaField::FLUID_DOMAIN_FIELD_VELOCITY_X as i32;
pub const FLUID_DOMAIN_FIELD_VELOCITY_Y: i32 = eFluidDomain_CobaField::FLUID_DOMAIN_FIELD_VELOCITY_Y as i32;
pub const FLUID_DOMAIN_FIELD_VELOCITY_Z: i32 = eFluidDomain_CobaField::FLUID_DOMAIN_FIELD_VELOCITY_Z as i32;
pub const FLUID_DOMAIN_FIELD_COLOR_R: i32 = eFluidDomain_CobaField::FLUID_DOMAIN_FIELD_COLOR_R as i32;
pub const FLUID_DOMAIN_FIELD_COLOR_G: i32 = eFluidDomain_CobaField::FLUID_DOMAIN_FIELD_COLOR_G as i32;
pub const FLUID_DOMAIN_FIELD_COLOR_B: i32 = eFluidDomain_CobaField::FLUID_DOMAIN_FIELD_COLOR_B as i32;
pub const FLUID_DOMAIN_FIELD_FORCE_X: i32 = eFluidDomain_CobaField::FLUID_DOMAIN_FIELD_FORCE_X as i32;
pub const FLUID_DOMAIN_FIELD_FORCE_Y: i32 = eFluidDomain_CobaField::FLUID_DOMAIN_FIELD_FORCE_Y as i32;
pub const FLUID_DOMAIN_FIELD_FORCE_Z: i32 = eFluidDomain_CobaField::FLUID_DOMAIN_FIELD_FORCE_Z as i32;
pub const FLUID_DOMAIN_FIELD_PHI: i32 = eFluidDomain_CobaField::FLUID_DOMAIN_FIELD_PHI as i32;
pub const FLUID_DOMAIN_FIELD_PHI_IN: i32 = eFluidDomain_CobaField::FLUID_DOMAIN_FIELD_PHI_IN as i32;
pub const FLUID_DOMAIN_FIELD_PHI_OUT: i32 = eFluidDomain_CobaField::FLUID_DOMAIN_FIELD_PHI_OUT as i32;
pub const FLUID_DOMAIN_FIELD_PHI_OBSTACLE: i32 = eFluidDomain_CobaField::FLUID_DOMAIN_FIELD_PHI_OBSTACLE as i32;
pub const FLUID_DOMAIN_FIELD_FLAGS: i32 = eFluidDomain_CobaField::FLUID_DOMAIN_FIELD_FLAGS as i32;
pub const FLUID_DOMAIN_FIELD_PRESSURE: i32 = eFluidDomain_CobaField::FLUID_DOMAIN_FIELD_PRESSURE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_GridlineColorField {
    FLUID_GRIDLINE_COLOR_TYPE_NONE = 0,
    FLUID_GRIDLINE_COLOR_TYPE_FLAGS = 1,
    FLUID_GRIDLINE_COLOR_TYPE_RANGE = 2,
}

impl Default for eFluidDomain_GridlineColorField {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_GRIDLINE_COLOR_TYPE_NONE: i32 = eFluidDomain_GridlineColorField::FLUID_GRIDLINE_COLOR_TYPE_NONE as i32;
pub const FLUID_GRIDLINE_COLOR_TYPE_FLAGS: i32 = eFluidDomain_GridlineColorField::FLUID_GRIDLINE_COLOR_TYPE_FLAGS as i32;
pub const FLUID_GRIDLINE_COLOR_TYPE_RANGE: i32 = eFluidDomain_GridlineColorField::FLUID_GRIDLINE_COLOR_TYPE_RANGE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_GridlineCellFilter {
    FLUID_CELL_TYPE_NONE = 0,
    FLUID_CELL_TYPE_FLUID = (1 << 0),
    FLUID_CELL_TYPE_OBSTACLE = (1 << 1),
    FLUID_CELL_TYPE_EMPTY = (1 << 2),
    FLUID_CELL_TYPE_INFLOW = (1 << 3),
    FLUID_CELL_TYPE_OUTFLOW = (1 << 4),
}

impl Default for eFluidDomain_GridlineCellFilter {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_CELL_TYPE_NONE: i32 = eFluidDomain_GridlineCellFilter::FLUID_CELL_TYPE_NONE as i32;
pub const FLUID_CELL_TYPE_FLUID: i32 = eFluidDomain_GridlineCellFilter::FLUID_CELL_TYPE_FLUID as i32;
pub const FLUID_CELL_TYPE_OBSTACLE: i32 = eFluidDomain_GridlineCellFilter::FLUID_CELL_TYPE_OBSTACLE as i32;
pub const FLUID_CELL_TYPE_EMPTY: i32 = eFluidDomain_GridlineCellFilter::FLUID_CELL_TYPE_EMPTY as i32;
pub const FLUID_CELL_TYPE_INFLOW: i32 = eFluidDomain_GridlineCellFilter::FLUID_CELL_TYPE_INFLOW as i32;
pub const FLUID_CELL_TYPE_OUTFLOW: i32 = eFluidDomain_GridlineCellFilter::FLUID_CELL_TYPE_OUTFLOW as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_Type {
    FLUID_DOMAIN_TYPE_GAS = 0,
    FLUID_DOMAIN_TYPE_LIQUID = 1,
}

impl Default for eFluidDomain_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_DOMAIN_TYPE_GAS: i32 = eFluidDomain_Type::FLUID_DOMAIN_TYPE_GAS as i32;
pub const FLUID_DOMAIN_TYPE_LIQUID: i32 = eFluidDomain_Type::FLUID_DOMAIN_TYPE_LIQUID as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_MeshGenerator {
    FLUID_DOMAIN_MESH_IMPROVED = 0,
    FLUID_DOMAIN_MESH_UNION = 1,
}

impl Default for eFluidDomain_MeshGenerator {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_DOMAIN_MESH_IMPROVED: i32 = eFluidDomain_MeshGenerator::FLUID_DOMAIN_MESH_IMPROVED as i32;
pub const FLUID_DOMAIN_MESH_UNION: i32 = eFluidDomain_MeshGenerator::FLUID_DOMAIN_MESH_UNION as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_GuideSource {
    FLUID_DOMAIN_GUIDE_SRC_DOMAIN = 0,
    FLUID_DOMAIN_GUIDE_SRC_EFFECTOR = 1,
}

impl Default for eFluidDomain_GuideSource {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_DOMAIN_GUIDE_SRC_DOMAIN: i32 = eFluidDomain_GuideSource::FLUID_DOMAIN_GUIDE_SRC_DOMAIN as i32;
pub const FLUID_DOMAIN_GUIDE_SRC_EFFECTOR: i32 = eFluidDomain_GuideSource::FLUID_DOMAIN_GUIDE_SRC_EFFECTOR as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_ActiveFields {
    FLUID_DOMAIN_ACTIVE_HEAT = (1 << 0),
    FLUID_DOMAIN_ACTIVE_FIRE = (1 << 1),
    FLUID_DOMAIN_ACTIVE_COLORS = (1 << 2),
    FLUID_DOMAIN_ACTIVE_COLOR_SET = (1 << 3),
    FLUID_DOMAIN_ACTIVE_OBSTACLE = (1 << 4),
    FLUID_DOMAIN_ACTIVE_GUIDE = (1 << 5),
    FLUID_DOMAIN_ACTIVE_INVEL = (1 << 6),
    FLUID_DOMAIN_ACTIVE_OUTFLOW = (1 << 7),
}

impl Default for eFluidDomain_ActiveFields {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_DOMAIN_ACTIVE_HEAT: i32 = eFluidDomain_ActiveFields::FLUID_DOMAIN_ACTIVE_HEAT as i32;
pub const FLUID_DOMAIN_ACTIVE_FIRE: i32 = eFluidDomain_ActiveFields::FLUID_DOMAIN_ACTIVE_FIRE as i32;
pub const FLUID_DOMAIN_ACTIVE_COLORS: i32 = eFluidDomain_ActiveFields::FLUID_DOMAIN_ACTIVE_COLORS as i32;
pub const FLUID_DOMAIN_ACTIVE_COLOR_SET: i32 = eFluidDomain_ActiveFields::FLUID_DOMAIN_ACTIVE_COLOR_SET as i32;
pub const FLUID_DOMAIN_ACTIVE_OBSTACLE: i32 = eFluidDomain_ActiveFields::FLUID_DOMAIN_ACTIVE_OBSTACLE as i32;
pub const FLUID_DOMAIN_ACTIVE_GUIDE: i32 = eFluidDomain_ActiveFields::FLUID_DOMAIN_ACTIVE_GUIDE as i32;
pub const FLUID_DOMAIN_ACTIVE_INVEL: i32 = eFluidDomain_ActiveFields::FLUID_DOMAIN_ACTIVE_INVEL as i32;
pub const FLUID_DOMAIN_ACTIVE_OUTFLOW: i32 = eFluidDomain_ActiveFields::FLUID_DOMAIN_ACTIVE_OUTFLOW as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_ParticleTypes {
    FLUID_DOMAIN_PARTICLE_FLIP = (1 << 0),
    FLUID_DOMAIN_PARTICLE_SPRAY = (1 << 1),
    FLUID_DOMAIN_PARTICLE_BUBBLE = (1 << 2),
    FLUID_DOMAIN_PARTICLE_FOAM = (1 << 3),
    FLUID_DOMAIN_PARTICLE_TRACER = (1 << 4),
}

impl Default for eFluidDomain_ParticleTypes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_DOMAIN_PARTICLE_FLIP: i32 = eFluidDomain_ParticleTypes::FLUID_DOMAIN_PARTICLE_FLIP as i32;
pub const FLUID_DOMAIN_PARTICLE_SPRAY: i32 = eFluidDomain_ParticleTypes::FLUID_DOMAIN_PARTICLE_SPRAY as i32;
pub const FLUID_DOMAIN_PARTICLE_BUBBLE: i32 = eFluidDomain_ParticleTypes::FLUID_DOMAIN_PARTICLE_BUBBLE as i32;
pub const FLUID_DOMAIN_PARTICLE_FOAM: i32 = eFluidDomain_ParticleTypes::FLUID_DOMAIN_PARTICLE_FOAM as i32;
pub const FLUID_DOMAIN_PARTICLE_TRACER: i32 = eFluidDomain_ParticleTypes::FLUID_DOMAIN_PARTICLE_TRACER as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_SimMethod {
    FLUID_DOMAIN_METHOD_FLIP = (1 << 0),
    FLUID_DOMAIN_METHOD_APIC = (1 << 1),
}

impl Default for eFluidDomain_SimMethod {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_DOMAIN_METHOD_FLIP: i32 = eFluidDomain_SimMethod::FLUID_DOMAIN_METHOD_FLIP as i32;
pub const FLUID_DOMAIN_METHOD_APIC: i32 = eFluidDomain_SimMethod::FLUID_DOMAIN_METHOD_APIC as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_CacheFlag {
    FLUID_DOMAIN_BAKING_DATA = (1 << 0),
    FLUID_DOMAIN_BAKED_DATA = (1 << 1),
    FLUID_DOMAIN_BAKING_NOISE = (1 << 2),
    FLUID_DOMAIN_BAKED_NOISE = (1 << 3),
    FLUID_DOMAIN_BAKING_MESH = (1 << 4),
    FLUID_DOMAIN_BAKED_MESH = (1 << 5),
    FLUID_DOMAIN_BAKING_PARTICLES = (1 << 6),
    FLUID_DOMAIN_BAKED_PARTICLES = (1 << 7),
    FLUID_DOMAIN_BAKING_GUIDE = (1 << 8),
    FLUID_DOMAIN_BAKED_GUIDE = (1 << 9),
    FLUID_DOMAIN_OUTDATED_DATA = (1 << 10),
    FLUID_DOMAIN_OUTDATED_NOISE = (1 << 11),
    FLUID_DOMAIN_OUTDATED_MESH = (1 << 12),
    FLUID_DOMAIN_OUTDATED_PARTICLES = (1 << 13),
    FLUID_DOMAIN_OUTDATED_GUIDE = (1 << 14),
}

impl Default for eFluidDomain_CacheFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_DOMAIN_BAKING_DATA: i32 = eFluidDomain_CacheFlag::FLUID_DOMAIN_BAKING_DATA as i32;
pub const FLUID_DOMAIN_BAKED_DATA: i32 = eFluidDomain_CacheFlag::FLUID_DOMAIN_BAKED_DATA as i32;
pub const FLUID_DOMAIN_BAKING_NOISE: i32 = eFluidDomain_CacheFlag::FLUID_DOMAIN_BAKING_NOISE as i32;
pub const FLUID_DOMAIN_BAKED_NOISE: i32 = eFluidDomain_CacheFlag::FLUID_DOMAIN_BAKED_NOISE as i32;
pub const FLUID_DOMAIN_BAKING_MESH: i32 = eFluidDomain_CacheFlag::FLUID_DOMAIN_BAKING_MESH as i32;
pub const FLUID_DOMAIN_BAKED_MESH: i32 = eFluidDomain_CacheFlag::FLUID_DOMAIN_BAKED_MESH as i32;
pub const FLUID_DOMAIN_BAKING_PARTICLES: i32 = eFluidDomain_CacheFlag::FLUID_DOMAIN_BAKING_PARTICLES as i32;
pub const FLUID_DOMAIN_BAKED_PARTICLES: i32 = eFluidDomain_CacheFlag::FLUID_DOMAIN_BAKED_PARTICLES as i32;
pub const FLUID_DOMAIN_BAKING_GUIDE: i32 = eFluidDomain_CacheFlag::FLUID_DOMAIN_BAKING_GUIDE as i32;
pub const FLUID_DOMAIN_BAKED_GUIDE: i32 = eFluidDomain_CacheFlag::FLUID_DOMAIN_BAKED_GUIDE as i32;
pub const FLUID_DOMAIN_OUTDATED_DATA: i32 = eFluidDomain_CacheFlag::FLUID_DOMAIN_OUTDATED_DATA as i32;
pub const FLUID_DOMAIN_OUTDATED_NOISE: i32 = eFluidDomain_CacheFlag::FLUID_DOMAIN_OUTDATED_NOISE as i32;
pub const FLUID_DOMAIN_OUTDATED_MESH: i32 = eFluidDomain_CacheFlag::FLUID_DOMAIN_OUTDATED_MESH as i32;
pub const FLUID_DOMAIN_OUTDATED_PARTICLES: i32 = eFluidDomain_CacheFlag::FLUID_DOMAIN_OUTDATED_PARTICLES as i32;
pub const FLUID_DOMAIN_OUTDATED_GUIDE: i32 = eFluidDomain_CacheFlag::FLUID_DOMAIN_OUTDATED_GUIDE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_GridType {
    FLUID_DOMAIN_GRID_FLOAT = 0,
    FLUID_DOMAIN_GRID_INT = 1,
    FLUID_DOMAIN_GRID_VEC3F = 2,
}

impl Default for eFluidDomain_GridType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_DOMAIN_GRID_FLOAT: i32 = eFluidDomain_GridType::FLUID_DOMAIN_GRID_FLOAT as i32;
pub const FLUID_DOMAIN_GRID_INT: i32 = eFluidDomain_GridType::FLUID_DOMAIN_GRID_INT as i32;
pub const FLUID_DOMAIN_GRID_VEC3F: i32 = eFluidDomain_GridType::FLUID_DOMAIN_GRID_VEC3F as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_CacheFilesMode {
    FLUID_DOMAIN_CACHE_FILES_SINGLE = 0,
    FLUID_DOMAIN_CACHE_FILES_COMBINED = 1,
}

impl Default for eFluidDomain_CacheFilesMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_DOMAIN_CACHE_FILES_SINGLE: i32 = eFluidDomain_CacheFilesMode::FLUID_DOMAIN_CACHE_FILES_SINGLE as i32;
pub const FLUID_DOMAIN_CACHE_FILES_COMBINED: i32 = eFluidDomain_CacheFilesMode::FLUID_DOMAIN_CACHE_FILES_COMBINED as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_CacheType {
    FLUID_DOMAIN_CACHE_REPLAY = 0,
    FLUID_DOMAIN_CACHE_MODULAR = 1,
    FLUID_DOMAIN_CACHE_ALL = 2,
}

impl Default for eFluidDomain_CacheType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_DOMAIN_CACHE_REPLAY: i32 = eFluidDomain_CacheType::FLUID_DOMAIN_CACHE_REPLAY as i32;
pub const FLUID_DOMAIN_CACHE_MODULAR: i32 = eFluidDomain_CacheType::FLUID_DOMAIN_CACHE_MODULAR as i32;
pub const FLUID_DOMAIN_CACHE_ALL: i32 = eFluidDomain_CacheType::FLUID_DOMAIN_CACHE_ALL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_OpenVDBCompression {
    VDB_COMPRESSION_BLOSC = 0,
    VDB_COMPRESSION_ZIP = 1,
    VDB_COMPRESSION_NONE = 2,
}

impl Default for eFluidDomain_OpenVDBCompression {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const VDB_COMPRESSION_BLOSC: i32 = eFluidDomain_OpenVDBCompression::VDB_COMPRESSION_BLOSC as i32;
pub const VDB_COMPRESSION_ZIP: i32 = eFluidDomain_OpenVDBCompression::VDB_COMPRESSION_ZIP as i32;
pub const VDB_COMPRESSION_NONE: i32 = eFluidDomain_OpenVDBCompression::VDB_COMPRESSION_NONE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_OpenVDBDepth {
    VDB_PRECISION_HALF_FLOAT = 0,
    VDB_PRECISION_FULL_FLOAT = 1,
    VDB_PRECISION_MINI_FLOAT = 2,
}

impl Default for eFluidDomain_OpenVDBDepth {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const VDB_PRECISION_HALF_FLOAT: i32 = eFluidDomain_OpenVDBDepth::VDB_PRECISION_HALF_FLOAT as i32;
pub const VDB_PRECISION_FULL_FLOAT: i32 = eFluidDomain_OpenVDBDepth::VDB_PRECISION_FULL_FLOAT as i32;
pub const VDB_PRECISION_MINI_FLOAT: i32 = eFluidDomain_OpenVDBDepth::VDB_PRECISION_MINI_FLOAT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_CacheComp {
    SM_CACHE_LIGHT = 0,
    SM_CACHE_HEAVY = 1,
}

impl Default for eFluidDomain_CacheComp {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const SM_CACHE_LIGHT: i32 = eFluidDomain_CacheComp::SM_CACHE_LIGHT as i32;
pub const SM_CACHE_HEAVY: i32 = eFluidDomain_CacheComp::SM_CACHE_HEAVY as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidDomain_HighresSampling {
    SM_HRES_NEAREST = 0,
    SM_HRES_LINEAR = 1,
    SM_HRES_FULLSAMPLE = 2,
}

impl Default for eFluidDomain_HighresSampling {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const SM_HRES_NEAREST: i32 = eFluidDomain_HighresSampling::SM_HRES_NEAREST as i32;
pub const SM_HRES_LINEAR: i32 = eFluidDomain_HighresSampling::SM_HRES_LINEAR as i32;
pub const SM_HRES_FULLSAMPLE: i32 = eFluidDomain_HighresSampling::SM_HRES_FULLSAMPLE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidFlow_Type {
    FLUID_FLOW_TYPE_SMOKE = 1,
    FLUID_FLOW_TYPE_FIRE = 2,
    FLUID_FLOW_TYPE_SMOKEFIRE = 3,
    FLUID_FLOW_TYPE_LIQUID = 4,
}

impl Default for eFluidFlow_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_FLOW_TYPE_SMOKE: i32 = eFluidFlow_Type::FLUID_FLOW_TYPE_SMOKE as i32;
pub const FLUID_FLOW_TYPE_FIRE: i32 = eFluidFlow_Type::FLUID_FLOW_TYPE_FIRE as i32;
pub const FLUID_FLOW_TYPE_SMOKEFIRE: i32 = eFluidFlow_Type::FLUID_FLOW_TYPE_SMOKEFIRE as i32;
pub const FLUID_FLOW_TYPE_LIQUID: i32 = eFluidFlow_Type::FLUID_FLOW_TYPE_LIQUID as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidFlow_Behavior {
    FLUID_FLOW_BEHAVIOR_INFLOW = 0,
    FLUID_FLOW_BEHAVIOR_OUTFLOW = 1,
    FLUID_FLOW_BEHAVIOR_GEOMETRY = 2,
}

impl Default for eFluidFlow_Behavior {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_FLOW_BEHAVIOR_INFLOW: i32 = eFluidFlow_Behavior::FLUID_FLOW_BEHAVIOR_INFLOW as i32;
pub const FLUID_FLOW_BEHAVIOR_OUTFLOW: i32 = eFluidFlow_Behavior::FLUID_FLOW_BEHAVIOR_OUTFLOW as i32;
pub const FLUID_FLOW_BEHAVIOR_GEOMETRY: i32 = eFluidFlow_Behavior::FLUID_FLOW_BEHAVIOR_GEOMETRY as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidFlow_Source {
    FLUID_FLOW_SOURCE_PARTICLES = 0,
    FLUID_FLOW_SOURCE_MESH = 1,
}

impl Default for eFluidFlow_Source {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_FLOW_SOURCE_PARTICLES: i32 = eFluidFlow_Source::FLUID_FLOW_SOURCE_PARTICLES as i32;
pub const FLUID_FLOW_SOURCE_MESH: i32 = eFluidFlow_Source::FLUID_FLOW_SOURCE_MESH as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidFlow_TextureType {
    FLUID_FLOW_TEXTURE_MAP_AUTO = 0,
    FLUID_FLOW_TEXTURE_MAP_UV = 1,
}

impl Default for eFluidFlow_TextureType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_FLOW_TEXTURE_MAP_AUTO: i32 = eFluidFlow_TextureType::FLUID_FLOW_TEXTURE_MAP_AUTO as i32;
pub const FLUID_FLOW_TEXTURE_MAP_UV: i32 = eFluidFlow_TextureType::FLUID_FLOW_TEXTURE_MAP_UV as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidFlow_Flags {
    FLUID_FLOW_ABSOLUTE = (1 << 1),
    FLUID_FLOW_INITVELOCITY = (1 << 2),
    FLUID_FLOW_TEXTUREEMIT = (1 << 3),
    FLUID_FLOW_USE_PART_SIZE = (1 << 4),
    FLUID_FLOW_USE_INFLOW = (1 << 5),
    FLUID_FLOW_USE_PLANE_INIT = (1 << 6),
    FLUID_FLOW_NEEDS_UPDATE = (1 << 7),
}

impl Default for eFluidFlow_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_FLOW_ABSOLUTE: i32 = eFluidFlow_Flags::FLUID_FLOW_ABSOLUTE as i32;
pub const FLUID_FLOW_INITVELOCITY: i32 = eFluidFlow_Flags::FLUID_FLOW_INITVELOCITY as i32;
pub const FLUID_FLOW_TEXTUREEMIT: i32 = eFluidFlow_Flags::FLUID_FLOW_TEXTUREEMIT as i32;
pub const FLUID_FLOW_USE_PART_SIZE: i32 = eFluidFlow_Flags::FLUID_FLOW_USE_PART_SIZE as i32;
pub const FLUID_FLOW_USE_INFLOW: i32 = eFluidFlow_Flags::FLUID_FLOW_USE_INFLOW as i32;
pub const FLUID_FLOW_USE_PLANE_INIT: i32 = eFluidFlow_Flags::FLUID_FLOW_USE_PLANE_INIT as i32;
pub const FLUID_FLOW_NEEDS_UPDATE: i32 = eFluidFlow_Flags::FLUID_FLOW_NEEDS_UPDATE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidEffector_Type {
    FLUID_EFFECTOR_TYPE_COLLISION = 0,
    FLUID_EFFECTOR_TYPE_GUIDE = 1,
}

impl Default for eFluidEffector_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_EFFECTOR_TYPE_COLLISION: i32 = eFluidEffector_Type::FLUID_EFFECTOR_TYPE_COLLISION as i32;
pub const FLUID_EFFECTOR_TYPE_GUIDE: i32 = eFluidEffector_Type::FLUID_EFFECTOR_TYPE_GUIDE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidEffector_GuideMode {
    FLUID_EFFECTOR_GUIDE_MAX = 0,
    FLUID_EFFECTOR_GUIDE_MIN = 1,
    FLUID_EFFECTOR_GUIDE_OVERRIDE = 2,
    FLUID_EFFECTOR_GUIDE_AVERAGED = 3,
}

impl Default for eFluidEffector_GuideMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_EFFECTOR_GUIDE_MAX: i32 = eFluidEffector_GuideMode::FLUID_EFFECTOR_GUIDE_MAX as i32;
pub const FLUID_EFFECTOR_GUIDE_MIN: i32 = eFluidEffector_GuideMode::FLUID_EFFECTOR_GUIDE_MIN as i32;
pub const FLUID_EFFECTOR_GUIDE_OVERRIDE: i32 = eFluidEffector_GuideMode::FLUID_EFFECTOR_GUIDE_OVERRIDE as i32;
pub const FLUID_EFFECTOR_GUIDE_AVERAGED: i32 = eFluidEffector_GuideMode::FLUID_EFFECTOR_GUIDE_AVERAGED as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFluidEffector_Flags {
    FLUID_EFFECTOR_USE_EFFEC = (1 << 1),
    FLUID_EFFECTOR_USE_PLANE_INIT = (1 << 2),
    FLUID_EFFECTOR_NEEDS_UPDATE = (1 << 3),
}

impl Default for eFluidEffector_Flags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const FLUID_EFFECTOR_USE_EFFEC: i32 = eFluidEffector_Flags::FLUID_EFFECTOR_USE_EFFEC as i32;
pub const FLUID_EFFECTOR_USE_PLANE_INIT: i32 = eFluidEffector_Flags::FLUID_EFFECTOR_USE_PLANE_INIT as i32;
pub const FLUID_EFFECTOR_NEEDS_UPDATE: i32 = eFluidEffector_Flags::FLUID_EFFECTOR_NEEDS_UPDATE as i32;

