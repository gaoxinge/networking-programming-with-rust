use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();
    stream.write(b"Hello world!").unwrap();
    
    let mut buffer = [0; 4];
    let mut n = 0;
    while n < 12 {
        let m = stream.read(&mut buffer).unwrap();
        print!("{}", String::from_utf8_lossy(&buffer));
        n += m;
    }
}
