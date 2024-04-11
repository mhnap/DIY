#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

fn main() {
    let range_with_mutex = std::sync::Mutex::new(Range { start: 1, end: 2 });

    // a) no deadlock
    {
        let range_locked = range_with_mutex.lock().unwrap();
        let range = Range {
            start: range_locked.start,
            end: range_locked.end,
        };
        dbg!(range);
    }

    // b) no deadlock
    {
        let start = range_with_mutex.lock().unwrap().start;
        let end = range_with_mutex.lock().unwrap().end;
        let range = Range { start, end };
        dbg!(range);
    }

    // c) similar to b), but here is deadlock
    // This is because temporary values lifetimes were extended.
    // More here: https://blog.m-ou.se/super-let
    {
        let range = Range {
            start: range_with_mutex.lock().unwrap().start,
            end: range_with_mutex.lock().unwrap().end,
        };
        dbg!(range);
    }
}
