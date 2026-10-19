//! Auto-transpiled C/C++ header module: GEO_xpbd_updater_velocity
//! Source: upstream/source/blender/geometry/xpbd/GEO_xpbd_updater_velocity.hh

use blender_math::Vec3;
use crate::GEO_xpbd_geometry_ref::GeometryRef;

/// Updater that writes the changes directly to the simulated points.
pub struct VelocityUpdater<'a> {
    pub geometry_refs: &'a mut [GeometryRef],
}

impl<'a> VelocityUpdater<'a> {
    pub fn new(geometry_refs: &'a mut [GeometryRef]) -> Self {
        Self { geometry_refs }
    }

    pub fn update_velocity(&mut self, geo_i: usize, point_i: usize, offset: Vec3) {
        self.geometry_refs[geo_i].velocities[point_i] += offset;
    }

    pub fn update_angular_velocity(&mut self, geo_i: usize, point_i: usize, offset: Vec3) {
        self.geometry_refs[geo_i].angular_velocities[point_i] += offset;
    }
}
