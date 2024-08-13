struct Closure<F> {
    x: i32,
    f: F,
}

impl<F> Closure<F> {
    fn new(x: i32, f: F) -> Self {
        Self { x, f }
    }
}

impl<F> Closure<F>
where
    F: for<'a, 'b> Fn(&'a i32, &'b i32) -> &'a i32
{
    fn call(&self) {
        (self.f)(&self.x, &self.x);
    }
}

#[test]
fn example() {
    fn foo<'a, 'b>(x: &'a i32, _y: &'b i32) -> &'a i32{
        println!("x: {x}");
        x
    }

    let closure = Closure::new(10, foo);
    closure.call();
}
