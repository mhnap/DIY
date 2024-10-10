// https://medium.com/codex/eda-needs-to-be-using-rust-pt-2-59d2263ebb03

use std::thread;

fn main() {
    // let mut msg = "Hello".to_string();
    // let t = thread::spawn(|| {
    //     println!("{}", &msg);
    // });
    // msg.push_str(", world!");
    // t.join().unwrap();
    //
    //     error[E0373]: closure may outlive the current function, but it borrows `msg`, which is owned by the current function
    //     --> my/cpp_comparison/src/bin/data_race.rs:7:27
    //      |
    //    7 |     let t = thread::spawn(|| {
    //      |                           ^^ may outlive borrowed value `msg`
    //    8 |         println!("{}", &msg);
    //      |                         --- `msg` is borrowed here
    //      |
    //    note: function requires argument type to outlive `'static`
    //     --> my/cpp_comparison/src/bin/data_race.rs:7:13
    //      |
    //    7 |       let t = thread::spawn(|| {
    //      |  _____________^
    //    8 | |         println!("{}", &msg);
    //    9 | |     });
    //      | |______^
    //    help: to force the closure to take ownership of `msg` (and any other referenced variables), use the `move` keyword
    //      |
    //    7 |     let t = thread::spawn(move || {
    //      |                           ++++
}
