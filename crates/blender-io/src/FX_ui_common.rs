/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::ffi::c_void;
use std::os::raw::c_char;
use crate::MOD_ui_common::PanelDrawFn;

extern "C" {
    pub fn shaderfx_panel_end(layout: *mut c_void, ptr: *mut c_void);
    pub fn shaderfx_panel_get_property_pointers(
        panel: *mut c_void,
        r_ob_ptr: *mut c_void,
    ) -> *mut c_void;
    pub fn shaderfx_panel_register(
        region_type: *mut c_void,
        fx_type: i32,
        draw: PanelDrawFn,
    ) -> *mut c_void;
    pub fn shaderfx_subpanel_register(
        region_type: *mut c_void,
        name: *const c_char,
        label: *const c_char,
        draw_header: PanelDrawFn,
        draw: PanelDrawFn,
        parent: *mut c_void,
    ) -> *mut c_void;
}
