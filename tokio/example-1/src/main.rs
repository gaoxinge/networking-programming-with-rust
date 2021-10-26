use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let server = TcpListener::bind("127.0.0.1:8000").await.unwrap();

    loop {
        let (mut connection, address) = server.accept().await.unwrap();
        println!("accept connection from {}", address);

        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            connection.read(&mut buffer).await.unwrap();
            println!("request:");
            println!("{}", String::from_utf8_lossy(&buffer));

            let response = b"HTTP/1.1 200 OK\r\n\
                           Content-Length: 39\r\n\r\n\
                           <html><body><p>Hello!</p></body></html>";
            connection.write_all(response).await.unwrap();
        });
    }
}

