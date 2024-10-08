use std::{
    io::{BufRead as _, BufReader, BufWriter, Write as _},
    net::TcpStream,
};

fn main() {
    let addr = std::env::args().nth(1).unwrap();

    let stream = TcpStream::connect(addr).unwrap();
    stream.set_nonblocking(true).unwrap();
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer= BufWriter::new(stream);

    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        writer.write_all(line.as_bytes()).unwrap();
        writer.flush().unwrap();

        loop {
            let mut input_msg = String::new();
            match reader.read_line(&mut input_msg) {
                Ok(_) => {
                    println!("> {input_msg}");
                }
                Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                    break;
                },
                Err(err) => panic!("{err}"),
            };
        }
    }
}
