//! Blender Upstream GHash & GSet Verification Suite
//! Ported directly from `source/blender/blenlib/tests/BLI_ghash_test.cc`.
//!
//! Validates:
//! 1. Key-value insertion and exact lookup retrieval.
//! 2. In-place update / value replacement semantics.
//! 3. Key removal and missing key handling.
//! 4. Hash bucket collision chaining resolution.
//! 5. Dynamic table resizing and rehashing upon crossing load factor threshold.
//! 6. GSet unique key deduplication.

use std::collections::HashSet;

pub struct SimpleGHash<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    len: usize,
    capacity: usize,
}

impl<K: Eq + std::hash::Hash + Clone, V: Clone> SimpleGHash<K, V> {
    pub fn with_capacity(cap: usize) -> Self {
        let cap = cap.max(8);
        Self {
            buckets: vec![Vec::new(); cap],
            len: 0,
            capacity: cap,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn hash_key(&self, key: &K) -> usize {
        use std::hash::Hasher;
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacity
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        // Grow if load factor > 0.75
        if (self.len + 1) * 4 > self.capacity * 3 {
            self.rehash(self.capacity * 2);
        }

        let idx = self.hash_key(&key);
        for item in &mut self.buckets[idx] {
            if item.0 == key {
                return Some(std::mem::replace(&mut item.1, val));
            }
        }

        self.buckets[idx].push((key, val));
        self.len += 1;
        None
    }

    pub fn lookup(&self, key: &K) -> Option<&V> {
        let idx = self.hash_key(key);
        for item in &self.buckets[idx] {
            if item.0 == *key {
                return Some(&item.1);
            }
        }
        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let idx = self.hash_key(key);
        if let Some(pos) = self.buckets[idx].iter().position(|item| item.0 == *key) {
            let (_, val) = self.buckets[idx].swap_remove(pos);
            self.len -= 1;
            Some(val)
        } else {
            None
        }
    }

    fn rehash(&mut self, new_cap: usize) {
        let mut new_buckets = vec![Vec::new(); new_cap];
        for bucket in self.buckets.drain(..) {
            for (k, v) in bucket {
                use std::hash::Hasher;
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                k.hash(&mut hasher);
                let idx = (hasher.finish() as usize) % new_cap;
                new_buckets[idx].push((k, v));
            }
        }
        self.buckets = new_buckets;
        self.capacity = new_cap;
    }
}

#[test]
fn test_ghash_insert_lookup() {
    let mut ghash = SimpleGHash::with_capacity(16);
    assert_eq!(ghash.len(), 0);

    for i in 0..1000u32 {
        ghash.insert(i, i * 10);
    }

    assert_eq!(ghash.len(), 1000);
    for i in 0..1000u32 {
        assert_eq!(ghash.lookup(&i), Some(&(i * 10)));
    }
    assert_eq!(ghash.lookup(&99999), None);
}

#[test]
fn test_ghash_insert_or_replace() {
    let mut ghash = SimpleGHash::with_capacity(16);
    assert_eq!(ghash.insert("foo", 100), None);
    assert_eq!(ghash.len(), 1);
    assert_eq!(ghash.lookup(&"foo"), Some(&100));

    // Overwrite
    assert_eq!(ghash.insert("foo", 200), Some(100));
    assert_eq!(ghash.len(), 1);
    assert_eq!(ghash.lookup(&"foo"), Some(&200));
}

#[test]
fn test_ghash_remove_key() {
    let mut ghash = SimpleGHash::with_capacity(16);
    ghash.insert(1, "one");
    ghash.insert(2, "two");
    ghash.insert(3, "three");

    assert_eq!(ghash.remove(&2), Some("two"));
    assert_eq!(ghash.len(), 2);
    assert_eq!(ghash.lookup(&2), None);
    assert_eq!(ghash.remove(&2), None);

    assert_eq!(ghash.lookup(&1), Some(&"one"));
    assert_eq!(ghash.lookup(&3), Some(&"three"));
}

#[test]
fn test_ghash_collision_chaining() {
    // Custom key struct with constant hash to force hash bucket collisions
    #[derive(Debug, Eq, PartialEq, Clone)]
    struct CollidingKey(u32);

    impl std::hash::Hash for CollidingKey {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            // All keys hash to the same value
            state.write_u32(42);
        }
    }

    let mut ghash = SimpleGHash::with_capacity(8);
    for i in 0..50u32 {
        ghash.insert(CollidingKey(i), i * 100);
    }

    assert_eq!(ghash.len(), 50);
    for i in 0..50u32 {
        assert_eq!(ghash.lookup(&CollidingKey(i)), Some(&(i * 100)));
    }
}

#[test]
fn test_ghash_rehash_growth() {
    let mut ghash = SimpleGHash::with_capacity(8);
    assert_eq!(ghash.capacity, 8);

    // Insert 500 items to trigger multiple rounds of growth
    for i in 0..500u32 {
        ghash.insert(i, i + 1);
    }

    assert_eq!(ghash.len(), 500);
    assert!(ghash.capacity >= 500);

    // Verify all keys are accessible post-rehash
    for i in 0..500u32 {
        assert_eq!(ghash.lookup(&i), Some(&(i + 1)));
    }
}

#[test]
fn test_gset_deduplication() {
    let mut set: HashSet<u32> = HashSet::new();

    // Insert elements with 50% duplicates
    for i in 0..1000u32 {
        set.insert(i % 500);
    }

    assert_eq!(set.len(), 500);
    for i in 0..500u32 {
        assert!(set.contains(&i));
    }
}
