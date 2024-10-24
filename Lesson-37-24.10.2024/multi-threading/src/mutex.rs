use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[test]
fn test() {
    let should_print = Arc::new(Mutex::new(false));

    let should_print_clone = Arc::clone(&should_print);
    let handle = thread::spawn(move || {
        for _ in 0..10 {
            if *should_print_clone.lock().unwrap() {
                println!("Thread: Hello!");
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    *should_print.lock().unwrap() = true;

    handle.join().unwrap();
}
