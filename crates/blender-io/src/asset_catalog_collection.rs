/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct AssetCatalogCollection {
    pub catalogs: HashMap<String, String>,
}

impl AssetCatalogCollection {
    pub fn new() -> Self {
        Self {
            catalogs: HashMap::new(),
        }
    }

    pub fn insert_catalog(&mut self, catalog_id: &str, catalog_path: &str) {
        self.catalogs
            .insert(catalog_id.to_string(), catalog_path.to_string());
    }

    pub fn find_catalog(&self, catalog_id: &str) -> Option<&String> {
        self.catalogs.get(catalog_id)
    }
}
