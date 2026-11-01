/* SPDX-FileCopyrightText: 2023-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Multi-function node utility functions and registration helpers.

#[repr(C)]
#[derive(Debug, Clone)]
pub struct bNodeType {
    pub idname: [i8; 64],
    pub r#type: i16,
    pub flag: i16,
}

impl Default for bNodeType {
    fn default() -> Self {
        Self {
            idname: [0; 64],
            r#type: 0,
            flag: 0,
        }
    }
}

pub fn fn_node_type_base(ntype: *mut bNodeType, idname: &str, _legacy_type: Option<i16>) {
    if ntype.is_null() {
        return;
    }
    unsafe {
        let nt = &mut *ntype;
        let bytes = idname.as_bytes();
        let len = bytes.len().min(nt.idname.len() - 1);
        for i in 0..len {
            nt.idname[i] = bytes[i] as i8;
        }
        nt.idname[len] = 0;
    }
}

pub fn fn_cmp_node_type_base(ntype: *mut bNodeType, idname: &str, legacy_type: Option<i16>) {
    fn_node_type_base(ntype, idname, legacy_type);
}
