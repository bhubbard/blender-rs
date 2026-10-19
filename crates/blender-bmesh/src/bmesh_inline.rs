//! Auto-transpiled C/C++ header module: bmesh_inline

use crate::bmesh_class::BMHeader;

#[inline]
pub fn _bm_elem_flag_test(head: &BMHeader, hflag: i8) -> i8 {
    head.hflag & hflag
}

#[inline]
pub fn _bm_elem_flag_test_bool(head: &BMHeader, hflag: i8) -> bool {
    (head.hflag & hflag) != 0
}

#[inline]
pub fn _bm_elem_flag_enable(head: &mut BMHeader, hflag: i8) {
    head.hflag |= hflag;
}

#[inline]
pub fn _bm_elem_flag_disable(head: &mut BMHeader, hflag: i8) {
    head.hflag &= !hflag;
}

#[inline]
pub fn _bm_elem_flag_set(head: &mut BMHeader, hflag: i8, val: i32) {
    if val != 0 {
        _bm_elem_flag_enable(head, hflag);
    } else {
        _bm_elem_flag_disable(head, hflag);
    }
}

#[inline]
pub fn _bm_elem_flag_toggle(head: &mut BMHeader, hflag: i8) {
    head.hflag ^= hflag;
}

#[inline]
pub fn _bm_elem_flag_merge(head_a: &mut BMHeader, head_b: &mut BMHeader) {
    let merged = head_a.hflag | head_b.hflag;
    head_a.hflag = merged;
    head_b.hflag = merged;
}

#[inline]
pub fn _bm_elem_flag_merge_ex(head_a: &mut BMHeader, head_b: &mut BMHeader, hflag_and: i8) {
    if ((head_a.hflag & head_b.hflag) & hflag_and) == 0 {
        head_a.hflag &= !hflag_and;
        head_b.hflag &= !hflag_and;
    }
    _bm_elem_flag_merge(head_a, head_b);
}

#[inline]
pub fn _bm_elem_flag_merge_into(head: &mut BMHeader, head_a: &BMHeader, head_b: &BMHeader) {
    head.hflag = head_a.hflag | head_b.hflag;
}

#[inline]
pub fn _bm_elem_index_set(head: &mut BMHeader, index: i32) {
    head.index = index;
}

#[inline]
pub fn _bm_elem_index_get(head: &BMHeader) -> i32 {
    head.index
}
