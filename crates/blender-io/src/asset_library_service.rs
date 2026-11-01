/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use crate::AS_asset_library::AssetLibrary;
use std::collections::HashMap;

#[derive(Default)]
pub struct AssetLibraryService {
    pub libraries: HashMap<String, AssetLibrary>,
}

impl AssetLibraryService {
    pub fn new() -> Self {
        Self {
            libraries: HashMap::new(),
        }
    }

    pub fn register_library(&mut self, name: &str, path: &str) {
        self.libraries
            .insert(name.to_string(), AssetLibrary::new(name, path));
    }

    pub fn get_library(&self, name: &str) -> Option<&AssetLibrary> {
        self.libraries.get(name)
    }
}
