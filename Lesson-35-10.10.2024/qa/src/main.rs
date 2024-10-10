fn main() {
    println!("Hello World!");
}

mod iter {
    use std::collections::HashSet;

    #[test]
    fn test_into_iter_by_value() {
        fn takes_by_value<T: IntoIterator<Item = i32>>(collection: T) {
            for item in collection {
                println!("{item}");
            }
        }

        let arr = [1, 2, 3, 4, 5];
        takes_by_value(arr);

        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        takes_by_value(set);
    }

    #[test]
    fn test_into_iter_by_ref() {
        fn takes_by_value<'item, T: IntoIterator<Item = &'item i32>>(collection: T) {
            for item in collection {
                println!("{item}");
            }
        }

        let arr = [1, 2, 3, 4, 5];
        takes_by_value(&arr);

        dbg!(arr);

        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        takes_by_value(&set);

        // let map = HashMap::from_iter([(5, "five")]);
    }

    #[test]
    fn test_iter() {
        let arr = [
            String::from("one"),
            String::from("two"),
            String::from("three"),
        ];

        arr.iter()
            .filter(|item| item.len() % 2 == 0)
            .for_each(|item| {
                println!("{item}");
            });

        dbg!(arr);
    }

    #[test]
    fn test_itertools_chunks() {
        use itertools::Itertools as _;

        let data = vec![1, 1, 2, -2, 6, 0, 3, 1];
        for chunk in &data.into_iter().chunks(3) {
            assert_eq!(4, chunk.sum());
        }
    }
}

mod replace {
    #[test]
    fn test_replace() {
        let mut v: Vec<i32> = vec![1, 2];

        let old_v = std::mem::replace(&mut v, vec![3, 4, 5]);

        assert_eq!(vec![1, 2], old_v);
        assert_eq!(vec![3, 4, 5], v);
    }
}

mod list {
    use std::{cell::RefCell, rc::Rc};

    struct List<T> {
        head: Option<Rc<RefCell<Node<T>>>>,
    }

    struct Node<T> {
        value: T,
        next: Option<Rc<RefCell<Node<T>>>>,
    }

    impl<T> IntoIterator for List<T> {
        type Item = T;

        type IntoIter = LinkedListIterator<T>;

        fn into_iter(self) -> Self::IntoIter {
            LinkedListIterator(self.head)
        }
    }

    struct LinkedListIterator<T>(Option<Rc<RefCell<Node<T>>>>);

    impl<T> Iterator for LinkedListIterator<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            let Some(node) = self.0.take() else {
                return None;
            };

            let node = Rc::into_inner(node).expect("No other references expected");
            let node = node.into_inner();

            self.0 = node.next;
            Some(node.value)
        }
    }

    #[test]
    fn test_into_iter() {
        let list = List {
            head: Some(Rc::new(RefCell::new(Node {
                value: 1,
                next: Some(Rc::new(RefCell::new(Node {
                    value: 2,
                    next: Some(Rc::new(RefCell::new(Node {
                        value: 3,
                        next: None,
                    })))
                })))
            })))
        };

        assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![1, 2, 3]);
    }
}

mod rc {
    #[test]
    fn test_rc_option() {
        let value = std::rc::Rc::new(Some(String::from("Hello")));

        match &*value {
            Some(n) => println!("{n}"),
            None => println!("No value"),
        }
    }
}
