use std::thread;
use std::thread_local;

thread_local! {
    static ID: std::cell::Cell<u32> = std::cell::Cell::new(0);
}

fn main() {
    thread::scope(|s| {
        for i in 0..4 {
            s.spawn(move || {
                ID.with(|id| id.set(i));
                ID.with(|id| println!("Thread {} has ID {}", i, id.get()));
            });
        }
    });
}
