//! Auto-transpiled C/C++ header module: GEO_mesh_boolean
//! Source: upstream/source/blender/geometry/GEO_mesh_boolean.hh

use blender_dna::DNA_mesh_types::Mesh;
use blender_math::Mat4;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Solver {
    MeshArr = 0,
    Float = 1,
    Manifold = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Intersect = 0,
    Union = 1,
    Difference = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BooleanErrorType {
    NoError = 0,
    NonManifold = 1,
    ResultTooBig = 2,
    SolverNotAvailable = 3,
    UnknownError = 4,
}

#[derive(Debug, Clone, Default)]
pub struct BooleanError {
    pub error_type: Option<BooleanErrorType>,
    pub non_manifold_mesh_indices: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct BooleanOpParameters {
    pub boolean_mode: Operation,
    pub no_self_intersections: bool,
    pub no_nested_components: bool,
    pub watertight: bool,
}

impl Default for BooleanOpParameters {
    fn default() -> Self {
        Self {
            boolean_mode: Operation::Difference,
            no_self_intersections: true,
            no_nested_components: true,
            watertight: true,
        }
    }
}

extern "C" {
    pub fn mesh_boolean(
        meshes: *const *const Mesh,
        meshes_len: usize,
        transforms: *const Mat4,
        transforms_len: usize,
        material_remaps: *const i16,
        material_remaps_len: usize,
        op_params: BooleanOpParameters,
        solver: Solver,
        r_intersecting_edges: *mut Vec<i32>,
        r_error: *mut BooleanError,
    ) -> *mut Mesh;
}
