use std::{io::Write, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                stream
                    .write_all(b"HTTP/1.1 200 OK\r\n\r\n")
                    .expect("Failed to write to stream");
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
