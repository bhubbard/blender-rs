//! Auto-transpiled C/C++ header module: GEO_xpbd_geometry_ref
//! Source: upstream/source/blender/geometry/xpbd/GEO_xpbd_geometry_ref.hh

use blender_math::{Vec3, Quat};

pub type Quaternion = Quat;

/// References to the data of a geometry that is being simulated.
#[derive(Debug, Clone, Default)]
pub struct GeometryRef {
    pub positions: Vec<Vec3>,
    pub velocities: Vec<Vec3>,
    pub prev_positions: Vec<Vec3>,
    pub inv_masses: Vec<f32>,
    pub rotations: Vec<Quat>,
    pub angular_velocities: Vec<Vec3>,
    pub prev_rotations: Vec<Quat>,
    pub moments_of_inertia: Vec<Vec3>,
    pub inv_moments_of_inertia: Vec<Vec3>,
}

impl GeometryRef {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(&self) -> usize {
        self.positions.len()
    }
}
