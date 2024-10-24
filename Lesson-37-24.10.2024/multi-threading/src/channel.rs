use std::{sync::mpsc::Sender, thread, time::Duration};

#[test]
fn test() {
    let (request_sender, request_receiver) = std::sync::mpsc::channel::<(String, Sender<String>)>();

    let handle = thread::spawn(move || {
        while let Ok((request, response_sender)) = request_receiver.recv() {
            let response = expensive_computations(request);
            response_sender.send(response).unwrap();
        }
    });

    let (response_sender, response_receiver) = std::sync::mpsc::channel();
    let request = String::from("Request");
    request_sender.send((request, response_sender)).unwrap();
    drop(request_sender);

    let response = response_receiver.recv().unwrap();
    println!("{response}");

    handle.join().unwrap();
}

fn expensive_computations(_request: String) -> String {
    thread::sleep(Duration::from_secs(1));

    String::from("Response")
}
