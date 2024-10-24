use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

#[test]
fn test() {
    let should_print = Arc::new(RwLock::new(false));

    let should_print_clone = Arc::clone(&should_print);
    let handle_1 = thread::spawn(move || {
        for _ in 0..10 {
            let guard = should_print_clone.read().unwrap();
            if *guard {
                println!("Thread 1: Hello!");
                thread::sleep(Duration::from_secs(10));
            }
        }
    });

    let should_print_clone_2 = Arc::clone(&should_print);
    let handle_2 = thread::spawn(move || {
        for _ in 0..10 {
            if *should_print_clone_2.read().unwrap() {
                println!("Thread 2: Hello!");
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    *should_print.write().unwrap() = true;

    handle_1.join().unwrap();
    handle_2.join().unwrap();
}
