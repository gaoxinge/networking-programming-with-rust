use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::task::Waker;
use std::time::Duration;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use futures::task::ArcWake;
use futures::task::waker_ref;

// Ready Future
struct Ready(u32);

impl Future for Ready {
    type Output = u32;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.0)
    }
}

// Counter Future
struct Counter {
    count: u32,
    total: u32,
}

impl Counter {
    fn new(total: u32) -> Counter {
        Counter { count: 0, total }
    }
}

impl Future for Counter {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.count {
            n if n == self.total => Poll::Ready(self.total),
            _ => {
                println!("count: {}", self.count);
                self.count += 1;
                ctx.waker().clone().wake();
                Poll::Pending
            }
        }
    }
}

// Timer Future
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

// block_on Executor
struct Task {
    ok: Mutex<bool>,
}

impl Task {
    fn new() -> Task {
        Task { ok: Mutex::new(true) }
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let mut ok = arc_self.ok.lock().unwrap();
        *ok = true;
    }
}

fn block_on<T>(future: T) -> T::Output
where
    T: Future,
{   
    let task = Arc::new(Task::new());
    let mut future = Box::pin(future);
    loop {
        let mut ok = task.ok.lock().unwrap();
        if *ok {
            *ok = false;
            drop(ok);
            let waker = waker_ref(&task);
            let ctx = &mut Context::from_waker(&*waker);
            if let Poll::Ready(val) = future.as_mut().poll(ctx) {
                return val;
            }
        }
    }
}

fn main() {
    let ready = Ready(5);
    let result = block_on(ready);
    println!("{}", result);

    let counter = Counter::new(5);
    let total = block_on(counter);
    println!("total: {}", total);

    let timer = Timer::new(Duration::new(5, 0));
    println!("Hello");
    block_on(timer);
    println!("World");
}
