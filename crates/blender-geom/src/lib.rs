//! Hand-ported geometry kernels from Blender's `BLI_math_geom.h` and `BLI_bounds`.
//!
//! This crate is deliberately independent of the auto-transpiled crates so it can be
//! developed and tested while the port-runner is rewriting them.

pub mod bounds;
pub mod isect;
pub mod kdtree;
pub mod line;
pub mod plane;
pub mod poly;
pub mod tri;

pub use glam::{Vec2, Vec3, Vec4};

/// Tolerance used for "parallel / degenerate" tests, matching Blender's `FLT_EPSILON` use.
pub const EPSILON: f32 = f32::EPSILON;
