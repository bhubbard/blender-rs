//! Auto-transpiled C/C++ header module: DNA_cloth_types

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ClothSimSettings {
    pub cache: *mut LinkNode,
    pub mingoal: f32,
    pub Cdis: f32,
    pub Cvi: f32,
    pub gravity: [f32; 3],
    pub dt: f32,
    pub mass: f32,
    pub structural: f32,
    pub shear: f32,
    pub bending: f32,
    pub max_bend: f32,
    pub max_struct: f32,
    pub max_shear: f32,
    pub max_sewing: f32,
    pub avg_spring_len: f32,
    pub timescale: f32,
    pub time_scale: f32,
    pub maxgoal: f32,
    pub eff_force_scale: f32,
    pub eff_wind_scale: f32,
    pub sim_time_old: f32,
    pub defgoal: f32,
    pub goalspring: f32,
    pub goalfrict: f32,
    pub velocity_smooth: f32,
    pub density_target: f32,
    pub density_strength: f32,
    pub collider_friction: f32,
    pub vel_damping: f32,
    pub shrink_min: f32,
    pub shrink_max: f32,
    pub uniform_pressure_force: f32,
    pub target_volume: f32,
    pub pressure_factor: f32,
    pub fluid_density: f32,
    pub vgroup_pressure: i16,
    pub _pad7: [i8; 6],
    pub bending_damping: f32,
    pub voxel_cell_size: f32,
    pub stepsPerFrame: i32,
    pub flags: CLOTH_SIMSETTINGS_FLAGS,
    pub preroll: i32,
    pub maxspringlen: i32,
    pub solver_type: i16,
    pub vgroup_bend: i16,
    pub vgroup_mass: i16,
    pub vgroup_struct: i16,
    pub vgroup_shrink: i16,
    pub shapekey_rest: i16,
    pub presets: i16,
    pub reset: i16,
    pub effector_weights: *mut EffectorWeights,
    pub bending_model: CLOTH_BENDING_MODEL,
    pub vgroup_shear: i16,
    pub tension: f32,
    pub compression: f32,
    pub max_tension: f32,
    pub max_compression: f32,
    pub tension_damp: f32,
    pub compression_damp: f32,
    pub shear_damp: f32,
    pub internal_spring_max_length: f32,
    pub internal_spring_max_diversion: f32,
    pub vgroup_intern: i16,
    pub _pad1: [i8; 2],
    pub internal_tension: f32,
    pub internal_compression: f32,
    pub max_internal_tension: f32,
    pub max_internal_compression: f32,
    pub _pad0: [i8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ClothCollSettings {
    pub collision_list: *mut LinkNode,
    pub epsilon: f32,
    pub self_friction: f32,
    pub friction: f32,
    pub damping: f32,
    pub selfepsilon: f32,
    pub repel_force: f32,
    pub distance_repel: f32,
    pub flags: CLOTH_COLLISIONSETTINGS_FLAGS,
    pub self_loop_count: i16,
    pub loop_count: i16,
    pub _pad: [i8; 4],
    pub group: *mut Collection,
    pub vgroup_selfcol: i16,
    pub vgroup_objcol: i16,
    pub _pad2: [i8; 4],
    pub clamp: f32,
    pub self_clamp: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct contains {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LinkNode {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct is {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EffectorWeights {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Collection {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLOTH_SIMSETTINGS_FLAGS {
    CLOTH_SIMSETTINGS_FLAG_COLLOBJ = (1 << 2),
    CLOTH_SIMSETTINGS_FLAG_GOAL = (1 << 3),
    CLOTH_SIMSETTINGS_FLAG_TEARING = (1 << 4),
    CLOTH_SIMSETTINGS_FLAG_PRESSURE = (1 << 5),
    CLOTH_SIMSETTINGS_FLAG_PRESSURE_VOL = (1 << 6),
    CLOTH_SIMSETTINGS_FLAG_INTERNAL_SPRINGS = (1 << 7),
    CLOTH_SIMSETTINGS_FLAG_SCALING = (1 << 8),
    CLOTH_SIMSETTINGS_FLAG_INTERNAL_SPRINGS_NORMAL = (1 << 9),
    CLOTH_SIMSETTINGS_FLAG_RESIST_SPRING_COMPRESS = (1 << 13),
    CLOTH_SIMSETTINGS_FLAG_SEW = (1 << 14),
    CLOTH_SIMSETTINGS_FLAG_DYNAMIC_BASEMESH = (1 << 15),
}

impl Default for CLOTH_SIMSETTINGS_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CLOTH_SIMSETTINGS_FLAG_COLLOBJ: i32 = CLOTH_SIMSETTINGS_FLAGS::CLOTH_SIMSETTINGS_FLAG_COLLOBJ as i32;
pub const CLOTH_SIMSETTINGS_FLAG_GOAL: i32 = CLOTH_SIMSETTINGS_FLAGS::CLOTH_SIMSETTINGS_FLAG_GOAL as i32;
pub const CLOTH_SIMSETTINGS_FLAG_TEARING: i32 = CLOTH_SIMSETTINGS_FLAGS::CLOTH_SIMSETTINGS_FLAG_TEARING as i32;
pub const CLOTH_SIMSETTINGS_FLAG_PRESSURE: i32 = CLOTH_SIMSETTINGS_FLAGS::CLOTH_SIMSETTINGS_FLAG_PRESSURE as i32;
pub const CLOTH_SIMSETTINGS_FLAG_PRESSURE_VOL: i32 = CLOTH_SIMSETTINGS_FLAGS::CLOTH_SIMSETTINGS_FLAG_PRESSURE_VOL as i32;
pub const CLOTH_SIMSETTINGS_FLAG_INTERNAL_SPRINGS: i32 = CLOTH_SIMSETTINGS_FLAGS::CLOTH_SIMSETTINGS_FLAG_INTERNAL_SPRINGS as i32;
pub const CLOTH_SIMSETTINGS_FLAG_SCALING: i32 = CLOTH_SIMSETTINGS_FLAGS::CLOTH_SIMSETTINGS_FLAG_SCALING as i32;
pub const CLOTH_SIMSETTINGS_FLAG_INTERNAL_SPRINGS_NORMAL: i32 = CLOTH_SIMSETTINGS_FLAGS::CLOTH_SIMSETTINGS_FLAG_INTERNAL_SPRINGS_NORMAL as i32;
pub const CLOTH_SIMSETTINGS_FLAG_RESIST_SPRING_COMPRESS: i32 = CLOTH_SIMSETTINGS_FLAGS::CLOTH_SIMSETTINGS_FLAG_RESIST_SPRING_COMPRESS as i32;
pub const CLOTH_SIMSETTINGS_FLAG_SEW: i32 = CLOTH_SIMSETTINGS_FLAGS::CLOTH_SIMSETTINGS_FLAG_SEW as i32;
pub const CLOTH_SIMSETTINGS_FLAG_DYNAMIC_BASEMESH: i32 = CLOTH_SIMSETTINGS_FLAGS::CLOTH_SIMSETTINGS_FLAG_DYNAMIC_BASEMESH as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLOTH_BENDING_MODEL {
    CLOTH_BENDING_LINEAR = 0,
    CLOTH_BENDING_ANGULAR = 1,
}

impl Default for CLOTH_BENDING_MODEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CLOTH_BENDING_LINEAR: i32 = CLOTH_BENDING_MODEL::CLOTH_BENDING_LINEAR as i32;
pub const CLOTH_BENDING_ANGULAR: i32 = CLOTH_BENDING_MODEL::CLOTH_BENDING_ANGULAR as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLOTH_COLLISIONSETTINGS_FLAGS {
    CLOTH_COLLSETTINGS_FLAG_ENABLED = (1 << 1),
    CLOTH_COLLSETTINGS_FLAG_SELF = (1 << 2),
}

impl Default for CLOTH_COLLISIONSETTINGS_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CLOTH_COLLSETTINGS_FLAG_ENABLED: i32 = CLOTH_COLLISIONSETTINGS_FLAGS::CLOTH_COLLSETTINGS_FLAG_ENABLED as i32;
pub const CLOTH_COLLSETTINGS_FLAG_SELF: i32 = CLOTH_COLLISIONSETTINGS_FLAGS::CLOTH_COLLSETTINGS_FLAG_SELF as i32;

