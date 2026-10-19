//! Auto-transpiled C/C++ header module: bmesh_iterators_parallel

use core::ffi::c_void;
use crate::bmesh_class::BMesh;

pub type TaskParallelMempoolFunc = Option<unsafe extern "C" fn(userdata: *mut c_void, iter: *mut c_void, tls: *const c_void)>;

extern "C" {
    pub fn BM_iter_parallel(
        bm: *mut BMesh,
        itype: i8,
        func: TaskParallelMempoolFunc,
        userdata: *mut c_void,
        settings: *const c_void,
    );
}
