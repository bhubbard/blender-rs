/* SPDX-FileCopyrightText: 2025 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

pub struct RemoteLibraryDefinitionRef<'a> {
    pub remote_url: &'a str,
    pub cache_dirpath: &'a str,
    pub auth_token: Option<&'a str>,
}
