/* SPDX-FileCopyrightText: 2023 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Asset system utility path resolution routines.

use std::path::{Path, PathBuf};

/// Returns a normalized directory path with a trailing slash.
pub fn resolve_directory_path(directory: &str) -> String {
    let mut normalized = directory.replace('\\', "/");
    if !normalized.ends_with('/') && !normalized.is_empty() {
        normalized.push('/');
    }
    normalized
}

/// Normalize the given path (remove redundant dots/slashes, handle max_len prefix).
pub fn resolve_path(path: &str, max_len: Option<usize>) -> String {
    let input = match max_len {
        Some(len) if len < path.len() => &path[..len],
        _ => path,
    };

    let p = Path::new(input);
    let mut components = Vec::new();
    for comp in p.components() {
        match comp {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                components.pop();
            }
            c => components.push(c.as_os_str().to_string_lossy().into_owned()),
        }
    }

    if components.is_empty() {
        return ".".to_string();
    }

    let is_absolute = input.starts_with('/') || (input.len() >= 2 && input.chars().nth(1) == Some(':'));
    let mut result = components.join("/");
    if is_absolute && !result.starts_with('/') && input.starts_with('/') {
        result.insert(0, '/');
    }
    result
}
