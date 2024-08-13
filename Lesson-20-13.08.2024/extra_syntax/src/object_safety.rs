struct Cat;

struct Dog;

trait MakeNoise {
    // fn new_animal() -> Self;

    fn new_animal() -> Self where Self: Sized;

    fn make_noise(&self);
}

impl MakeNoise for Cat {
    fn new_animal() -> Cat {
        Cat
    }

    fn make_noise(&self) {
        println!("Meow");
    }
}

impl MakeNoise for Dog {
    fn new_animal() -> Dog {
        Dog
    }

    fn make_noise(&self) {
        println!("Woof");
    }
}

#[test]
fn example() {
    let mut v = Vec::<Box<dyn MakeNoise>>::new();
    v.push(Box::new(Cat));
    v.push(Box::new(Dog));

    for animal in v {
        animal.make_noise();
    }
}
