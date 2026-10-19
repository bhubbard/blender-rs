//! Auto-transpiled C/C++ header module: GEO_xpbd_constraint_set

#[derive(Debug, Clone, Default)]
pub struct ConstraintSet {
    pub constraints_num: i32,
    pub affected_geo_indices: Vec<i32>,
}

#[derive(Debug, Clone, Default)]
pub struct VelocityConstraintSet {
    pub affected_geo_indices: Vec<i32>,
}
