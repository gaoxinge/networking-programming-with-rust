use std::io::prelude::*;
use std::collections::HashMap;
use mio::Token;
use mio::Interest;
use mio::Poll;
use mio::event::Events;
use mio::net::TcpListener;

fn main() {
    let mut token_index = 0;
    let mut poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(128);
    let mut connections = HashMap::new();

    let addr = "127.0.0.1:8000".parse().unwrap();
    let mut server = TcpListener::bind(addr).unwrap();
    poll.registry().register(&mut server, Token(0), Interest::READABLE).unwrap();

    loop {
        poll.poll(&mut events, None).unwrap();
        for event in events.iter() {
            match event.token() {
                Token(0) => {
                    let (mut connection, address) = server.accept().unwrap();
                    println!("accept connection from {}", address);

                    token_index += 1;
                    let token = Token(token_index);
                    poll.registry().register(&mut connection, token, Interest::READABLE).unwrap();
                    connections.insert(token, connection);
                    
                    // poll.registry().deregister(&mut server).unwrap();
                    // poll.registry().register(&mut server, Token(0), Interest::READABLE).unwrap();
                },
                token => {
                    let connection = connections.get_mut(&token).unwrap();
                    if event.is_readable() {
                        let mut buffer = [0; 1024];
                        connection.read(&mut buffer).unwrap();
                        println!("request:");
                        println!("{}", String::from_utf8_lossy(&buffer));          
                        poll.registry().reregister(connection, token, Interest::WRITABLE).unwrap();
                    }
                    
                    if event.is_writable() {
                        let response = b"HTTP/1.1 200 OK\r\n\
                                       Content-Length: 39\r\n\r\n\
                                       <html><body><p>Hello!</p></body></html>";
                        connection.write(response).unwrap();
                        connection.flush().unwrap();
                        connections.remove(&token);
                    }
                }
            }
        }
    }
}
