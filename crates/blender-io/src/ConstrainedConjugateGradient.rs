/* SPDX-FileCopyrightText: Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use crate::eigen_utils::Scalar;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SolverParameters {
    pub max_iterations: usize,
    pub tolerance: Scalar,
}

impl Default for SolverParameters {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            tolerance: 1e-4,
        }
    }
}

pub trait ConstrainedLinearSolver {
    fn solve(
        &self,
        mat: &[Scalar],
        rhs: &[Scalar],
        x: &mut [Scalar],
        params: &SolverParameters,
    ) -> (usize, Scalar);
}
