//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[allow(non_camel_case_types)]
type int32_t = i32;
#[allow(non_camel_case_types)]
type uint32_t = u32;
#[allow(non_camel_case_types)]
type int16_t = i16;
#[allow(non_camel_case_types)]
type uint16_t = u16;
#[allow(non_camel_case_types)]
type int64_t = i64;
#[allow(non_camel_case_types)]
type uint64_t = u64;
#[allow(non_camel_case_types)]
type int8_t = i8;
#[allow(non_camel_case_types)]
type uint8_t = u8;
#[allow(non_camel_case_types)]
type uchar = u8;
#[allow(non_camel_case_types)]
type ushort = u16;
#[allow(non_camel_case_types)]
type uint = u32;
#[allow(non_camel_case_types)]
type ulong = u64;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRNAStatus(pub i32);

impl eRNAStatus {
    pub const Success: Self = Self((0) as i32);
    pub const IndexOutOfRange: Self = Self(1 as i32);
    pub const Immutable: Self = Self(2 as i32);
    pub const Unsupported: Self = Self(3 as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRNACompareMode(pub i32);

impl eRNACompareMode {
    pub const RNA_EQ_STRICT: Self = Self(0 as i32);
    pub const RNA_EQ_UNSET_MATCH_ANY: Self = Self(1 as i32);
    pub const RNA_EQ_UNSET_MATCH_NONE: Self = Self(2 as i32);
    pub const RNA_EQ_COMPARE: Self = Self(3 as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRNAOverrideMatch(pub i32);

impl eRNAOverrideMatch {
    pub const RNA_OVERRIDE_COMPARE_IGNORE_NON_OVERRIDABLE: Self = Self((1 << 0) as i32);
    pub const RNA_OVERRIDE_COMPARE_IGNORE_OVERRIDDEN: Self = Self((1 << 1) as i32);
    pub const RNA_OVERRIDE_COMPARE_CREATE: Self = Self((1 << 16) as i32);
    pub const RNA_OVERRIDE_COMPARE_RESTORE: Self = Self((1 << 17) as i32);
    pub const RNA_OVERRIDE_COMPARE_TAG_FOR_RESTORE: Self = Self((1 << 18) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRNAOverrideMatchResult(pub i32);

impl eRNAOverrideMatchResult {
    pub const RNA_OVERRIDE_MATCH_RESULT_INIT: Self = Self((0) as i32);
    pub const RNA_OVERRIDE_MATCH_RESULT_CREATED: Self = Self((1 << 0) as i32);
    pub const RNA_OVERRIDE_MATCH_RESULT_RESTORE_TAGGED: Self = Self((1 << 1) as i32);
    pub const RNA_OVERRIDE_MATCH_RESULT_RESTORED: Self = Self((1 << 2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRNAOverrideStatus(pub i32);

impl eRNAOverrideStatus {
    pub const LibOverridable: Self = Self((1 << 0) as i32);
    pub const LibOverridden: Self = Self((1 << 1) as i32);
    pub const LibOverrideMandatory: Self = Self((1 << 2) as i32);
    pub const LibOverrideLocked: Self = Self((1 << 3) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRNAOverrideApplyFlag(pub i32);

impl eRNAOverrideApplyFlag {
    pub const RNA_OVERRIDE_APPLY_FLAG_NOP: Self = Self((0) as i32);
    pub const RNA_OVERRIDE_APPLY_FLAG_IGNORE_ID_POINTERS: Self = Self((1 << 0) as i32);
    pub const RNA_OVERRIDE_APPLY_FLAG_SKIP_RESYNC_CHECK: Self = Self((1 << 1) as i32);
    pub const RNA_OVERRIDE_APPLY_FLAG_RESTORE_ONLY: Self = Self((1 << 2) as i32);
}

