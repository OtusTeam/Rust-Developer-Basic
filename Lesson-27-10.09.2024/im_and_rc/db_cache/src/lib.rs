use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

pub trait KeyValueStorage<K, V> {
    fn get(&self, key: &K) -> Option<V>;
}

pub struct CachedStorage<K, V, S> {
    storage: S,
    cache: RefCell<HashMap<K, Option<V>>>,
}

impl<K, V, S> CachedStorage<K, V, S> {
    pub fn new(storage: S) -> Self {
        Self {
            storage,
            cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn into_inner(self) -> S {
        self.storage
    }
}

impl<K: Eq + Hash + Clone, V: Clone, S: KeyValueStorage<K, V>> KeyValueStorage<K, V>
    for CachedStorage<K, V, S>
{
    fn get(&self, key: &K) -> Option<V> {
        use std::collections::hash_map::Entry;

        match self.cache.borrow_mut().entry(key.clone()) {
            Entry::Occupied(occupied) => occupied.get().clone(),
            Entry::Vacant(vacant) => {
                let value_opt = self.storage.get(key);
                vacant.insert(value_opt.clone());
                value_opt
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;

    struct HashMapStorage<K, V> {
        map: HashMap<K, V>,
        access_counter: Cell<u32>,
    }

    impl<K, V> HashMapStorage<K, V> {
        fn new(map: HashMap<K, V>) -> Self {
            Self { map, access_counter: Cell::new(0)  }
        }
    }

    impl<K: Eq + Hash, V: Clone> KeyValueStorage<K, V> for HashMapStorage<K, V> {
        fn get(&self, key: &K) -> Option<V> {
            let counter = self.access_counter.get();
            self.access_counter.set(counter + 1);

            self.map.get(key).cloned()
        }
    }

    #[test]
    fn cache_works() {
        let storage = HashMapStorage::new(HashMap::from([(1, "one"), (2, "two")]));
        let cached_storage = CachedStorage::new(storage);

        let first = cached_storage.get(&1);
        let second = cached_storage.get(&1);
        assert_eq!(first, Some("one"));
        assert_eq!(second, Some("one"));

        let storage = cached_storage.into_inner();
        assert_eq!(storage.access_counter.get(), 1);
    }
}
