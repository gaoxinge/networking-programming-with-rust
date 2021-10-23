use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;

type Job = Box<dyn FnOnce() + Send + 'static>; 

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            job();
        });
        Worker { id, thread }
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            let receiver = Arc::clone(&receiver);
            workers.push(Worker::new(id, receiver));
        }
        ThreadPool { workers, sender }
    }

    fn spawn(&self, f: impl FnOnce() + Send + 'static) {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

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
    let threadpool = ThreadPool::new(4);
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        threadpool.spawn(|| {
            handle_connection(stream);
        });
    }
}
