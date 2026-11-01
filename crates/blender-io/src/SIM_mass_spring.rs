/* SPDX-FileCopyrightText: Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use blender_dna::{ClothModifierData, ListBaseT, Object};
use crate::DEG_depsgraph::Depsgraph;

pub const SIM_SOLVER_SUCCESS: i32 = 1 << 0;
pub const SIM_SOLVER_NUMERICAL_ISSUE: i32 = 1 << 1;
pub const SIM_SOLVER_NO_CONVERGENCE: i32 = 1 << 2;
pub const SIM_SOLVER_INVALID_INPUT: i32 = 1 << 3;

#[repr(C)]
pub struct ImplicitData {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct EffectorCache {
    _unused: [u8; 0],
}

extern "C" {
    pub fn SIM_mass_spring_solver_create(numverts: i32, numsprings: i32) -> *mut ImplicitData;
    pub fn SIM_mass_spring_solver_free(id: *mut ImplicitData);
    pub fn SIM_mass_spring_solver_numvert(id: *mut ImplicitData) -> i32;

    pub fn SIM_cloth_solver_init(ob: *mut Object, clmd: *mut ClothModifierData) -> i32;
    pub fn SIM_mass_spring_set_implicit_vertex_mass(
        data: *mut ImplicitData,
        index: i32,
        mass: f32,
    );

    pub fn SIM_cloth_solver_free(clmd: *mut ClothModifierData);
    pub fn SIM_cloth_solve(
        depsgraph: *mut Depsgraph,
        ob: *mut Object,
        frame: f32,
        clmd: *mut ClothModifierData,
        effectors: *mut ListBaseT<EffectorCache>,
    ) -> i32;
    pub fn SIM_cloth_solver_set_positions(clmd: *mut ClothModifierData);
    pub fn SIM_cloth_solver_set_volume(clmd: *mut ClothModifierData);
}
