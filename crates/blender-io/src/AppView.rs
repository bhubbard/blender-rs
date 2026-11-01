/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

pub struct AppView {
    pub width: u32,
    pub height: u32,
}

impl AppView {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
