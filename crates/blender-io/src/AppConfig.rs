/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

pub struct PathConfig {
    pub project_dir: String,
    pub models_path: String,
    pub patterns_path: String,
    pub brushes_path: String,
    pub env_map_dir: String,
    pub maps_dir: String,
    pub home_dir: String,
}

impl Default for PathConfig {
    fn default() -> Self {
        Self {
            project_dir: String::new(),
            models_path: String::new(),
            patterns_path: String::new(),
            brushes_path: String::new(),
            env_map_dir: String::new(),
            maps_dir: String::new(),
            home_dir: String::new(),
        }
    }
}
