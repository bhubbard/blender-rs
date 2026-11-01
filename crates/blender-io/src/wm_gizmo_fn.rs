/* SPDX-FileCopyrightText: 2023-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Gizmo callback signatures and function prototypes.

use std::os::raw::c_void;
use blender_dna::rcti;
use crate::WM_gizmo_types::{eWM_GizmoFlagTweak, wmGizmo};

pub type wmGizmoFnSetup = Option<extern "C" fn(*mut wmGizmo)>;
pub type wmGizmoFnDraw = Option<extern "C" fn(*const c_void, *mut wmGizmo)>;
pub type wmGizmoFnTestSelect = Option<extern "C" fn(*mut c_void, *mut wmGizmo, *const i32) -> i32>;
pub type wmGizmoFnModal = Option<
    extern "C" fn(
        *mut c_void,
        *mut wmGizmo,
        *const c_void,
        eWM_GizmoFlagTweak,
    ) -> i32,
>;
pub type wmGizmoFnScreenBoundsGet =
    Option<extern "C" fn(*const c_void, *mut wmGizmo, *mut rcti) -> bool>;
