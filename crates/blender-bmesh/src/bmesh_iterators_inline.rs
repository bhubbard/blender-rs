//! Auto-transpiled C/C++ header module: bmesh_iterators_inline

use core::ffi::c_void;
use crate::bmesh_class::*;
use crate::bmesh_iterators::*;

extern "C" {
    pub fn bmiter__elem_of_mesh_begin(iter: *mut c_void);
    pub fn bmiter__elem_of_mesh_step(iter: *mut c_void) -> *mut c_void;
    pub fn bmiter__edge_of_vert_begin(iter: *mut c_void);
    pub fn bmiter__edge_of_vert_step(iter: *mut c_void) -> *mut c_void;
    pub fn bmiter__face_of_vert_begin(iter: *mut c_void);
    pub fn bmiter__face_of_vert_step(iter: *mut c_void) -> *mut c_void;
    pub fn bmiter__loop_of_vert_begin(iter: *mut c_void);
    pub fn bmiter__loop_of_vert_step(iter: *mut c_void) -> *mut c_void;
    pub fn bmiter__vert_of_edge_begin(iter: *mut c_void);
    pub fn bmiter__vert_of_edge_step(iter: *mut c_void) -> *mut c_void;
    pub fn bmiter__face_of_edge_begin(iter: *mut c_void);
    pub fn bmiter__face_of_edge_step(iter: *mut c_void) -> *mut c_void;
    pub fn bmiter__vert_of_face_begin(iter: *mut c_void);
    pub fn bmiter__vert_of_face_step(iter: *mut c_void) -> *mut c_void;
    pub fn bmiter__edge_of_face_begin(iter: *mut c_void);
    pub fn bmiter__edge_of_face_step(iter: *mut c_void) -> *mut c_void;
    pub fn bmiter__loop_of_face_begin(iter: *mut c_void);
    pub fn bmiter__loop_of_face_step(iter: *mut c_void) -> *mut c_void;
    pub fn bmiter__loop_of_loop_begin(iter: *mut c_void);
    pub fn bmiter__loop_of_loop_step(iter: *mut c_void) -> *mut c_void;
    pub fn bmiter__loop_of_edge_begin(iter: *mut c_void);
    pub fn bmiter__loop_of_edge_step(iter: *mut c_void) -> *mut c_void;
}

#[inline]
pub unsafe fn BM_iter_step(iter: *mut BMIter) -> *mut c_void {
    if let Some(step) = (*iter).step {
        step(iter as *mut c_void)
    } else {
        core::ptr::null_mut()
    }
}

#[inline]
pub unsafe fn BM_iter_init(iter: *mut BMIter, bm: *mut BMesh, itype: i8, data: *mut c_void) -> bool {
    (*iter).itype = itype;
    match itype {
        1 => {
            if bm.is_null() {
                return false;
            }
            (*iter).begin = Some(bmiter__elem_of_mesh_begin);
            (*iter).step = Some(bmiter__elem_of_mesh_step);
            (*iter).data.elem_of_mesh.pooliter.pool = (*bm).vpool;
        }
        2 => {
            if bm.is_null() {
                return false;
            }
            (*iter).begin = Some(bmiter__elem_of_mesh_begin);
            (*iter).step = Some(bmiter__elem_of_mesh_step);
            (*iter).data.elem_of_mesh.pooliter.pool = (*bm).epool;
        }
        3 => {
            if bm.is_null() {
                return false;
            }
            (*iter).begin = Some(bmiter__elem_of_mesh_begin);
            (*iter).step = Some(bmiter__elem_of_mesh_step);
            (*iter).data.elem_of_mesh.pooliter.pool = (*bm).fpool;
        }
        4 => {
            if data.is_null() {
                return false;
            }
            (*iter).begin = Some(bmiter__edge_of_vert_begin);
            (*iter).step = Some(bmiter__edge_of_vert_step);
            (*iter).data.edge_of_vert.vdata = data as *mut BMVert;
        }
        5 => {
            if data.is_null() {
                return false;
            }
            (*iter).begin = Some(bmiter__face_of_vert_begin);
            (*iter).step = Some(bmiter__face_of_vert_step);
            (*iter).data.face_of_vert.vdata = data as *mut BMVert;
        }
        6 => {
            if data.is_null() {
                return false;
            }
            (*iter).begin = Some(bmiter__loop_of_vert_begin);
            (*iter).step = Some(bmiter__loop_of_vert_step);
            (*iter).data.loop_of_vert.vdata = data as *mut BMVert;
        }
        7 => {
            if data.is_null() {
                return false;
            }
            (*iter).begin = Some(bmiter__vert_of_edge_begin);
            (*iter).step = Some(bmiter__vert_of_edge_step);
            (*iter).data.vert_of_edge.edata = data as *mut BMEdge;
        }
        8 => {
            if data.is_null() {
                return false;
            }
            (*iter).begin = Some(bmiter__face_of_edge_begin);
            (*iter).step = Some(bmiter__face_of_edge_step);
            (*iter).data.face_of_edge.edata = data as *mut BMEdge;
        }
        9 => {
            if data.is_null() {
                return false;
            }
            (*iter).begin = Some(bmiter__vert_of_face_begin);
            (*iter).step = Some(bmiter__vert_of_face_step);
            (*iter).data.vert_of_face.pdata = data as *mut BMFace;
        }
        10 => {
            if data.is_null() {
                return false;
            }
            (*iter).begin = Some(bmiter__edge_of_face_begin);
            (*iter).step = Some(bmiter__edge_of_face_step);
            (*iter).data.edge_of_face.pdata = data as *mut BMFace;
        }
        11 => {
            if data.is_null() {
                return false;
            }
            (*iter).begin = Some(bmiter__loop_of_face_begin);
            (*iter).step = Some(bmiter__loop_of_face_step);
            (*iter).data.loop_of_face.pdata = data as *mut BMFace;
        }
        12 => {
            if data.is_null() {
                return false;
            }
            (*iter).begin = Some(bmiter__loop_of_loop_begin);
            (*iter).step = Some(bmiter__loop_of_loop_step);
            (*iter).data.loop_of_loop.ldata = data as *mut BMLoop;
        }
        13 => {
            if data.is_null() {
                return false;
            }
            (*iter).begin = Some(bmiter__loop_of_edge_begin);
            (*iter).step = Some(bmiter__loop_of_edge_step);
            (*iter).data.loop_of_edge.edata = data as *mut BMEdge;
        }
        _ => return false,
    }

    if let Some(begin) = (*iter).begin {
        begin(iter as *mut c_void);
    }
    true
}

#[inline]
pub unsafe fn BM_iter_new(iter: *mut BMIter, bm: *mut BMesh, itype: i8, data: *mut c_void) -> *mut c_void {
    if BM_iter_init(iter, bm, itype, data) {
        BM_iter_step(iter)
    } else {
        core::ptr::null_mut()
    }
}
