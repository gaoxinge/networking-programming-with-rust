use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::task::Waker;
use std::time::Duration;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use futures::executor::block_on;

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

struct Timer {
    shared_state: Arc<Mutex<SharedState>>,
}

impl Timer {
    fn new(duration: Duration) -> Timer {
        let shared_state = Arc::new(Mutex::new(SharedState { completed: false, waker: None }));
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
            }
        });
        Timer { shared_state }
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(ctx.waker().clone());
            Poll::Pending
        }
    }
}

fn main() {
    let timer = Timer::new(Duration::new(5, 0));
    println!("Hello");
    block_on(timer);
    println!("World");
}
