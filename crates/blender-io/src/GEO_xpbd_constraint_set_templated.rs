//! Auto-transpiled C/C++ header module: GEO_xpbd_constraint_set_templated
//! Source: upstream/source/blender/geometry/xpbd/GEO_xpbd_constraint_set_templated.hh

pub use crate::GEO_xpbd_constraint_set::{ConstraintSet, VelocityConstraintSet};
use crate::GEO_xpbd_constraint_set_params::ConstraintSetParams;
use crate::GEO_xpbd_updater_velocity::VelocityUpdater;

pub trait TemplatedConstraintSet {
    fn constraints_num(&self) -> usize;
    fn reset_forces(&mut self);
    fn debug_name(&self) -> &'static str;
}

pub trait TemplatedVelocityConstraintSet {
    fn constraints_num(&self) -> usize;
    fn reset_forces(&mut self);
    fn solve_sequential(&mut self, params: &ConstraintSetParams, updater: &mut VelocityUpdater);
    fn debug_name(&self) -> &'static str;
}
