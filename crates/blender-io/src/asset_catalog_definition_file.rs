/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

pub const CATALOG_DEFINITION_FILE_HEADER: &str = "# Blender Asset Catalog Definition File";
pub const CATALOG_DEFINITION_FILE_VERSION: &str = "VERSION 1";

#[derive(Debug, Clone, Default)]
pub struct AssetCatalogDefinitionFile {
    pub filepath: String,
    pub lines: Vec<String>,
}

impl AssetCatalogDefinitionFile {
    pub fn new(filepath: &str) -> Self {
        Self {
            filepath: filepath.to_string(),
            lines: Vec::new(),
        }
    }
}
