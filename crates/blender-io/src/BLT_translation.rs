/* SPDX-FileCopyrightText: 2011 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::os::raw::c_char;

pub const TEXT_DOMAIN_NAME: &str = "blender";

extern "C" {
    pub fn BLT_translate() -> bool;
    pub fn BLT_translate_iface() -> bool;
    pub fn BLT_translate_tooltips() -> bool;
    pub fn BLT_translate_reports() -> bool;
    pub fn BLT_translate_new_dataname() -> bool;

    pub fn BLT_pgettext(msgctxt: *const c_char, msgid: *const c_char) -> *const c_char;
    pub fn BLT_translate_do(msgctxt: *const c_char, msgid: *const c_char) -> *const c_char;
    pub fn BLT_translate_do_iface(msgctxt: *const c_char, msgid: *const c_char) -> *const c_char;
    pub fn BLT_translate_do_tooltip(msgctxt: *const c_char, msgid: *const c_char) -> *const c_char;
    pub fn BLT_translate_do_report(msgctxt: *const c_char, msgid: *const c_char) -> *const c_char;
    pub fn BLT_translate_do_new_dataname(
        msgctxt: *const c_char,
        msgid: *const c_char,
    ) -> *const c_char;
}
