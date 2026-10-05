//! Auto-transpiled C/C++ header module: DNA_rigidbody_types

use core::ffi::c_void;
use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RigidBodyWorld_Shared {
    pub pointcache: *mut PointCache,
    pub ptcaches: ListBaseT<PointCache>,
    pub runtime: *mut RigidBodyWorld_Runtime,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RigidBodyWorld {
    pub effector_weights: *mut EffectorWeights,
    pub group: *mut Collection,
    pub objects: *mut *mut Object,
    pub constraints: *mut Collection,
    pub _pad: [i8; 4],
    pub ltime: f32,
    pub shared: *mut RigidBodyWorld_Shared,
    pub pointcache: *mut PointCache,
    pub ptcaches: ListBaseT<PointCache>,
    pub numbodies: i32,
    pub substeps_per_frame: i16,
    pub num_solver_iterations: i16,
    pub flag: eRigidBodyWorld_Flag,
    pub time_scale: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RigidBodyOb_Shared {
    pub physics_object: *mut core::ffi::c_void,
    pub physics_shape: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RigidBodyOb {
    pub r#type: eRigidBodyOb_Type,
    pub shape: eRigidBody_Shape,
    pub flag: eRigidBodyOb_Flag,
    pub col_groups: i32,
    pub mesh_source: eRigidBody_MeshSource,
    pub _pad: [i8; 2],
    pub mass: f32,
    pub friction: f32,
    pub restitution: f32,
    pub margin: f32,
    pub lin_damping: f32,
    pub ang_damping: f32,
    pub lin_sleep_thresh: f32,
    pub ang_sleep_thresh: f32,
    pub orn: [f32; 4],
    pub pos: [f32; 3],
    pub _pad1: [i8; 4],
    pub shared: *mut RigidBodyOb_Shared,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RigidBodyCon {
    pub ob1: *mut Object,
    pub ob2: *mut Object,
    pub r#type: eRigidBodyCon_Type,
    pub num_solver_iterations: i16,
    pub flag: eRigidBodyCon_Flag,
    pub breaking_threshold: f32,
    pub spring_type: eRigidBodyCon_SpringType,
    pub _pad: [i8; 3],
    pub limit_lin_x_lower: f32,
    pub limit_lin_x_upper: f32,
    pub limit_lin_y_lower: f32,
    pub limit_lin_y_upper: f32,
    pub limit_lin_z_lower: f32,
    pub limit_lin_z_upper: f32,
    pub limit_ang_x_lower: f32,
    pub limit_ang_x_upper: f32,
    pub limit_ang_y_lower: f32,
    pub limit_ang_y_upper: f32,
    pub limit_ang_z_lower: f32,
    pub limit_ang_z_upper: f32,
    pub spring_stiffness_x: f32,
    pub spring_stiffness_y: f32,
    pub spring_stiffness_z: f32,
    pub spring_stiffness_ang_x: f32,
    pub spring_stiffness_ang_y: f32,
    pub spring_stiffness_ang_z: f32,
    pub spring_damping_x: f32,
    pub spring_damping_y: f32,
    pub spring_damping_z: f32,
    pub spring_damping_ang_x: f32,
    pub spring_damping_ang_y: f32,
    pub spring_damping_ang_z: f32,
    pub motor_lin_target_velocity: f32,
    pub motor_ang_target_velocity: f32,
    pub motor_lin_max_impulse: f32,
    pub motor_ang_max_impulse: f32,
    pub physics_constraint: *mut core::ffi::c_void,
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
pub struct RigidBodyWorld_Runtime {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PointCache {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Object {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct so {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eRigidBodyWorld_Flag {
    RBW_FLAG_MUTED = (1 << 0),
    RBW_FLAG_USE_SPLIT_IMPULSE = (1 << 2),
}

impl Default for eRigidBodyWorld_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const RBW_FLAG_MUTED: i32 = eRigidBodyWorld_Flag::RBW_FLAG_MUTED as i32;
pub const RBW_FLAG_USE_SPLIT_IMPULSE: i32 = eRigidBodyWorld_Flag::RBW_FLAG_USE_SPLIT_IMPULSE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eRigidBodyOb_Type {
    RBO_TYPE_ACTIVE = 0,
    RBO_TYPE_PASSIVE = 1,
}

impl Default for eRigidBodyOb_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const RBO_TYPE_ACTIVE: i32 = eRigidBodyOb_Type::RBO_TYPE_ACTIVE as i32;
pub const RBO_TYPE_PASSIVE: i32 = eRigidBodyOb_Type::RBO_TYPE_PASSIVE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eRigidBodyOb_Flag {
    RBO_FLAG_KINEMATIC = (1 << 0),
    RBO_FLAG_NEEDS_VALIDATE = (1 << 1),
    RBO_FLAG_NEEDS_RESHAPE = (1 << 2),
    RBO_FLAG_USE_DEACTIVATION = (1 << 3),
    RBO_FLAG_START_DEACTIVATED = (1 << 4),
    RBO_FLAG_DISABLED = (1 << 5),
    RBO_FLAG_USE_MARGIN = (1 << 6),
    RBO_FLAG_USE_DEFORM = (1 << 7),
}

impl Default for eRigidBodyOb_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const RBO_FLAG_KINEMATIC: i32 = eRigidBodyOb_Flag::RBO_FLAG_KINEMATIC as i32;
pub const RBO_FLAG_NEEDS_VALIDATE: i32 = eRigidBodyOb_Flag::RBO_FLAG_NEEDS_VALIDATE as i32;
pub const RBO_FLAG_NEEDS_RESHAPE: i32 = eRigidBodyOb_Flag::RBO_FLAG_NEEDS_RESHAPE as i32;
pub const RBO_FLAG_USE_DEACTIVATION: i32 = eRigidBodyOb_Flag::RBO_FLAG_USE_DEACTIVATION as i32;
pub const RBO_FLAG_START_DEACTIVATED: i32 = eRigidBodyOb_Flag::RBO_FLAG_START_DEACTIVATED as i32;
pub const RBO_FLAG_DISABLED: i32 = eRigidBodyOb_Flag::RBO_FLAG_DISABLED as i32;
pub const RBO_FLAG_USE_MARGIN: i32 = eRigidBodyOb_Flag::RBO_FLAG_USE_MARGIN as i32;
pub const RBO_FLAG_USE_DEFORM: i32 = eRigidBodyOb_Flag::RBO_FLAG_USE_DEFORM as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eRigidBody_Shape {
    RB_SHAPE_BOX = 0,
    RB_SHAPE_SPHERE = 1,
    RB_SHAPE_CAPSULE = 2,
    RB_SHAPE_CYLINDER = 3,
    RB_SHAPE_CONE = 4,
    RB_SHAPE_CONVEXH = 5,
    RB_SHAPE_TRIMESH = 6,
    RB_SHAPE_COMPOUND = 7,
}

impl Default for eRigidBody_Shape {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const RB_SHAPE_BOX: i32 = eRigidBody_Shape::RB_SHAPE_BOX as i32;
pub const RB_SHAPE_SPHERE: i32 = eRigidBody_Shape::RB_SHAPE_SPHERE as i32;
pub const RB_SHAPE_CAPSULE: i32 = eRigidBody_Shape::RB_SHAPE_CAPSULE as i32;
pub const RB_SHAPE_CYLINDER: i32 = eRigidBody_Shape::RB_SHAPE_CYLINDER as i32;
pub const RB_SHAPE_CONE: i32 = eRigidBody_Shape::RB_SHAPE_CONE as i32;
pub const RB_SHAPE_CONVEXH: i32 = eRigidBody_Shape::RB_SHAPE_CONVEXH as i32;
pub const RB_SHAPE_TRIMESH: i32 = eRigidBody_Shape::RB_SHAPE_TRIMESH as i32;
pub const RB_SHAPE_COMPOUND: i32 = eRigidBody_Shape::RB_SHAPE_COMPOUND as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eRigidBody_MeshSource {
    RBO_MESH_BASE = 0,
    RBO_MESH_DEFORM = 1,
    RBO_MESH_FINAL = 2,
}

impl Default for eRigidBody_MeshSource {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const RBO_MESH_BASE: i32 = eRigidBody_MeshSource::RBO_MESH_BASE as i32;
pub const RBO_MESH_DEFORM: i32 = eRigidBody_MeshSource::RBO_MESH_DEFORM as i32;
pub const RBO_MESH_FINAL: i32 = eRigidBody_MeshSource::RBO_MESH_FINAL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eRigidBodyCon_Type {
    RBC_TYPE_POINT = 0,
    RBC_TYPE_HINGE = 1,
    RBC_TYPE_SLIDER = 3,
    RBC_TYPE_6DOF = 5,
    RBC_TYPE_6DOF_SPRING = 6,
    RBC_TYPE_FIXED = 8,
    RBC_TYPE_PISTON = 9,
    RBC_TYPE_MOTOR = 11,
}

impl Default for eRigidBodyCon_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const RBC_TYPE_POINT: i32 = eRigidBodyCon_Type::RBC_TYPE_POINT as i32;
pub const RBC_TYPE_HINGE: i32 = eRigidBodyCon_Type::RBC_TYPE_HINGE as i32;
pub const RBC_TYPE_SLIDER: i32 = eRigidBodyCon_Type::RBC_TYPE_SLIDER as i32;
pub const RBC_TYPE_6DOF: i32 = eRigidBodyCon_Type::RBC_TYPE_6DOF as i32;
pub const RBC_TYPE_6DOF_SPRING: i32 = eRigidBodyCon_Type::RBC_TYPE_6DOF_SPRING as i32;
pub const RBC_TYPE_FIXED: i32 = eRigidBodyCon_Type::RBC_TYPE_FIXED as i32;
pub const RBC_TYPE_PISTON: i32 = eRigidBodyCon_Type::RBC_TYPE_PISTON as i32;
pub const RBC_TYPE_MOTOR: i32 = eRigidBodyCon_Type::RBC_TYPE_MOTOR as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eRigidBodyCon_SpringType {
    RBC_SPRING_TYPE1 = 0,
    RBC_SPRING_TYPE2 = 1,
}

impl Default for eRigidBodyCon_SpringType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const RBC_SPRING_TYPE1: i32 = eRigidBodyCon_SpringType::RBC_SPRING_TYPE1 as i32;
pub const RBC_SPRING_TYPE2: i32 = eRigidBodyCon_SpringType::RBC_SPRING_TYPE2 as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eRigidBodyCon_Flag {
    RBC_FLAG_ENABLED = (1 << 0),
    RBC_FLAG_NEEDS_VALIDATE = (1 << 1),
    RBC_FLAG_DISABLE_COLLISIONS = (1 << 2),
    RBC_FLAG_USE_BREAKING = (1 << 3),
    RBC_FLAG_OVERRIDE_SOLVER_ITERATIONS = (1 << 4),
    RBC_FLAG_USE_LIMIT_LIN_X = (1 << 5),
    RBC_FLAG_USE_LIMIT_LIN_Y = (1 << 6),
    RBC_FLAG_USE_LIMIT_LIN_Z = (1 << 7),
    RBC_FLAG_USE_LIMIT_ANG_X = (1 << 8),
    RBC_FLAG_USE_LIMIT_ANG_Y = (1 << 9),
    RBC_FLAG_USE_LIMIT_ANG_Z = (1 << 10),
    RBC_FLAG_USE_SPRING_X = (1 << 11),
    RBC_FLAG_USE_SPRING_Y = (1 << 12),
    RBC_FLAG_USE_SPRING_Z = (1 << 13),
    RBC_FLAG_USE_MOTOR_LIN = (1 << 14),
    RBC_FLAG_USE_MOTOR_ANG = (1 << 15),
    RBC_FLAG_USE_SPRING_ANG_X = (1 << 16),
    RBC_FLAG_USE_SPRING_ANG_Y = (1 << 17),
    RBC_FLAG_USE_SPRING_ANG_Z = (1 << 18),
}

impl Default for eRigidBodyCon_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const RBC_FLAG_ENABLED: i32 = eRigidBodyCon_Flag::RBC_FLAG_ENABLED as i32;
pub const RBC_FLAG_NEEDS_VALIDATE: i32 = eRigidBodyCon_Flag::RBC_FLAG_NEEDS_VALIDATE as i32;
pub const RBC_FLAG_DISABLE_COLLISIONS: i32 = eRigidBodyCon_Flag::RBC_FLAG_DISABLE_COLLISIONS as i32;
pub const RBC_FLAG_USE_BREAKING: i32 = eRigidBodyCon_Flag::RBC_FLAG_USE_BREAKING as i32;
pub const RBC_FLAG_OVERRIDE_SOLVER_ITERATIONS: i32 = eRigidBodyCon_Flag::RBC_FLAG_OVERRIDE_SOLVER_ITERATIONS as i32;
pub const RBC_FLAG_USE_LIMIT_LIN_X: i32 = eRigidBodyCon_Flag::RBC_FLAG_USE_LIMIT_LIN_X as i32;
pub const RBC_FLAG_USE_LIMIT_LIN_Y: i32 = eRigidBodyCon_Flag::RBC_FLAG_USE_LIMIT_LIN_Y as i32;
pub const RBC_FLAG_USE_LIMIT_LIN_Z: i32 = eRigidBodyCon_Flag::RBC_FLAG_USE_LIMIT_LIN_Z as i32;
pub const RBC_FLAG_USE_LIMIT_ANG_X: i32 = eRigidBodyCon_Flag::RBC_FLAG_USE_LIMIT_ANG_X as i32;
pub const RBC_FLAG_USE_LIMIT_ANG_Y: i32 = eRigidBodyCon_Flag::RBC_FLAG_USE_LIMIT_ANG_Y as i32;
pub const RBC_FLAG_USE_LIMIT_ANG_Z: i32 = eRigidBodyCon_Flag::RBC_FLAG_USE_LIMIT_ANG_Z as i32;
pub const RBC_FLAG_USE_SPRING_X: i32 = eRigidBodyCon_Flag::RBC_FLAG_USE_SPRING_X as i32;
pub const RBC_FLAG_USE_SPRING_Y: i32 = eRigidBodyCon_Flag::RBC_FLAG_USE_SPRING_Y as i32;
pub const RBC_FLAG_USE_SPRING_Z: i32 = eRigidBodyCon_Flag::RBC_FLAG_USE_SPRING_Z as i32;
pub const RBC_FLAG_USE_MOTOR_LIN: i32 = eRigidBodyCon_Flag::RBC_FLAG_USE_MOTOR_LIN as i32;
pub const RBC_FLAG_USE_MOTOR_ANG: i32 = eRigidBodyCon_Flag::RBC_FLAG_USE_MOTOR_ANG as i32;
pub const RBC_FLAG_USE_SPRING_ANG_X: i32 = eRigidBodyCon_Flag::RBC_FLAG_USE_SPRING_ANG_X as i32;
pub const RBC_FLAG_USE_SPRING_ANG_Y: i32 = eRigidBodyCon_Flag::RBC_FLAG_USE_SPRING_ANG_Y as i32;
pub const RBC_FLAG_USE_SPRING_ANG_Z: i32 = eRigidBodyCon_Flag::RBC_FLAG_USE_SPRING_ANG_Z as i32;

