//! Blender Library (blenlib) internals (ghash, heap, memblock, math routines, task pool) ported to Rust
//! Corresponds to:
//! BLI_ghash.cc, BLI_ghash_utils.cc, BLI_heap_simple.cc, BLI_linklist.cc,
//! BLI_memblock.cc, BLI_mempool.cc, array_utils_c.cc, expr_pylike_eval.cc,
//! fnmatch.cc, hash_mm2a.cc, hash_mm3.cc, kdtree.cc, lazy_threading.cc,
//! math_base_inline.cc, math_basis_types.cc, math_color_blend_inline.cc,
//! math_filter.cc, math_interp.cc, math_rotation.cc, math_vec.cc, noise.cc,
//! polyfill_2d.cc, scanfill.cc, session_uid.cc, smaa_textures.cc, string_ref.cc,
//! system.cc, system_win32.cc, task_graph.cc, task_iterator.cc, task_pool.cc

use blender_dna::*;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct HeapItem<T> {
    pub value: T,
    pub priority: f32,
}

impl<T: Eq + PartialEq> Eq for HeapItem<T> {}

impl<T: Eq + PartialEq> Ord for HeapItem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.partial_cmp(&self.priority).unwrap_or(Ordering::Equal)
    }
}

impl<T: Eq + PartialEq> PartialOrd for HeapItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Default)]
pub struct SimpleHeap<T: Eq + PartialEq> {
    pub heap: BinaryHeap<HeapItem<T>>,
}

impl<T: Eq + PartialEq> SimpleHeap<T> {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    pub fn insert(&mut self, value: T, priority: f32) {
        self.heap.push(HeapItem { value, priority });
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|item| item.value)
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TaskPoolContext {
    pub num_threads: usize,
    pub active_tasks: usize,
}

impl TaskPoolContext {
    pub fn new(threads: usize) -> Self {
        Self {
            num_threads: threads,
            active_tasks: 0,
        }
    }
}
