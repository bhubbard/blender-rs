use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IndexRange {
    start: usize,
    end: usize,
}

pub struct AlignedIndexRanges {
    prefix: IndexRange,
    aligned: IndexRange,
    suffix: IndexRange,
}
