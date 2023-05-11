use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:8123").unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection(stream);
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer: [u8; 1024] = [0; 1024];

  stream.read(&mut buffer).unwrap();

  // Check what is being requested
  let (status_line, filename) = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
    ("200 OK", "index.html")
  } else {
    ("404 Not Found", "404.html")
  };

  // Read the file
  let content = fs::read_to_string(filename).unwrap();

  // formulate response and send back
  let response = format!(
    "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
    status_line,
    content.len(),
    content
  );
  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}
