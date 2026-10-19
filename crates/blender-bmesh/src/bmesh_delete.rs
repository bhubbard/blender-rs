//! Auto-transpiled C/C++ header module: bmesh_delete

use crate::*;

pub fn BMO_mesh_delete_oflag_tagged(_bm: *mut BMesh, _oflag: i16, _htype: i8) {}

pub fn BM_mesh_delete_hflag_tagged(_bm: *mut BMesh, _hflag: i8, _htype: i8) {}

pub fn BMO_mesh_delete_oflag_context(
    _bm: *mut BMesh,
    _oflag: i16,
    _type: i32,
    _prepare_fn: Option<fn()>,
) {}

pub fn BM_mesh_delete_hflag_context(_bm: *mut BMesh, _hflag: i8, _type: i32) {}
