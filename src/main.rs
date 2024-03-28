use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use itertools::Itertools;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut buf = [0; 512];
                // let mut buf = String::new();
                stream.read(&mut buf).unwrap();
                // let mut buf_reader = BufReader::new(stream.try_clone().unwrap());
                // buf_reader.read_line(&mut buf).unwrap();
                let request = std::str::from_utf8(&buf).unwrap();
                println!("request: {request}");
                let mut lines = request.split("\r\n");
                let first_line = lines.nth(0).unwrap();
                let first = first_line.split(" ").collect::<Vec<_>>()[1];
                // println!("first_line = {first_line}");
                if first == "/" {
                    respond_ok(&mut stream, None, None);
                } else if let Some(rest) = first.strip_prefix("/echo/") {
                    respond_ok(
                        &mut stream,
                        Some(ContentType::TEXT_PLAIN),
                        Some(rest.to_owned()),
                    );
                } else if first.starts_with("/user-agent") {
                    let mut body: Option<&str> = None;
                    for line in lines {
                        if line.starts_with("User-Agent: ") {
                            body = Some(line.split("User-Agent: ").collect::<Vec<_>>()[1]);
                        }
                    }
                    match body {
                        Some(body) => respond_ok(
                            &mut stream,
                            Some(ContentType::TEXT_PLAIN),
                            Some(body.to_owned()),
                        ),
                        None => respond_not_found(&mut stream),
                    };
                } else {
                    respond_not_found(&mut stream);
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn respond_ok(stream: &mut TcpStream, content_type: Option<ContentType>, body: Option<String>) {
    let mut content = String::new();
    content.push_str("HTTP/1.1 200 OK\r\n");
    if let Some(content_type) = content_type {
        content.push_str(format!("Content-Type: {}\r\n", content_type.0).as_str());
    }
    if let Some(body) = body {
        content.push_str(format!("Content-Length: {}\r\n", body.len()).as_str());
        content.push_str("\r\n");
        content.push_str(format!("{}\r\n", body).as_str());
    }
    content.push_str("\r\n");
    stream
        .write_all(content.as_bytes())
        .expect("Failed to write to stream");
}

fn respond_not_found(stream: &mut TcpStream) {
    stream
        .write_all(b"HTTP/1.1 404 Not Found\r\n\r\n")
        .expect("Failed to write to stream");
}

struct ContentType(&'static str);

impl ContentType {
    const TEXT_PLAIN: Self = Self("text/plain");
}
