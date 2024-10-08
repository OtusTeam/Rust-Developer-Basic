use std::{io::{BufRead as _, BufReader, BufWriter, Write}, net::{SocketAddr, TcpListener, TcpStream}};

struct Client {
    addr: SocketAddr,
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

struct Message {
    from: SocketAddr,
    content: String,
}

fn main() {
    let addr = std::env::args().nth(1).unwrap();
    let listener = TcpListener::bind(addr).unwrap();
    listener.set_nonblocking(true).unwrap();

    let mut clients = Vec::new();

    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                println!("Connected with {addr}");
                clients.push(Client {
                    addr,
                    reader: BufReader::new(stream.try_clone().unwrap()),
                    writer: BufWriter::new(stream),
                })
            },
            Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {},
            Err(err) => panic!("{err}"),
        };

        let mut messages = Vec::new();
        for client in &mut clients {
            let mut input_msg = String::new();
            match client.reader.read_line(&mut input_msg) {
                Ok(_) => {
                    println!("{}: {input_msg}", client.addr);
                    messages.push(Message {
                        from: client.addr,
                        content: input_msg,
                    });
                }
                Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {},
                Err(err) => panic!("{err}"),
            };
        }

        for msg in messages {
            for client in clients.iter_mut().filter(|client| client.addr != msg.from) {
                println!("Sending {} to {}", msg.content, client.addr);
                client.writer.write_all(msg.content.as_bytes()).unwrap();
                client.writer.flush().unwrap();
            }
        }
    }
}
