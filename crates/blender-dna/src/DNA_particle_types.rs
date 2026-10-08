//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[allow(non_camel_case_types)]
type int32_t = i32;
#[allow(non_camel_case_types)]
type uint32_t = u32;
#[allow(non_camel_case_types)]
type int16_t = i16;
#[allow(non_camel_case_types)]
type uint16_t = u16;
#[allow(non_camel_case_types)]
type int64_t = i64;
#[allow(non_camel_case_types)]
type uint64_t = u64;
#[allow(non_camel_case_types)]
type int8_t = i8;
#[allow(non_camel_case_types)]
type uint8_t = u8;
#[allow(non_camel_case_types)]
type uchar = u8;
#[allow(non_camel_case_types)]
type ushort = u16;
#[allow(non_camel_case_types)]
type uint = u32;
#[allow(non_camel_case_types)]
type ulong = u64;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSPH_Flag(pub i32);

impl eSPH_Flag {
    pub const SPH_VISCOELASTIC_SPRINGS: Self = Self((1 << 0) as i32);
    pub const SPH_CURRENT_REST_LENGTH: Self = Self((1 << 1) as i32);
    pub const SPH_FAC_REPULSION: Self = Self((1 << 2) as i32);
    pub const SPH_FAC_DENSITY: Self = Self((1 << 3) as i32);
    pub const SPH_FAC_RADIUS: Self = Self((1 << 4) as i32);
    pub const SPH_FAC_VISCOSITY: Self = Self((1 << 5) as i32);
    pub const SPH_FAC_REST_LENGTH: Self = Self((1 << 6) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSPH_Solver(pub i16);

impl eSPH_Solver {
    pub const SPH_SOLVER_DDR: Self = Self((0) as i16);
    pub const SPH_SOLVER_CLASSICAL: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleDrawFlag(pub i32);

impl eParticleDrawFlag {
    pub const PART_DRAW_VEL: Self = Self(((1 << 0)) as i32);
    pub const PART_DRAW_GLOBAL_OB: Self = Self(((1 << 1)) as i32);
    pub const PART_DRAW_SIZE: Self = Self(((1 << 2)) as i32);
    pub const PART_DRAW_EMITTER: Self = Self(((1 << 3)) as i32);
    pub const PART_DRAW_HEALTH: Self = Self(((1 << 4)) as i32);
    pub const PART_ABS_PATH_TIME: Self = Self(((1 << 5)) as i32);
    pub const PART_DRAW_COUNT_GR: Self = Self(((1 << 6)) as i32);
    pub const PART_DRAW_ROTATE_OB: Self = Self(((1 << 7)) as i32);
    pub const PART_DRAW_PARENT: Self = Self(((1 << 8)) as i32);
    pub const PART_DRAW_NUM: Self = Self(((1 << 9)) as i32);
    pub const PART_DRAW_RAND_GR: Self = Self(((1 << 10)) as i32);
    pub const PART_DRAW_REN_ADAPT: Self = Self(((1 << 11)) as i32);
    pub const PART_DRAW_VEL_LENGTH: Self = Self(((1 << 12)) as i32);
    pub const PART_DRAW_MAT_COL: Self = Self(((1 << 13)) as i32);
    pub const PART_DRAW_WHOLE_GR: Self = Self(((1 << 14)) as i32);
    pub const PART_DRAW_REN_STRAND: Self = Self(((1 << 15)) as i32);
    pub const PART_DRAW_NO_SCALE_OB: Self = Self(((1 << 16)) as i32);
    pub const PART_DRAW_GUIDE_HAIRS: Self = Self(((1 << 17)) as i32);
    pub const PART_DRAW_HAIR_GRID: Self = Self(((1 << 18)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleType(pub i16);

impl eParticleType {
    pub const PART_EMITTER: Self = Self((0) as i16);
    pub const PART_HAIR: Self = Self((2) as i16);
    pub const PART_FLUID: Self = Self((3) as i16);
    pub const PART_FLUID_FLIP: Self = Self((4) as i16);
    pub const PART_FLUID_SPRAY: Self = Self((5) as i16);
    pub const PART_FLUID_BUBBLE: Self = Self((6) as i16);
    pub const PART_FLUID_FOAM: Self = Self((7) as i16);
    pub const PART_FLUID_TRACER: Self = Self((8) as i16);
    pub const PART_FLUID_SPRAYFOAM: Self = Self((9) as i16);
    pub const PART_FLUID_SPRAYBUBBLE: Self = Self((10) as i16);
    pub const PART_FLUID_FOAMBUBBLE: Self = Self((11) as i16);
    pub const PART_FLUID_SPRAYFOAMBUBBLE: Self = Self((12) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMantaflowParticleType(pub i32);

impl eMantaflowParticleType {
    pub const PARTICLE_TYPE_SPRAY: Self = Self(((1 << 1)) as i32);
    pub const PARTICLE_TYPE_BUBBLE: Self = Self(((1 << 2)) as i32);
    pub const PARTICLE_TYPE_FOAM: Self = Self(((1 << 3)) as i32);
    pub const PARTICLE_TYPE_TRACER: Self = Self(((1 << 4)) as i32);
    pub const PARTICLE_TYPE_DELETE: Self = Self(((1 << 10)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleFlag(pub i32);

impl eParticleFlag {
    pub const PART_REACT_STA_END: Self = Self((1 << 0) as i32);
    pub const PART_REACT_MULTIPLE: Self = Self((1 << 1) as i32);
    pub const PART_DS_EXPAND: Self = Self((1 << 3) as i32);
    pub const PART_HAIR_REGROW: Self = Self((1 << 4) as i32);
    pub const PART_UNBORN: Self = Self((1 << 5) as i32);
    pub const PART_DIED: Self = Self((1 << 6) as i32);
    pub const PART_TRAND: Self = Self((1 << 7) as i32);
    pub const PART_EDISTR: Self = Self((1 << 8) as i32);
    pub const PART_ROTATIONS: Self = Self((1 << 9) as i32);
    pub const PART_HAIR_BSPLINE: Self = Self((1 << 10) as i32);
    pub const PART_DIE_ON_COL: Self = Self((1 << 12) as i32);
    pub const PART_SIZE_DEFL: Self = Self((1 << 13) as i32);
    pub const PART_ROT_DYN: Self = Self((1 << 14) as i32);
    pub const PART_HIDE_ADVANCED_HAIR: Self = Self((1 << 15) as i32);
    pub const PART_SIZEMASS: Self = Self((1 << 16) as i32);
    pub const PART_BOIDS_2D: Self = Self((1 << 19) as i32);
    pub const PART_SELF_EFFECT: Self = Self((1 << 22) as i32);
    pub const PART_GRID_HEXAGONAL: Self = Self((1 << 24) as i32);
    pub const PART_GRID_INVERT: Self = Self((1 << 26) as i32);
    pub const PART_CHILD_EFFECT: Self = Self((1 << 27) as i32);
    pub const PART_CHILD_LONG_HAIR: Self = Self((1 << 28) as i32);
    pub const PART_CHILD_GUIDE: Self = Self(21 as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleFrom(pub i16);

impl eParticleFrom {
    pub const PART_FROM_VERT: Self = Self((0) as i16);
    pub const PART_FROM_FACE: Self = Self((1) as i16);
    pub const PART_FROM_VOLUME: Self = Self((2) as i16);
    pub const PART_FROM_CHILD: Self = Self((4) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleDistribution(pub i16);

impl eParticleDistribution {
    pub const PART_DISTR_JIT: Self = Self((0) as i16);
    pub const PART_DISTR_RAND: Self = Self((1) as i16);
    pub const PART_DISTR_GRID: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticlePhysicsType(pub i16);

impl eParticlePhysicsType {
    pub const PART_PHYS_NO: Self = Self((0) as i16);
    pub const PART_PHYS_NEWTON: Self = Self((1) as i16);
    pub const PART_PHYS_KEYED: Self = Self((2) as i16);
    pub const PART_PHYS_BOIDS: Self = Self((3) as i16);
    pub const PART_PHYS_FLUID: Self = Self((4) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleKink(pub i16);

impl eParticleKink {
    pub const PART_KINK_NO: Self = Self((0) as i16);
    pub const PART_KINK_CURL: Self = Self((1) as i16);
    pub const PART_KINK_RADIAL: Self = Self((2) as i16);
    pub const PART_KINK_WAVE: Self = Self((3) as i16);
    pub const PART_KINK_BRAID: Self = Self((4) as i16);
    pub const PART_KINK_SPIRAL: Self = Self((5) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleChildFlag(pub i32);

impl eParticleChildFlag {
    pub const PART_CHILD_USE_CLUMP_NOISE: Self = Self(((1 << 0)) as i32);
    pub const PART_CHILD_USE_CLUMP_CURVE: Self = Self(((1 << 1)) as i32);
    pub const PART_CHILD_USE_ROUGH_CURVE: Self = Self(((1 << 2)) as i32);
    pub const PART_CHILD_USE_TWIST_CURVE: Self = Self(((1 << 3)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleShapeFlag(pub i16);

impl eParticleShapeFlag {
    pub const PART_SHAPE_CLOSE_TIP: Self = Self(((1 << 0)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleDrawCol(pub i16);

impl eParticleDrawCol {
    pub const PART_DRAW_COL_NONE: Self = Self((0) as i16);
    pub const PART_DRAW_COL_MAT: Self = Self((1) as i16);
    pub const PART_DRAW_COL_VEL: Self = Self((2) as i16);
    pub const PART_DRAW_COL_ACC: Self = Self((3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleTimeFlag(pub i16);

impl eParticleTimeFlag {
    pub const PART_TIME_AUTOSF: Self = Self((1 << 0) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleDrawAs(pub i16);

impl eParticleDrawAs {
    pub const PART_DRAW_NOT: Self = Self((0) as i16);
    pub const PART_DRAW_DOT: Self = Self((1) as i16);
    pub const PART_DRAW_HALO: Self = Self((1) as i16);
    pub const PART_DRAW_CIRC: Self = Self((2) as i16);
    pub const PART_DRAW_CROSS: Self = Self((3) as i16);
    pub const PART_DRAW_AXIS: Self = Self((4) as i16);
    pub const PART_DRAW_LINE: Self = Self((5) as i16);
    pub const PART_DRAW_PATH: Self = Self((6) as i16);
    pub const PART_DRAW_OB: Self = Self((7) as i16);
    pub const PART_DRAW_GR: Self = Self((8) as i16);
    pub const PART_DRAW_BB: Self = Self((9) as i16);
    pub const PART_DRAW_REND: Self = Self((10) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleIntegrator(pub i16);

impl eParticleIntegrator {
    pub const PART_INT_EULER: Self = Self((0) as i16);
    pub const PART_INT_MIDPOINT: Self = Self((1) as i16);
    pub const PART_INT_RK4: Self = Self((2) as i16);
    pub const PART_INT_VERLET: Self = Self((3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleRotationMode(pub i16);

impl eParticleRotationMode {
    pub const PART_ROT_NOR: Self = Self((1) as i16);
    pub const PART_ROT_VEL: Self = Self((2) as i16);
    pub const PART_ROT_GLOB_X: Self = Self((3) as i16);
    pub const PART_ROT_GLOB_Y: Self = Self((4) as i16);
    pub const PART_ROT_GLOB_Z: Self = Self((5) as i16);
    pub const PART_ROT_OB_X: Self = Self((6) as i16);
    pub const PART_ROT_OB_Y: Self = Self((7) as i16);
    pub const PART_ROT_OB_Z: Self = Self((8) as i16);
    pub const PART_ROT_NOR_TAN: Self = Self((9) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleAngularVelocityMode(pub i16);

impl eParticleAngularVelocityMode {
    pub const PART_AVE_VELOCITY: Self = Self((1) as i16);
    pub const PART_AVE_RAND: Self = Self((2) as i16);
    pub const PART_AVE_HORIZONTAL: Self = Self((3) as i16);
    pub const PART_AVE_VERTICAL: Self = Self((4) as i16);
    pub const PART_AVE_GLOBAL_X: Self = Self((5) as i16);
    pub const PART_AVE_GLOBAL_Y: Self = Self((6) as i16);
    pub const PART_AVE_GLOBAL_Z: Self = Self((7) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleReactEvent(pub i16);

impl eParticleReactEvent {
    pub const PART_EVENT_DEATH: Self = Self((0) as i16);
    pub const PART_EVENT_COLLIDE: Self = Self((1) as i16);
    pub const PART_EVENT_NEAR: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleChildType(pub i16);

impl eParticleChildType {
    pub const PART_CHILD_PARTICLES: Self = Self((1) as i16);
    pub const PART_CHILD_FACES: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleSystem_Flag(pub i32);

impl eParticleSystem_Flag {
    pub const PSYS_CURRENT: Self = Self((1 << 0) as i32);
    pub const PSYS_GLOBAL_HAIR: Self = Self((1 << 1) as i32);
    pub const PSYS_HAIR_DYNAMICS: Self = Self((1 << 2) as i32);
    pub const PSYS_KEYED_TIMING: Self = Self((1 << 3) as i32);
    pub const PSYS_HAIR_UPDATED: Self = Self((1 << 5) as i32);
    pub const PSYS_DELETE: Self = Self((1 << 8) as i32);
    pub const PSYS_HAIR_DONE: Self = Self((1 << 9) as i32);
    pub const PSYS_KEYED: Self = Self((1 << 10) as i32);
    pub const PSYS_EDITED: Self = Self((1 << 11) as i32);
    pub const PSYS_DISABLED: Self = Self((1 << 13) as i32);
    pub const PSYS_OB_ANIM_RESTORE: Self = Self((1 << 14) as i32);
    pub const PSYS_SHARED_CACHES: Self = Self((1 << 15) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticle_Flag(pub i16);

impl eParticle_Flag {
    pub const PARS_UNEXIST: Self = Self((1 << 0) as i16);
    pub const PARS_NO_DISP: Self = Self((1 << 1) as i16);
    pub const PARS_REKEY: Self = Self((1 << 3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticle_AliveState(pub i16);

impl eParticle_AliveState {
    pub const PARS_KILLED: Self = Self((0) as i16);
    pub const PARS_DEAD: Self = Self((1) as i16);
    pub const PARS_UNBORN: Self = Self((2) as i16);
    pub const PARS_ALIVE: Self = Self((3) as i16);
    pub const PARS_DYING: Self = Self((4) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleDupliWeight_Flag(pub i16);

impl eParticleDupliWeight_Flag {
    pub const PART_DUPLIW_CURRENT: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleSystem_Vgroup(pub i16);

impl eParticleSystem_Vgroup {
    pub const PSYS_VG_DENSITY: Self = Self((0) as i16);
    pub const PSYS_VG_VEL: Self = Self((1) as i16);
    pub const PSYS_VG_LENGTH: Self = Self((2) as i16);
    pub const PSYS_VG_CLUMP: Self = Self((3) as i16);
    pub const PSYS_VG_KINK: Self = Self((4) as i16);
    pub const PSYS_VG_ROUGH1: Self = Self((5) as i16);
    pub const PSYS_VG_ROUGH2: Self = Self((6) as i16);
    pub const PSYS_VG_ROUGHE: Self = Self((7) as i16);
    pub const PSYS_VG_SIZE: Self = Self((8) as i16);
    pub const PSYS_VG_TAN: Self = Self((9) as i16);
    pub const PSYS_VG_ROT: Self = Self((10) as i16);
    pub const PSYS_VG_EFFECTOR: Self = Self((11) as i16);
    pub const PSYS_VG_TWIST: Self = Self((12) as i16);
    pub const PSYS_TOT_VG: Self = Self((13) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleTarget_Flag(pub i16);

impl eParticleTarget_Flag {
    pub const PTARGET_CURRENT: Self = Self((1) as i16);
    pub const PTARGET_VALID: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleTarget_Mode(pub i16);

impl eParticleTarget_Mode {
    pub const PTARGET_MODE_NEUTRAL: Self = Self((0) as i16);
    pub const PTARGET_MODE_FRIEND: Self = Self((1) as i16);
    pub const PTARGET_MODE_ENEMY: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleTextureInfluence(pub i32);

impl eParticleTextureInfluence {
    pub const PAMAP_TIME: Self = Self(((1 << 0)) as i32);
    pub const PAMAP_LIFE: Self = Self(((1 << 1)) as i32);
    pub const PAMAP_DENS: Self = Self(((1 << 2)) as i32);
    pub const PAMAP_SIZE: Self = Self(((1 << 3)) as i32);
    pub const PAMAP_INIT: Self = Self(4 as i32);
    pub const PAMAP_IVEL: Self = Self(((1 << 5)) as i32);
    pub const PAMAP_FIELD: Self = Self(((1 << 6)) as i32);
    pub const PAMAP_GRAVITY: Self = Self(((1 << 10)) as i32);
    pub const PAMAP_DAMP: Self = Self(((1 << 11)) as i32);
    pub const PAMAP_PHYSICS: Self = Self(9 as i32);
    pub const PAMAP_CLUMP: Self = Self(((1 << 7)) as i32);
    pub const PAMAP_KINK_FREQ: Self = Self(((1 << 8)) as i32);
    pub const PAMAP_KINK_AMP: Self = Self(((1 << 12)) as i32);
    pub const PAMAP_ROUGH: Self = Self(((1 << 9)) as i32);
    pub const PAMAP_LENGTH: Self = Self(((1 << 4)) as i32);
    pub const PAMAP_TWIST: Self = Self(((1 << 13)) as i32);
    pub const PAMAP_CHILD: Self = Self(16 as i32);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct HairKey {
    pub co: [f32; 3],
    pub time: f32,
    pub weight: f32,
    pub editflag: i16,
    pub _pad: [u8; 2],
    pub world_co: [f32; 3],
}

impl Default for HairKey {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ParticleKey {
    pub co: [f32; 3],
    pub vel: [f32; 3],
    pub rot: [f32; 4],
    pub ave: [f32; 3],
    pub time: f32,
}

impl Default for ParticleKey {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BoidParticle {
    pub ground: *mut core::ffi::c_void,
    pub data: BoidData,
    pub gravity: [f32; 3],
}

impl Default for BoidParticle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ParticleSpring {
    pub rest_length: f32,
    pub particle_index: [u32; 2],
    pub delete_flag: u32,
}

impl Default for ParticleSpring {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ChildParticle {
    pub num: i32,
    pub parent: i32,
    pub pa: [i32; 4],
    pub w: [f32; 4],
    pub fuv: [f32; 4],
    pub foffset: f32,
    pub _pad0: [u8; 4],
}

impl Default for ChildParticle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ParticleTarget {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub ob: *mut core::ffi::c_void,
    pub psys: i32,
    pub flag: eParticleTarget_Flag,
}

impl Default for ParticleTarget {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ParticleDupliWeight {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub ob: *mut core::ffi::c_void,
    pub count: i16,
    pub flag: eParticleDupliWeight_Flag,
}

impl Default for ParticleDupliWeight {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SPHFluidSettings {
    pub radius: f32,
    pub spring_k: f32,
    pub rest_length: f32,
    pub plasticity_constant: f32,
    pub yield_ratio: f32,
    pub plasticity_balance: f32,
    pub yield_balance: f32,
    pub viscosity_omega: f32,
    pub viscosity_beta: f32,
    pub stiffness_k: f32,
    pub stiffness_knear: f32,
    pub rest_density: f32,
    pub buoyancy: f32,
    pub flag: eSPH_Flag,
}

impl Default for SPHFluidSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ParticleSettings {
    pub adt: *mut core::ffi::c_void,
    pub boids: *mut core::ffi::c_void,
    pub fluid: *mut core::ffi::c_void,
    pub effector_weights: *mut core::ffi::c_void,
    pub collision_group: *mut core::ffi::c_void,
    pub flag: eParticleFlag,
    pub _pad1: [u8; 4],
}

impl Default for ParticleSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ParticleSystem {
    pub part: *mut core::ffi::c_void,
    pub particles: *mut core::ffi::c_void,
    pub child: *mut core::ffi::c_void,
    pub edit: *mut core::ffi::c_void,
    pub pathcache: *mut core::ffi::c_void,
    pub childcache: *mut core::ffi::c_void,
    pub pathcachebufs: ListBaseT<LinkData>,
    pub childcachebufs: ListBaseT<LinkData>,
    pub clmd: *mut core::ffi::c_void,
    pub hair_in_mesh: *mut core::ffi::c_void,
    pub hair_out_mesh: *mut core::ffi::c_void,
    pub target_ob: *mut core::ffi::c_void,
    pub lattice_deform_data: *mut core::ffi::c_void,
    pub parent: *mut core::ffi::c_void,
    pub targets: ListBaseT<ParticleTarget>,
    pub nullptr: ListBaseT<ParticleTarget>,
}

impl Default for ParticleSystem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

