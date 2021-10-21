use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_connection(mut stream: TcpStream) {
    println!("start handle connection");

    let mut buffer = [0; 4];
    let mut n = 0;
    while n < 12 {
        let m = stream.read(&mut buffer).unwrap();
        print!("{}", String::from_utf8_lossy(&buffer));
        stream.write(&buffer[..m]).unwrap();
        n += m;
    }
    println!("");

    stream.flush().unwrap();
    println!("end handle connection");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
