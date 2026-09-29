//! Fast generational chunked memory pool mirroring Blender's `BLI_mempool`.

use std::marker::PhantomData;

/// Strongly-typed generational handle for elements inside a `MemPool`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Handle<T> {
    pub index: u32,
    pub generation: u32,
    _marker: PhantomData<fn() -> T>,
}

impl<T> Copy for Handle<T> {}

impl<T> Clone for Handle<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Handle<T> {
    #[inline]
    pub const fn new(index: u32, generation: u32) -> Self {
        Self {
            index,
            generation,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub const fn invalid() -> Self {
        Self {
            index: u32::MAX,
            generation: 0,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.index != u32::MAX
    }
}

enum Slot<T> {
    Occupied { value: T, generation: u32 },
    Vacant { next_free: Option<u32>, generation: u32 },
}

/// Chunked memory pool for fixed-size elements with free list recycling.
pub struct MemPool<T> {
    slots: Vec<Slot<T>>,
    free_head: Option<u32>,
    active_count: usize,
}

impl<T> MemPool<T> {
    pub fn new() -> Self {
        Self::with_capacity(64)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            slots: Vec::with_capacity(capacity),
            free_head: None,
            active_count: 0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.active_count
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.active_count == 0
    }

    /// Allocates an element in the pool and returns its generational handle.
    pub fn alloc(&mut self, value: T) -> Handle<T> {
        self.active_count += 1;
        if let Some(free_idx) = self.free_head {
            let slot = &mut self.slots[free_idx as usize];
            match slot {
                Slot::Vacant { next_free, generation } => {
                    self.free_head = *next_free;
                    let gen = *generation;
                    *slot = Slot::Occupied {
                        value,
                        generation: gen,
                    };
                    Handle::new(free_idx, gen)
                }
                Slot::Occupied { .. } => unreachable!("corrupt free list in MemPool"),
            }
        } else {
            let index = self.slots.len() as u32;
            let generation = 1;
            self.slots.push(Slot::Occupied { value, generation });
            Handle::new(index, generation)
        }
    }

    /// Returns a reference to the element if the handle is valid and generation matches.
    #[inline]
    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        let slot = self.slots.get(handle.index as usize)?;
        match slot {
            Slot::Occupied { value, generation } if *generation == handle.generation => Some(value),
            _ => None,
        }
    }

    /// Returns a mutable reference to the element if the handle is valid and generation matches.
    #[inline]
    pub fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T> {
        let slot = self.slots.get_mut(handle.index as usize)?;
        match slot {
            Slot::Occupied { value, generation } if *generation == handle.generation => Some(value),
            _ => None,
        }
    }

    /// Frees an element by handle and increments its generation to invalidate dangling handles.
    pub fn free(&mut self, handle: Handle<T>) -> Option<T> {
        let slot = self.slots.get_mut(handle.index as usize)?;
        match slot {
            Slot::Occupied { generation, .. } if *generation == handle.generation => {
                let next_gen = generation.wrapping_add(1);
                let old_slot = std::mem::replace(
                    slot,
                    Slot::Vacant {
                        next_free: self.free_head,
                        generation: next_gen,
                    },
                );
                self.free_head = Some(handle.index);
                self.active_count -= 1;
                match old_slot {
                    Slot::Occupied { value, .. } => Some(value),
                    Slot::Vacant { .. } => unreachable!(),
                }
            }
            _ => None,
        }
    }

    /// Iterate over active (handle, &value) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (Handle<T>, &T)> {
        self.slots.iter().enumerate().filter_map(|(idx, slot)| match slot {
            Slot::Occupied { value, generation } => {
                Some((Handle::new(idx as u32, *generation), value))
            }
            Slot::Vacant { .. } => None,
        })
    }

    /// Iterate mutably over active (handle, &mut value) pairs.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Handle<T>, &mut T)> {
        self.slots.iter_mut().enumerate().filter_map(|(idx, slot)| match slot {
            Slot::Occupied { value, generation } => {
                Some((Handle::new(idx as u32, *generation), value))
            }
            Slot::Vacant { .. } => None,
        })
    }
}

impl<T> Default for MemPool<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mempool_alloc_and_free() {
        let mut pool = MemPool::new();
        let h1 = pool.alloc(10);
        let h2 = pool.alloc(20);

        assert_eq!(pool.len(), 2);
        assert_eq!(*pool.get(h1).unwrap(), 10);
        assert_eq!(*pool.get(h2).unwrap(), 20);

        let val = pool.free(h1);
        assert_eq!(val, Some(10));
        assert_eq!(pool.len(), 1);
        assert!(pool.get(h1).is_none());

        // Reallocate reuses slot with new generation
        let h3 = pool.alloc(30);
        assert_eq!(h3.index, h1.index);
        assert_ne!(h3.generation, h1.generation);
        assert_eq!(*pool.get(h3).unwrap(), 30);
        // Stale handle still fails
        assert!(pool.get(h1).is_none());
    }
}





pub mod vk_memory_layout;

pub mod vk_memory_pool;





pub mod BLI_map;

pub mod BLI_resource_scope;

pub mod eevee_shadow_page_defrag_bsl;

pub mod eevee_shadow_page_ops_bsl;

pub mod eevee_shadow_tilemap_amend_bsl;

pub mod workbench_shadow_raytrace_bsl;

pub mod sculpt_intern;

pub mod grease_pencil_intern;














pub mod BKE_preview_image;





pub mod eevee_deferred_thickness_amend_bsl;

pub mod eevee_depth_of_field_gather_bsl;



pub mod gpu_query;

pub mod mtl_texture;

pub mod gpu_shader_sequencer_scope_bsl;



pub mod vk_device;

pub mod vk_memory;
pub mod BLI_mempool;
pub use BLI_mempool::*;
pub mod BLI_ghash;
pub use BLI_ghash::*;








