// https://fasterthanli.me/articles/a-rust-match-made-in-hell
// https://rust-lang.github.io/rust-clippy/master/index.html#await_holding_lock
// https://rust-lang.github.io/rfcs/3014-must-not-suspend-lint.html
// https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/builtin/static.MUST_NOT_SUSPEND.html

#![deny(clippy::await_holding_lock)]
#![feature(must_not_suspend)]
#![deny(must_not_suspend)]

use futures::future::join_all;
use std::{sync::Mutex, time::Duration};
use tokio::time::sleep;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // There is no deadlock here.
    let res: Mutex<String> = Default::default();
    join_all("abc".chars().map(|name| {
        let res = &res;
        async move {
            for _ in 0..5 {
                sleep(Duration::from_millis(10)).await;
                res.lock().unwrap().push(name);
            }
        }
    }))
    .await;
    println!("res = {}", res.into_inner().unwrap());

    // There is a deadlock here.
    let res: Mutex<String> = Default::default();
    join_all("abc".chars().map(|_name| {
        let _res = &res;
        async move {
            for _ in 0..5 {
                // let mut guard = res.lock().unwrap();
                sleep(Duration::from_millis(10)).await;
                // guard.push(name);
            }
        }
    }))
    .await;
    println!("res = {}", res.into_inner().unwrap());

    // There would be no deadlock with [`futures::lock::Mutex`] or [`tokio::sync::Mutex`].

    // There is Clippy `await_holding_lock` lint that might help with this.
    //     error: this `MutexGuard` is held across an `await` point
    //     --> my/async/src/bin/deadlock_with_regular_mutex.rs:36:21
    //      |
    //   36 |                 let mut guard = res.lock().unwrap();
    //      |                     ^^^^^^^^^
    //      |
    //      = help: consider using an async-aware `Mutex` type or ensuring the `MutexGuard` is dropped before calling await
    //   note: these are all the `await` points this lock is held through
    //     --> my/async/src/bin/deadlock_with_regular_mutex.rs:37:50
    //      |
    //   37 |                 sleep(Duration::from_millis(10)).await;
    //      |                                                  ^^^^^
    //      = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#await_holding_lock
    //   note: the lint level is defined here
    //     --> my/async/src/bin/deadlock_with_regular_mutex.rs:8:9
    //      |
    //   6  | #![deny(clippy::await_holding_lock)]
    //      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^

    // And also nightly-only `must_not_suspend` lint.
    //     error: `std::sync::MutexGuard` held across a suspend point, but should not be
    //     --> my/async/src/bin/deadlock_with_regular_mutex.rs:36:21
    //      |
    //   36 |                 let mut guard = res.lock().unwrap();
    //      |                     ^^^^^^^^^
    //   37 |                 sleep(Duration::from_millis(10)).await;
    //      |                                                  ----- the value is held across this suspend point
    //      |
    //   note: holding a MutexGuard across suspend points can cause deadlocks, delays, and cause Futures to not implement `Send`
    //     --> my/async/src/bin/deadlock_with_regular_mutex.rs:36:21
    //      |
    //   36 |                 let mut guard = res.lock().unwrap();
    //      |                     ^^^^^^^^^
    //   help: consider using a block (`{ ... }`) to shrink the value's scope, ending before the suspend point
    //     --> my/async/src/bin/deadlock_with_regular_mutex.rs:36:21
    //      |
    //   36 |                 let mut guard = res.lock().unwrap();
    //      |                     ^^^^^^^^^
    //   note: the lint level is defined here
    //     --> my/async/src/bin/deadlock_with_regular_mutex.rs:7:9
    //      |
    //   8  | #![deny(must_not_suspend)]
    //      |         ^^^^^^^^^^^^^^^^
}
