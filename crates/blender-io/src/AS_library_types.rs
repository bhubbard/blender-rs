/* SPDX-FileCopyrightText: 2023-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Asset library types and wrappers for the Blender Asset System.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use blender_dna::DNA_asset_types::{eAssetImportMethod, eAssetLibraryType, AssetLibraryReference};
use blender_dna::DNA_userdef_types::bUserAssetLibrary;
use crate::AS_asset_library::AssetLibrary;

/// Helper function to convert C-style i8 buffers to Rust String.
fn c_char_slice_to_string(bytes: &[i8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    let u8_bytes: Vec<u8> = bytes[..end].iter().map(|&b| b as u8).collect();
    String::from_utf8_lossy(&u8_bytes).into_owned()
}

/// Wrapper to get the `bUserAssetLibrary` from preferences safely.
#[derive(Debug, Clone)]
pub struct UserAssetLibraryWrapper {
    pub user_asset_library: Option<*const bUserAssetLibrary>,
}

impl UserAssetLibraryWrapper {
    pub fn new(user_asset_library: *const bUserAssetLibrary) -> Self {
        Self {
            user_asset_library: if user_asset_library.is_null() {
                None
            } else {
                Some(user_asset_library)
            },
        }
    }

    pub fn user_asset_library(&self) -> Option<&bUserAssetLibrary> {
        self.user_asset_library.and_then(|ptr| unsafe { ptr.as_ref() })
    }
}

unsafe impl Send for UserAssetLibraryWrapper {}
unsafe impl Sync for UserAssetLibraryWrapper {}

/// Aggregate library containing all asset catalogs across multiple sources.
pub struct AllAssetLibrary {
    pub base: AssetLibrary,
    catalogs_dirty: AtomicBool,
    rebuild_mutex: Mutex<()>,
}

impl AllAssetLibrary {
    pub fn new() -> Self {
        Self {
            base: AssetLibrary::new("All", ""),
            catalogs_dirty: AtomicBool::new(true),
            rebuild_mutex: Mutex::new(()),
        }
    }

    pub fn force_remote_listing_download(&self) {}

    pub fn library_reference(&self) -> Option<AssetLibraryReference> {
        None
    }

    pub fn import_method(&self) -> Option<eAssetImportMethod> {
        None
    }

    pub fn refresh_catalogs(&mut self) {
        self.rebuild_catalogs_from_nested(false);
    }

    pub fn rebuild_catalogs_from_nested(&self, _reload_nested_catalogs: bool) {
        let _guard = self.rebuild_mutex.lock();
        self.catalogs_dirty.store(false, Ordering::Relaxed);
    }

    pub fn tag_catalogs_dirty(&self) {
        self.catalogs_dirty.store(true, Ordering::Relaxed);
    }

    pub fn is_catalogs_dirty(&self) -> bool {
        self.catalogs_dirty.load(Ordering::Relaxed)
    }
}

impl Default for AllAssetLibrary {
    fn default() -> Self {
        Self::new()
    }
}

/// Asset library loaded from a local disk path.
pub struct OnDiskAssetLibrary {
    pub base: AssetLibrary,
    pub library_type: eAssetLibraryType,
    pub is_read_only: bool,
    pub enabled: bool,
}

impl OnDiskAssetLibrary {
    pub fn new(library_type: eAssetLibraryType, name: &str, root_path: &str, is_read_only: bool) -> Self {
        Self {
            base: AssetLibrary::new(name, root_path),
            library_type,
            is_read_only,
            enabled: true,
        }
    }

    pub fn library_reference(&self) -> Option<AssetLibraryReference> {
        None
    }

    pub fn import_method(&self) -> Option<eAssetImportMethod> {
        None
    }

    pub fn refresh_catalogs(&mut self) {}

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// On-disk asset library connected to Blender user preferences.
pub struct PreferencesOnDiskAssetLibrary {
    pub on_disk: OnDiskAssetLibrary,
    pub user_library: UserAssetLibraryWrapper,
}

impl PreferencesOnDiskAssetLibrary {
    pub fn new(user_asset_library: *const bUserAssetLibrary) -> Self {
        let name = if let Some(lib) = unsafe { user_asset_library.as_ref() } {
            c_char_slice_to_string(&lib.name)
        } else {
            String::new()
        };

        let path = if let Some(lib) = unsafe { user_asset_library.as_ref() } {
            c_char_slice_to_string(&lib.dirpath)
        } else {
            String::new()
        };

        Self {
            on_disk: OnDiskAssetLibrary::new(eAssetLibraryType::ASSET_LIBRARY_CUSTOM, &name, &path, false),
            user_library: UserAssetLibraryWrapper::new(user_asset_library),
        }
    }

    pub fn library_reference(&self) -> Option<AssetLibraryReference> {
        self.on_disk.library_reference()
    }

    pub fn import_method(&self) -> Option<eAssetImportMethod> {
        self.user_library.user_asset_library().map(|lib| eAssetImportMethod(lib.import_method as i32))
    }

    pub fn use_relative_paths(&self) -> bool {
        false
    }

    pub fn is_enabled(&self) -> bool {
        self.user_library.user_asset_library().is_some()
    }
}

/// Remote asset library fetched over HTTP / CDN.
pub struct RemoteAssetLibrary {
    pub base: AssetLibrary,
    pub remote_url: String,
    pub is_read_only: bool,
    pub library_type: eAssetLibraryType,
}

impl RemoteAssetLibrary {
    pub fn new(
        library_type: eAssetLibraryType,
        is_read_only: bool,
        remote_url: &str,
        name: &str,
        root_path: &str,
    ) -> Self {
        Self {
            base: AssetLibrary::new(name, root_path),
            remote_url: remote_url.to_string(),
            is_read_only,
            library_type,
        }
    }

    pub fn force_remote_listing_download(&self) {}

    pub fn import_method(&self) -> Option<eAssetImportMethod> {
        None
    }

    pub fn remote_url(&self) -> Option<&str> {
        Some(&self.remote_url)
    }

    pub fn refresh_catalogs(&mut self) {}
}

/// Remote library backed by user preferences.
pub struct PreferencesRemoteAssetLibrary {
    pub remote: RemoteAssetLibrary,
    pub user_library: UserAssetLibraryWrapper,
}

impl PreferencesRemoteAssetLibrary {
    pub fn new(custom_library: *const bUserAssetLibrary) -> Self {
        Self {
            remote: RemoteAssetLibrary::new(eAssetLibraryType::ASSET_LIBRARY_CUSTOM, true, "", "", ""),
            user_library: UserAssetLibraryWrapper::new(custom_library),
        }
    }

    pub fn library_reference(&self) -> Option<AssetLibraryReference> {
        None
    }

    pub fn auth_token(&self) -> Option<String> {
        None
    }

    pub fn is_enabled(&self) -> bool {
        self.user_library.user_asset_library().is_some()
    }
}

/// Pure in-memory asset library for unsaved documents.
pub struct RuntimeAssetLibrary {
    pub base: AssetLibrary,
}

impl RuntimeAssetLibrary {
    pub fn new() -> Self {
        Self {
            base: AssetLibrary::new("Current File", ""),
        }
    }

    pub fn library_reference(&self) -> Option<AssetLibraryReference> {
        None
    }

    pub fn import_method(&self) -> Option<eAssetImportMethod> {
        None
    }
}

impl Default for RuntimeAssetLibrary {
    fn default() -> Self {
        Self::new()
    }
}
