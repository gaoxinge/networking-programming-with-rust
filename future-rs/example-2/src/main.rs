use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use futures::executor::block_on;

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

fn main() {
    let counter = Counter::new(5);
    let total = block_on(counter);
    println!("total: {}", total);
}
