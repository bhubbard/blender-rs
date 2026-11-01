/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::ffi::c_void;
use std::os::raw::c_char;

pub type PanelDrawFn = Option<unsafe extern "C" fn(c: *const c_void, panel: *mut c_void)>;

extern "C" {
    pub fn modifier_vgroup_ui(
        layout: *mut c_void,
        ptr: *mut c_void,
        ob_ptr: *mut c_void,
        vgroup_prop: *const c_char,
        invert_vgroup_prop: *const c_char,
        text: *const c_char,
    );
    pub fn modifier_grease_pencil_curve_header_draw(c: *const c_void, panel: *mut c_void);
    pub fn modifier_grease_pencil_curve_panel_draw(c: *const c_void, panel: *mut c_void);
    pub fn modifier_error_message_draw(layout: *mut c_void, ptr: *mut c_void);
    pub fn modifier_panel_get_property_pointers(
        panel: *mut c_void,
        r_ob_ptr: *mut c_void,
    ) -> *mut c_void;
    pub fn modifier_panel_register(
        region_type: *mut c_void,
        mod_type: i32,
        draw: PanelDrawFn,
    ) -> *mut c_void;
    pub fn modifier_subpanel_register(
        region_type: *mut c_void,
        name: *const c_char,
        label: *const c_char,
        draw_header: PanelDrawFn,
        draw: PanelDrawFn,
        parent: *mut c_void,
    ) -> *mut c_void;
}
