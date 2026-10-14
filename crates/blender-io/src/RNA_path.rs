//! Safe Rust port of Blender's `source/blender/makesrna/RNA_path.hh`.
//!
//! RNA paths are a way to refer to pointers and properties with a string,
//! using syntax like `scenes[0].objects["Cube"].data.verts[7].co`.

use std::hash::{Hash, Hasher};

/// An RNA path to a property, including an optional key/index for array and
/// collection properties.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct RNAPath {
    pub path: String,
    pub key: Option<String>,
    pub index: Option<i32>,
}

impl RNAPath {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            key: None,
            index: None,
        }
    }

    pub fn with_index(path: impl Into<String>, index: i32) -> Self {
        Self {
            path: path.into(),
            key: None,
            index: Some(index),
        }
    }

    pub fn with_key(path: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            key: Some(key.into()),
            index: None,
        }
    }

    pub fn with_key_and_index(path: impl Into<String>, key: impl Into<String>, index: i32) -> Self {
        Self {
            path: path.into(),
            key: Some(key.into()),
            index: Some(index),
        }
    }

    pub fn hash_value(&self) -> i64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish() as i64
    }
}

pub mod rna_path {
    /// Access a property of an RNA type, like `object.location`.
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
    pub struct Member {
        pub identifier: String,
    }

    /// Access an element in a collection property by index, like `data.points[10]`.
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
    pub struct LookupIndex {
        pub index: i64,
    }

    /// Quoted access with brackets, like `object["my_custom_property"]`.
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
    pub struct LookupKey {
        pub key: String,
    }

    /// An element in a parsed RNA path.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Item {
        Member(Member),
        LookupIndex(LookupIndex),
        LookupKey(LookupKey),
    }

    pub fn to_string(items: &[Item]) -> String {
        let mut out = String::new();
        for (i, item) in items.iter().enumerate() {
            match item {
                Item::Member(m) => {
                    if i > 0 {
                        out.push('.');
                    }
                    out.push_str(&m.identifier);
                }
                Item::LookupIndex(idx) => {
                    out.push('[');
                    out.push_str(&idx.index.to_string());
                    out.push(']');
                }
                Item::LookupKey(k) => {
                    out.push_str("[\"");
                    out.push_str(&k.key);
                    out.push_str("\"]");
                }
            }
        }
        out
    }
}

/// Storage for a parsed path.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParsedRNAPath {
    pub items: Vec<rna_path::Item>,
}

impl ParsedRNAPath {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn from_items(items: Vec<rna_path::Item>) -> Self {
        Self { items }
    }

    pub fn to_string(&self) -> String {
        rna_path::to_string(&self.items)
    }
}
