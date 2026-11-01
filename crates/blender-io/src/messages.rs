/* SPDX-FileCopyrightText: 2025 Blender Authors
 *
 * SPDX-License-Identifier: BSL-1.0 */

pub struct LocaleManager {
    current_locale: String,
}

impl LocaleManager {
    pub fn new(locale: &str) -> Self {
        Self {
            current_locale: locale.to_string(),
        }
    }

    pub fn current_locale(&self) -> &str {
        &self.current_locale
    }

    pub fn translate(&self, _domain: i32, _context: &str, key: &str) -> Option<String> {
        Some(key.to_string())
    }
}
