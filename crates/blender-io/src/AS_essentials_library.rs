/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

pub fn essentials_directory_path() -> &'static str {
    "assets/essentials"
}

pub fn online_essentials_cache_directory_path() -> &'static str {
    "assets/online_cache"
}

pub fn online_essentials_url() -> &'static str {
    "https://extensions.blender.org/assets"
}

pub fn is_online_essentials_url(url: &str) -> bool {
    url.starts_with(online_essentials_url())
}

pub fn is_online_essentials_dirpath(dirpath: &str) -> bool {
    dirpath.contains(online_essentials_cache_directory_path())
}
