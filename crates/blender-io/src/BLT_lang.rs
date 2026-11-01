/* SPDX-FileCopyrightText: 2011 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::os::raw::c_char;

extern "C" {
    pub fn BLT_lang_init();
    pub fn BLT_lang_free();
    pub fn BLT_lang_set(locale: *const c_char);
    pub fn BLT_lang_get() -> *const c_char;
    pub fn BLT_lang_locale_explode(
        locale: *const c_char,
        language: *mut *mut c_char,
        country: *mut *mut c_char,
        variant: *mut *mut c_char,
        language_country: *mut *mut c_char,
        language_variant: *mut *mut c_char,
    );
}
