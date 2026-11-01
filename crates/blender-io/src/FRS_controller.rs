/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use crate::AppCanvas::AppCanvas;
use crate::AppConfig::PathConfig;
use crate::AppView::AppView;

pub struct FreestyleController {
    pub canvas: Option<AppCanvas>,
    pub view: Option<AppView>,
    pub config: PathConfig,
}

impl Default for FreestyleController {
    fn default() -> Self {
        Self {
            canvas: None,
            view: None,
            config: PathConfig::default(),
        }
    }
}
