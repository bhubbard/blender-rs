/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AssetCatalogPath {
    pub path: String,
}

impl AssetCatalogPath {
    pub const SEPARATOR: char = '/';

    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        self.path.rsplit('/').next().unwrap_or(&self.path)
    }

    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    pub fn parent(&self) -> Option<Self> {
        if let Some((parent, _)) = self.path.rsplit_once('/') {
            Some(Self::new(parent))
        } else {
            None
        }
    }
}
