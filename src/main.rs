use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut buf = String::new();
                let mut buf_reader = BufReader::new(stream.try_clone().unwrap());
                buf_reader.read_line(&mut buf).unwrap();
                println!("{buf}");
                let s = buf.split(' ').collect::<Vec<_>>()[1];
                println!("s = {s}");
                if s == "/" {
                    stream
                        .write_all(b"HTTP/1.1 200 OK\r\n\r\n")
                        .expect("Failed to write to stream");
                } else {
                    stream
                        .write_all(b"HTTP/1.1 404 Not Found\r\n\r\n")
                        .expect("Failed to write to stream");
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
