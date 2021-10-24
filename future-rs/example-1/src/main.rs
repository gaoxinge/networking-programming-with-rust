use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use futures::executor::block_on;

struct Ready(u32);

impl Future for Ready {
    type Output = u32;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.0)
    }
}

fn main() {
    let ready = Ready(5);
    let result = block_on(ready);
    println!("{}", result);
}
