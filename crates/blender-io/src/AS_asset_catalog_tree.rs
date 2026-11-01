/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use crate::AS_asset_catalog_path::AssetCatalogPath;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct AssetCatalogTreeItem {
    pub name: String,
    pub simple_name: String,
    pub catalog_id: String,
    pub children: BTreeMap<String, AssetCatalogTreeItem>,
}

#[derive(Debug, Clone, Default)]
pub struct AssetCatalogTree {
    pub root_items: BTreeMap<String, AssetCatalogTreeItem>,
}

impl AssetCatalogTree {
    pub fn new() -> Self {
        Self {
            root_items: BTreeMap::new(),
        }
    }

    pub fn insert_path(&mut self, path: &AssetCatalogPath, catalog_id: &str) {
        let parts: Vec<&str> = path.path.split('/').filter(|s| !s.is_empty()).collect();
        if parts.is_empty() {
            return;
        }

        let mut current_map = &mut self.root_items;
        for (i, part) in parts.iter().enumerate() {
            let is_leaf = i == parts.len() - 1;
            let entry = current_map
                .entry((*part).to_string())
                .or_insert_with(|| AssetCatalogTreeItem {
                    name: (*part).to_string(),
                    simple_name: (*part).to_string(),
                    catalog_id: if is_leaf {
                        catalog_id.to_string()
                    } else {
                        String::new()
                    },
                    children: BTreeMap::new(),
                });
            if is_leaf {
                entry.catalog_id = catalog_id.to_string();
            }
            current_map = &mut entry.children;
        }
    }
}
