use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("request:");
    println!("{}", String::from_utf8_lossy(&buffer));

    let response = b"HTTP/1.1 200 OK\r\n\
                   Content-Length: 39\r\n\r\n\
                   <html><body><p>Hello!</p></body></html>";
    stream.write(response).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
