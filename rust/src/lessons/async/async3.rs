// https://hegdenu.net/posts/understanding-async-await-1
// The simplest `Future` implementation example.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// `Hello` task that can be in a couple of states.
enum Hello {
    Init { name: &'static str },
    Done,
}

// Implement `Future` trait for `Hello` task to make it real task with `poll` method.
impl Future for Hello {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match *self {
            Hello::Init { name } => println!("hello, {name}!"),
            Hello::Done => panic!("Please stop polling me!"),
        };

        *self = Hello::Done;
        Poll::Ready(())
    }
}

// `hello` function returns task that implements `Future` trait.
// In this case it's `Hello` task.
fn hello(name: &'static str) -> impl Future<Output = ()> {
    Hello::Init { name }
}

#[tokio::main]
async fn main() {
    // Get `Hello` task and `.await` it.
    hello("world").await;

    // Note, that task won't do anything until `.await` it.
    // Our task doesn't execute immediately because it's created in `Init` state and just waits for somebody to `poll` it.
    hello("world");
    // warning: unused implementer of `Future` that must be used
    //   --> src/lessons/async/async3.rs:39:5
    //    |
    // 39 |     hello("world");
    //    |     ^^^^^^^^^^^^^^
    //    |
    //    = note: futures do nothing unless you `.await` or poll them
    //    = note: `#[warn(unused_must_use)]` on by default

    // This means that our `Future`s are lazy, we can save them and do what we want without running.
    let _task = hello("world");
}
