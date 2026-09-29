//! Auto-transpiled C/C++ header module: bmesh_operators

use crate::*;

pub const SUBD_CORNER_INNERVERT: i32 = 0;
pub const SUBD_CORNER_PATH: i32 = 1;
pub const SUBD_CORNER_FAN: i32 = 2;
pub const SUBD_CORNER_STRAIGHT_CUT: i32 = 3;

pub const SUBD_FALLOFF_SMOOTH: i32 = 0;
pub const SUBD_FALLOFF_SPHERE: i32 = 0;
pub const SUBD_FALLOFF_ROOT: i32 = 1;
pub const SUBD_FALLOFF_SHARP: i32 = 2;
pub const SUBD_FALLOFF_LIN: i32 = 3;
pub const SUBD_FALLOFF_INVSQUARE: i32 = 7;

pub const SUBDIV_SELECT_NONE: i32 = 0;
pub const SUBDIV_SELECT_ORIG: i32 = 1;
pub const SUBDIV_SELECT_INNER: i32 = 2;
pub const SUBDIV_SELECT_LOOPCUT: i32 = 3;

pub const SUBD_RING_INTERP_LINEAR: i32 = 0;
pub const SUBD_RING_INTERP_PATH: i32 = 1;
pub const SUBD_RING_INTERP_SURF: i32 = 2;

pub const SIMFACE_MATERIAL: i32 = 201;
pub const SIMFACE_AREA: i32 = 0;
pub const SIMFACE_SIDES: i32 = 1;
pub const SIMFACE_PERIMETER: i32 = 2;
pub const SIMFACE_NORMAL: i32 = 3;
pub const SIMFACE_COPLANAR: i32 = 4;
pub const SIMFACE_SMOOTH: i32 = 5;
pub const SIMFACE_FREESTYLE: i32 = 6;

pub const SIMEDGE_LENGTH: i32 = 101;
pub const SIMEDGE_DIR: i32 = 0;
pub const SIMEDGE_FACE: i32 = 1;
pub const SIMEDGE_FACE_ANGLE: i32 = 2;
pub const SIMEDGE_CREASE: i32 = 3;
pub const SIMEDGE_BEVEL: i32 = 4;
pub const SIMEDGE_SEAM: i32 = 5;
pub const SIMEDGE_SHARP: i32 = 6;
pub const SIMEDGE_FREESTYLE: i32 = 7;

pub const SIMVERT_NORMAL: i32 = 0;
pub const SIMVERT_FACE: i32 = 0;
pub const SIMVERT_VGROUP: i32 = 1;
pub const SIMVERT_EDGE: i32 = 2;
pub const SIMVERT_CREASE: i32 = 3;

pub const BMOP_POKE_MEDIAN_WEIGHTED: i32 = 0;
pub const BMOP_POKE_MEDIAN: i32 = 0;
pub const BMOP_POKE_BOUNDS: i32 = 1;

pub const BEVEL_AMT_OFFSET: i32 = 0;
pub const BEVEL_AMT_WIDTH: i32 = 1;
pub const BEVEL_AMT_DEPTH: i32 = 2;
pub const BEVEL_AMT_PERCENT: i32 = 3;
pub const BEVEL_AMT_ABSOLUTE: i32 = 4;

pub const BEVEL_PROFILE_SUPERELLIPSE: i32 = 0;
pub const BEVEL_PROFILE_CUSTOM: i32 = 1;

pub const BEVEL_FACE_STRENGTH_NONE: i32 = 0;
pub const BEVEL_FACE_STRENGTH_NEW: i32 = 1;
pub const BEVEL_FACE_STRENGTH_AFFECTED: i32 = 2;
pub const BEVEL_FACE_STRENGTH_ALL: i32 = 3;

pub const BEVEL_MITER_SHARP: i32 = 0;
pub const BEVEL_MITER_PATCH: i32 = 1;
pub const BEVEL_MITER_ARC: i32 = 2;

pub const BEVEL_VMESH_ADJ: i32 = 0;
pub const BEVEL_VMESH_CUTOFF: i32 = 1;

pub const BEVEL_AFFECT_VERTICES: i32 = 0;
pub const BEVEL_AFFECT_EDGES: i32 = 1;

pub const FACE_STRENGTH_WEAK: i32 = -16384;
pub const FACE_STRENGTH_MEDIUM: i32 = 0;
pub const FACE_STRENGTH_STRONG: i32 = 16384;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpaceInterpolationMethod {
    SPACE_EDGE_LOOPS_EVENLY_INTERP_CUBIC = 0,
    SPACE_EDGE_LOOPS_EVENLY_INTERP_LINEAR = 1,
}

impl Default for SpaceInterpolationMethod {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelaxInterpolationMethod {
    RELAX_EDGE_LOOPS_INTERP_CUBIC = 0,
    RELAX_EDGE_LOOPS_INTERP_LINEAR = 1,
}

impl Default for RelaxInterpolationMethod {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlattenMethod {
    FLATTEN_BEST_FIT = 0,
    FLATTEN_NORMAL = 1,
    FLATTEN_VIEW = 2,
}

impl Default for FlattenMethod {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuadSphereMethod {
    QUADSPHERE_METHOD_EQUI_ANGULAR_EVEN_AREA = 0,
    QUADSPHERE_METHOD_EQUI_ANGULAR = 1,
}

impl Default for QuadSphereMethod {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
