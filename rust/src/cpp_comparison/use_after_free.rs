// https://jasonmccampbell.medium.com/eda-needs-to-be-using-rust-a6a09911da74

use std::env;

fn main() {
    let str: &String;
    let args: Vec<String> = env::args().collect();
    let arg0 = args[0].clone();
    if args.len() == 1 {
        str = &args[0];
    } else {
        let local_var = arg0 + &args[1];
        str = &local_var;
    }
    println!("Result = {str}");
}

// error[E0597]: `local_var` does not live long enough
//   --> src/cpp_comparison/use_after_free.rs:13:15
//    |
// 13 |         str = &local_var;
//    |               ^^^^^^^^^^ borrowed value does not live long enough
// 14 |     }
//    |     - `local_var` dropped here while still borrowed
// 15 |     println!("Result = {str}");
//    |                         --- borrow later used here
