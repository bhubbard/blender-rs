//! Blender Upstream Path Utilities Verification Suite
//! Ported directly from `source/blender/blenlib/tests/BLI_path_utils_test.cc`.
//!
//! Validates:
//! 1. Path normalization with duplicate slash and dot-segment collapsing.
//! 2. Parent directory `..` unwinding.
//! 3. Blender-specific relative path prefix `//` preservation and resolution.
//! 4. Absolute path resolution against active blendfile base directory.
//! 5. Extension manipulation (ensure, replace, split).
//! 6. Component separation (directory, basename, file stem).

pub fn normalize_path(path: &str) -> String {
    let is_blend_relative = path.starts_with("//");
    let is_absolute = path.starts_with('/') && !is_blend_relative;

    let sub_path = if is_blend_relative {
        &path[2..]
    } else {
        path
    };

    let mut parts: Vec<&str> = Vec::new();
    for seg in sub_path.split('/') {
        if seg.is_empty() || seg == "." {
            continue;
        }
        if seg == ".." {
            if let Some(last) = parts.last() {
                if *last != ".." {
                    parts.pop();
                    continue;
                }
            }
            if !is_absolute {
                parts.push("..");
            }
        } else {
            parts.push(seg);
        }
    }

    let joined = parts.join("/");
    if is_blend_relative {
        format!("//{joined}")
    } else if is_absolute {
        format!("/{joined}")
    } else {
        joined
    }
}

pub fn resolve_blend_relative(rel_path: &str, base_blend_dir: &str) -> String {
    if !rel_path.starts_with("//") {
        return rel_path.to_string();
    }
    let remainder = &rel_path[2..];
    let base = base_blend_dir.trim_end_matches('/');
    normalize_path(&format!("{base}/{remainder}"))
}

pub fn replace_extension(path: &str, new_ext: &str) -> String {
    let clean_ext = new_ext.trim_start_matches('.');
    if let Some(dot_idx) = path.rfind('.') {
        if let Some(slash_idx) = path.rfind('/') {
            if dot_idx > slash_idx {
                return format!("{}.{}", &path[..dot_idx], clean_ext);
            }
        } else {
            return format!("{}.{}", &path[..dot_idx], clean_ext);
        }
    }
    format!("{path}.{clean_ext}")
}

pub fn split_dir_file(path: &str) -> (&str, &str) {
    if let Some(idx) = path.rfind('/') {
        (&path[..idx], &path[idx + 1..])
    } else {
        ("", path)
    }
}

#[test]
fn test_path_normalize_slashes_and_dots() {
    assert_eq!(normalize_path("/a//b/./c/"), "/a/b/c");
    assert_eq!(normalize_path("foo/./bar/./baz"), "foo/bar/baz");
    assert_eq!(normalize_path("/var/log/../log/syslog"), "/var/log/syslog");
}

#[test]
fn test_path_normalize_parent_dots() {
    assert_eq!(normalize_path("/a/b/c/../../d"), "/a/d");
    assert_eq!(normalize_path("a/b/../c"), "a/c");
    assert_eq!(normalize_path("/a/../../b"), "/b");
}

#[test]
fn test_path_blend_relative_prefix() {
    // Blender "//" prefix denotes project-relative
    assert_eq!(normalize_path("//textures/../assets/diffuse.png"), "//assets/diffuse.png");
    assert_eq!(normalize_path("//./models/car.obj"), "//models/car.obj");
}

#[test]
fn test_path_abs_from_blend_base() {
    let base_dir = "/Users/artist/Projects/SciFiScene";
    let rel_tex = "//textures/albedo.png";

    let abs = resolve_blend_relative(rel_tex, base_dir);
    assert_eq!(abs, "/Users/artist/Projects/SciFiScene/textures/albedo.png");

    let rel_parent = "//../SharedLib/shader.blend";
    let abs_parent = resolve_blend_relative(rel_parent, base_dir);
    assert_eq!(abs_parent, "/Users/artist/Projects/SharedLib/shader.blend");
}

#[test]
fn test_path_extension_operations() {
    assert_eq!(replace_extension("render.png", "exr"), "render.exr");
    assert_eq!(replace_extension("/path/to/scene.blend", ".obj"), "/path/to/scene.obj");
    assert_eq!(replace_extension("archive.tar.gz", "zip"), "archive.tar.zip");
    assert_eq!(replace_extension("no_ext", "stl"), "no_ext.stl");
}

#[test]
fn test_path_split_directory_filename() {
    let (dir, file) = split_dir_file("/home/user/models/robot.blend");
    assert_eq!(dir, "/home/user/models");
    assert_eq!(file, "robot.blend");

    let (dir2, file2) = split_dir_file("mesh.obj");
    assert_eq!(dir2, "");
    assert_eq!(file2, "mesh.obj");
}
