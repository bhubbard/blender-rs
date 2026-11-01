/* SPDX-FileCopyrightText: 2016-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Gizmo flags, highlight states, and interaction types.

pub const WM_GIZMO_STATE_HIGHLIGHT: i32 = 1 << 0;
pub const WM_GIZMO_STATE_MODAL: i32 = 1 << 1;
pub const WM_GIZMO_STATE_SELECT: i32 = 1 << 2;

pub const WM_GIZMO_DRAW_HOVER: i32 = 1 << 0;
pub const WM_GIZMO_DRAW_MODAL: i32 = 1 << 1;
pub const WM_GIZMO_DRAW_VALUE: i32 = 1 << 2;
pub const WM_GIZMO_HIDDEN: i32 = 1 << 3;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eWM_GizmoFlagTweak {
    None = 0,
    Precise = 1 << 0,
    Snap = 1 << 1,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct wmGizmo {
    pub name: [u8; 64],
    pub flag: i32,
    pub state: i32,
    pub matrix_basis: [f32; 16],
    pub matrix_offset: [f32; 16],
    pub scale: f32,
}

impl Default for wmGizmo {
    fn default() -> Self {
        Self {
            name: [0; 64],
            flag: 0,
            state: 0,
            matrix_basis: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
            matrix_offset: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
            scale: 1.0,
        }
    }
}
