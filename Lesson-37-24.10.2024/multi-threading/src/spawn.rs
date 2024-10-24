use std::thread;

#[test]
fn test() {
    thread::scope(|s| {
        println!("Before spawn");

        s.spawn(|| {
            println!("Hello from Thread!");
        });

        println!("After spawn");
    });

    println!("After scope");
}
