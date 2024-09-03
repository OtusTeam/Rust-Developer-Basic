use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Debug, Default)]
pub struct Deduplicator {
    items: HashMap<TypeId, Vec<Box<dyn Any>>>,
}

impl Deduplicator {
    pub fn check<T: Any + Eq>(&mut self, item: T) -> bool {
        use std::collections::hash_map::Entry;

        let type_id = item.type_id();

        match self.items.entry(type_id) {
            Entry::Occupied(mut occupied) => {
                let previous_items = occupied.get_mut();

                let found = previous_items
                    .iter()
                    .find(|prev| {
                        let prev = prev.downcast_ref::<T>().unwrap();
                        prev == &item
                    })
                    .is_some();

                if found {
                    false
                } else {
                    previous_items.push(Box::new(item));
                    true
                }
            }
            Entry::Vacant(vacant) => {
                vacant.insert(vec![Box::new(item)]);
                true
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Clone, PartialEq, Eq)]
    struct Message {
        value: u32,
    }

    #[test]
    fn check_not_fail_for_first() {
        let mut deduplicator = Deduplicator::default();

        let msg = Message { value: 5 };
        assert!(deduplicator.check(msg.clone()));
    }

    #[test]
    fn check_fail_for_duplicate() {
        let mut deduplicator = Deduplicator::default();

        let msg = Message { value: 5 };
        deduplicator.check(msg.clone());
        assert!(!deduplicator.check(msg));
    }

    #[test]
    fn check_different_types() {
        let mut deduplicator = Deduplicator::default();

        let msg = Message { value: 5 };
        assert!(deduplicator.check(msg.clone()));

        #[derive(Clone, PartialEq, Eq)]
        struct Message2 {
            value: u32,
        }

        let msg = Message2 { value: 5 };
        assert!(deduplicator.check(msg.clone()));
    }
}
