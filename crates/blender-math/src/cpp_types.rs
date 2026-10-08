//! Blender C++20 Standard Utility and Container Shims
//! ====================================================
//! Provides drop-in Rust representations of Blender's foundational C++ types:
//! - `Span<T>` and `MutableSpan<T>` (zero-allocation non-owning views)
//! - `Vector<T>` (growable vector alias)
//! - `StringRef` and `StringRefNull` (string views)
//! - `IndexRange` and `IndexMask` (index iteration)
//! - `VArray<T>` and `Field<T>` (geometry node data streams)
//! - `GeoNodeExecParams` (geometry node execution context)
//! - Standard vector/matrix type aliases (float4x4, int2, int3, etc.)

use core::ops::{Deref, DerefMut, Index, IndexMut, Range};
use core::slice;

/// Non-owning read-only view of a contiguous sequence, mirroring Blender's `blender::Span<T>`.
/// Does not require an explicit lifetime parameter, enabling seamless C++ to Rust translation.
#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct Span<T> {
    pub ptr: *const T,
    pub size: usize,
}

impl<T> Span<T> {
    #[inline]
    pub const fn empty() -> Self {
        Self {
            ptr: core::ptr::null(),
            size: 0,
        }
    }

    #[inline]
    pub fn from_slice(s: &[T]) -> Self {
        Self {
            ptr: s.as_ptr(),
            size: s.len(),
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        if self.ptr.is_null() || self.size == 0 {
            &[]
        } else {
            unsafe { slice::from_raw_parts(self.ptr, self.size) }
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<T> Default for Span<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> Clone for Span<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Span<T> {}

impl<T> Deref for Span<T> {
    type Target = [T];
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> Index<usize> for Span<T> {
    type Output = T;
    #[inline]
    fn index(&self, idx: usize) -> &Self::Output {
        &self.as_slice()[idx]
    }
}

/// Non-owning mutable view of a contiguous sequence, mirroring Blender's `blender::MutableSpan<T>`.
#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct MutableSpan<T> {
    pub ptr: *mut T,
    pub size: usize,
}

impl<T> MutableSpan<T> {
    #[inline]
    pub const fn empty() -> Self {
        Self {
            ptr: core::ptr::null_mut(),
            size: 0,
        }
    }

    #[inline]
    pub fn from_mut_slice(s: &mut [T]) -> Self {
        Self {
            size: s.len(),
            ptr: s.as_mut_ptr(),
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        if self.ptr.is_null() || self.size == 0 {
            &[]
        } else {
            unsafe { slice::from_raw_parts(self.ptr, self.size) }
        }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        if self.ptr.is_null() || self.size == 0 {
            &mut []
        } else {
            unsafe { slice::from_raw_parts_mut(self.ptr, self.size) }
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<T> Default for MutableSpan<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> Deref for MutableSpan<T> {
    type Target = [T];
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> DerefMut for MutableSpan<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T> Index<usize> for MutableSpan<T> {
    type Output = T;
    #[inline]
    fn index(&self, idx: usize) -> &Self::Output {
        &self.as_slice()[idx]
    }
}

impl<T> IndexMut<usize> for MutableSpan<T> {
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.as_mut_slice()[idx]
    }
}

/// Standard growable vector alias mirroring `blender::Vector<T>`.
pub type Vector<T> = Vec<T>;

/// Non-owning string reference mirroring `blender::StringRef`.
#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct StringRef {
    pub ptr: *const u8,
    pub size: usize,
}

impl StringRef {
    #[inline]
    pub const fn empty() -> Self {
        Self {
            ptr: core::ptr::null(),
            size: 0,
        }
    }

    #[inline]
    pub fn from_str(s: &str) -> Self {
        Self {
            ptr: s.as_ptr(),
            size: s.len(),
        }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        if self.ptr.is_null() || self.size == 0 {
            ""
        } else {
            unsafe {
                let bytes = slice::from_raw_parts(self.ptr, self.size);
                core::str::from_utf8(bytes).unwrap_or("")
            }
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl Default for StringRef {
    fn default() -> Self {
        Self::empty()
    }
}

impl Clone for StringRef {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for StringRef {}

impl Deref for StringRef {
    type Target = str;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

pub type StringRefNull = StringRef;

/// Half-open index interval mirroring Blender's `blender::IndexRange`.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct IndexRange {
    pub start: usize,
    pub size: usize,
}

impl IndexRange {
    #[inline]
    pub const fn new(start: usize, size: usize) -> Self {
        Self { start, size }
    }

    #[inline]
    pub const fn from_size(size: usize) -> Self {
        Self { start: 0, size }
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.size
    }

    #[inline]
    pub const fn first(&self) -> usize {
        self.start
    }

    #[inline]
    pub const fn last(&self) -> usize {
        if self.size == 0 {
            self.start
        } else {
            self.start + self.size - 1
        }
    }

    #[inline]
    pub const fn as_range(&self) -> Range<usize> {
        self.start..(self.start + self.size)
    }
}

impl IntoIterator for IndexRange {
    type Item = usize;
    type IntoIter = Range<usize>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.as_range()
    }
}

/// Mask of selected indices mirroring `blender::IndexMask`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IndexMask {
    pub indices: Vec<usize>,
}

impl IndexMask {
    pub fn from_indices(indices: Vec<usize>) -> Self {
        Self { indices }
    }
    pub fn len(&self) -> usize {
        self.indices.len()
    }
    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }
}

/// Generic virtual array wrapper mirroring `blender::VArray<T>`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VArray<T> {
    pub data: Vec<T>,
}

impl<T> VArray<T> {
    pub fn from_vec(data: Vec<T>) -> Self {
        Self { data }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Node field wrapper mirroring `blender::fn::Field<T>`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Field<T> {
    pub _marker: core::marker::PhantomData<T>,
}

/// Geometry Node execution context mirroring `blender::nodes::GeoNodeExecParams`.
#[derive(Debug, Clone, Default)]
pub struct GeoNodeExecParams {
    pub _opaque: [u8; 0],
}

// Matrix and multi-dimensional integer aliases
pub type float4x4 = glam::Mat4;
pub type float3x3 = glam::Mat3;
pub type int2 = [i32; 2];
pub type int3 = [i32; 3];
pub type int4 = [i32; 4];
pub type uint2 = [u32; 2];
pub type uint3 = [u32; 3];
pub type uint4 = [u32; 4];
pub type uchar2 = [u8; 2];
pub type uchar3 = [u8; 3];
pub type uchar4 = [u8; 4];
