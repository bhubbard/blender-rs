//! Auto-transpiled C/C++ header module: bmesh_query_inline

use crate::bmesh_class::*;
use crate::bmesh_structure_inline::*;

#[inline]
pub unsafe fn BM_vert_in_edge(e: *const BMEdge, v: *const BMVert) -> bool {
    v == (*e).v1 || v == (*e).v2
}

#[inline]
pub unsafe fn BM_edge_in_loop(e: *const BMEdge, l: *const BMLoop) -> bool {
    (*l).e == (e as *mut BMEdge) || (!(*l).prev.is_null() && (*(*l).prev).e == (e as *mut BMEdge))
}

#[inline]
pub unsafe fn BM_verts_in_edge(v1: *const BMVert, v2: *const BMVert, e: *const BMEdge) -> bool {
    ((*e).v1 == (v1 as *mut BMVert) && (*e).v2 == (v2 as *mut BMVert))
        || ((*e).v1 == (v2 as *mut BMVert) && (*e).v2 == (v1 as *mut BMVert))
}

#[inline]
pub unsafe fn BM_edge_other_vert(e: *mut BMEdge, v: *const BMVert) -> *mut BMVert {
    if (*e).v1 == (v as *mut BMVert) {
        (*e).v2
    } else if (*e).v2 == (v as *mut BMVert) {
        (*e).v1
    } else {
        core::ptr::null_mut()
    }
}

#[inline]
pub unsafe fn BM_edge_is_wire(e: *const BMEdge) -> bool {
    (*e).l.is_null()
}

#[inline]
pub unsafe fn BM_edge_is_manifold(e: *const BMEdge) -> bool {
    let l = (*e).l;
    if l.is_null() {
        return false;
    }
    let r_next = (*l).radial_next;
    if r_next.is_null() || r_next == l {
        return false;
    }
    (*r_next).radial_next == l
}

#[inline]
pub unsafe fn BM_edge_is_contiguous(e: *const BMEdge) -> bool {
    let l = (*e).l;
    if l.is_null() {
        return false;
    }
    let l_other = (*l).radial_next;
    if l_other.is_null() || l_other == l {
        return false;
    }
    (*l_other).radial_next == l && (*l_other).v != (*l).v
}

#[inline]
pub unsafe fn BM_edge_is_boundary(e: *const BMEdge) -> bool {
    let l = (*e).l;
    !l.is_null() && (*l).radial_next == l
}

#[inline]
pub unsafe fn BM_loop_is_adjacent(l_a: *const BMLoop, l_b: *const BMLoop) -> bool {
    let lb_mut = l_b as *mut BMLoop;
    (*l_a).next == lb_mut || (*l_a).prev == lb_mut
}

#[inline]
pub unsafe fn BM_loop_is_manifold(l: *const BMLoop) -> bool {
    let r_next = (*l).radial_next;
    !r_next.is_null() && r_next != (l as *mut BMLoop) && (*r_next).radial_next == (l as *mut BMLoop)
}

#[inline]
pub unsafe fn BM_vert_is_wire_endpoint(v: *const BMVert) -> bool {
    let e = (*v).e;
    if !e.is_null() && (*e).l.is_null() {
        bmesh_disk_edge_next(e, v) == e
    } else {
        false
    }
}
