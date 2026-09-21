use indexmap::IndexMap;
use std::hash::Hash;
use std::sync::RwLock;

// Map that can only get and insert values. Inserted values can not be removed or updated.
pub struct GrowMap<K, V> {
    inner: RwLock<IndexMap<K, V>>,
}

impl<K, V> GrowMap<K, V> {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(IndexMap::new()),
        }
    }
}

unsafe impl<K, V> Send for GrowMap<K, V> {}
unsafe impl<K, V> Sync for GrowMap<K, V> {}

impl<K, V> GrowMap<K, V>
where
    K: Eq + Hash,
{
    pub fn get_or_insert_with<F: FnOnce() -> V>(&self, key: K, default: F) -> &V {
        if let Some(existing) = self.get(&key) {
            return existing;
        }
        let mut write = self.inner.write().unwrap();
        let new_value = default();
        let created = write.entry(key).insert_entry(new_value);
        // SAFETY: created entries can not be removed or changed due to this struct's API, so it
        // is safe to treat references to its entries as immutable. `IndexMap` also guarantees
        // that order of entries can not change.
        return unsafe { core::mem::transmute::<&V, &V>(created.get()) };
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let read = self.inner.read().unwrap();
        if let Some(existing) = read.get(key) {
            // SAFETY: created entries can not be removed or changed due to this struct's API, so it
            // is safe to treat references to its entries as immutable. `IndexMap` also guarantees
            // that order of entries can not change.
            return Some(unsafe { core::mem::transmute::<&V, &V>(existing) });
        }
        return None;
    }
}
