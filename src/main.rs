use std::{
    env,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_stream(stream));
            }
            Err(e) => println!("error: {}", e),
        }
    }
}

fn handle_stream(mut stream: TcpStream) {
    println!("accepted new connection");
    let mut buf = [0; 512];
    stream.read(&mut buf).unwrap();
    let request = std::str::from_utf8(&buf).unwrap();
    println!("request: {request}");

    let mut lines = request.split("\r\n");
    let first_line = lines.nth(0).unwrap();
    let first = first_line.split(" ").collect::<Vec<_>>()[1];
    // println!("first_line = {first_line}");

    if first == "/" {
        respond_ok(&mut stream, None, None, None);
    } else if let Some(rest) = first.strip_prefix("/echo/") {
        respond_ok(
            &mut stream,
            Some(ContentType::TEXT_PLAIN),
            Some(rest.to_owned()),
            Some(rest.len()),
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
                Some(body.len()),
            ),
            None => respond_not_found(&mut stream),
        };
    } else if first.starts_with("/files/") {
        let filename = match first.strip_prefix("/files/") {
            Some(rest) => rest,
            None => {
                respond_not_found(&mut stream);
                return;
            }
        };
        println!("filename: {filename}");
        let args = env::args().collect::<Vec<_>>();
        println!("args: {:?}", args);
        let directory = args.get(2);
        match directory {
            Some(dir) => {
                let path = if dir.ends_with("/") {
                    format!("{dir}{filename}")
                } else {
                    format!("{dir}/{filename}")
                };
                let path = Path::new(path.as_str());
                let path_display = path.display().to_string();
                println!("path: {path_display}");
                if path.exists() {
                    match std::fs::read(path) {
                        Ok(content) => respond_ok(
                            &mut stream,
                            Some(ContentType::FILE),
                            None,
                            Some(content.len()),
                        ),
                        Err(_) => todo!(),
                    }
                } else {
                    respond_not_found(&mut stream);
                }
            }
            None => respond_not_found(&mut stream),
        }
    } else {
        respond_not_found(&mut stream);
    }
}

fn respond_ok(
    stream: &mut TcpStream,
    content_type: Option<ContentType>,
    body: Option<String>,
    len: Option<usize>,
) {
    let mut content = String::new();
    content.push_str("HTTP/1.1 200 OK\r\n");
    if let Some(content_type) = content_type {
        content.push_str(format!("Content-Type: {}\r\n", content_type.0).as_str());
    }
    if let Some(len) = len {
        content.push_str(format!("Content-Length: {}\r\n", len).as_str());
    }
    if let Some(body) = body {
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
    const FILE: Self = Self("application/octet-stream");
}
