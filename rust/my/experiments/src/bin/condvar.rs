use std::{
    sync::{Arc, Condvar, Mutex},
    thread::{self, sleep},
    time::Duration,
};

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    // Inside of our lock, spawn a new thread, and then wait for it to start.
    let handle = thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        println!("1");
        let mut started = lock.lock().unwrap();
        println!("2");
        *started = true;
        // We notify the condvar that the value has changed.
        cvar.notify_one();
        println!("3");
        sleep(Duration::from_secs(2));
        // The mutex guard will be released at the end of the scope,
        // which will unlock the mutex,
        // allowing the condition variable wait to complete.
    });

    // Wait for the thread to start up.
    let (lock, cvar) = &*pair;
    let started = lock.lock().unwrap();
    println!("0");
    let _guard = cvar.wait_while(started, |started| !*started).unwrap();
    println!("4");

    handle.join().unwrap();
}
