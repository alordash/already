use std::collections::HashMap;
use std::hash::Hash;
use std::sync::RwLock;

pub struct GrowHashMap<K, V> {
    inner: RwLock<HashMap<K, V>>,
}

impl<K, V> GrowHashMap<K, V> {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
        }
    }
}

unsafe impl<K, V> Send for GrowHashMap<K, V> {}
unsafe impl<K, V> Sync for GrowHashMap<K, V> {}

impl<K, V> GrowHashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn get_or_insert_with<'a, F: FnOnce() -> V>(&self, key: K, default: F) -> &V {
        let read = self.inner.read().unwrap();
        if let Some(existing) = read.get(&key) {
            // TODO: SAFETY (write something along the lines that once created entries can not change)
            return unsafe { core::mem::transmute(existing) };
        }
        drop(read);
        let mut write = self.inner.write().unwrap();
        let new_value = default();
        let created = write.entry(key).insert_entry(new_value);
        // TODO: SAFETY (write something along the lines that once created entries can not change)
        return unsafe { core::mem::transmute(created.get()) };
    }
}
