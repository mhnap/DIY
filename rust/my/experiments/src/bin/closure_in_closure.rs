// https://stackoverflow.com/questions/76260428/the-error-about-async-block-outlive-the-current-function
// https://users.rust-lang.org/t/closure-may-outlive-the-current-function/79282

fn main() {
    // Just regular closures.
    let _closure1 = |s: String| {
        let s1 = s;
        println!("{s1}");
    };
    let _closure2 = |s: String, b: bool| {
        let s1 = s;
        let b1 = b;
        println!("{s1} {b1}");
    };

    // Closures with async block.
    // Async blocks are closer to closures than they are to "normal" blocks,
    // and will similarly perform the "loosest" capture they need
    // (they will capture by reference if all operations inside work on references).
    // Similarly to closures, async blocks are not executed immediately and may capture closed-over data by reference.
    let _closure1 = |s: String| async {
        let s1 = s;
        println!("{s1}");
    };
    // let _closure2 = |s: String, b: bool| async {
    //     let s1 = s;
    //     let b1 = b;
    //     println!("{s1} {b1}");
    // };
    //
    // error[E0373]: async block may outlive the current function, but it borrows `b`, which is owned by the current function
    //   --> my/experiments/src/bin/closure_with_copy.rs:24:42
    //    |
    // 24 |     let _closure2 = |s: String, b: bool| async {
    //    |                                          ^^^^^ may outlive borrowed value `b`
    // 25 |         let s1 = s;
    // 26 |         let b1 = b;
    //    |                  - `b` is borrowed here
    //    |
    // note: async block is returned here
    //   --> my/experiments/src/bin/closure_with_copy.rs:24:42
    //    |
    // 24 |       let _closure2 = |s: String, b: bool| async {
    //    |  __________________________________________^
    // 25 | |         let s1 = s;
    // 26 | |         let b1 = b;
    // 27 | |         println!("{s1} {b1}");
    // 28 | |     };
    //    | |_____^
    // help: to force the async block to take ownership of `b` (and any other referenced variables), use the `move` keyword
    //    |
    // 24 |     let _closure2 = |s: String, b: bool| async move {
    //    |                                                ++++

    // The same behavior can be observed for closure inside another closure.
    let _closure1 = |s: String| {
        || {
            let s1 = s;
            println!("{s1}");
        }
    };
    // let _closure2 = |s: String, b: bool| {
    //     || {
    //         let s1 = s;
    //         let b1 = b;
    //         println!("{s1} {b1}");
    //     }
    // };
    //
    // error[E0373]: closure may outlive the current function, but it borrows `b`, which is owned by the current function
    //   --> my/experiments/src/bin/closure_with_copy.rs:61:9
    //    |
    // 61 |         || {
    //    |         ^^ may outlive borrowed value `b`
    // 62 |             let s1 = s;
    // 63 |             let b1 = b;
    //    |                      - `b` is borrowed here
    //    |
    // note: closure is returned here
    //   --> my/experiments/src/bin/closure_with_copy.rs:61:9
    //    |
    // 61 | /         || {
    // 62 | |             let s1 = s;
    // 63 | |             let b1 = b;
    // 64 | |             println!("{s1} {b1}");
    // 65 | |         }
    //    | |_________^
    // help: to force the closure to take ownership of `b` (and any other referenced variables), use the `move` keyword
    //    |
    // 61 |         move || {
    //    |         ++++
}
