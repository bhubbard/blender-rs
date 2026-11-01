/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use blender_dna::DNA_asset_types::AssetMetaData;
use blender_dna::ID;

#[repr(C)]
pub struct OnlineAssetInfo {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct OnlineAssetFile {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct URLWithHash {
    pub url: *const std::os::raw::c_char,
    pub hash: *const std::os::raw::c_char,
}

pub struct AssetRepresentation {
    pub name: String,
    pub metadata: *mut AssetMetaData,
    pub local_id: *mut ID,
}

impl AssetRepresentation {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            metadata: std::ptr::null_mut(),
            local_id: std::ptr::null_mut(),
        }
    }
}
