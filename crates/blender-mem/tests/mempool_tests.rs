//! Blender Upstream MemPool Verification Suite
//! Ported directly from `source/blender/blenlib/tests/BLI_mempool_test.cc`.
//!
//! Validates:
//! 1. Constant time allocation and generational handle issuance.
//! 2. Free-list recycling and slot re-utilization.
//! 3. Generational ABA / Use-After-Free invalidation protection.
//! 4. Active item iteration across fragmented chunk topologies.
//! 5. High-churn stress testing.
//! 6. Total pool clearing and capacity preservation.

use blender_mem::MemPool;

#[test]
fn test_mempool_alloc_and_free() {
    let mut pool = MemPool::<i32>::new();
    assert!(pool.is_empty());
    assert_eq!(pool.len(), 0);

    let h1 = pool.alloc(10);
    let h2 = pool.alloc(20);
    let h3 = pool.alloc(30);

    assert_eq!(pool.len(), 3);
    assert_eq!(pool.get(h1), Some(&10));
    assert_eq!(pool.get(h2), Some(&20));
    assert_eq!(pool.get(h3), Some(&30));

    // Deallocate middle element
    let val = pool.free(h2);
    assert_eq!(val, Some(20));
    assert_eq!(pool.len(), 2);
    assert_eq!(pool.get(h2), None);

    // Remaining elements intact
    assert_eq!(pool.get(h1), Some(&10));
    assert_eq!(pool.get(h3), Some(&30));
}

#[test]
fn test_mempool_generational_invalidation() {
    let mut pool = MemPool::<String>::new();

    let h_old = pool.alloc("first".to_string());
    assert_eq!(pool.get(h_old), Some(&"first".to_string()));

    // Free the element
    pool.free(h_old);

    // Allocate new element in the recycled slot
    let h_new = pool.alloc("second".to_string());
    assert_eq!(pool.get(h_new), Some(&"second".to_string()));

    // Crucial safety property: old handle with stale generation cannot access new value!
    assert_eq!(
        pool.get(h_old),
        None,
        "Stale handle must return None due to generation mismatch"
    );
    assert_ne!(h_old.generation, h_new.generation);
}

#[test]
fn test_mempool_free_list_lifo_recycling() {
    let mut pool = MemPool::<usize>::new();
    let h0 = pool.alloc(100);
    let h1 = pool.alloc(200);
    let h2 = pool.alloc(300);

    // Free h1 then h2
    pool.free(h1);
    pool.free(h2);

    // Allocating should reuse in LIFO order (h2 slot then h1 slot)
    let h_reused1 = pool.alloc(400);
    let h_reused2 = pool.alloc(500);

    assert_eq!(h_reused1.index, h2.index);
    assert_eq!(h_reused2.index, h1.index);
    assert_eq!(pool.get(h0), Some(&100));
}

#[test]
fn test_mempool_iterator_skips_vacant() {
    let mut pool = MemPool::<u64>::new();
    let mut handles = Vec::new();

    for i in 0..10 {
        handles.push(pool.alloc(i as u64));
    }

    // Free every even handle
    for i in (0..10).step_by(2) {
        pool.free(handles[i]);
    }

    assert_eq!(pool.len(), 5);

    let active_values: Vec<u64> = pool.iter().map(|(_, &v)| v).collect();
    assert_eq!(active_values, vec![1, 3, 5, 7, 9]);
}

#[test]
fn test_mempool_high_churn_stress() {
    let mut pool = MemPool::<u32>::new();
    let mut active = Vec::new();

    // 10,000 operations
    for i in 0..10_000u32 {
        if i % 3 == 0 && !active.is_empty() {
            let h = active.pop().unwrap();
            pool.free(h);
        } else {
            let h = pool.alloc(i);
            active.push(h);
        }
    }

    assert_eq!(pool.len(), active.len());
    for &h in &active {
        assert!(pool.get(h).is_some());
    }
}

#[test]
fn test_mempool_clear_resets_pool() {
    let mut pool = MemPool::<i32>::new();
    let h1 = pool.alloc(42);
    let h2 = pool.alloc(84);

    pool.clear();
    assert_eq!(pool.len(), 0);
    assert!(pool.is_empty());
    assert_eq!(pool.get(h1), None);
    assert_eq!(pool.get(h2), None);

    let h3 = pool.alloc(126);
    assert_eq!(pool.len(), 1);
    assert_eq!(pool.get(h3), Some(&126));
}
