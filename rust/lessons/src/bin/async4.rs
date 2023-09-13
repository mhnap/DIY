// https://hegdenu.net/posts/understanding-async-await-2

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// `Ready` task doesn't have states, it is always ready.
struct Ready;

impl Future for Ready {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Ready: poll()");

        Poll::Ready(())
        // No handling of the future being (incorrectly) polled after returning Poll::Ready.
    }
}

// `ready` function returns `Ready` task (that implements `Future` trait).
fn ready() -> Ready {
    Ready {}
}

#[tokio::main]
async fn main() {
    println!("Before ready().await");
    // What happens behind the `.await` syntax is that the poll function gets called.
    ready().await;
    println!("After ready().await");
}
