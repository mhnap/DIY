// https://medium.com/codex/eda-needs-to-be-using-rust-pt-2-59d2263ebb03

use std::thread;

fn main() {
    let mut msg = "Hello".to_string();
    let t = thread::spawn(|| {
        println!("{}", &msg);
    });
    msg.push_str(", world!");
    t.join().unwrap();
}
