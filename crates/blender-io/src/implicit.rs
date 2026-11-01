/* SPDX-FileCopyrightText: Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use crate::SIM_mass_spring::ImplicitData;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImplicitSolverResult {
    pub status: i32,
    pub iterations: i32,
    pub error: f32,
}

#[repr(C)]
pub struct HairGrid {
    _unused: [u8; 0],
}

pub const MAX_HAIR_GRID_RES: usize = 256;

extern "C" {
    pub fn SIM_mass_spring_set_vertex_mass(data: *mut ImplicitData, index: i32, mass: f32);
    pub fn SIM_mass_spring_set_rest_transform(
        data: *mut ImplicitData,
        index: i32,
        tfm: *const [[f32; 3]; 3],
    );
    pub fn SIM_mass_spring_set_motion_state(
        data: *mut ImplicitData,
        index: i32,
        x: *const [f32; 3],
        v: *const [f32; 3],
    );
    pub fn SIM_mass_spring_set_position(data: *mut ImplicitData, index: i32, x: *const [f32; 3]);
    pub fn SIM_mass_spring_set_velocity(data: *mut ImplicitData, index: i32, v: *const [f32; 3]);
    pub fn SIM_mass_spring_get_motion_state(
        data: *mut ImplicitData,
        index: i32,
        x: *mut [f32; 3],
        v: *mut [f32; 3],
    );
    pub fn SIM_mass_spring_get_position(data: *mut ImplicitData, index: i32, x: *mut [f32; 3]);
    pub fn SIM_mass_spring_get_velocity(data: *mut ImplicitData, index: i32, v: *mut [f32; 3]);

    pub fn SIM_mass_spring_solve_velocities(
        data: *mut ImplicitData,
        dt: f32,
        result: *mut ImplicitSolverResult,
    ) -> bool;
    pub fn SIM_mass_spring_solve_positions(data: *mut ImplicitData, dt: f32) -> bool;
    pub fn SIM_mass_spring_apply_result(data: *mut ImplicitData);

    pub fn SIM_hair_volume_create_vertex_grid(
        cellsize: f32,
        gmin: *const [f32; 3],
        gmax: *const [f32; 3],
    ) -> *mut HairGrid;
    pub fn SIM_hair_volume_free_vertex_grid(grid: *mut HairGrid);
    pub fn SIM_hair_volume_grid_clear(grid: *mut HairGrid);
}
