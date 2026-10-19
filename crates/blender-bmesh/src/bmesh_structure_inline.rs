//! Auto-transpiled C/C++ header module: bmesh_structure_inline

use crate::bmesh_class::*;

#[inline]
pub unsafe fn bmesh_disk_edge_link_from_vert(e: *const BMEdge, v: *const BMVert) -> *mut BMDiskLink {
    if v == (*e).v2 {
        &mut (*(e as *mut BMEdge)).v2_disk_link
    } else {
        &mut (*(e as *mut BMEdge)).v1_disk_link
    }
}

#[inline]
pub unsafe fn bmesh_disk_edge_next_safe(e: *const BMEdge, v: *const BMVert) -> *mut BMEdge {
    if v == (*e).v1 {
        return (*e).v1_disk_link.next;
    }
    if v == (*e).v2 {
        return (*e).v2_disk_link.next;
    }
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn bmesh_disk_edge_prev_safe(e: *const BMEdge, v: *const BMVert) -> *mut BMEdge {
    if v == (*e).v1 {
        return (*e).v1_disk_link.prev;
    }
    if v == (*e).v2 {
        return (*e).v2_disk_link.prev;
    }
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn bmesh_disk_edge_next(e: *const BMEdge, v: *const BMVert) -> *mut BMEdge {
    bmesh_disk_edge_next_safe(e, v)
}

#[inline]
pub unsafe fn bmesh_disk_edge_prev(e: *const BMEdge, v: *const BMVert) -> *mut BMEdge {
    bmesh_disk_edge_prev_safe(e, v)
}
