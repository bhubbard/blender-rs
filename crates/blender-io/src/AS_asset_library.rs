/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use crate::AS_asset_catalog_tree::AssetCatalogTree;
use crate::AS_asset_representation::AssetRepresentation;
use std::collections::HashMap;

pub struct AssetLibrary {
    pub name: String,
    pub path: String,
    pub catalog_tree: AssetCatalogTree,
    pub assets: HashMap<String, AssetRepresentation>,
}

impl AssetLibrary {
    pub fn new(name: &str, path: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            catalog_tree: AssetCatalogTree::new(),
            assets: HashMap::new(),
        }
    }
}
