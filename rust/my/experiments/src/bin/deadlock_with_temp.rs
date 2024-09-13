// https://blog.m-ou.se/super-let
// https://fasterthanli.me/articles/a-rust-match-made-in-hell
// https://github.com/rust-lang/rust/issues/93883
// https://rust-lang.github.io/rust-clippy/master/#/significant_drop_in_scrutinee

#![deny(clippy::significant_drop_in_scrutinee)]

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

fn main() {
    let range_with_mutex = std::sync::Mutex::new(Range { start: 1, end: 2 });

    // a) no deadlock.
    {
        let range_locked = range_with_mutex.lock().unwrap();
        let range = Range { start: range_locked.start, end: range_locked.end };
        dbg!(range);
    }

    // b) no deadlock.
    {
        let start = range_with_mutex.lock().unwrap().start;
        let end = range_with_mutex.lock().unwrap().end;
        let range = Range { start, end };
        dbg!(range);
    }

    // c) similar to b), but here is deadlock.
    // This is because temporary values lifetimes were extended.
    {
        let range = Range {
            start: range_with_mutex.lock().unwrap().start,
            end: range_with_mutex.lock().unwrap().end,
        };
        dbg!(range);
    }

    // d) here is also deadlock.
    // more such cases can be found in the `drop_order.rs` file.
    {
        match range_with_mutex.lock().unwrap().start {
            0..=64 => range_with_mutex.lock().unwrap().end = 0,
            start => range_with_mutex.lock().unwrap().end = start,
        };

        // Can be detected by `significant_drop_in_scrutinee` lint.
        //     error: temporary with significant `Drop` in `match` scrutinee will live until the end of the `match` expression
        //     --> my/experiments/src/bin/deadlock_with_temp.rs:45:15
        //      |
        //   45 |         match range_with_mutex.lock().unwrap().start {
        //      |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //   46 |             0..=64 => range_with_mutex.lock().unwrap().end = 0,
        //      |                       -------------------------------- another value with significant `Drop` created here
        //   47 |             start => range_with_mutex.lock().unwrap().end = start,
        //      |                      -------------------------------- another value with significant `Drop` created here
        //   48 |         };
        //      |          - temporary lives until here
        //      |
        //      = note: this might lead to deadlocks or other unexpected behavior
        //      = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#significant_drop_in_scrutinee
        //   note: the lint level is defined here
        //     --> my/experiments/src/bin/deadlock_with_temp.rs:3:9
        //      |
        //   3  | #![deny(clippy::significant_drop_in_scrutinee)]
        //      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //   help: try moving the temporary above the match
        //      |
        //   45 ~         let value = range_with_mutex.lock().unwrap().start;
        //   46 ~         match value {
        //      |
    }
}
