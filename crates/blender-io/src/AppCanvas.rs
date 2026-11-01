/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

pub struct AppCanvas {
    pub width: u32,
    pub height: u32,
}

impl AppCanvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn erase(&mut self) {}
    pub fn pre_draw(&mut self) {}
    pub fn post_draw(&mut self) {}
}
