// https://www.sobyte.net/post/2022-02/rust-mutex-send
// https://users.rust-lang.org/t/sending-type-that-doesnt-implement-send/69665

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

struct A {
    val: Rc<u32>,
}

fn main() {
    let mutex = Mutex::new(A { val: Rc::new(5) });
    let target = Arc::new(mutex);

    let t = thread::spawn(move || {
        target.lock();
        // do something...
    });
    // error[E0277]: `Rc<u32>` cannot be sent between threads safely
    //    --> src/experiments/rc_in_mutex.rs:13:27
    //     |
    // 13  |       let t = thread::spawn(move || {
    //     |  _____________-------------_^
    //     | |             |
    //     | |             required by a bound introduced by this call
    // 14  | |         target.lock();
    // 15  | |         // do something...
    // 16  | |     });
    //     | |_____^ `Rc<u32>` cannot be sent between threads safely
    //     |
    //     = help: within `A`, the trait `Send` is not implemented for `Rc<u32>`
    // note: required because it appears within the type `A`
    //    --> src/experiments/rc_in_mutex.rs:5:8
    //     |
    // 5   | struct A {
    //     |        ^
    //     = note: required for `Mutex<A>` to implement `Sync`
    //     = note: required for `Arc<Mutex<A>>` to implement `Send`
    // note: required because it's used within this closure
    //    --> src/experiments/rc_in_mutex.rs:13:27
    //     |
    // 13  |     let t = thread::spawn(move || {
    //     |                           ^^^^^^^
    // note: required by a bound in `spawn`
    //    --> /home/mhnap/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/mod.rs:683:8
    //     |
    // 680 | pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    //     |        ----- required by a bound in this function
    // ...
    // 683 |     F: Send + 'static,
    //     |        ^^^^ required by this bound in `spawn`

    t.join().unwrap();
}
