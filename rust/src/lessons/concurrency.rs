// https://doc.rust-lang.org/book/ch16-00-concurrency.html

// Initially, the Rust team thought that ensuring memory safety and preventing concurrency problems were two separate challenges to be solved with different methods.
// Over time, the team discovered that the ownership and type systems are a powerful set of tools to help manage memory safety and concurrency problems!
// By leveraging ownership and type checking, many concurrency errors are compile-time errors in Rust rather than runtime errors.
// Therefore, rather than making you spend lots of time trying to reproduce the exact circumstances under which a runtime concurrency bug occurs, incorrect code will refuse to compile and present an error explaining the problem.
// As a result, you can fix your code while you’re working on it rather than potentially after it has been shipped to production.
// We’ve nicknamed this aspect of Rust fearless concurrency.
// Fearless concurrency allows you to write code that is free of subtle bugs and is easy to refactor without introducing new bugs.

use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // https://doc.rust-lang.org/book/ch16-01-threads.html

    // The Rust standard library uses a 1:1 model of thread implementation, whereby a program uses one operating system thread per one language thread.
    // To create a new thread, we call the thread::spawn function and pass it a closure containing the code we want to run in the new thread.
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Note that when the main thread of a Rust program completes, all spawned threads are shut down, whether or not they have finished running.
    // The calls to thread::sleep force a thread to stop its execution for a short duration, allowing a different thread to run.
    // The threads will probably take turns, but that isn’t guaranteed: it depends on how your operating system schedules the threads.

    //

    // The code above not only stops the spawned thread prematurely most of the time due to the main thread ending, but because there is no guarantee on the order in which threads run, we also can’t guarantee that the spawned thread will get to run at all!
    // We can fix the problem of the spawned thread not running or ending prematurely by saving the return value of thread::spawn in a variable.
    // The return type of thread::spawn is JoinHandle.
    // A JoinHandle is an owned value that, when we call the join method on it, will wait for its thread to finish.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hello number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hello number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates.
    // Blocking a thread means that thread is prevented from performing work or exiting.
    // The two threads continue alternating, but the main thread waits because of the call to handle.join() and does not end until the spawned thread is finished.

    // But let’s see what happens when we instead move handle.join() before the for loop in main.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hello number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hello number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // The main thread will wait for the spawned thread to finish and then run its for loop, so the output won’t be interleaved anymore.
    // Small details, such as where join is called, can affect whether or not your threads run at the same time.

    //

    // We'll often use the move keyword with closures passed to thread::spawn because the closure will then take ownership of the values it uses from the environment, thus transferring ownership of those values from one thread to another.
    // To use data from the main thread in the spawned thread, the spawned thread’s closure must capture the values it needs.
    let v = vec![1, 2, 3];

    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);
    // });
    // error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
    //   --> src/lessons/concurrency.rs:84:32
    //    |
    // 84 |     let handle = thread::spawn(|| {
    //    |                                ^^ may outlive borrowed value `v`
    // 85 |         println!("Here's a vector: {:?}", v);
    //    |                                           - `v` is borrowed here
    //    |
    // note: function requires argument type to outlive `'static`
    //   --> src/lessons/concurrency.rs:84:18
    //    |
    // 84 |       let handle = thread::spawn(|| {
    //    |  __________________^
    // 85 | |         println!("Here's a vector: {:?}", v);
    // 86 | |     });
    //    | |______^
    // help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
    //    |
    // 84 |     let handle = thread::spawn(move || {
    //    |                                ++++

    // Rust infers how to capture v, and because println! only needs a reference to v, the closure tries to borrow v.
    // However, there’s a problem: Rust can’t tell how long the spawned thread will run, so it doesn’t know if the reference to v will always be valid.

    // By adding the move keyword before the closure, we force the closure to take ownership of the values it’s using rather than allowing Rust to infer that it should borrow the values.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v);
    // error[E0382]: use of moved value: `v`
    //    --> src/lessons/concurrency.rs:116:10
    //     |
    // 82  |     let v = vec![1, 2, 3];
    //     |         - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
    // ...
    // 112 |     let handle = thread::spawn(move || {
    //     |                                ------- value moved into closure here
    // 113 |         println!("Here's a vector: {:?}", v);
    //     |                                           - variable moved due to use in closure
    // ...
    // 116 |     drop(v);
    //     |          ^ value used here after move

    handle.join().unwrap();

    // By telling Rust to move ownership of v to the spawned thread, we’re guaranteeing Rust that the main thread won’t use v anymore.
    // The move keyword overrides Rust’s conservative default of borrowing; it doesn’t let us violate the ownership rules.

    //

    // https://doc.rust-lang.org/book/ch16-02-message-passing.html

    // One increasingly popular approach to ensuring safe concurrency is message passing, where threads or actors communicate by sending each other messages containing data.
    // Here’s the idea in a slogan from the Go language documentation: “Do not communicate by sharing memory; instead, share memory by communicating.”

    // To accomplish message-sending concurrency, Rust's standard library provides an implementation of channels.
    // A channel is a general programming concept by which data is sent from one thread to another.
    // A channel has two halves: a transmitter and a receiver.
    // One part of your code calls methods on the transmitter with the data you want to send, and another part checks the receiving end for arriving messages.
    // A channel is said to be closed if either the transmitter or receiver half is dropped.

    // Here, we’ll work up to a program that has one thread to generate values and send them down a channel, and another thread that will receive the values and print them out.

    // We create a new channel using the mpsc::channel function; mpsc stands for multiple producer, single consumer.
    // In short, the way Rust’s standard library implements channels means a channel can have multiple sending ends that produce values but only one receiving end that consumes those values.

    // The mpsc::channel function returns a tuple, the first element of which is the sending end--the transmitter--and the second element is the receiving end--the receiver.
    // The abbreviations tx and rx are traditionally used in many fields for transmitter and receiver respectively, so we name our variables as such to indicate each end.
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // The spawned thread needs to own the transmitter to be able to send messages through the channel.
    // The transmitter has a send method that takes the value we want to send.
    // The send method returns a Result<T, E> type, so if the receiver has already been dropped and there’s nowhere to send a value, the send operation will return an error.

    // The receiver has two useful methods: recv and try_recv.
    // We’re using recv, short for receive, which will block the main thread’s execution and wait until a value is sent down the channel.
    // Once a value is sent, recv will return it in a Result<T, E>.
    // When the transmitter closes, recv will return an error to signal that no more values will be coming.

    // The try_recv method doesn’t block, but will instead return a Result<T, E> immediately: an Ok value holding a message if one is available and an Err value if there aren’t any messages this time.
    // Using try_recv is useful if this thread has other work to do while waiting for messages: we could write a loop that calls try_recv every so often, handles a message if one is available, and otherwise does other work for a little while until checking again.

    //

    // We’ve made some modifications that will prove the code is running concurrently: the spawned thread will now send multiple messages and pause for a second between each message.
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // In the main thread, we’re not calling the recv function explicitly anymore: instead, we’re treating rx as an iterator.
    // For each value received, we’re printing it.
    // When the channel is closed, iteration will end.

    // Because we don’t have any code that pauses or delays in the for loop in the main thread, we can tell that the main thread is waiting to receive values from the spawned thread.

    //

    // Earlier we mentioned that mpsc was an acronym for multiple producer, single consumer.
    // Let’s put mpsc to use and expand the code to create multiple threads that all send values to the same receiver.
    // We can do so by cloning the transmitter.
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // This time, before we create the first spawned thread, we call clone on the transmitter.
    // This will give us a new transmitter we can pass to the first spawned thread.
    // We pass the original transmitter to a second spawned thread.
    // This gives us two threads, each sending different messages to the one receiver.

    //

    // https://doc.rust-lang.org/book/ch16-03-shared-state.html

    // Message passing is a fine way of handling concurrency, but it’s not the only one.
    // Another method would be for multiple threads to access the same shared data.
    // Consider this part of the slogan from the Go language documentation again: “do not communicate by sharing memory.”

    // In a way, channels in any programming language are similar to single ownership, because once you transfer a value down a channel, you should no longer use that value.
    // Shared memory concurrency is like multiple ownership: multiple threads can access the same memory location at the same time.

    //

    // Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access some data at any given time.
    // To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s lock.
    // The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data.
    // Therefore, the mutex is described as guarding the data it holds via the locking system.

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    // To access the data inside the mutex, we use the lock method to acquire the lock.
    // This call will block the current thread so it can’t do any work until it’s our turn to have the lock.

    // The call to lock would fail if another thread holding the lock panicked.
    // In that case, no one would ever be able to get the lock, so we’ve chosen to unwrap and have this thread panic if we’re in that situation.

    // The type of m is Mutex<i32>, not i32, so we must call lock to be able to use the i32 value.
    // We can’t forget; the type system won’t let us access the inner i32 otherwise.

    //

    // Now, let’s try to share a value between multiple threads using Mutex<T>.
    // let counter = Mutex::new(0);
    // let mut handles = vec![];
    // for _ in 0..10 {
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    // error[E0382]: use of moved value: `counter`
    //    --> src/lessons/concurrency.rs:295:36
    //     |
    // 291 |     let counter = Mutex::new(0);
    //     |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
    // ...
    // 295 |         let handle = thread::spawn(move || {
    //     |                                    ^^^^^^^ value moved into closure here, in previous iteration of loop
    // 296 |             let mut num = counter.lock().unwrap();
    //     |                           ------- use occurs due to use in closure

    // The error message states that the counter value was moved in the previous iteration of the loop.
    // Rust is telling us that we can’t move the ownership of lock counter into multiple threads.
    // Let’s fix the compiler error with a multiple-ownership method

    //

    // We’ll wrap the Mutex<T> in Rc<T> and clone the Rc<T> before moving ownership to the thread.
    // let counter = Rc::new(Mutex::new(0));
    // let mut handles = vec![];
    // for _ in 0..10 {
    //     let counter = Rc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    // error[E0277]: `Rc<Mutex<i32>>` cannot be sent between threads safely
    //    --> src/lessons/concurrency.rs:326:36
    //     |
    // 326 |           let handle = thread::spawn(move || {
    //     |                        ------------- ^------
    //     |                        |             |
    //     |  ______________________|_____________within this `[closure@src/lessons/concurrency.rs:326:36: 326:43]`
    //     | |                      |
    //     | |                      required by a bound introduced by this call
    // 327 | |             let mut num = counter.lock().unwrap();
    // 328 | |
    // 329 | |             *num += 1;
    // 330 | |         });
    //     | |_________^ `Rc<Mutex<i32>>` cannot be sent between threads safely
    //     |
    //     = help: within `[closure@src/lessons/concurrency.rs:326:36: 326:43]`, the trait `Send` is not implemented for `Rc<Mutex<i32>>`
    // note: required because it's used within this closure
    //    --> src/lessons/concurrency.rs:326:36
    //     |
    // 326 |         let handle = thread::spawn(move || {
    //     |                                    ^^^^^^^
    // note: required by a bound in `spawn`
    //    --> /home/mhnap/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/mod.rs:683:8
    //     |
    // 680 | pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    //     |        ----- required by a bound in this function
    // ...
    // 683 |     F: Send + 'static,
    //     |        ^^^^ required by this bound in `spawn`

    // Here’s the important part to focus on: `Rc<Mutex<i32>>` cannot be sent between threads safely.
    // The compiler is also telling us the reason why: the trait `Send` is not implemented for `Rc<Mutex<i32>>`.

    // Unfortunately, Rc<T> is not safe to share across threads.
    // When Rc<T> manages the reference count, it adds to the count for each call to clone and subtracts from the count when each clone is dropped.
    // But it doesn’t use any concurrency primitives to make sure that changes to the count can’t be interrupted by another thread.
    // This could lead to wrong counts—subtle bugs that could in turn lead to memory leaks or a value being dropped before we’re done with it.
    // What we need is a type exactly like Rc<T> but one that makes changes to the reference count in a thread-safe way.

    //

    // Fortunately, Arc<T> is a type like Rc<T> that is safe to use in concurrent situations.
    // The a stands for atomic, meaning it’s an atomically reference counted type.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // You might have noticed that counter is immutable but we could get a mutable reference to the value inside it; this means Mutex<T> provides interior mutability, as the Cell family does.
    // In the same way we used RefCell<T> to allow us to mutate contents inside an Rc<T>, we use Mutex<T> to mutate contents inside an Arc<T>.

    //

    // https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html

    // Interestingly, the Rust language has very few concurrency features.
    // Almost every concurrency feature we’ve talked about so far in this chapter has been part of the standard library, not the language.
    // Your options for handling concurrency are not limited to the language or the standard library; you can write your own concurrency features or use those written by others.
    // However, two concurrency concepts are embedded in the language: the std::marker traits Sync and Send.

    // The Send marker trait indicates that ownership of values of the type implementing Send can be transferred between threads.
    // Any type composed entirely of Send types is automatically marked as Send as well.
    // Almost all primitive types are Send, aside from raw pointers.

    // The Sync marker trait indicates that it is safe for the type implementing Sync to be referenced from multiple threads.
    // In other words, any type T is Sync if &T (an immutable reference to T) is Send, meaning the reference can be sent safely to another thread.
    // Similar to Send, primitive types are Sync, and types composed entirely of types that are Sync are also Sync.

    // Because types that are made up of Send and Sync traits are automatically also Send and Sync, we don’t have to implement those traits manually.
    // As marker traits, they don’t even have any methods to implement.
    // They’re just useful for enforcing invariants related to concurrency.
    // Manually implementing these traits involves implementing unsafe Rust code.
    // Building new concurrent types not made up of Send and Sync parts requires careful thought to uphold the safety guarantees.
}
