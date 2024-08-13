struct MyIterator {
    value: u32,
}

impl MyIterator {
    fn new(value: u32) -> MyIterator {
        MyIterator { value }
    }
}

// impl Iterator for MyIterator {
//     type Item = &u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         Some(&self.value)
//     }
// }

trait LendingIterator {
    type Item<'a> where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}

impl LendingIterator for MyIterator {
    type Item<'a> = &'a u32;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        Some(&self.value)
    }
}

#[test]
fn example() {
    let mut it = MyIterator::new(10);
    assert_eq!(it.next(), Some(&10));
}
