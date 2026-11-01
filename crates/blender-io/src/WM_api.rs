/* SPDX-FileCopyrightText: 2007-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Window Manager API: window creation, event dispatch loop, timers, and operator invokers.

use std::os::raw::{c_char, c_void};
use blender_dna::{wmWindow, wmWindowManager};
use crate::wm_gizmo_wmapi::bContext;

extern "C" {
    pub fn WM_init(C: *mut bContext, argc: i32, argv: *const *const c_char);
    pub fn WM_exit(C: *mut bContext);
    pub fn WM_main(C: *mut bContext);

    pub fn WM_window_open(
        C: *mut bContext,
        title: *const c_char,
        x: i32,
        y: i32,
        sizex: i32,
        sizey: i32,
    ) -> *mut wmWindow;

    pub fn WM_window_close(C: *mut bContext, wm: *mut wmWindowManager, win: *mut wmWindow);

    pub fn WM_event_add_timer(
        wm: *mut wmWindowManager,
        win: *mut wmWindow,
        timer_type: i32,
        timestep: f64,
    ) -> *mut c_void;

    pub fn WM_event_remove_timer(
        wm: *mut wmWindowManager,
        win: *mut wmWindow,
        timer: *mut c_void,
    );
}
